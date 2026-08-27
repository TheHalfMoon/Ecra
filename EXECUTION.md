# Ecra Execution Guide

> **Operational start-here document.** Recover live work from this file, the active slice status/tasks, and exact GitHub truth; do not depend on private chat state.

## Source-of-truth order

1. `.specify/memory/constitution.md`
2. `EXECUTION.md`
3. `specs/000-ecra-platform/roadmap.md`
4. `specs/000-ecra-platform/STATUS.md`
5. relevant platform architecture/threat/gap/risk/benchmark/decision artifacts
6. `specs/README.md`
7. active package `specs/001-trusted-domain-kernel/`, especially `STATUS.md`, spec/research/data-model/contracts/clarifications/plan/quickstart/tasks/analyze/traceability
8. exact live branch/head, PR, CI, reviews and changed files

Stale prose must be updated to live evidence, never the reverse.

## Current execution truth

```text
Active slice: ECR-001 — Trusted Domain Kernel
Branch: 001-trusted-domain-kernel
PR: #1 — OPEN / DRAFT / mergeable at last live check
Lifecycle: FINAL_CONVERGENCE_VERIFICATION
Current blocker class: EXTERNAL_GITHUB_HOSTED_RUNNER_ACCOUNT_STATE
```

Evidence anchors:

```text
Implementation baseline: 5dfe4c09b2abceeec14bc94b8e13d2dccddfd37c
CI:                      33086490495 — success

Converged executable gate: a7f1ea27e55fe7d41d70a6101dd3f44502e260f0
CI:                        33087744071 — success

Latest CI-topology parent head:
d84f01ad4e1c71aac58a89fcdea8d67179fc89fe
```

Always re-read the current branch after this file; these SHAs are evidence anchors, not permanent pointers.

## Phase ledger

| Phase | Tasks | State |
|---|---:|---|
| 1–9 | T001–T062 | `VERIFIED_ON_BRANCH` |
| 10 | T063–T069 | `VERIFIED_ON_BRANCH` |
| 11 | T070–T076 | T070–T072 + T074–T075 complete; T073/T076 remain |
| 12 | T077–T080 | T077–T079 complete; T080 `BLOCKED_BY_EXTERNAL_GITHUB_HOSTED_RUNNER_ACCOUNT_STATE` |

## Convergence truth

- Primary `data-model.md` and `contracts/domain-v1.md` are synchronized with implemented v1 semantics.
- `implementation-clarifications.md` is folded historical rationale, not a competing wire contract.
- `quickstart.md` and CI define the full security/contract gate surface.
- `traceability-closure-2026-08-27.md` maps FR-001–FR-055, SC-001–SC-020, constitution G1–G15 and P-001–P-035.
- `final-convergence-analyze-2026-08-27.md` reports zero blocking semantic drift and zero unowned requirement/review blockers.

## Current blocker — GitHub account state prevents hosted runner allocation

The failure occurs before any Ecra command executes. Multiple independent experiments all terminate with no allocated runner, no job steps and no logs:

```text
3a6b2c87…  standard ubuntu-latest  run 33088269829  four attempts  zero-step
238f49a1…  fresh standard run       run 33089816127  zero-step
a7f1ea27…  re-run of known PASS     control rerun      zero-step
5a42bf77…  alternate ubuntu-slim    run 33090299540  zero-step
57c70a76…  restored standard        push/PR runs       zero-step
d84f01ad…  single PR topology       run 33090580208  zero-step
```

The `ubuntu-slim` experiment ruled out a single standard-runner pool as the repository-side cause. It was reverted immediately; the final workflow remains on `ubuntu-latest`.

GitHub check-run metadata reports one failure annotation for these jobs, but the connected repository API cannot read the annotation body and explicitly blocks the personal billing-usage endpoint. GitHub's documentation identifies payment state, exhausted private-repository included usage, and blocking Actions budgets as causes of hosted jobs being stopped before execution.

Therefore do not mutate product code to chase this failure. The next required action is outside repository content: unlock the owner account's GitHub Actions billing/budget/usage state, or explicitly register an approved self-hosted runner.

## CI usage hardening already applied

The workflow no longer runs duplicate feature-branch jobs. Its intended trigger topology is now:

```text
pull_request        -> exact feature-head gate
push: main          -> post-merge canonical-main gate
workflow_dispatch   -> explicit recheck/recovery
concurrency          -> cancel superseded work for same PR/ref
```

This preserves exact-head and post-merge evidence while reducing hosted-runner consumption.

## Required external remediation before retry

For the repository owner account:

```text
GitHub Settings
  -> Billing & Licensing
  -> Budgets and alerts / Actions usage
```

Resolve the actual account condition:

```text
- valid payment method if overage is needed
- positive/non-exhausted Actions budget
- no overlapping stricter budget that blocks Actions
- sufficient included private-repository usage, or paid overage authorization
```

If billing is intentionally unavailable, an isolated self-hosted runner is the only non-public-repository CI alternative supported by the current architecture. Do not make the repository public as a billing workaround and do not register an untrusted persistent runner.

## Required exact-head CI surface after unlock

```bash
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo test -p ecra-core --test valid_fixtures --locked
cargo test -p ecra-core --test invalid_fixtures --locked
cargo test -p ecra-core --test contract_fixtures --locked
cargo test -p ecra-core --test canonicalization --locked
cargo test -p ecra-core --test action_digest --locked
cargo test -p ecra-core --test properties --locked
cargo test -p ecra-core --test portability --locked
cargo test -p ecra-core --test non_authoritative_metadata --locked
cargo test --doc --workspace --locked
cargo test --workspace --locked --offline
bash scripts/check-core-unsafe.sh
bash scripts/check-core-deps.sh
cargo tree -p ecra-core
```

## Next eligible work

```text
A. unlock GitHub Actions account billing/budget/usage, or register an approved isolated self-hosted runner
B. trigger/observe the exact current PR-head workflow
C. require the entire gate above to PASS
D. finalize T073/T080 with exact head/run evidence
E. if the evidence-ledger commit moves the head, run the exact-head gate again
F. inspect reviews/threads/checks and make PR ready only when all pre-merge requirements are met
G. merge, verify canonical main, then and only then mark T076/CLOSED_CANONICAL
```

## Non-negotiable invariants

```text
Actor != authenticated Principal
CapabilityRequest != CapabilityGrant
classification != permission
InformationUse != authorization
locator/provider/free-form text != authority or resource identity
ActionDigest != signature/approval
ActionIntent != ActionAttemptRef != ActionReceipt != VerificationReceipt
executor_observed_success != verified
UNKNOWN remains UNKNOWN
ContentDigest != ActionDigest/security proof
```

No dependent ECR implementation is eligible before ECR-001 closes canonically.
