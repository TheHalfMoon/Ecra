# Ecra Execution Guide

> **Operational start-here document.** Recover live work from this file, the platform roadmap/status, the active slice package, and exact GitHub truth; do not depend on private chat state.

## Source-of-truth order

1. `.specify/memory/constitution.md`
2. `EXECUTION.md`
3. `specs/000-ecra-platform/roadmap.md`
4. `specs/000-ecra-platform/STATUS.md`
5. relevant platform architecture/threat/gap/risk/benchmark/decision artifacts
6. `specs/README.md`
7. active package `specs/002-durable-run-ledger/`
8. exact live branch/head, PR, CI, reviews and changed files

Stale prose must be updated to live evidence, never the reverse.

## Current execution truth

```text
ECR-001 — Trusted Domain Kernel: CLOSED_CANONICAL
ECR-001 closure-ledger head: 85e4bf657b6c33e3f88d83e92e7a35279d177349
ECR-001 closure-ledger CI: 33099434232 — SUCCESS

Active slice: ECR-002 — Durable Run, Ledger & Budgets
Lifecycle: TASKS_READY
Planning package commit: c83a208ad84b2d1da892a80a6911989eaff25ade
Planning analyze: ZERO_BLOCKING_PLANNING_DRIFT_FOUND
Tasks: T001–T073
Implementation branch: create `002-durable-run-ledger` from exact canonical main after planning-status synchronization
```

ECR-002 implementation is authorized only within the package's bounded local/synthetic/non-sensitive durability scope. This does not authorize real sensitive persistence, authentication/trust roots, authorization/declassification, independent verification/reconciliation, provider execution, distributed workflow infrastructure or multi-device sync.

## ECR-002 planning package

Read in order:

```text
specs/002-durable-run-ledger/STATUS.md
specs/002-durable-run-ledger/spec.md
specs/002-durable-run-ledger/research.md
specs/002-durable-run-ledger/data-model.md
specs/002-durable-run-ledger/contracts/run-ledger-v1.md
specs/002-durable-run-ledger/threat-model.md
specs/002-durable-run-ledger/plan.md
specs/002-durable-run-ledger/tasks.md
specs/002-durable-run-ledger/quickstart.md
specs/002-durable-run-ledger/analyze.md
specs/002-durable-run-ledger/checklists/requirements.md
```

Planning result:

```text
FR-001–FR-057: OWNED
SC-001–SC-016: OWNED
G1–G15: PASS / explicit N/A
unresolved security decisions: 0
unresolved dependency decisions: 0
real-sensitive-state authorization: NO
```

## ECR-002 fixed architecture decisions

```text
authoritative truth     append-only RunEventEnvelope history
ordering                EventSequence only
projection              rebuildable/non-authoritative RunState cache
attempt safety          committed AttemptPrepared before provider invocation
missing receipt         UNKNOWN / reconciliation-required; never inferred success/failure
integrity               domain-separated RFC8785 + SHA-256 LedgerDigest
local store             SQLite via bounded rusqlite adapter
SQLite durability       WAL + synchronous=FULL, asserted at open
write transaction       BEGIN IMMEDIATE equivalent + expected-head compare
budget accounting       typed I-JSON-safe checked integer dimensions
portable artifact       deterministic strict Stored-only ZIP `.ecra`
archive/store content   synthetic/non-sensitive v1 acceptance only
hostile rewrite claim   NOT provided by plain hash chain
```

## ECR-002 implementation order

```text
Phase 1 T001–T008  workspace/crate/CI/dependencies
Phase 2 T009–T018  errors/primitives/events/digest
Phase 3 T019–T026  reducer/state machine
Phase 4 T027–T034  budgets
Phase 5 T035–T044  SQLite/migrations/store/projections
Phase 6 T045–T051  attempt guard/recovery/concurrency
Phase 7 T052–T059  deterministic .ecra
Phase 8 T060–T066  portability/security/docs/gates
Phase 9 T067–T073  traceability/convergence/review/merge/closure
```

Do not skip ahead across dependency boundaries merely because files can be edited in parallel.

## CI architecture

The approved repository-scoped self-hosted macOS runner `macbook` remains the trusted execution oracle. Persistent personal runners must not execute untrusted fork PR code.

ECR-001 workflow remains authoritative for closed core regression on `main`. ECR-002 T006 adds a trusted push-only workflow for `002-durable-run-ledger` and `main` with the full workspace, core-regression and run-specific gate surfaces.

## ECR-002 full verification target

When implementation exists:

```bash
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --workspace --locked
cargo test --workspace --locked --offline
bash scripts/check-core-unsafe.sh
bash scripts/check-core-deps.sh
bash scripts/check-run-unsafe.sh
bash scripts/check-run-deps.sh
cargo test -p ecra-core --locked
cargo test -p ecra-run --test event_contract --locked
cargo test -p ecra-run --test reducer --locked
cargo test -p ecra-run --test attempts --locked
cargo test -p ecra-run --test budgets --locked
cargo test -p ecra-run --test sqlite_store --locked
cargo test -p ecra-run --test migration --locked
cargo test -p ecra-run --test crash_recovery --locked
cargo test -p ecra-run --test archive --locked
cargo test -p ecra-run --test portability --locked
cargo test -p ecra-run --test boundaries --locked
cargo tree -p ecra-core
cargo tree -p ecra-run
```

## Immediate next work

```text
A. finish lifecycle synchronization on canonical main: roadmap + platform status + spec index
B. require canonical planning/status head CI to remain healthy
C. create branch `002-durable-run-ledger` from exact canonical main
D. update branch-local lifecycle to IMPLEMENTING and open Draft PR
E. execute T001 then T002... in dependency order
F. after each material phase, require exact-head CI and repair any actual failure
G. complete T067–T070 convergence/analyze after implementation
H. mark Ready only on exact-head green + clean review state
I. merge with expected head, require post-merge main ECR-002 CI
J. only then T073 / CLOSED_CANONICAL and re-read roadmap for next eligible slice
```

## Non-negotiable inherited invariants

```text
Actor != authenticated Principal
CapabilityRequest != CapabilityGrant
classification != permission
InformationUse != authorization
ActionDigest != signature/approval
ActionIntent != ActionAttemptRef != ActionReceipt != VerificationReceipt
executor_observed_success != verified
UNKNOWN remains UNKNOWN
projection != authoritative event history
LedgerDigest != authentication/signature/MAC/VerificationReceipt
budget != authority
`.ecra` != protected secret container
```
