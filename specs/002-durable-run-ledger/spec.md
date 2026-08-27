# Specification: Durable Run, Ledger & Budgets

**Feature:** ECR-002  
**Lifecycle target:** SPEC_READY → PLAN_READY → TASKS_READY  
**Depends on:** ECR-001 `CLOSED_CANONICAL`  
**Constitution:** v1.1.0  
**Scope class:** local-first, synthetic/non-sensitive durability substrate

## 1. Purpose

ECR-002 defines Ecra's durable execution truth above the zero-I/O ECR-001 domain kernel. It provides a serializable run state machine, durable action-attempt lifecycle, append-only integrity-chained local event ledger, bounded resource accounting, crash/restart recovery semantics, and a portable `.ecra` run artifact for synthetic/non-sensitive state.

This slice makes a run recoverable without treating a chat transcript, model statement, UI projection, executor receipt, or mutable database row as the source of execution truth.

ECR-002 does **not** authorize actions, authenticate principals, verify outcomes independently, persist real secrets/sensitive user payloads, execute browser/model/tool/process providers, or claim hostile tamper resistance.

## 2. Binding inherited invariants

ECR-002 MUST preserve ECR-001 semantics:

```text
Actor != authenticated Principal
CapabilityRequest != CapabilityGrant
InformationUse != authorization
ActionIntent != ActionAttemptRef
ActionReceipt != VerificationReceipt
executor-observed success != VERIFIED
UNKNOWN remains UNKNOWN
ContentDigest != ActionDigest != ledger-integrity digest
```

A run ledger records execution truth; it does not manufacture authority, identity, declassification, verification, or authenticity.

## 3. User stories

### US1 — Recover a durable run after interruption (P1)

As an Ecra runtime, I can reconstruct the same inspectable run state from durable ordered events after process interruption without relying on in-memory/chat state.

Acceptance:
- a run is reconstructable from its event ledger alone;
- replay is deterministic for the same accepted event sequence;
- malformed, reordered, missing, duplicated, or integrity-invalid events fail closed;
- a projection can be deleted and rebuilt without changing authoritative history.

### US2 — Never lose attempt identity around a consequential side effect (P1)

As an executor boundary, I must durably establish the exact `ActionAttemptRef` before invoking an effectful provider, and a crash after provider invocation but before receipt persistence must recover as unresolved/UNKNOWN rather than silently retrying or fabricating success/failure.

Acceptance:
- one `ActionRef` may have multiple distinct attempts;
- attempt preparation is durably committed before external invocation is permitted;
- receipt recording must bind the same exact attempt/action;
- unresolved prepared attempts are surfaced for reconciliation and block blind retry when ECR-001 retry semantics require reconciliation/never-blind behavior.

### US3 — Bound recursive/expensive execution (P1)

As a user/runtime, I can set explicit run budgets and observe deterministic usage/exhaustion so work suspends/fails safely instead of consuming unbounded time, calls, tokens, cost, processes, output, network, storage, or recursion depth.

Acceptance:
- budget dimensions are typed and I-JSON-safe;
- hard limits cannot be exceeded by arithmetic overflow or implicit widening;
- budget exhaustion creates durable run evidence and prevents further governed work;
- budget exhaustion never grants authority or converts UNKNOWN into retry permission.

### US4 — Inspect an append-only local execution history (P1)

As a user/debugger, I can inspect ordered run events, their integrity chain, attempt/receipt bindings, interventions, cancellation, and budget events while knowing projections are derived views.

Acceptance:
- authoritative event rows are append-only through the Ecra store API;
- event sequence is contiguous per run;
- every event after genesis binds the previous event digest;
- whole-store hostile rewrite is explicitly outside the hash-chain guarantee until ECR-031 supplies a protected anchor.

### US5 — Export/import a deterministic `.ecra` fixture artifact (P2)

As a developer/test harness, I can export a synthetic/non-sensitive run into a deterministic portable artifact and load it on another supported machine with identical semantic history and integrity results.

Acceptance:
- live SQLite/WAL files are never the interchange format;
- archive entry order/metadata/profile are deterministic;
- path traversal, symlinks, duplicate entries, unsupported compression, excessive entry count/size, malformed JSON, digest mismatch, and unsupported versions fail closed;
- repeated export of identical logical content is byte-identical.

