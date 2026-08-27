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
PR: #1 — OPEN / DRAFT; mergeable at last live check
Lifecycle: FINAL_CONVERGENCE_VERIFICATION
```

Latest fully verified implementation baseline:

```text
5dfe4c09b2abceeec14bc94b8e13d2dccddfd37c
CI 33086490495 — success
```

Latest executable convergence gate before final analyze/ledger output:

```text
a7f1ea27e55fe7d41d70a6101dd3f44502e260f0
CI 33087744071 — success
```

That convergence CI mirrors the revised quickstart and passed build, format, strict Clippy, full workspace tests, eight dedicated contract/security targets, rustdoc, offline replay, unsafe boundary, dependency boundary and `cargo tree -p ecra-core`.

Always re-read the current head before mutation; the SHAs above are evidence anchors, not permanent branch pointers.

## ECR-001 phase ledger

| Phase | Tasks | State |
|---|---:|---|
| 1–9 | T001–T062 | `VERIFIED_ON_BRANCH` |
| 10 — Cross-cutting gates | T063–T069 | `VERIFIED_ON_BRANCH` at `5dfe4c09…` / CI `33086490495` |
| 11 — Closure / traceability | T070–T076 | T070–T072 + T074–T075 complete; T073/T076 remain |
| 12 — Convergence | T077–T080 | T077–T079 complete; T080 `ACTIVE_FINAL_GATE` |

## Completed convergence

### T077

Primary `data-model.md` and `contracts/domain-v1.md` are synchronized with implemented v1 semantics: strict envelope/error behavior, exact error API, C1–C12, bound parameters, full retry matrix, bounded receipt/verification semantics and fixture/wire convention.

### T078

`quickstart.md`, `tasks.md`, active `STATUS.md`, and `EXECUTION.md` reflect live implementation/gate truth. The active status path is `specs/001-trusted-domain-kernel/STATUS.md`; no root `STATUS.md` exists.

### T079

`specs/001-trusted-domain-kernel/traceability-closure-2026-08-27.md` maps FR-001–FR-055, SC-001–SC-020, constitution G1–G15 and P-001–P-035 to code/tests/contracts or named downstream enforcement owners.

No downstream runtime control is falsely claimed as implemented by ECR-001 value types.

## Immediate next work — T080

Perform in order:

```text
A. Run final analyze-equivalent review against converged spec/research/data-model/contract/plan/tasks/implementation/traceability.
B. Record zero-blocker or blocker result in the active package.
C. Trigger/observe the full revised CI on the exact report-containing head.
D. If green and zero blocker, finalize T073/T080 ledgers with exact run/head evidence.
E. Run one final exact-head CI because ledger finalization itself changes the branch head.
F. Inspect PR #1 exact state, reviews, required checks and readiness.
```

Do not mark PR ready, merge, or advance the roadmap while any of A–F is unresolved.

## Required final CI surface

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

## Non-negotiable ECR-001 invariants

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

Additional v1 convergence truth:
- malformed/missing strict version envelope -> `serialization_failed`;
- unsupported major/minor -> dedicated compatibility codes;
- exact machine API = 16 ErrorCategory / 19 ErrorCode variants;
- every non-empty ActionParametersRef binds payload semantics with SecurityDigest;
- `ActionSemantics` is a Rust construction helper only; wire fields stay flat;
- verified/rejected/inconclusive verification requires evidence; not_evaluated may have none;
- repository semantic inner-body fixtures do not weaken the public `Versioned<T>` contract.

## Closure sequence

```text
T080 zero-blocking-drift + exact-head green
  ↓
T073 exact final evidence record
  ↓
PR readiness + required review/fixes
  ↓
merge without force-push/rebase/destructive history rewriting
  ↓
post-merge exact-main verification
  ↓
T076 roadmap/platform/active status advancement
  ↓
CLOSED_CANONICAL
  ↓
select next dependency-eligible ECR slice
```

No dependent ECR implementation is eligible before that sequence completes.

## Platform direction

The canonical dependency graph remains `specs/000-ecra-platform/roadmap.md`. ECR-001 is the trust substrate; ECR-002/ECR-031/ECR-003/ECR-004 own durable execution, identity/trust, authorization/information-flow enforcement, and verification orchestration respectively. Browser/search/workspace/skills/protocol/runtime work remains blocked by roadmap dependencies.

## Handoff rule

A future executor should need only:

```text
EXECUTION.md
specs/001-trusted-domain-kernel/STATUS.md
tasks.md + converged contract/data-model/quickstart
post-implementation analyze + traceability + final convergence analyze
live PR #1 head/CI/reviews
```

Update these ledgers whenever execution materially advances.