# ECR-004 Implementation Clarifications

## IC-001 — Read-only canonical EvidenceRef accessors

### Discovery

ECR-004 decision-grade verification must inspect the canonical ECR-001 evidence binding already present in `EvidenceRef`: optional artifact, observation, receipt, external reference, content digest, and as-of time.

Live ECR-001 currently keeps those fields private and exposes public accessors only for `id()` and `kind()`. ECR-004 must not recover the hidden fields by serializing/parsing JSON, duplicating the wire type, or introducing a second evidence model.

### Canonical resolution

ECR-004 implementation MAY add minimal read-only accessors to `ecra_core::EvidenceRef` for the already-existing fields required by verification:

```text
artifact() -> Option<ArtifactId>
observation() -> Option<ObservationId>
receipt() -> Option<ReceiptId>
external_ref() -> Option<&str>
content_digest() -> Option<&ContentDigest>
as_of() -> Option<EpochMillis>
```

Exact borrowing/copy signatures may follow existing Rust conventions, but the semantic surface is read-only.

### Prohibited changes

IC-001 does NOT authorize:

- adding/removing/renaming `EvidenceRef` fields;
- changing ECR-001 JSON/wire shape or canonical bytes;
- changing validation semantics;
- changing provenance/freshness/dispute ownership;
- adding a verification flag to `EvidenceRef`, `Fact`, `ActionReceipt`, or other ECR-001 types;
- parsing serialized ECR-001 JSON inside ECR-004 to bypass the typed API;
- adding authority/policy semantics.

### Evidence requirement

The accessor commit must run the full ECR-001 regression suite and include tests proving serialization/canonical semantics are unchanged. It is a prerequisite to ECR-004 decision-grade evidence implementation.

### Task ordering

Execute IC-001 after the ECR-004 workspace/CI foundation is green and before T012. `tasks.md` owns it as T011A.

### Constitution impact

G1/G3/G5 improve: ECR-004 consumes the canonical ECR-001 evidence type directly instead of creating a competing representation. G2 is unchanged because accessors carry no authority. No other gate changes.

## IC-002 — Reconciliation evidence does not clear ECR-002 unresolved run state

### Discovery

Post-planning review found a second MUST-level boundary mismatch. ECR-002 v1 deliberately leaves a prepared-without-receipt attempt in `unresolved_attempts` and blocks resume/completion/blind retry while that state remains. In the canonical ECR-002 reducer, only a real `ReceiptRecorded` for the exact prepared attempt removes that unresolved marker. `ReconciliationRequested` is a durable hook only; it does not resolve or clear the attempt.

The original ECR-004 planning language correctly prohibited fabricating `ActionReceipt` and mutating `RunEvent`, but some retry-disposition wording could still be read as if `no_effect_confirmed` made the same ECR-002 run directly retryable. That would contradict the closed ECR-002 v1 state machine.

### Canonical resolution

ECR-004 v1 reconciliation establishes independent effect truth only:

```text
effect_confirmed
no_effect_confirmed
still_unknown
```

A `ReconciliationRecordV1` is append-only ECR-004 evidence. It MUST NOT:

- remove an attempt from ECR-002 `unresolved_attempts`;
- mark an ECR-002 `PreparedAttemptState` resolved;
- synthesize or append `ReceiptRecorded`;
- append a new ECR-002 event variant;
- resume or complete the same ECR-002 run;
- schedule or execute a retry.

`RetryDispositionV1` remains a non-authoritative advisory about whether a future owning runtime/policy path may consider proposing a **new attempt**. `semantically_retryable` and `semantically_retryable_same_key` never mean the existing ECR-002 run is resumable or that execution is permitted.

### Same-run behavior

For ECR-002 v1, an unresolved prepared attempt remains an execution-state blocker even after ECR-004 records `no_effect_confirmed` or `effect_confirmed`. ECR-004 consumers may present the reconciliation evidence to a future explicitly versioned run-repair/resolution protocol, but ECR-004 v1 does not define or counterfeit that protocol.

### Why ECR-004 does not patch ECR-002 v1

ECR-002 is `CLOSED_CANONICAL`; its strict run-event/reducer/archive contract is dependency truth. Adding a resolution event or changing reducer semantics inside ECR-004 would silently reopen that contract and require explicit versioned ECR-002 evolution, migration and regression ownership. That is outside ECR-004 v1.

### Evidence requirement

Phase 5 tests must prove:

- all three reconciliation outcomes leave the supplied ECR-002 `RunState` byte/semantic state unchanged;
- `unresolved_attempts` remains unchanged;
- no ECR-002 event/receipt is constructed as a reconciliation side effect;
- same-run `RunResumed`/`ExecutionCompleted` and blind-retry guards remain blocked by the unresolved attempt;
- retry disposition is advisory for a future new-attempt proposal only.

### Task ordering

IC-002 is owned by Phase 5 tasks T027–T029 and by the final ECR-002 regression/architecture gates. No implementation may weaken ECR-002 guards to make reconciliation appear operationally resolved.

### Constitution impact

G1/G4/G5/G6 improve: execution truth, verification truth and reconciliation evidence remain separate; UNKNOWN/run durability semantics are not rewritten. G2 remains unchanged because no execution authorization is introduced.

## IC-003 — Empty support is permitted only for evidence-absent `still_unknown`

### Discovery

The canonical data model described `ReconciliationRecordV1.verification_receipts` as unconditionally non-empty, while T026 and FR-030 require reconciliation to return `still_unknown` when supporting verification evidence is absent, insufficient or conflicting. Requiring a fabricated support identifier to represent evidence absence would violate FR-032, FR-044 and the no-fabrication boundary.

### Canonical resolution

`verification_receipts` remains bounded and normally non-empty. The only permitted empty list is:

```text
outcome == still_unknown
AND
no supporting canonical VerificationReceipt exists for the exact reconciliation target
```

For all other cases:

- `effect_confirmed` MUST reference at least one resolved supporting canonical `VerificationReceipt`;
- `no_effect_confirmed` MUST reference at least one resolved supporting canonical `VerificationReceipt` that explicitly supports no-effect truth;
- `still_unknown` with insufficient or conflicting supplied receipts retains those receipt IDs rather than discarding them;
- absence of a provider `ActionReceipt`, elapsed time, notes, or an empty support list MUST NOT become `no_effect_confirmed`.

### Prohibited interpretation

IC-003 does NOT authorize:

- empty support for either conclusive reconciliation outcome;
- synthetic/fabricated verification IDs;
- treating evidence absence as evidence of no effect;
- bypassing exact run/attempt/action binding;
- mutating ECR-002 run state or fabricating provider receipts.

### Evidence requirement

Phase 5 tests must cover:

- evidence-absent `still_unknown` with an empty support list;
- insufficient/conflicting `still_unknown` retaining all supplied support IDs;
- conclusive outcomes rejecting empty support;
- no-effect never inferred from missing provider receipt alone.

### Task ordering

IC-003 is a prerequisite clarification for T023–T026 and must converge with `data-model.md` before Phase 5 implementation is accepted.

### Constitution impact

G1/G4/G5/G6 improve: the representation can express genuine UNKNOWN without fabricating evidence while preserving strict conclusive-evidence requirements. G2 remains unchanged because no authority is introduced.