### US6 — Pause, cancel, resume, and record intervention without inventing policy semantics (P2)

As a runtime, I can durably record pause/suspend, resume, cancellation request/completion, takeover/hand-back/intervention, and reconciliation-request events without treating them as authorization or verification.

Acceptance:
- state transitions are explicit and validated;
- terminal states cannot resume implicitly;
- cancellation is distinct from cancellation requested;
- intervention actor attribution does not authenticate a principal.

## 4. Functional requirements

### Versioning and event identity

- **FR-001** Every persisted/interchange ECR-002 value MUST carry an explicit supported schema version or live inside a versioned parent envelope with equivalent strict dispatch.
- **FR-002** Unknown major/newer minor persisted versions MUST fail closed before state mutation.
- **FR-003** Every authoritative run event MUST be identified by the tuple `(RunId, EventSequence)`; sequence values MUST be positive, contiguous, unique per run and within the I-JSON exact integer range.
- **FR-004** Event order MUST be derived from `EventSequence`, never timestamp, insertion rowid, filename enumeration, UI order, or model output.
- **FR-005** Event timestamps/durations MUST be caller/runtime supplied typed values; deterministic replay/reducer logic MUST NOT read the OS clock.

### Run state machine

- **FR-006** ECR-002 MUST define explicit run phases at least for `created`, `running`, `suspended`, `cancellation_requested`, `cancelled`, `failed`, and `execution_completed`.
- **FR-007** `execution_completed` MUST describe runtime execution completion only and MUST NOT imply `VERIFIED`.
- **FR-008** State transitions MUST be deterministic, explicit, and fail closed for impossible/out-of-order transitions.
- **FR-009** Terminal `cancelled`, `failed`, or `execution_completed` state MUST NOT resume without a future explicitly versioned migration/repair protocol; v1 rejects implicit reopening.
- **FR-010** Projections/current-state rows MUST be rebuildable from authoritative events and MUST NOT become independent run truth.

### Attempt lifecycle and UNKNOWN

- **FR-011** A consequential provider call MUST require a durably committed `AttemptPrepared` event containing the exact ECR-001 `ActionAttemptRef` before invocation is permitted.
- **FR-012** Attempt preparation MUST reject duplicate `ActionAttemptRef` identity and reject an attempt whose `ActionRef` conflicts with prior binding.
- **FR-013** `ReceiptRecorded` MUST contain an ECR-001 `ActionReceipt` that binds the same exact action/attempt established by `AttemptPrepared`.
- **FR-014** A prepared attempt with no durable receipt at a recovery boundary MUST remain unresolved and be surfaced as UNKNOWN/reconciliation-required evidence; recovery MUST NOT infer success/failure.
- **FR-015** ECR-002 MUST expose enough durable state for ECR-004 to reconcile an unresolved attempt later, without implementing ECR-004 verification/reconciliation policy itself.
- **FR-016** Retry/resume helpers MUST obey the ECR-001 retry class and MUST refuse blind retry for unknown/non-idempotent/reconciliation-required semantics.
- **FR-017** Multiple attempts for one action MUST remain distinct in history and projection.

### Append-only integrity ledger

- **FR-018** Authoritative run history MUST be append-only through the Ecra persistence API; mutation/deletion of existing event rows is unsupported and storage-level guardrails MUST reject ordinary update/delete attempts.
- **FR-019** Each event MUST carry a domain-separated SHA-256 ledger digest over canonical event material including run id, sequence, previous digest and event body; genesis has an explicit no-previous representation.
- **FR-020** The ledger digest MUST use its own strong type and MUST NOT be presented as a signature, MAC, authorization proof, VerificationReceipt, or generic ContentDigest.
- **FR-021** Chain verification MUST reject gaps, duplicate sequence, previous-digest mismatch, event-digest mismatch, run-id mismatch and unsupported event versions.
- **FR-022** The integrity claim MUST be scoped: it detects accidental/local mutation relative to the inspected chain; an attacker able to rewrite the whole store can recompute the chain unless a later ECR-031 protected anchor is present.
- **FR-023** Atomic append MUST use an expected-head check so competing writers cannot silently fork or overwrite a run history.

### Durable local storage

