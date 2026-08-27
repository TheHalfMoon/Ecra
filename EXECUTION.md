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
PR: #1 — OPEN / READY / mergeable at last live check
Lifecycle: READY_FOR_MERGE_PENDING_EXACT_HEAD_LEDGER_CI
Current blocker class: NONE_SEMANTIC; exact-head CI/review required on final ledger-finalization head
```

Verified remediation evidence:

```text
Head:   face8d7448afc617a6c04e53237b066bf2ef5b63
CI:     33097623599 — success
Runner: macbook — self-hosted macOS
Rust:   1.98.0-aarch64-apple-darwin
```

PR #1 ready-review originally exposed three actionable defects. Phase 13 remediated all three, the full exact-head gate passed, all original threads are resolved/outdated, and CodeRabbit completed successfully with no new actionable thread on the remediation head.

## Phase ledger

| Phase | Tasks | State |
|---|---:|---|
| 1–9 | T001–T062 | `VERIFIED_ON_BRANCH` |
| 10 | T063–T069 | `VERIFIED_ON_BRANCH` |
| 11 | T070–T076 | T070–T075 complete; T076 waits for merge/post-merge evidence |
| 12 | T077–T080 | complete on branch |
| 13 | T081–T084 | complete on verified remediation head |

## Phase 13 closure evidence

```text
T081 Versioned<T> strict public Deserialize
  -> ordinary serde_json::from_* rejects unsupported major/newer minor
  -> Versioned::from_json_slice retains typed compatibility DomainError codes

T082 FactValue numeric construction
  -> integers outside I-JSON exact range cannot be constructed
  -> non-canonical decimal strings cannot be constructed
  -> validated constructed values serialize and strict-round-trip

T083 lifecycle synchronization
  -> platform STATUS + roadmap agree ECR-001 is IMPLEMENTING
  -> active STATUS + EXECUTION + tasks describe the more specific final merge gate

T084 full exact-head CI + review closure
  -> run 33097623599 PASS on face8d7448afc617a6c04e53237b066bf2ef5b63
  -> all original Qodo threads resolved/outdated
  -> PR returned Ready
  -> CodeRabbit success; no new actionable thread
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
A. require the complete exact-head gate to PASS on the final documentation/status ledger head
B. re-read PR #1 head, canonical main, reviews, threads, checks and mergeability
C. require no actionable review blocker on that exact head
D. merge with expected head SHA using a non-rebase method
E. require post-merge canonical-main CI to PASS
F. only then mark T076 and ECR-001 CLOSED_CANONICAL across roadmap/platform/active status/EXECUTION/tasks
G. re-read platform roadmap/dependencies and begin the next genuinely eligible slice
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
