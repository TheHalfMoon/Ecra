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
Lifecycle: READY_FOR_REVIEW_PENDING_EXACT_HEAD_LEDGER_CI
Current blocker class: NONE_SEMANTIC; final documentation-only ledger head requires exact-head CI
```

Evidence anchors:

```text
Implementation baseline: 5dfe4c09b2abceeec14bc94b8e13d2dccddfd37c
CI:                      33086490495 — success

Final semantic analyze:   ZERO_BLOCKING_DRIFT_FOUND

Latest full exact-head branch gate:
20a56b10257609426e5b66ec0c2ba2f884822039
CI 33095158577 — success
Runner macbook — self-hosted macOS
Rust 1.98.0-aarch64-apple-darwin
```

Always re-read the current branch after this file; these SHAs are evidence anchors, not permanent pointers.

## Phase ledger

| Phase | Tasks | State |
|---|---:|---|
| 1–9 | T001–T062 | `VERIFIED_ON_BRANCH` |
| 10 | T063–T069 | `VERIFIED_ON_BRANCH` |
| 11 | T070–T076 | T070–T075 complete; T076 waits for merge/post-merge evidence |
| 12 | T077–T080 | complete on branch |

## Convergence truth

- Primary `data-model.md` and `contracts/domain-v1.md` are synchronized with implemented v1 semantics.
- `implementation-clarifications.md` is folded historical rationale, not a competing wire contract.
- `quickstart.md` and CI define the full security/contract gate surface.
- `traceability-closure-2026-08-27.md` maps FR-001–FR-055, SC-001–SC-020, constitution G1–G15 and P-001–P-035.
- `final-convergence-analyze-2026-08-27.md` reports zero blocking semantic drift and zero unowned requirement/review blockers.
- T073 and T080 are complete on the feature branch with exact-head evidence `20a56b10…` / run `33095158577`.

## CI recovery architecture

GitHub-hosted runners were blocked before execution because the owner account had an Actions budget of `$0` with `Stop usage: Yes` and no payment method. The owner intentionally declined paid overage.

The approved recovery path is a repository-scoped self-hosted macOS runner named `macbook`, installed as a launchd service. This avoids paid hosted-runner usage and preserves the full gate surface.

The trusted workflow topology is:

```text
push: 001-trusted-domain-kernel -> exact feature-head gate
push: main                      -> post-merge canonical-main gate
workflow_dispatch               -> explicit recovery/recheck
concurrency                      -> cancel superseded same-ref runs
runs-on                          -> self-hosted
permissions                      -> contents: read
```

Do not restore `pull_request` execution on this persistent self-hosted machine without an explicit security design for untrusted/forked code. Do not make the repository public merely to bypass billing.

## Exact-head CI surface

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

The latest successful run `33095158577` executed this full surface after checking out exact head `20a56b10257609426e5b66ec0c2ba2f884822039`.

## Next eligible work

```text
A. observe the workflow on the current final ledger head
B. require the entire exact-head gate to PASS
C. re-read PR #1 head, canonical main, reviews, threads, checks and mergeability
D. make PR ready only if pre-merge governance remains satisfied
E. address actionable review findings; any head mutation requires another exact-head gate
F. merge without force-push/rebase/destructive history rewriting
G. require post-merge canonical-main CI to PASS
H. only then mark T076 and ECR-001 CLOSED_CANONICAL
I. re-read platform roadmap/dependencies and begin the next genuinely eligible ECR slice
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