- **FR-024** The v1 local store MUST be SQLite through a narrowly configured Rust adapter, not raw ad-hoc file append logic.
- **FR-025** SQLite v1 MUST use WAL mode and `synchronous=FULL` for committed run events, with explicit verification that requested pragmas took effect.
- **FR-026** Event append/head-projection update MUST occur in one write transaction acquired eagerly (`BEGIN IMMEDIATE` semantics or equivalent reviewed rusqlite API).
- **FR-027** The store MUST reject unsupported newer schema/user versions and provide deterministic schema migration fixtures from the initial v1 baseline.
- **FR-028** WAL/database files are one live persistence unit; copying only the main SQLite database file while WAL state is live MUST NOT be documented or implemented as export.
- **FR-029** Large artifact bytes, when supported in this slice, MUST be content-addressed by the ECR-001 content digest and bounded by storage budgets; authoritative domain metadata remains `ArtifactRef` rather than a second artifact identity.
- **FR-030** ECR-002 acceptance/storage fixtures MUST be synthetic/non-sensitive. Real authenticated secrets, real sensitive workspace/browser payloads, or equivalent high-value state are blocked pending ECR-031/ECR-003/ECR-025 protection contracts.

### Budgets and accounting

- **FR-031** ECR-002 MUST define typed budget dimensions for at least active wall milliseconds, steps, tool calls, model calls, input tokens, output tokens, cost microunits, process count, process milliseconds, output bytes, network requests, network bytes, storage/artifact bytes, and recursion/delegation depth.
- **FR-032** All persisted budget amounts/counters MUST fit the I-JSON exact integer range and use checked arithmetic.
- **FR-033** A budget limit MUST have an explicit hard limit; optional soft limit MUST be `<= hard`.
- **FR-034** Duplicate dimensions, zero/invalid semantics where prohibited, negative values, overflow, and malformed units MUST fail closed.
- **FR-035** Usage MUST be recorded as durable typed events; reducer-computed cumulative usage is authoritative over mutable counters/projections.
- **FR-036** When a unit of work declares an upper bound, preflight MUST reject starting it if that bound exceeds remaining hard budget.
- **FR-037** When exact resource usage is only known after work, recording usage that reaches/exceeds a hard limit MUST durably suspend/terminate further governed work; it MUST NOT rewrite the already-observed external effect.
- **FR-038** Hard budget exhaustion MUST create durable evidence with dimension, limit, observed/charged usage and resulting run suspension/failure state.
- **FR-039** v1 budgets MUST NOT silently expand themselves. Any future budget revision must be an explicit versioned event with an authorization owner; ECR-002 v1 does not invent policy to approve increases.

### Cancellation, intervention, recovery

- **FR-040** Cancellation request and terminal cancellation MUST be distinct durable events/states.
- **FR-041** Pause/suspend and resume MUST be explicit durable transitions and preserve unresolved-attempt state.
- **FR-042** Human takeover/hand-back/intervention events MUST retain Actor attribution while remaining non-authoritative with respect to Principal/authentication/approval.
- **FR-043** Recovery MUST identify all prepared-without-receipt attempts before permitting continued action scheduling.
- **FR-044** A run with a reconciliation-required unresolved attempt MUST not schedule a blind retry of that attempt.
- **FR-045** Crash/restart recovery MUST be idempotent: replaying the same durable events does not append new events or alter state by itself.

### Portable `.ecra` artifact

- **FR-046** `.ecra` v1 MUST be a deterministic ZIP container independent of the live SQLite storage representation.
- **FR-047** The v1 writer MUST use stored/no-compression entries, deterministic normalized metadata, stable UTF-8 names, sorted entries, no encryption, no symlinks and no archive/file comments.
- **FR-048** The archive MUST contain a strict versioned manifest, canonical event files and optional content-addressed blobs; it MUST NOT contain SQLite database/WAL files.
- **FR-049** The reader MUST reject absolute paths, `..`, backslash ambiguity, duplicate names, symlinks, unsupported compression/encryption, unsupported versions, unexpected entries, entry-count/size-limit breaches and digest mismatch before materializing trusted output.
- **FR-050** Exporting identical logical run content with identical blobs MUST produce byte-identical `.ecra` bytes on supported platforms.
- **FR-051** Imported/replayed bundle events MUST pass the same strict event/schema/chain validation as live persisted events.
- **FR-052** ECR-002 v1 `.ecra` fixtures are synthetic/non-sensitive; import/export of real sensitive user state remains gated to later protection/portability work.

