# Tasks: Durable Run, Ledger & Budgets

**Feature:** ECR-002  
**Status:** TASKS_READY  
**Canonical inputs:** `spec.md`, `research.md`, `data-model.md`, `contracts/run-ledger-v1.md`, `threat-model.md`, `plan.md`, `quickstart.md`, `analyze.md`  
**Base:** canonical `main` after ECR-001 `CLOSED_CANONICAL`

`[x]` means complete on the ECR-002 implementation branch with required evidence; it does not mean `CLOSED_CANONICAL`.

## Phase 1 — Workspace, crate and CI boundaries

- [x] T001 Add `crates/ecra-run` to the root workspace while preserving `crates/ecra-core` boundaries. **Paths:** `Cargo.toml`, `crates/ecra-run/Cargo.toml`, `crates/ecra-run/src/lib.rs`.
- [x] T002 Add reviewed exact dependency candidates with minimal features: `rusqlite` bundled SQLite, `zip` Stored-only profile support, workspace-aligned serialization/digest/error dependencies. **Paths:** `crates/ecra-run/Cargo.toml`, `Cargo.lock`.
- [x] T003 Add `#![forbid(unsafe_code)]` and crate-level architecture/misuse docs. **Paths:** `crates/ecra-run/src/lib.rs`, `crates/ecra-run/README.md`. **FR-054–FR-057**
- [x] T004 Add ECR-002 dependency allowlist/native-boundary checker without weakening ECR-001 checks. **Paths:** `scripts/check-run-deps.sh`, existing `scripts/check-core-deps.sh`. **SC-014**
- [x] T005 Add Ecra-owned unsafe/source-I/O boundary checker for `ecra-run`. **Paths:** `scripts/check-run-unsafe.sh`. **SC-013, SC-014**
- [x] T006 Add trusted push-only ECR-002 workflow for branch `002-durable-run-ledger` and `main`, mirroring workspace/core gates plus ECR-002 gates. **Path:** `.github/workflows/ecr-002.yml`. **SC-015**
- [x] T007 Record exact locked dependency/license/provenance delta including bundled SQLite and zip; no source-copy claim. **Path:** `research/donor-license-ledger.md`. **G10, FR-055**
- [x] T008 Verify baseline build/fmt/Clippy/tests/core gates on the first workspace head before semantic implementation. **Paths:** CI evidence + `specs/002-durable-run-ledger/STATUS.md`.

## Phase 2 — Errors, primitives, events and ledger digest

- [x] T009 Implement typed `RunErrorCategory`, `RunErrorCode`, `RunError` exactly from contract. **Path:** `crates/ecra-run/src/error.rs`. **FR-053**
- [x] T010 Implement `EventSequence` and `BudgetAmount` I-JSON-safe checked wrappers. **Paths:** `crates/ecra-run/src/event.rs`, `crates/ecra-run/src/budget.rs`. **FR-003, FR-032–FR-034**
- [x] T011 Implement distinct `LedgerDigest` and domain-separated RFC8785+SHA-256 event digest. **Path:** `crates/ecra-run/src/digest.rs`. **FR-019–FR-022**
- [x] T012 Implement strict `RunPhase`, `SuspensionReason`, `RunErrorSummary`. **Paths:** `crates/ecra-run/src/state.rs`, `crates/ecra-run/src/error.rs`. **FR-006–FR-009**
- [x] T013 Implement strict `RunEvent` tagged enum and all v1 event bodies. **Path:** `crates/ecra-run/src/event.rs`. **FR-001–FR-017, FR-031–FR-045**
- [x] T014 Implement strict `RunEventEnvelope` version/sequence/previous-digest/event-digest validation. **Paths:** `crates/ecra-run/src/event.rs`, `crates/ecra-run/src/digest.rs`. **FR-001–FR-005, FR-019–FR-021**
- [x] T015 Add fixed canonical event bytes + LedgerDigest golden. **Paths:** `contracts/ecra-run-v1/expected/run-created-golden.v1.jcs`, `contracts/ecra-run-v1/expected/run-created-golden.sha256`, `crates/ecra-run/tests/event_contract.rs`. **SC-001, SC-003, SC-006**
- [x] T016 Add valid event/envelope fixtures for every event kind. **Paths:** `contracts/ecra-run-v1/valid/*.json`, `crates/ecra-run/tests/event_contract.rs`. **SC-001**
- [x] T017 Add invalid version/sequence/unknown-field/digest/previous-link/cross-run fixtures. **Paths:** `contracts/ecra-run-v1/invalid/*.json`, `crates/ecra-run/tests/event_contract.rs`. **SC-002, SC-006**
- [x] T018 Add error-category/code coverage without display-string parsing. **Path:** `crates/ecra-run/tests/event_contract.rs`. **FR-053, SC-002**

