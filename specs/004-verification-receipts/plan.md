# Plan: ECR-004 Verification & Reconciliation

**Branch:** `004-verification-receipts` (planning)  
**Implementation branch after planning convergence:** `004-verification-receipts-impl`  
**Base:** exact canonical `main` after planning PR merge and required regression gates  
**Dependencies:** ECR-001/ECR-002 `CLOSED_CANONICAL`

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

`ecra-verify` may depend on `ecra-core`, `ecra-run`, minimal serialization/hash/SQLite dependencies already accepted in the repository after exact implementation-time re-verification. It must not depend on browser, network, model, policy, provider, protocol, process-execution, identity-backend, or UI crates.

`ecra-verify` MUST NOT depend on private/internal ECR-002 mutation APIs or introduce a bridge that appends run events/receipts. `RunState` is verification input only.

## 3. Implementation phases

### Phase A — crate/dependency/CI boundary

- add workspace crate with `#![forbid(unsafe_code)]`;
- add dependency/unsafe boundary scripts;
- add push-only `.github/workflows/ecr-004.yml` for implementation branch and `main`;
- retain permanent ECR-001 and ECR-002 regression targets;
- exact-head green before semantic code.

### Phase B — strict IDs/errors/request contract

- typed ECR-004 IDs;
- strict version/bounds/errors;
- `VerificationRequestV1` validation;
- canonical ECR-001 `VerificationReceipt` construction with exact target/evidence/method/outcome binding;
- negative fixtures for unknown fields/unsupported versions/evidence omissions/duplicates.

### Phase C — decision-grade evidence and aggregation

- immutable-binding/freshness/self-attestation checks;
- deterministic aggregate state and conflict handling;
- property tests for order independence and 1,000-repeat determinism;
- no provenance mutation.

### Phase D — checkpoints

- strict `VerificationCheckpointV1` and requirements;
- duplicate-target rejection;
- deterministic evaluation with unsatisfied/conflicted target reporting;
- architecture tests proving no authority semantics.

### Phase E — reconciliation and retry safety

- strict `ReconciliationRecordV1`;
- exact `RunId`/attempt/action binding against ECR-002 `RunState`;
- conclusive-effect/no-effect/still-unknown validation;
- retry-disposition matrix over ECR-001 retry/idempotency classes;
- no synthetic `ActionReceipt`;
- ECR-002 state remains byte/semantically untouched for reconciliation purposes;
- explicit tests prove the prior attempt remains unresolved and same-run resume/completion/blind retry remain blocked after every reconciliation outcome;
- retry dispositions are advisory for a future new-attempt proposal only.

### Phase F — append-only journal and persistence

- canonical `VerificationJournalEntryV1` plus domain-separated digest chain;
- transactional SQLite v1 store and indexes;
- expected-head compare-and-append concurrency;
- corruption/sequence/digest/duplicate/migration/rebuild tests;
- explicit integrity-only claim.

### Phase G — hostile input, docs, convergence

- resource-bound/property/adversarial tests;
- secret/sensitive sentinel scans;
- README with exact claims/non-claims, including the unresolved-run boundary;
- quickstart/full exact-head gate;
- traceability, constitution recheck, post-implementation analyze;
- review/merge/post-merge evidence.

## 4. Persistence strategy

ECR-004 uses a sidecar SQLite database distinct from ECR-002 run storage. Canonical journal rows are immutable append-only verification truth; indexes are derived projections. The store exposes no generic SQL surface.

Suggested v1 schema:

