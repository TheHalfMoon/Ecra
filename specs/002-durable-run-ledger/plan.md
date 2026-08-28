# Implementation Plan: Durable Run, Ledger & Budgets

**Feature:** ECR-002  
**Lifecycle:** IMPLEMENTED_PENDING_CLOSURE  
**Base dependency:** ECR-001 `CLOSED_CANONICAL`  
**Language:** Rust 1.98 / Edition 2024

## 1. Objective

Implement a bounded local-first durable execution substrate that owns run history, attempt preparation/recovery, resource budgets, SQLite persistence, and deterministic `.ecra` fixture portability while preserving all ECR-001 trust boundaries.

## 2. Implemented repository structure

```text
crates/ecra-run/
  Cargo.toml
  README.md
  src/
    lib.rs
    error.rs
    event.rs
    digest.rs
    state.rs
    budget.rs
    recovery.rs
    store.rs
    sqlite.rs
    migration.rs
    archive.rs
  tests/
    event_contract.rs
    reducer.rs
    attempts.rs
    budgets.rs
    sqlite_store.rs
    migration.rs
    crash_recovery.rs
    archive.rs
    portability.rs
    boundaries.rs

contracts/ecra-run-v1/
  valid/
  invalid/
  expected/
  migrations/

scripts/
  check-run-deps.sh
  check-run-unsafe.sh

.github/workflows/ecr-002.yml
```

No new I/O enters `crates/ecra-core`.

## 3. Crate boundaries

### `ecra-core`

Unchanged semantic owner of ECR-001 domain values. Remains zero-I/O and dependency-bounded.

### `ecra-run`

Owns:
- event/reducer/budget/recovery semantics;
- LedgerDigest;
- SQLite store adapter;
- deterministic archive reader/writer;
- schema migrations;
- local durable run APIs.

Does not own:
- principal authentication/trust roots;
- authorization/declassification/approval;
- provider execution;
- independent verification/reconciliation decision logic;
- encryption/secret storage.

## 4. Locked dependency state

Exact implemented runtime boundary:

```text
path dependency: ecra-core
serde / serde_json / serde_jcs / sha2 / thiserror (workspace-aligned lock)
rusqlite = 0.40.2, default-features = false, features = ["bundled"]
libsqlite3-sys = 0.38.2 (transitive native boundary)
bundled SQLite = 3.53.2
zip = 8.6.0, default-features = false
```

Dev-only locked dependencies include `tempfile 3.27.0` and workspace-aligned `proptest 1.11.0`.

The implementation-time exact dependency evidence is recorded in `research/donor-license-ledger.md` and verified by `.github/workflows/ecr-002.yml`. `Cargo.lock` SHA-256 at the Phase 8 verified baseline is `b720472bf40a554ab61afb74eae95dd625bc6b2604e47a632991faea630e42c6`.

Why SQLite rather than redb:
- direct schema/check/unique constraints;
- ordered event queries;
- mature transactional migration model;
- standard inspection tooling;
- less custom persistence logic in Ecra.

Why bundled SQLite:
- consistent reviewed version across supported machines;
- no dependency on missing/old system SQLite;
- native boundary isolated outside ecra-core.

## 5. SQLite configuration

On create/open:

```text
journal_mode=WAL
synchronous=FULL
foreign_keys=ON
trusted_schema=OFF
```

The adapter reads back critical values and fails closed if configuration does not match the contract.

Write transaction:

```text
rusqlite TransactionBehavior::Immediate
```

Atomic append performs expected-head validation, event insertion, reducer evaluation and projection update in the same transaction.

No external action/provider call is permitted while the transaction is open.

## 6. Authoritative data model

`run_events` is authoritative. `run_heads` is rebuildable.

Invariant:

```text
same accepted run event history
  => same reduced RunState
  => same canonical state bytes/digest
```

Rebuild deletes/replaces projections only after full chain/reducer validation.

## 7. Attempt-before-effect API

The persistence layer exposes a guard/receipt pattern rather than executing providers:

```text
prepare_attempt(expected_head, attempt, recorded_at)
  -> committed PreparedAttemptGuard

record_receipt(expected_head, receipt, recorded_at)
  -> committed updated RunState
```

A later executor may invoke a provider only after receiving the committed preparation result. Recovery scans prepared-without-receipt attempts before allowing scheduling.

No `execute()` or generic callback that runs arbitrary provider work inside a database transaction is introduced.

## 8. Reducer design

Reducer is pure:

```text
reduce(previous_state, RunEventEnvelope) -> Result<RunState, RunError>
```

Forbidden inside reducer:
- system clock;
- random;
- filesystem/database/network;
- environment variables;
- process execution;
- model/tool/browser calls.

Reducer validates:
- event sequence/order;
- phase transition;
- attempt uniqueness/binding;
- receipt binding;
- unresolved-attempt blockers;
- budget threshold consistency;
- checked usage arithmetic.

## 9. Budget implementation

Typed enum dimensions exactly match contract. `BudgetAmount` is a checked I-JSON-safe wrapper.

Helpers:

```text
preflight(dimension, declared_upper_bound)
charge_usage(dimension, actual_amount)
remaining(dimension)
```

`charge_usage` persists usage and, when threshold crossings occur, appends derived soft/exhaustion events in the same write transaction with contiguous ledger sequence/digests.