## Phase 3 — Pure reducer and run state machine

- [x] T019 Implement derived `RunState`, `PreparedAttemptState`, ordered usage/attempt collections. **Path:** `crates/ecra-run/src/state.rs`. **FR-006–FR-010, FR-017**
- [x] T020 Implement pure `RunReducer` with no clock/random/I/O/environment/process access. **Path:** `crates/ecra-run/src/state.rs`. **FR-004–FR-010, FR-045**
- [x] T021 Implement exact phase transition matrix including terminal-state rejection. **Paths:** `crates/ecra-run/src/state.rs`, `crates/ecra-run/tests/reducer.rs`. **FR-006–FR-009, FR-040–FR-045**
- [x] T022 Implement attempt uniqueness and exact receipt/action/attempt binding in reducer. **Paths:** `crates/ecra-run/src/state.rs`, `crates/ecra-run/tests/attempts.rs`. **FR-011–FR-017**
- [x] T023 Implement recovery-boundary reduction that marks prepared-without-receipt attempts unresolved without fabricating outcome. **Paths:** `crates/ecra-run/src/recovery.rs`, `crates/ecra-run/src/state.rs`, `crates/ecra-run/tests/attempts.rs`. **FR-014–FR-016, FR-043–FR-045**
- [x] T024 Implement v1 resume blockers for budget/reconciliation/cancellation suspension. **Paths:** `crates/ecra-run/src/state.rs`, `crates/ecra-run/tests/reducer.rs`. **FR-008, FR-041, FR-044**
- [x] T025 Add exhaustive valid/invalid transition-table tests. **Path:** `crates/ecra-run/tests/reducer.rs`. **SC-001, SC-002**
- [x] T026 Add deterministic replay/property test: same accepted history reduced 1,000 times yields identical canonical state bytes/digest. **Paths:** `crates/ecra-run/tests/reducer.rs`, `crates/ecra-run/tests/portability.rs`. **SC-003**

## Phase 4 — Budgets and bounded execution

- [x] T027 Implement exact `BudgetDimension`, `BudgetLimit`, `RunBudget`, `BudgetUsage`. **Path:** `crates/ecra-run/src/budget.rs`. **FR-031–FR-039**
- [x] T028 Reject duplicate dimensions, soft>hard, malformed/negative/out-of-range values. **Paths:** `crates/ecra-run/src/budget.rs`, `crates/ecra-run/tests/budgets.rs`. **FR-032–FR-034**
- [x] T029 Implement checked cumulative accounting and remaining-budget calculation. **Paths:** `crates/ecra-run/src/budget.rs`, `crates/ecra-run/src/state.rs`. **FR-032, FR-035**
- [x] T030 Implement known-upper-bound preflight refusal. **Paths:** `crates/ecra-run/src/budget.rs`, `crates/ecra-run/tests/budgets.rs`. **FR-036**
- [x] T031 Implement soft-limit first-crossing validation and hard-exhaustion suspension semantics. **Paths:** `crates/ecra-run/src/budget.rs`, `crates/ecra-run/src/state.rs`, `crates/ecra-run/tests/budgets.rs`. **FR-037–FR-039**
- [x] T032 Add boundary/property tests for all 14 dimensions at zero/soft/hard/MAX_SAFE_INTEGER/overflow. **Path:** `crates/ecra-run/tests/budgets.rs`. **SC-009**
- [x] T033 Add recursive/tool-loop fixture stopped deterministically by hard budget with durable exhaustion evidence. **Path:** `crates/ecra-run/tests/budgets.rs`. **SC-010**
- [x] T034 Prove budget exhaustion never clears unresolved attempt or changes ECR-001 retry semantics. **Paths:** `crates/ecra-run/tests/budgets.rs`, `crates/ecra-run/tests/attempts.rs`. **FR-037–FR-039**

## Phase 5 — SQLite schema, migrations, append and projections