```sql
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
  sequence INTEGER NOT NULL
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

Schema triggers/API checks should reject ordinary update/delete of canonical journal rows. All projections must be rebuildable from the journal.

No sidecar row is a mutable projection of ECR-002 run resolution. ECR-004 persistence cannot mark an ECR-002 attempt resolved.

## 5. Canonicalization/integrity

Use repository-aligned strict versioned JSON + RFC 8785/JCS and SHA-256 semantics rather than introducing another canonicalizer.

Journal digest:

```text
SHA-256("ecra/verification-journal-entry/v1\0" || JCS(version, sequence, previous_digest, body))
```

The chain detects normal corruption/substitution. It is not a protected authenticity anchor and must not be described as hostile-store tamper resistance.

## 6. Reconciliation algorithm

For an exact unresolved attempt:

1. validate the attempt exists in the supplied ECR-002 `RunState` and binds the exact `ActionIntent`/`ActionRef`;
2. snapshot/retain the relevant ECR-002 read-only attempt/run-state facts needed for postcondition testing;
3. load supporting canonical verification receipts by ID;
4. reject irrelevant/cross-target supporting receipts;
5. aggregate relevant effect-presence/effect-absence evidence under the declared reconciliation rule;
6. if both effect and no-effect are conclusive -> `still_unknown` / typed conflict;
7. if effect is conclusive -> `effect_confirmed`;
8. if no-effect is conclusive -> `no_effect_confirmed`;
9. otherwise -> `still_unknown`;
10. persist the reconciliation record append-only;
11. derive retry disposition separately from ECR-001 semantics, never execution authorization;
12. prove no ECR-002 event/receipt/state mutation occurred and the prior unresolved marker remains unchanged.

No step creates or modifies an ECR-002 `ActionReceipt`, run event, prepared-attempt state, unresolved set, phase, resume/completion state, or scheduler state.

## 7. Retry-disposition interpretation

`RetryDispositionV1` answers a narrow semantic question about a future new-attempt proposal:

```text
Would the current reconciliation evidence plus ECR-001 retry/idempotency semantics immediately forbid proposing a new attempt for duplicate-effect reasons?
```

It does **not** answer:

```text
May the existing ECR-002 run resume?
May the unresolved prior attempt be retried?
May execution occur?
Is authorization granted?
```

Those answers remain NO/not-owned in ECR-004 v1. Existing ECR-002 guards remain authoritative and fail closed.

## 8. Verification checkpoint algorithm

For each checkpoint requirement:

1. compute aggregate for the exact target;
2. compare aggregate state against the requirement's accepted states;
3. collect satisfied, unsatisfied and conflicted targets deterministically;
4. checkpoint is satisfied only when every requirement is satisfied and none conflict.

This is a derived verification view. It never changes `RunPhase` or creates completion truth in ECR-002.

## 9. Testing strategy

### Unit/contract
- strict IDs/errors/version parsing;
- request->receipt exact binding;
- evidence quality rules;
- aggregate truth table;
- checkpoint evaluation;
- reconciliation/retry matrix;
- journal digest goldens.

### Property/adversarial
- permutation invariance of receipt aggregation;
- 1,000 identical reductions -> identical canonical output;
- target mutation always changes/fails binding;
- hostile oversized input bounded before expensive work;
- conflicting verification never becomes verified;
- absence-of-evidence never becomes no-effect proof.

### ECR-002 compatibility
- reconciliation takes `RunState` read-only and leaves canonical bytes/state unchanged;
- `unresolved_attempts` membership for the prior attempt is unchanged for all three reconciliation outcomes;
- no `RunEvent`/`ActionReceipt` construction side effect;
- `RunResumed`/`ExecutionCompleted` remain rejected where the unresolved attempt blocks them;
- blind-retry guard remains rejected for the unresolved prior attempt;
- `semantically_retryable*` is tested only as advisory output for a future new-attempt proposal.

### Persistence
- transactional initialization/migration;
- append/reopen/replay equivalence;
- competing expected-head appenders: exactly one succeeds;
- ordinary update/delete blocked;
- corrupt JSON/digest/sequence/index fails or rebuilds as appropriate;
- projection delete/rebuild byte-equivalent derived views.

### Regression
- full ECR-001 contract/property suite;
- full ECR-002 event/reducer/attempt/budget/migration/store/crash/archive/boundary suite;
- workspace/rustdoc/offline.

## 10. CI gate

Permanent ECR-004 CI must include:

```text
cargo metadata --locked
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
explicit ECR-001 regression targets
explicit ECR-002 regression targets
explicit ECR-004 request/evidence/aggregate/checkpoint/reconcile/journal/store/boundary targets
cargo test --doc --workspace --locked
cargo test --workspace --locked --offline
scripts/check-core-unsafe.sh + check-core-deps.sh
scripts/check-run-unsafe.sh + check-run-deps.sh
scripts/check-verify-unsafe.sh + check-verify-deps.sh
cargo tree -p ecra-verify --locked
```

Historical green cannot be reused after a content change to claim exact-head PASS.

## 11. Constitution G1–G15

- **G1 Domain coherence — PASS:** reuses canonical VerificationReceipt/targets/evidence and ECR-002 attempt truth.
- **G2 Authority — PASS:** verification/retry disposition grants no capability/approval/execution authority.
- **G3 Provenance — PASS:** verification does not rewrite provenance; evidence refs retained.
- **G4 Side effects — PASS:** ECR-004 performs only local journal mutation; external side effects are observed/reconciled, never executed; ECR-002 unresolved state is not altered.
- **G5 Verification — PASS:** one canonical VerificationReceipt path; executor receipts are not verifier truth.
- **G6 Durability — PASS:** sidecar journal restart/reopen/replay specified; ECR-002 execution history/state remains dependency truth.
- **G7 Privacy/secrets — PASS:** synthetic/non-sensitive metadata only; raw evidence payload/secret persistence excluded.
- **G8 Local-first — PASS:** fully offline fixture operation; no cloud dependency.
- **G9 Interoperability — PASS-N/A:** no external protocol adapter in v1.
- **G10 Donor/license — PASS:** no donor code planned; dependencies already repository-known and reverified before adoption.
- **G11 Upstream/browser maintenance — PASS-N/A:** no browser privileged surface.
- **G12 Benchmarks — PASS:** claims limited to deterministic fixtures/resource gates; statistical verifier accuracy deferred.
- **G13 Information flow/egress — PASS:** no remote acquisition/egress; evidence input only.
- **G14 Identity/principal — PASS:** verifier principal is optional evidence only; no authentication/identity minting.
- **G15 Bounded execution — PASS:** explicit counts/bytes/query limits and no recursive/provider execution.

## 12. Complexity tracking

### New sidecar journal instead of extending ECR-002 v1 events

**Cost:** one additional local schema/store and combined consumer view.

**Why simpler alternatives are insufficient:** extending `RunEvent` would change a `CLOSED_CANONICAL` strict v1 wire contract and conflate execution truth with verification truth. A sidecar preserves both owners cleanly. It intentionally cannot resolve the old run state.

### Advisory retry disposition instead of ECR-002 run repair

**Cost:** reconciliation evidence and execution-state resolution remain separate; a later explicitly versioned repair protocol is required to consume that evidence operationally.

**Why:** ECR-002 v1 has no canonical resolution event other than a real receipt path. Silently clearing the unresolved marker from ECR-004 would counterfeit execution truth and violate the closed dependency contract.

### New `ecra-verify` crate instead of placing logic in ecra-run

**Cost:** one workspace crate/dependency boundary.

**Why:** verifier logic must stay independent from executor/reducer semantics, and later browser/search/provider adapters need a narrow verification interface rather than direct run-store access.

## 13. Implementation authorization rule

This planning branch does not authorize code. Implementation begins only after:

1. spec/research/data-model/contracts/threat-model/plan/tasks/quickstart/checklist exist;
2. analyze finds zero blocking planning drift after IC-001 and IC-002 convergence;
3. planning PR is merged to canonical `main`;
4. exact merged planning head passes required ECR-001/ECR-002 regressions;
5. implementation branch is created from that exact eligible head.