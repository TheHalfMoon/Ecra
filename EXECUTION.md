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
Lifecycle: IMPLEMENTING / PHASE_9_FINAL_READINESS
Branch: 002-durable-run-ledger
Canonical planning base: 5caf5dc4e7f26d07fabac3333713a44f0af22ea1
PR: #2 OPEN / DRAFT

Phase 1 T001–T008: VERIFIED_ON_BRANCH
Phase 2 T009–T018: VERIFIED_ON_BRANCH
Phase 3 T019–T026: VERIFIED_ON_BRANCH
Phase 4 T027–T034: VERIFIED_ON_BRANCH
Phase 5 T035–T044: VERIFIED_ON_BRANCH
Phase 6 T045–T051: VERIFIED_ON_BRANCH
Phase 7 T052–T059: VERIFIED_ON_BRANCH
Phase 8 T060–T066: VERIFIED_ON_BRANCH
Phase 9 T067–T070: COMPLETE_ON_BRANCH
Phase 9 T071: ACTIVE
Phase 9 T072–T073: BLOCKED_BY_DEPENDENCY_ORDER

Phase 8 ledger head: e86e1822e621c0563f2764fe784902e3204b0085
Phase 8 CI: 33152251783 — SUCCESS
Convergence verified head: 84d8cb5a8c0a28ab7adba42d2cd049e014c8f368
Convergence CI: 33153174953 — SUCCESS
Convergence job: 98789740534 — SUCCESS

T067 traceability mapping: COMPLETE_ON_BRANCH
T068 constitution/risk re-check: COMPLETE_ON_BRANCH
T069 post-implementation analyze: COMPLETE_ON_BRANCH
T070 convergence: COMPLETE_ON_BRANCH
T071 final exact-head readiness: ACTIVE
T072 merge/post-merge main gate: BLOCKED_BY_T071
T073 canonical closure: BLOCKED_BY_T072
```

ECR-002 implementation remains authorized only inside its local/synthetic/non-sensitive durability scope. Real sensitive persistence, authentication/trust roots, authorization/declassification, independent verification/reconciliation, provider execution, distributed workflow infrastructure and multi-device sync remain outside this slice.

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
specs/002-durable-run-ledger/traceability-closure.md
specs/002-durable-run-ledger/post-implementation-analyze.md
specs/002-durable-run-ledger/checklists/requirements.md
```

Current post-implementation result:

```text
FR-001–FR-057 PASS
SC-001–SC-014 PASS
SC-015 PASS_BASELINE / FINAL_FEATURE_AND_POST_MERGE_MAIN_REQUIRED
SC-016 PASS_TRACEABILITY_AND_CONVERGENCE
G1–G15 PASS / explicit PASS-N/A
UNOWNED_FR=0
UNOWNED_SC=0
FAILED_CONSTITUTION_GATES=0
IMPLICITLY_ACCEPTED_CRITICAL_RISKS=0
MUST_LEVEL_IMPLEMENTATION_DEFECTS_FOUND=0
```

## Fixed implementation decisions

```text
authoritative truth     append-only RunEventEnvelope history
ordering                EventSequence only
projection              rebuildable/non-authoritative RunState cache
attempt safety          committed AttemptPrepared before provider invocation
missing receipt         UNKNOWN / reconciliation-required
integrity               domain-separated RFC8785 + SHA-256 LedgerDigest
local store             SQLite via rusqlite 0.40.2
SQLite engine           bundled SQLite 3.53.2 via libsqlite3-sys 0.38.2
SQLite durability       WAL + synchronous=FULL, asserted at open
write transaction       Immediate + expected-head compare
budget accounting       typed checked I-JSON-safe integers
portable artifact       deterministic strict Stored-only ZIP via zip 8.6.0
ecra-run unsafe         forbidden in Ecra-authored Rust
archive/store fixtures  synthetic/non-sensitive only
hostile rewrite claim   not provided by plain hash chain
Cargo.lock SHA-256      b720472bf40a554ab61afb74eae95dd625bc6b2604e47a632991faea630e42c6
```

## Active task order

```text
T071 final feature-head readiness:
  - require full ECR-002 CI SUCCESS on the exact ledger head that records T070 complete
  - require PR head equals the verified head
  - require mergeable state
  - require no unresolved reviews or inline review threads
  - classify conversation comments and require no actionable blocker
  - move PR out of Draft only after the exact-head gate is green
  - re-check reviews/comments/checks after Ready-for-review transition

T072 after T071:
  - merge exact expected head using a non-rebase method
  - require canonical-main ECR-002 CI SUCCESS

T073 after T072:
  - record exact merge/post-merge evidence
  - mark ECR-002 CLOSED_CANONICAL
  - converge roadmap/platform status/EXECUTION
  - identify next genuinely dependency-eligible slice from canonical main
```

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

The permanent workflow also records exact dependency/toolchain evidence and keeps explicit archive/boundaries/portability targets in the gate.

## Execution rule

Continue T001–T073 in dependency order. Fix actual CI/review blockers and immediately resume. Do not weaken tests or boundaries to make a gate green. No force-push, rebase or destructive history rewriting. Never mark PASS, MERGED or `CLOSED_CANONICAL` without exact-head/post-merge evidence.

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
