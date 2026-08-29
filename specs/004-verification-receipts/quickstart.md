# Quickstart: ECR-004 Verification & Reconciliation

This is the verification workflow for implementation and closure. Commands become executable only after the implementation branch/crate/CI are materialized.

## 1. Preconditions

```text
ECR-001 CLOSED_CANONICAL
ECR-002 CLOSED_CANONICAL
ECR-004 planning package TASKS_READY on canonical main
implementation branch created from exact authorized planning head
```

No ECR-031 dependency is required for ECR-004 v1 because acceptance persists only synthetic/non-sensitive verification metadata and does not claim protected hostile-tamper resistance.

ECR-004 also does not reopen ECR-002 v1. Reconciliation records effect evidence only; an unresolved ECR-002 attempt remains unresolved unless ECR-002 itself later evolves through an explicit versioned repair/resolution protocol.

## 2. Locked workspace gate

```bash
cargo metadata --format-version 1 --locked --no-deps
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --workspace --locked
cargo test --workspace --locked --offline
```

## 3. Explicit ECR-004 targets

```bash
cargo test -p ecra-verify --lib --locked
cargo test -p ecra-verify --test request_contract --locked
cargo test -p ecra-verify --test evidence --locked
cargo test -p ecra-verify --test aggregate --locked
cargo test -p ecra-verify --test checkpoint --locked
cargo test -p ecra-verify --test reconcile --locked
cargo test -p ecra-verify --test journal --locked
cargo test -p ecra-verify --test sqlite_store --locked
cargo test -p ecra-verify --test migration --locked
cargo test -p ecra-verify --test boundaries --locked
cargo test -p ecra-verify --test portability --locked
cargo test -p ecra-verify --test review_hardening --locked
```

## 4. ECR-001 regressions

Run the permanent explicit core contract/property targets, including verification and action binding. At minimum:

```bash
cargo test -p ecra-core --test valid_fixtures --locked
cargo test -p ecra-core --test invalid_fixtures --locked
cargo test -p ecra-core --test contract_fixtures --locked
cargo test -p ecra-core --test canonicalization --locked
cargo test -p ecra-core --test action_digest --locked
cargo test -p ecra-core --test properties --locked
cargo test -p ecra-core --test portability --locked
cargo test -p ecra-core --test non_authoritative_metadata --locked
```

## 5. ECR-002 regressions

```bash
cargo test -p ecra-run --test event_contract --locked
cargo test -p ecra-run --test reducer --locked
cargo test -p ecra-run --test attempts --locked
cargo test -p ecra-run --test budgets --locked
cargo test -p ecra-run --test migration --locked
cargo test -p ecra-run --test sqlite_store --locked
cargo test -p ecra-run --test crash_recovery --locked
cargo test -p ecra-run --test archive --locked
cargo test -p ecra-run --test boundaries --locked
cargo test -p ecra-run --test portability --locked
cargo test -p ecra-run --locked
```

These regressions are semantic acceptance for the IC-002 boundary, not merely compatibility smoke tests.

## 6. Boundary checks

```bash
bash scripts/check-core-unsafe.sh
bash scripts/check-core-deps.sh
bash scripts/check-run-unsafe.sh
bash scripts/check-run-deps.sh
bash scripts/check-verify-unsafe.sh
bash scripts/check-verify-deps.sh
cargo tree -p ecra-verify --locked
```

Expected:
- no Ecra-authored unsafe in `ecra-verify`;
- no browser/network/model/provider/process/policy/authorization dependencies;
- only accepted canonical/serialization/hash/local-store dependencies;
- no dependency path that gives ECR-004 private ECR-002 mutation/event-append authority.

## 7. Verification contract acceptance

Prove with committed fixtures/tests:

```text
ActionReceipt/network receipt never self-promotes to independent VerificationReceipt truth
wrong target/action/attempt/evidence binding fails closed
Verified/Rejected/Inconclusive require evidence
mutable decision-grade evidence requires immutable digest/snapshot binding
an immutable binding does not turn executor self-report into independent evaluation
method label alone never determines truth
verification never rewrites provenance/fact assessment axes
```

## 8. Aggregation acceptance

Required truth table:

```text
no conclusive/inconclusive receipts        -> absent
verified only                              -> verified
rejected only                              -> rejected
inconclusive only                          -> inconclusive
verified + rejected in any ordering        -> conflicted
```