No ambient budget increase API exists in v1.

## 10. Crash/recovery testing strategy

Tests cover four attempt boundaries:

```text
A before attempt-prepared commit
B after prepared commit before provider marker
C after simulated external effect before receipt commit
D after receipt commit
```

A child integration-test process is used where process termination is needed. Test-only process execution is allowed; production `ecra-run` contains no process/provider execution.

Recovery assertions:
- A: no durable attempt;
- B/C: prepared without receipt becomes unresolved after recovery boundary;
- C never auto-retries/fabricates result;
- D recovers exact receipt.

## 11. Concurrent append strategy

Two independent SQLite connections use the same expected run head. Exactly one may append sequence `n+1`; the other receives a typed head/busy conflict and must re-read before deciding anything.

No last-write-wins mutation of authoritative history.

## 12. Archive design

Archive writer takes validated logical run history/blobs, not raw SQLite files.

Profile:
- ZIP Stored method only;
- no encryption/comments/symlinks/directories;
- fixed metadata/time/permissions;
- canonical JSON;
- stable entry order.

Reader validates names/features/count/size before materialization, then manifest digests, then event chain/reducer.

No generic extraction-to-directory API exists; the reader returns validated logical content through the bounded archive/store interface.

## 13. Migration strategy

Initial DB schema v1 is represented by explicit migration code and fixed migration fixtures. Future migrations must:
- run transactionally;
- reject newer unsupported schema;
- preserve authoritative event bytes/meaning unless an explicit event migration exists;
- include before/after fixture tests;
- leave old store unchanged on failure.

## 14. Security and privacy boundaries

- all Ecra-owned production Rust in `ecra-run` uses `#![forbid(unsafe_code)]`;
- native SQLite is a reviewed dependency boundary, not trusted core code;
- archive parser has hard count/size/path/method limits;
- no network or telemetry;
- no real sensitive acceptance fixtures;
- hash chain is not marketed as hostile tamper resistance;
- Actor attribution remains distinct from authenticated Principal;
- receipts remain distinct from VerificationReceipt.

## 15. Constitution gates G1–G15

| Gate | Implementation disposition |
|---|---|
| G1 Domain coherence | PASS — reuses ECR-001 types; new run-only types have single owner |
| G2 Authority | PASS — no authorization/grant synthesis; persistence does not grant execution |
| G3 Provenance | PASS/N/A — execution history preserves typed refs/receipts; no new Fact truth model |
| G4 Side effects | PASS — durable attempt-before-effect, UNKNOWN and retry guard explicit |
| G5 Verification | PASS — execution_completed/receipt never equal verified; no verifier implementation |
| G6 Durability | PASS — event replay, crash recovery, migrations, projections defined and tested |
| G7 Privacy/secrets | PASS — synthetic/non-sensitive v1 gate; no protected-storage claims |
| G8 Local-first | PASS — local SQLite/archive only; no cloud dependency |
| G9 Interoperability | PASS — `.ecra` contract is bounded; no protocol/auth mapping |
| G10 Donor/license | PASS — exact implementation lock/license/native-boundary evidence recorded |
| G11 Upstream/browser | N/A — no browser patch/bridge |
| G12 Benchmarks | PASS — deterministic/replay/crash/concurrency/archive criteria are reproducible |
| G13 Information flow/egress | PASS — no network/remote sink; persistence does not authorize future disclosure |
| G14 Identity/principal | PASS — Actor retained as attribution only; ECR-031 remains owner |
| G15 Bounded execution | PASS — typed budgets and parser limits are binding and tested |

No constitutional gate is knowingly failed.

## 16. Complexity tracking

### Native SQLite dependency

Simpler alternative: pure-Rust KV (`redb`). Rejected for v1 because Ecra would own more schema/index/query/migration/constraint logic. Native dependency cost is isolated to `ecra-run` and justified by a smaller Ecra-owned persistence surface.

### ZIP dependency

Simpler alternative: custom framing. Rejected because inventing a container/parser increases interoperability and parser-security burden. A strict Stored-only ZIP subset is narrower and independently inspectable.

### Event sourcing plus projection

Simpler alternative: mutable current-state rows only. Rejected because it cannot provide deterministic restart/audit/attempt truth and conflicts with the constitution.

## 17. Implementation phases

```text
P1 workspace/crate/dependencies/CI boundaries                  VERIFIED_ON_BRANCH
P2 typed event/error/digest primitives                         VERIFIED_ON_BRANCH
P3 reducer/state-machine + fixtures                            VERIFIED_ON_BRANCH
P4 budgets/accounting                                          VERIFIED_ON_BRANCH
P5 SQLite schema/store/expected-head/projection rebuild        VERIFIED_ON_BRANCH
P6 attempt preparation/recovery/crash/concurrency              VERIFIED_ON_BRANCH
P7 deterministic .ecra archive/import/export                   VERIFIED_ON_BRANCH
P8 cross-cutting portability/security/documentation gates      VERIFIED_ON_BRANCH
P9 traceability/analyze/convergence/review/merge/closure        ACTIVE
```

Every implemented phase received exact-head CI evidence before the next semantic phase was authorized. T071–T073 retain the final exact-head, merge and post-merge closure gates.
