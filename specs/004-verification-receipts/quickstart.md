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

Expected target set after implementation:

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
- only accepted canonical/serialization/hash/local-store dependencies.

## 7. Verification contract acceptance

Prove with committed fixtures/tests:

```text
ActionReceipt never self-promotes to VerificationReceipt
wrong target/action/attempt/evidence binding fails closed
Verified/Rejected/Inconclusive require evidence
mutable decision-grade evidence requires immutable digest/snapshot binding
method label alone never determines truth
verification never rewrites provenance
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

Permutation/property tests must prove ordering independence and 1,000 identical evaluations must produce identical canonical views.

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

Then prove retry safety:

```text
effect_confirmed -> duplicate retry blocked
still_unknown -> blind retry blocked
no_effect_confirmed + naturally idempotent safe -> semantically retryable, not authorized
no_effect_confirmed + same-key class -> exact same key required
non-idempotent/unknown/never-blind paths remain fail-closed
```

No test may fabricate an `ActionReceipt` to resolve the scenario.

## 10. Journal/persistence acceptance

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
```

The integrity-chain test wording must explicitly avoid hostile full-store tamper-resistance claims.

## 11. Resource/hostile input acceptance

Test exact v1 ceilings for evidence refs, receipts per target, checkpoint requirements, reconciliation support IDs, notes/rule strings, journal entry bytes and query materialization. Over-limit input must return typed resource-limit errors without panic.

## 12. Synthetic/non-sensitive audit

Committed ECR-004 fixtures and journal bytes must contain only synthetic IDs/metadata/digests. Add sentinels proving raw private/secret payload strings are absent from stored entries, Debug/Display and errors.

## 13. Closure gate

Before PR leaves Draft/implementation review state:

1. all ECR-004 tasks complete;
2. FR/SC traceability has zero unowned requirement;
3. constitution G1–G15 rechecked;
4. post-implementation analyze has zero unresolved MUST drift;
5. exact feature head passes permanent ECR-004 CI plus ECR-001/ECR-002 regressions;
6. all actionable review threads resolved;
7. merge exact expected head by allowed non-rebase method;
8. required ECR-004/ECR-001/ECR-002 post-merge `main` workflows pass;
9. only then mark `CLOSED_CANONICAL`.