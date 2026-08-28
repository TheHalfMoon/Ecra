# Ecra Execution Guide

> **Operational start-here document.** Recover live work from this file, platform roadmap/status, the active slice package, and exact GitHub truth; do not depend on private chat state.

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
Lifecycle: IMPLEMENTING
Branch: 002-durable-run-ledger
Canonical planning base: 5caf5dc4e7f26d07fabac3333713a44f0af22ea1
Planning/status CI: 33103802150 — SUCCESS
Phase 1 verified head: 4577123486fcaf856a3640aeacb3b7dcee733cc3
Phase 1 CI: 33105751992 — SUCCESS
Phase 2 verified head: 2ab8d6d80f43bf7dd07ee43659555a573c47021b
Phase 2 CI: 33107289499 — SUCCESS
Current phase: Phase 3 T019–T026
PR: #2 OPEN / DRAFT / mergeable at last live check
```

ECR-002 implementation is authorized only inside its local/synthetic/non-sensitive durability scope. Real sensitive persistence, authentication/trust roots, authorization/declassification, independent verification/reconciliation, provider execution, distributed workflow infrastructure and multi-device sync remain outside this slice.

## ECR-002 package

Read in order:

```text
specs/002-durable-run-ledger/STATUS.md
specs/002-durable-run-ledger/spec.md
specs/002-durable-run-ledger/research.md
specs/002-durable-run-ledger/data-model.md
specs/002-durable-run-ledger/contracts/run-ledger-v1.md
specs/002-durable-run-ledger/implementation-clarifications.md
specs/002-durable-run-ledger/threat-model.md
specs/002-durable-run-ledger/plan.md
specs/002-durable-run-ledger/tasks.md
specs/002-durable-run-ledger/quickstart.md
specs/002-durable-run-ledger/analyze.md
specs/002-durable-run-ledger/checklists/requirements.md
```

Planning result remains:

```text
FR-001–FR-057 OWNED
SC-001–SC-016 OWNED
G1–G15 PASS / explicit N/A
ZERO_BLOCKING_PLANNING_DRIFT_FOUND
```

## Fixed architecture decisions

```text
authoritative truth     append-only RunEventEnvelope history
ordering                EventSequence only
projection              rebuildable/non-authoritative RunState cache
attempt safety          committed AttemptPrepared before provider invocation
missing receipt         UNKNOWN / reconciliation-required
integrity               domain-separated RFC8785 + SHA-256 LedgerDigest
local store             SQLite via bounded rusqlite adapter
SQLite durability       WAL + synchronous=FULL, asserted at open
write transaction       Immediate + expected-head compare
budget accounting       typed I-JSON-safe checked integers
portable artifact       deterministic strict Stored-only ZIP `.ecra`
archive/store fixtures  synthetic/non-sensitive only
hostile rewrite claim   not provided by plain hash chain
```

## Active task order

```text
T019 derived RunState / PreparedAttemptState / ordered projections
T020 pure RunReducer
T021 exact phase transition matrix and terminal rejection
T022 exact attempt identity and receipt binding
T023 recovery boundary -> unresolved without fabricated outcome
T024 resume blockers
T025 exhaustive transition tests
T026 1,000x deterministic replay property test
```

Phase 4 and Phase 5 become eligible only after the complete ECR-002 exact-head gate verifies Phase 3.

## CI architecture

The repository-scoped self-hosted macOS runner `macbook` remains the trusted execution oracle. Persistent personal runners must not execute untrusted fork PR code.

The trusted ECR-002 workflow is push-only for:

```text
push: 002-durable-run-ledger
push: main
workflow_dispatch
runs-on: self-hosted
permissions: contents: read
```

The workflow runs the full workspace gate, ECR-001 regression boundary and ECR-002 dedicated tests/checkers.

## Full target verification surface

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

Only targets that exist for the current implementation phase are invoked by the phase-specific ECR-002 workflow; later task-owned targets join the gate when their owning phase lands.

## Execution rule

Continue T001–T073 in dependency order. Fix actual CI/review blockers and immediately resume. Do not weaken tests or boundaries to make a gate green. No force-push, rebase or destructive history rewriting.

## Non-negotiable invariants

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