### Errors, dependencies and boundaries

- **FR-053** Storage, migration, chain, state-transition, budget, archive and recovery failures MUST have typed machine-readable error categories/codes; callers MUST NOT branch on display text.
- **FR-054** `ecra-core` MUST remain zero-I/O and MUST NOT gain SQLite/ZIP/runtime dependencies.
- **FR-055** New production dependencies MUST be bounded to the ECR-002 crate, exact-version locked, license/provenance reviewed and minimized by features.
- **FR-056** ECR-002 MUST perform no network access, remote telemetry, model calls, browser calls, protocol calls or external action execution as part of persistence/replay/export logic.
- **FR-057** ECR-002 MUST not define AuthenticationDecision, AuthorizationDecision, declassification, VerificationReceipt aggregation, provider execution, secret storage protection, or hostile-tamper trust roots; those remain owned by later slices.

## 5. Success criteria

- **SC-001** 100% of committed valid run/event/budget/store/archive fixtures parse, validate and strict-round-trip where round-trip is defined.
- **SC-002** 100% of committed invalid fixtures fail closed in the expected typed category/code.
- **SC-003** Replaying the same event sequence at least 1,000 times produces the same serialized derived RunState digest.
- **SC-004** Crash-injection fixtures at every attempt boundary prove: no attempt can execute before durable preparation; crash after preparation/before receipt recovers unresolved; crash after external effect/before receipt never fabricates/retries; crash after receipt commit recovers the same receipt.
- **SC-005** Concurrent append tests prove exactly one writer succeeds for a given expected head and no silent history fork occurs.
- **SC-006** Chain mutation/gap/reorder/duplicate/cross-run substitution tests are rejected deterministically.
- **SC-007** SQLite process-crash recovery retains committed events; WAL+FULL configuration is asserted in tests. Power-loss durability claims remain scoped to SQLite/VFS/storage assumptions.
- **SC-008** Deleting/rebuilding all projections produces state byte-equivalent to the pre-delete derived state for the same authoritative history.
- **SC-009** Budget property tests cover every dimension, exact boundary, soft boundary, hard exhaustion, overflow and I-JSON limits without wraparound.
- **SC-010** At least one recursive/tool-loop fixture is deterministically stopped by a hard budget with a durable exhaustion event.
- **SC-011** Identical logical input exports to byte-identical `.ecra` archives across repeated runs and supported CI platforms available to the project.
- **SC-012** Malicious archive fixtures for traversal, duplicate entry, unsupported method, excessive size/count, malformed manifest/event and digest mismatch all fail before trusted materialization.
- **SC-013** The implementation test suite performs no real remote/network/provider action and persists no real secret/sensitive fixture.
- **SC-014** `ecra-core` direct dependency/zero-I/O/zero-unsafe boundaries remain green after ECR-002 is added.
- **SC-015** Full workspace build/fmt/Clippy/test/rustdoc/offline/security/dependency/migration/crash/archive gates pass on the exact implementation head and post-merge canonical main.
- **SC-016** Analyze-equivalent traceability maps every FR/SC and constitution G1–G15 with zero unowned critical requirement before implementation starts and before closure.

## 6. Explicit non-goals

ECR-002 does not include:
- authenticated principal validation, trust roots, keys, MAC/signature anchors or protected sensitive storage (ECR-031);
- capability authorization, declassification, approval policy, secret mediation or disclosure policy (ECR-003);
- independent verifier orchestration or actual external-state reconciliation (ECR-004);
- distributed consensus/multi-node workflow service, cloud durability or multi-device sync;
- browser/model/tool/process execution providers;
- arbitrary user scripting/workflow language;
- production persistence of real secrets/PHI/credentials/private browsing payloads;
- generic ZIP extraction API;
- claims that a plain SHA-256 chain is tamper-proof/tamper-resistant against a full-store rewriter.

## 7. Dependency boundary

ECR-002 may depend on the closed ECR-001 public API. It must not modify ECR-001 semantics merely to simplify persistence. If an actual incompatibility is discovered, record a convergence defect and amend the owning contract deliberately rather than creating parallel types.

ECR-031 and ECR-003 remain blockers for real sensitive/privileged persistence even if ECR-002 itself becomes `CLOSED_CANONICAL`.
