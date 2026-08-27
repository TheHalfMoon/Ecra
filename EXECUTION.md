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
```

Evidence anchors:

```text
Implementation baseline: 5dfe4c09b2abceeec14bc94b8e13d2dccddfd37c
CI:                      33086490495 — success

Converged executable gate: a7f1ea27e55fe7d41d70a6101dd3f44502e260f0
CI:                        33087744071 — success

Final semantic analyze head before this ledger update:
3a6b2c8794d322aee06771a36dd46ed891a95c62
```

Always re-read the current branch after this file; these SHAs are evidence anchors, not permanent pointers.

## Phase ledger

| Phase | Tasks | State |
|---|---:|---|
| 1–9 | T001–T062 | `VERIFIED_ON_BRANCH` |
| 10 | T063–T069 | `VERIFIED_ON_BRANCH` |
| 11 | T070–T076 | T070–T072 + T074–T075 complete; T073/T076 remain |
| 12 | T077–T080 | T077–T079 complete; T080 `BLOCKED_BY_EXACT_HEAD_ACTIONS_STARTUP` |

## Convergence truth

- Primary `data-model.md` and `contracts/domain-v1.md` are synchronized with implemented v1 semantics.
- `implementation-clarifications.md` is folded historical rationale, not a competing wire contract.
- `quickstart.md` and CI run the current full security/contract gate surface.
- `traceability-closure-2026-08-27.md` maps FR-001–FR-055, SC-001–SC-020, constitution G1–G15 and P-001–P-035.
- `final-convergence-analyze-2026-08-27.md` reports zero blocking semantic drift and zero unowned requirement/review blockers.

## Current blocker — Actions starts no runner

Final head `3a6b2c8794d322aee06771a36dd46ed891a95c62` has no executable failure evidence. Run `33088269829` was attempted four times; every attempt terminated before checkout/build/test with zero executable steps and no allocated runner/log.

Control experiment: re-running the previously successful `33087744071` job at known-good head `a7f1ea27…` now fails in the same pre-runner/zero-step state. Therefore the current blocker is not specific to the final documentation head or an ECR-001 code/test delta.

The delta from last successful converged gate `a7f1ea27…` to final semantic head `3a6b2c87…` is docs-only: `EXECUTION.md`, active `STATUS.md`, `tasks.md`, `implementation-clarifications.md`, and final analyze. No production source, test, fixture, Cargo graph, workflow, or security script changed.

GitHub Status records recent Actions runner-start/assignment incidents; those Actions incidents are marked resolved, while a separate Billing incident remains active. The repository API exposes no specific billing/account annotation for these zero-step jobs, so do not claim a narrower cause without new evidence.

## Required exact-head CI surface

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

Do not mutate implementation simply to manufacture a new CI result. The next executor must:

```text
A. observe the fresh workflow run created by the latest ledger-only commit
B. if a runner starts, require the entire gate above to PASS
C. if it fails before runner again, keep PR Draft and record the external blocker without claiming PASS
D. after exact-head green, finalize T073/T080 evidence
E. re-run exact-head CI if evidence-ledger commits move the head
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
