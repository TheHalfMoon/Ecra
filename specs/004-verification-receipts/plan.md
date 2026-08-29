# Plan: ECR-004 Verification & Reconciliation

**Planning branch:** `004-verification-receipts`  
**Implementation branch:** `004-verification-receipts-impl`  
**Canonical implementation base:** `4fb61f8b41267983fc460c666fddd7781d91653c`  
**Dependencies:** ECR-001/ECR-002 `CLOSED_CANONICAL`  
**Implementation state:** implemented through Phase 7; Phase 8 convergence/review/closure in progress

## 1. Technical objective

Add one auditable `ecra-verify` crate that:

1. validates strict verification requests over canonical ECR-001 targets/evidence;
2. emits canonical `VerificationReceipt` only;
3. aggregates immutable receipts deterministically and exposes conflict;
4. evaluates bounded critical verification checkpoints;
5. reconciles exact ECR-002 unresolved attempts as **independent effect evidence** without fabricating execution receipts or resolving the ECR-002 run state;
6. derives fail-closed retry safety dispositions for future new-attempt proposals without granting authority or scheduling execution;
7. persists synthetic/non-sensitive verification/checkpoint/reconciliation records in a separate append-only local journal;
8. remains offline, provider-neutral, bounded, and dependency-minimal.

## 2. Repository architecture

```text
crates/ecra-core
  canonical target/evidence/VerificationReceipt/action semantics
          ↓
crates/ecra-run
  durable RunState/attempt/recovery/retry truth
          ↓ read-only compatibility boundary
crates/ecra-verify
  ├─ error.rs          typed machine-readable ECR-004 errors
  ├─ ids.rs            CheckpointId/ReconciliationId
  ├─ request.rs        strict VerificationRequestV1 + receipt construction
  ├─ evidence.rs       decision-grade evidence checks
  ├─ aggregate.rs      deterministic receipt aggregation
  ├─ checkpoint.rs     bounded checkpoint definitions/evaluation
  ├─ reconcile.rs      exact effect reconciliation + advisory RetryDispositionV1
  ├─ journal.rs        versioned canonical journal entry/digest/replay
  └─ store.rs          local SQLite append-only journal + migrations
```

`ecra-verify` depends only on `ecra-core`, `ecra-run`, and the exact T001/T044-reviewed serialization/hash/SQLite set. It does not add browser, network, model, policy, provider, protocol, process-execution, identity-backend, telemetry, or UI dependencies.

`ecra-verify` does not depend on private/internal ECR-002 mutation APIs and exposes no bridge that appends ECR-002 run events/receipts. `RunState` is read-only verification input.

## 3. Implemented phases

### Phase A — crate/dependency/CI boundary

Workspace crate, `#![forbid(unsafe_code)]`, unsafe/dependency scripts, push-only ECR-004 workflow, permanent ECR-001/ECR-002 regressions and exact-head gate.

### Phase B — strict IDs/errors/request contract

Typed IDs, strict version/bounds/errors, `VerificationRequestV1`, canonical ECR-001 `VerificationReceipt` construction and negative contract fixtures.

### Phase C — decision-grade evidence and aggregation

Immutable-binding/freshness/self-attestation rules, deterministic aggregate states, conflict retention, order invariance, 1,000-repeat determinism and provenance non-mutation.

### Phase D — checkpoints

Strict bounded `VerificationCheckpointV1`, duplicate-target rejection, deterministic satisfied/unsatisfied/conflicted views and no authority semantics.

### Phase E — reconciliation and retry safety

Strict `ReconciliationRecordV1`, exact RunId/attempt/action binding, explicit effect/no-effect/still-unknown derivation, future-new-attempt-only retry disposition, no synthetic `ActionReceipt`, unchanged ECR-002 unresolved state and explicit same-run guard regressions.

### Phase F — append-only journal and persistence

Canonical `VerificationJournalEntryV1`, transactional SQLite v1 store, expected-head append concurrency, immutable canonical rows, rebuildable projections, corruption/migration/reopen/replay tests and integrity-only claims.

### Phase G — hostile input, docs, convergence

Exact resource ceilings, bounded arbitrary input, secret sentinels, portability, README claims/non-claims, complete quickstart exact-head gate, donor/license reconciliation, traceability, G1–G15 recheck and post-implementation analyze.

## 4. Persistence strategy

ECR-004 uses a sidecar SQLite database distinct from ECR-002 run storage. Canonical journal rows are immutable append-only verification truth; indexes are derived projections. The store exposes no generic SQL surface.

Implemented v1 schema:

```sql
PRAGMA user_version = 1;

CREATE TABLE verification_meta (
  singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
  schema_version INTEGER NOT NULL
);

CREATE TABLE verification_journal (
  sequence INTEGER PRIMARY KEY,
  entry_json TEXT NOT NULL,
  entry_digest TEXT NOT NULL UNIQUE
);

CREATE TABLE verification_receipt_index (
  verification_id TEXT PRIMARY KEY,
  sequence INTEGER NOT NULL,
  target_key TEXT NOT NULL
);

CREATE TABLE checkpoint_index (
  checkpoint_id TEXT PRIMARY KEY,
  sequence INTEGER NOT NULL
);

CREATE TABLE reconciliation_index (
  reconciliation_id TEXT PRIMARY KEY,
  run_id TEXT NOT NULL,
  attempt_id TEXT NOT NULL,
  sequence INTEGER NOT NULL
);
```

`verification_journal_no_update` and `verification_journal_no_delete` triggers reject ordinary canonical-row mutation. Projections are rebuildable from the journal. No sidecar row represents mutable ECR-002 run resolution.

## 5. Canonicalization/integrity

