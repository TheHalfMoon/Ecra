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
Lifecycle: REVIEW_REMEDIATION
Current blocker class: ACTIONABLE_REVIEW_FINDINGS; exact-head verification required after fixes
```

Pre-review evidence anchor:

```text
Head:   12c7029dbde30d2d860fe70447f79b6432ff2f96
CI:     33095782152 — success
Runner: macbook — self-hosted macOS
```

PR #1 was made Ready only after that exact-head success. Qodo then produced three actionable review threads, including two High correctness findings. The PR was immediately returned to Draft and Phase 13 was activated. Do not merge based on the pre-review head.

## Phase ledger

| Phase | Tasks | State |
|---|---:|---|
| 1–9 | T001–T062 | `VERIFIED_ON_BRANCH` |
| 10 | T063–T069 | `VERIFIED_ON_BRANCH` |
| 11 | T070–T076 | T070–T075 complete; T076 waits for merge/post-merge evidence |
| 12 | T077–T080 | complete on branch before ready-review |
| 13 | T081–T084 | `REVIEW_REMEDIATION` |

## Phase 13 findings and required proof

```text
T081 Versioned<T> strict public Deserialize
  -> ordinary serde_json::from_* must reject unsupported major/newer minor
  -> Versioned::from_json_slice must retain typed compatibility DomainError codes

T082 FactValue numeric construction
  -> API-created integer must be impossible outside I-JSON exact range
  -> canonical decimal construction must be equally fail-closed
  -> serialized constructed values must strict-round-trip

T083 lifecycle synchronization
  -> platform STATUS + roadmap + active STATUS + EXECUTION + tasks agree ECR-001 is IMPLEMENTING/REVIEW_REMEDIATION, not TASKS_READY or CLOSED_CANONICAL

T084 full exact-head CI + review-thread closure
  -> every quickstart gate PASS
  -> re-read all review threads
  -> resolve only proven-remediated threads
  -> return PR to Ready only with zero actionable blocker
```

## CI recovery architecture

GitHub-hosted runners were blocked because the owner account had an Actions budget of `$0` with stop-usage enabled and no payment method. The owner intentionally declined paid overage.

The approved recovery path is a repository-scoped self-hosted macOS runner named `macbook`, installed as a launchd service. This avoids paid hosted-runner usage and preserves the full gate surface.

```text
push: 001-trusted-domain-kernel -> exact feature-head gate
push: main                      -> post-merge canonical-main gate
workflow_dispatch               -> explicit recovery/recheck
concurrency                      -> cancel superseded same-ref runs
runs-on                          -> self-hosted
permissions                      -> contents: read
```

Do not restore `pull_request` execution on this persistent self-hosted machine without an explicit security design for untrusted/forked code.

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

## Next eligible work

```text
A. finish Phase 13 source/test/lifecycle remediation
B. require full exact-head CI PASS on the final remediation head
C. re-read reviews/threads and resolve only findings proven fixed
D. mark T081–T084 complete and re-run exact-head CI if that ledger mutation moves the head
E. return PR #1 to Ready
F. observe final bot/human review/check state
G. merge without force-push/rebase/destructive history rewriting only with clean exact-head evidence
H. require post-merge canonical-main CI PASS
I. only then T076 and ECR-001 CLOSED_CANONICAL
J. re-read platform roadmap/dependencies and begin the next genuinely eligible slice
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