Permutation/property tests must prove ordering independence and 1,000 identical evaluations must produce identical canonical views. `VerificationAggregateViewV1` is derived from canonical receipts and is not accepted from hostile serialized input as an asserted truth view.

## 9. Reconciliation acceptance

Fixture matrix must include:

```text
unreceipted attempt + no external evidence       -> still_unknown
unreceipted attempt + conflicting evidence       -> still_unknown/conflict
unreceipted attempt + effect-present evidence    -> effect_confirmed
unreceipted attempt + explicit no-effect evidence-> no_effect_confirmed
cross-run attempt evidence                        -> reject
cross-action attempt evidence                     -> reject
```

Then prove retry-safety classification:

```text
effect_confirmed -> duplicate retry blocked
still_unknown -> blind retry blocked
no_effect_confirmed + naturally idempotent safe -> semantically retryable advisory only
no_effect_confirmed + same-key class -> exact same key required for any future new-attempt proposal
non-idempotent/unknown/never-blind paths remain fail-closed
```

A supplied reconciliation record must be revalidated against the exact ECR-002 state and its canonical supporting verification receipts before it can influence retry disposition. A merely deserialized or caller-constructed reconciliation record is not trusted.

No test may fabricate an `ActionReceipt` to resolve the scenario.

## 10. ECR-002 unresolved-state compatibility acceptance

For each reconciliation outcome (`effect_confirmed`, `no_effect_confirmed`, `still_unknown`), prove all of the following against the supplied canonical ECR-002 state:

```text
RunState canonical/semantic execution state unchanged
PreparedAttemptState remains unreceipted when originally unreceipted
PreparedAttemptState unresolved flag unchanged
RunState unresolved_attempts membership unchanged
RunPhase unchanged
no ECR-002 RunEvent constructed/appended
no ActionReceipt constructed/synthesized
```

Then prove the ECR-002 guards remain authoritative:

```text
same-run RunResumed remains rejected when unresolved state blocks it
same-run ExecutionCompleted remains rejected when unresolved state blocks it
blind retry of the unresolved prior attempt remains rejected
semantically_retryable* does not call/override ensure_retry_allowed
```

A passing reconciliation test MUST NOT claim operational run resolution. `semantically_retryable*` is only evidence for a future owning path to consider a **new attempt**.

## 11. Journal/persistence acceptance

```text
empty store -> transactional v1 initialization
append -> reopen -> replay -> identical aggregate/checkpoint/reconciliation view
expected-head competing appenders -> exactly one wins
ordinary canonical row UPDATE/DELETE -> blocked
sequence gap -> fail closed
previous digest mismatch -> fail closed
entry digest mismatch -> fail closed
duplicate IDs -> fail closed
projection delete/rebuild -> equivalent view
newer unsupported schema -> no mutation + fail closed
failed migration -> original state preserved
append at 4,096 authoritative entries -> fail before a 4,097th entry can poison replay
```

The integrity-chain test wording must explicitly avoid hostile full-store tamper-resistance claims.

## 12. Resource/hostile input acceptance

Test exact v1 ceilings for complete request bytes, evidence refs, receipts per target, checkpoint requirements/accepted-state sequences/complete checkpoint bytes, reconciliation support IDs and available-receipt query size, notes/rule strings, journal entry bytes and journal query materialization. Over-limit input must return typed resource-limit errors without panic.

## 13. Synthetic/non-sensitive audit

Committed ECR-004 fixtures and journal bytes must contain only synthetic IDs/metadata/digests. Sentinel scans prove raw private/secret payload strings are absent from the authorized synthetic persistence corpus, Debug/Display and error paths. ECR-004 does not claim to heuristically redact a secret deliberately inserted into canonical notes; real sensitive payload acceptance remains outside v1.

## 14. Closure gate

Before merge:

1. all ECR-004 implementation/convergence tasks before merge complete;
2. FR/SC traceability has zero unowned requirement, including FR-046/SC-013;
3. constitution G1–G15 rechecked;
4. post-implementation analyze has zero unresolved MUST drift;
5. exact final feature head passes permanent ECR-004 CI plus ECR-001/ECR-002 regressions;
6. all actionable review threads are resolved and automated review/check state has no remaining actionable blocker;
7. merge exact expected head by allowed non-rebase method;
8. required ECR-004/ECR-001/ECR-002 post-merge `main` workflows pass;
9. only then mark `CLOSED_CANONICAL`.
