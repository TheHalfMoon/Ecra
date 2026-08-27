# Ecra Execution Guide

> **Operational start-here document.** Recover live work from this file, the platform roadmap/status, the relevant slice package, and exact GitHub truth; do not depend on private chat state.

## Source-of-truth order

1. `.specify/memory/constitution.md`
2. `EXECUTION.md`
3. `specs/000-ecra-platform/roadmap.md`
4. `specs/000-ecra-platform/STATUS.md`
5. relevant platform architecture/threat/gap/risk/benchmark/decision artifacts
6. `specs/README.md`
7. the current or next eligible bounded Spec Kit package
8. exact live branch/head, PR, CI, reviews and changed files

Stale prose must be updated to live evidence, never the reverse.

## Current execution truth

```text
ECR-001 — Trusted Domain Kernel: CLOSED_CANONICAL
PR #1: MERGED
Merge commit: d1021616eae721e0b89bd5d4114531c4b9cc8a58
Post-merge CI: 33099033214 — SUCCESS
Current phase: ECR-001 closure-ledger finalization on canonical main
Next dependency-eligible candidate: ECR-002 — Durable Run, Ledger & Budgets
```

Do not begin ECR-002 implementation merely because its ECR-001 dependency is now satisfied. First require the final ECR-001 closure-ledger head on `main` to pass the complete exact-head CI surface, then re-read the ECR-002 package and advance it through its own Spec Kit lifecycle.

## ECR-001 canonical closure evidence

Final feature verification:

```text
Head:   1d3c319c3317d3572baad1784f18eea771c5ac6e
CI:     33098892820 — SUCCESS
Runner: macbook — self-hosted macOS
Rust:   1.98.0-aarch64-apple-darwin
```

Merge and canonical verification:

```text
PR:           #1 — MERGED
Merge commit: d1021616eae721e0b89bd5d4114531c4b9cc8a58
Main CI:      33099033214 — SUCCESS
```

The final feature head and resulting canonical merge commit both passed the complete ECR-001 gate. All actionable Qodo review threads were resolved/outdated before merge, including final Phase 13 task-path traceability, and CodeRabbit was successful.

## ECR-001 phase ledger

| Phase | Tasks | State |
|---|---:|---|
| 1–9 | T001–T062 | complete |
| 10 | T063–T069 | complete |
| 11 | T070–T076 | complete |
| 12 | T077–T080 | complete |
| 13 | T081–T084 | complete |

ECR-001 is no longer the active implementation slice. Its package remains the canonical record for the trusted-domain kernel.

## CI architecture

GitHub-hosted runners were blocked because the owner account had an Actions budget of `$0` with stop-usage enabled and no payment method. The approved repository-scoped self-hosted macOS runner `macbook` preserves the full verification surface without making the repository public or enabling paid overage.

Current trusted workflow topology:

```text
push: 001-trusted-domain-kernel -> trusted feature-head verification
push: main                      -> canonical-main verification
workflow_dispatch               -> explicit recovery/recheck
concurrency                      -> cancel superseded same-ref runs
runs-on                          -> self-hosted
permissions                     -> contents: read
```

Do not restore untrusted `pull_request` execution on this persistent self-hosted machine without an explicit security design.

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

## Next eligible work after closure-ledger CI

```text
A. require exact-head main CI PASS on the final ECR-001 closure-ledger head
B. re-read canonical roadmap/platform status and confirm ECR-002 is still the next dependency-eligible slice
C. inspect `specs/002-durable-run-ledger/` and any active STATUS/spec/research/plan/tasks/analyze artifacts
D. determine the exact ECR-002 lifecycle state from repository truth
E. if planning artifacts are incomplete, continue specify/research/plan/contracts/tasks/analyze before implementation
F. if ECR-002 is genuinely TASKS_READY with clean analyze and constitution gates, create/use its bounded feature branch and begin only its first eligible task
G. preserve the same exact-head/review/merge/post-merge evidence discipline
```

## Non-negotiable inherited invariants

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

ECR-002 must build on ECR-001 rather than reopening or duplicating these trusted-domain semantics.