ECR-004 reuses repository-aligned strict versioned JSON + RFC 8785/JCS and SHA-256 semantics.

Canonical v1 journal digest:

```text
SHA-256("ecra/verification-journal/v1\0" || JCS(version, sequence, previous_digest, body))
```

The committed journal golden material/digest is normative. The chain detects corruption/substitution under stated local integrity assumptions; it is not a protected authenticity anchor and is not hostile complete-store tamper resistance.

## 6. Reconciliation algorithm

For an exact unresolved attempt:

1. validate the attempt exists in the supplied ECR-002 `RunState` and binds the exact `ActionRef`;
2. retain the relevant ECR-002 read-only state for postcondition comparison;
3. resolve supporting canonical verification receipts by ID;
4. reject missing, duplicate, irrelevant and cross-target support;
5. evaluate effect-presence/effect-absence evidence;
6. effect + no-effect conflict -> `still_unknown`;
7. effect conclusive -> `effect_confirmed`;
8. no-effect conclusive -> `no_effect_confirmed`;
9. otherwise -> `still_unknown`;
10. append the reconciliation record to ECR-004 persistence when requested by the caller;
11. derive retry disposition separately from ECR-001 semantics;
12. prove no ECR-002 event/receipt/state mutation occurred and the prior unresolved marker remains unchanged.

No step creates or modifies an ECR-002 `ActionReceipt`, run event, prepared-attempt state, unresolved set, phase, resume/completion state, or scheduler state.

## 7. Retry-disposition interpretation

`RetryDispositionV1` answers only whether reconciliation evidence plus ECR-001 retry/idempotency semantics immediately forbid a future **new-attempt proposal** for duplicate-effect reasons. It does not answer whether the existing ECR-002 run may resume, whether the unresolved prior attempt may be retried, whether execution may occur, or whether authorization exists.

Existing ECR-002 guards remain authoritative and fail closed.

## 8. Verification checkpoint algorithm

For each checkpoint requirement:

1. compute aggregate for the exact target;
2. compare the aggregate against allowed satisfying states;
3. collect satisfied, unsatisfied and conflicted targets deterministically;
4. satisfy the checkpoint only when every requirement is satisfied and none conflict.

`Absent`, `Inconclusive`, and `Conflicted` are not satisfying states. This remains a derived verification view and never changes ECR-002 `RunPhase`.

## 9. Testing strategy

### Unit/contract
- strict IDs/errors/version parsing;
- request -> canonical receipt exact binding;
- evidence quality rules;
- aggregate truth table;
- checkpoint evaluation;
- reconciliation/retry matrix;
- journal digest goldens.

### Property/adversarial
- receipt permutation invariance;
- 1,000 identical aggregate evaluations -> identical canonical output;
- target mutation/binding failures;
- exact maxima and max+1 resource errors;
- bounded arbitrary JSON input;
- conflict never becomes verified;
- absence of evidence never becomes no-effect proof.

### ECR-002 compatibility
- `RunState` remains byte/semantically unchanged;
- unresolved membership remains unchanged for all outcomes;
- no `RunEvent`/`ActionReceipt` construction bridge;
- `RunResumed`/`ExecutionCompleted` remain rejected where unresolved state blocks them;
- blind-retry guard remains rejected for the unresolved prior attempt;
- `semantically_retryable*` is advisory only.

### Persistence
- transactional initialization/migration;
- append/reopen/replay equivalence;
- expected-head concurrency exactly one winner;
- update/delete blocked;
- corrupt JSON/digest/sequence/index fail/rebuild behavior;
- projection rebuild equivalence;
- 4,096-entry materialization ceiling.

### Regression
- complete ECR-001 explicit contract/property targets;
- complete ECR-002 event/reducer/attempt/budget/migration/store/crash/archive/boundary targets;
- workspace/rustdoc/offline.

## 10. Permanent CI gate

`.github/workflows/ecr-004.yml` runs on the implementation branch and `main` and includes:

```text
cargo metadata --locked
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
explicit ECR-001 regressions
explicit ECR-002 regressions
all explicit ECR-004 quickstart targets
explicit ECR-002 unresolved-state compatibility acceptance
cargo test --doc --workspace --locked
cargo test --workspace --locked --offline
core/run/verify unsafe + dependency boundary scripts
cargo tree dependency evidence
```

Historical green is never reused after a content change as exact-head PASS.

## 11. Constitution G1–G15

Post-implementation T047 re-check: G1–G15 PASS or explicit PASS/N-A with zero exception. `traceability-closure.md` owns the detailed evidence mapping.

## 12. Complexity tracking

### Sidecar journal instead of extending ECR-002 v1 events

The additional local schema is justified because extending a `CLOSED_CANONICAL` strict run-event contract would conflate execution truth with verification truth. The sidecar intentionally cannot resolve the old run state.

### Advisory retry disposition instead of ECR-002 run repair

Reconciliation evidence and execution-state resolution remain separate because ECR-002 v1 has no versioned repair event. Silently clearing unresolved state from ECR-004 would counterfeit execution truth.

### Dedicated `ecra-verify` crate

The extra crate boundary keeps independent verification auditable and prevents executor/reducer semantics from becoming verification authority.

## 13. Implementation authorization and closure

The original planning authorization prerequisites were satisfied when PR #5 merged and exact canonical ECR-001/ECR-002 regressions succeeded on `4fb61f8b41267983fc460c666fddd7781d91653c`. Implementation then proceeded on `004-verification-receipts-impl` in task order.

ECR-004 is not `CLOSED_CANONICAL` until T050 final exact-head CI, T051 review closure, T052 exact-head non-rebase merge plus required canonical-main workflows, and T053 post-merge lifecycle convergence are complete.