- [x] T035 Implement SQLite open/configuration adapter and read-back assertion for WAL/FULL/foreign_keys/trusted_schema. **Path:** `crates/ecra-run/src/sqlite.rs`. **FR-024–FR-028**
- [x] T036 Implement deterministic schema v1 (`run_events`, `run_heads`, `artifact_blobs`) with STRICT/check/unique constraints and UPDATE/DELETE-deny triggers. **Paths:** `crates/ecra-run/src/migration.rs`, `crates/ecra-run/tests/migration.rs`. **FR-018, FR-023–FR-030**
- [x] T037 Implement database schema-version check: create v1, reject newer, transactionally migrate supported older fixtures. **Paths:** `crates/ecra-run/src/migration.rs`, `contracts/ecra-run-v1/migrations/*`, `crates/ecra-run/tests/migration.rs`. **FR-001, FR-002, FR-027**
- [x] T038 Implement `ExpectedRunHead` and atomic append with Immediate transaction/expected-head check. **Paths:** `crates/ecra-run/src/store.rs`, `crates/ecra-run/src/sqlite.rs`. **FR-023, FR-026**
- [x] T039 Ensure append validates canonical envelope/reducer result before authoritative commit and atomically updates projection. **Paths:** `crates/ecra-run/src/store.rs`, `crates/ecra-run/src/sqlite.rs`, `crates/ecra-run/tests/sqlite_store.rs`. **FR-008–FR-010, FR-021–FR-023**
- [x] T040 Implement strict history load ordered only by EventSequence with full chain verification. **Paths:** `crates/ecra-run/src/store.rs`, `crates/ecra-run/tests/sqlite_store.rs`. **FR-003–FR-005, FR-021**
- [x] T041 Implement projection deletion/rebuild from authoritative events with publish-after-full-validation. **Paths:** `crates/ecra-run/src/store.rs`, `crates/ecra-run/tests/sqlite_store.rs`. **FR-010, SC-008**
- [x] T042 Implement synthetic content-addressed blob put/get with ECR-001 ContentDigest/size validation and storage-budget hook. **Paths:** `crates/ecra-run/src/store.rs`, `crates/ecra-run/tests/sqlite_store.rs`. **FR-029, FR-030**
- [x] T043 Add ordinary UPDATE/DELETE rejection tests and corruption/chain mismatch load failures. **Path:** `crates/ecra-run/tests/sqlite_store.rs`. **FR-018–FR-023, SC-006**
- [x] T044 Add process-crash persistence test for committed SQLite events and assert WAL+FULL configuration. **Path:** `crates/ecra-run/tests/crash_recovery.rs`. **SC-007**

## Phase 6 — Attempt guard, recovery and concurrency

- [x] T045 Implement `prepare_attempt` store API returning only after durable commit. **Paths:** `crates/ecra-run/src/store.rs`, `crates/ecra-run/src/recovery.rs`. **FR-011–FR-012**
- [x] T046 Implement `record_receipt` exact-binding store API. **Paths:** `crates/ecra-run/src/store.rs`, `crates/ecra-run/src/recovery.rs`. **FR-013**
- [x] T047 Implement recovery scan + explicit recovery-boundary append before continued scheduling. **Paths:** `crates/ecra-run/src/recovery.rs`, `crates/ecra-run/tests/crash_recovery.rs`. **FR-014–FR-016, FR-043–FR-045**
- [x] T048 Implement retry guard preserving ECR-001 idempotency/retry classes and blocking blind unresolved retries. **Paths:** `crates/ecra-run/src/recovery.rs`, `crates/ecra-run/tests/attempts.rs`. **FR-016, FR-044**
- [x] T049 Add crash matrix A–D covering before-preparation, after-preparation, after simulated external effect/before receipt, and after receipt commit. **Path:** `crates/ecra-run/tests/crash_recovery.rs`. **SC-004**
- [x] T050 Add multiple-attempt/one-action tests proving attempts remain distinct and receipts cannot cross-bind. **Path:** `crates/ecra-run/tests/attempts.rs`. **FR-012–FR-017**
- [x] T051 Add two-connection expected-head concurrency test proving exactly one competing append succeeds. **Path:** `crates/ecra-run/tests/sqlite_store.rs`. **SC-005**

## Phase 7 — Deterministic `.ecra` archive

- [ ] T052 Implement strict manifest/path/entry metadata model and v1 hard parser limits. **Path:** `crates/ecra-run/src/archive.rs`. **FR-046–FR-052**
- [ ] T053 Implement deterministic Stored-only writer with fixed timestamp/permissions/no comments/encryption/symlinks/directories and stable entry order. **Paths:** `crates/ecra-run/src/archive.rs`, `crates/ecra-run/tests/archive.rs`. **FR-046–FR-050**
- [ ] T054 Implement archive reader preflight for path, duplicate, method/encryption/symlink, count and size limits before trusted materialization. **Paths:** `crates/ecra-run/src/archive.rs`, `crates/ecra-run/tests/archive.rs`. **FR-049**
- [ ] T055 Implement manifest-entry whitelist, ContentDigest/size checks, strict event parse, full ledger validation and reducer validation before import. **Paths:** `crates/ecra-run/src/archive.rs`, `crates/ecra-run/tests/archive.rs`. **FR-048–FR-051**
- [ ] T056 Add deterministic export golden/hash: identical logical content produces byte-identical archive bytes. **Paths:** `contracts/ecra-run-v1/expected/`, `crates/ecra-run/tests/archive.rs`. **SC-011**
- [ ] T057 Add malicious archive corpus: absolute/traversal/backslash/NUL/duplicate/symlink/unsupported compression/encryption/count/size breaches. **Path:** `crates/ecra-run/tests/archive.rs`. **SC-012**
- [ ] T058 Add malformed manifest/event/content/ledger digest mismatch import failures. **Path:** `crates/ecra-run/tests/archive.rs`. **SC-002, SC-012**
- [ ] T059 Prove archive never contains/exports live SQLite database/WAL files and import/export fixtures remain synthetic/non-sensitive. **Paths:** `crates/ecra-run/tests/archive.rs`, `crates/ecra-run/tests/boundaries.rs`. **FR-028, FR-030, FR-048, FR-052, SC-013**

## Phase 8 — Cross-cutting portability, security and documentation gates

- [ ] T060 Add production-source scan proving reducer/canonical archive logic has no OS clock/random/network/environment/process dependency and library has no network/provider call surface. **Path:** `crates/ecra-run/tests/boundaries.rs`. **FR-005, FR-056, SC-013**
- [ ] T061 Prove `ecra-core` dependency/zero-I/O/zero-unsafe gates remain green after workspace/dependency expansion. **Paths:** existing core scripts + `.github/workflows/ecr-002.yml`. **FR-054, SC-014**
- [ ] T062 Add `ecra-run` unsafe/dependency boundary gates including explicit native SQLite rationale. **Paths:** `scripts/check-run-unsafe.sh`, `scripts/check-run-deps.sh`, `.github/workflows/ecr-002.yml`. **FR-055**
- [ ] T063 Add strict portability tests for LF/CRLF/JSON formatting input where semantically applicable and deterministic reducer/archive output. **Path:** `crates/ecra-run/tests/portability.rs`. **SC-003, SC-011**
- [ ] T064 Audit committed fixtures for synthetic/non-sensitive content and docs for no hostile-tamper/verification/authorization overclaim. **Paths:** `crates/ecra-run/tests/boundaries.rs`, `crates/ecra-run/README.md`, `specs/002-durable-run-ledger/threat-model.md`. **FR-030, FR-057, SC-013**
- [ ] T065 Add run architecture map and misuse warnings covering receipt!=verification, Actor!=Principal, ledger digest!=authenticity, unresolved!=retryable, projection!=truth, budget!=authority, `.ecra`!=protected secret container. **Path:** `crates/ecra-run/README.md`.
- [ ] T066 Run full exact-head quickstart gate and record dependency versions/licenses/SQLite version/archive library configuration. **Paths:** `specs/002-durable-run-ledger/STATUS.md`, `research/donor-license-ledger.md`. **SC-015**

## Phase 9 — Traceability, convergence, PR and canonical closure

- [ ] T067 Map FR-001–FR-057 and SC-001–SC-016 to implementation/test/contract evidence. **Path:** `specs/002-durable-run-ledger/traceability-closure.md`. **SC-016**
- [ ] T068 Re-check constitution G1–G15 and platform risks R-006/R-019/R-033/R-039/R-042/R-052/R-053. **Path:** `specs/002-durable-run-ledger/traceability-closure.md`.
- [ ] T069 Run post-implementation analyze-equivalent consistency review; create convergence tasks for any MUST-level drift rather than hiding it. **Path:** `specs/002-durable-run-ledger/post-implementation-analyze.md`. **SC-016**
- [ ] T070 Converge spec/data-model/contract/plan/quickstart/tasks/status/EXECUTION with exact implementation truth. **Paths:** `specs/002-durable-run-ledger/*`, `EXECUTION.md`, platform status/roadmap as lifecycle changes.
- [ ] T071 Require complete exact-head CI on final feature head, clean review threads/checks, and no actionable blocker before merge. **SC-015**
- [ ] T072 Merge with exact expected head using a non-rebase method; require post-merge canonical-main ECR-002 CI. **SC-015**
- [ ] T073 Mark ECR-002 `CLOSED_CANONICAL` only after merge + post-merge evidence; update roadmap/platform status/EXECUTION and identify the next genuinely dependency-eligible slice.

## Dependency graph

```text
T001–T008 foundation
  ↓
T009–T018 contract primitives
  ↓
T019–T026 reducer
  ├───────────────┐
  ↓               ↓
T027–T034 budgets T035–T044 SQLite
        \          /
         T045–T051 recovery/concurrency
                 ↓
            T052–T059 archive
                 ↓
            T060–T066 gates
                 ↓
            T067–T073 closure
```

No ECR-031/ECR-004/ECR-003 implementation becomes eligible merely from partial ECR-002 progress; roadmap dependencies require ECR-002 `CLOSED_CANONICAL`.
