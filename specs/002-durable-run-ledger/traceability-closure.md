# ECR-002 Implementation Traceability / Closure Evidence

**Date:** 2026-08-28  
**Slice:** ECR-002 — Durable Run, Ledger & Budgets  
**Status:** T067_T068_COMPLETE_PENDING_CONVERGENCE  
**Phase 8 ledger head:** `e86e1822e621c0563f2764fe784902e3204b0085`  
**Phase 8 ledger CI:** `33152251783` — SUCCESS  
**Phase 8 ledger job:** `98786745867` — SUCCESS

This artifact satisfies T067 and T068 by mapping every ECR-002 functional requirement and success criterion to implemented repository evidence, then re-checking constitution v1.1.0 G1–G15 and the platform risks explicitly owned by this slice. It does not declare T069–T073, PR readiness, merge, or `CLOSED_CANONICAL`.

## Evidence index

### Production modules

```text
crates/ecra-run/src/error.rs
crates/ecra-run/src/event.rs
crates/ecra-run/src/digest.rs
crates/ecra-run/src/state.rs
crates/ecra-run/src/budget.rs
crates/ecra-run/src/recovery.rs
crates/ecra-run/src/store.rs
crates/ecra-run/src/sqlite.rs
crates/ecra-run/src/migration.rs
crates/ecra-run/src/archive.rs
```

### Primary tests

```text
crates/ecra-run/tests/event_contract.rs
crates/ecra-run/tests/reducer.rs
crates/ecra-run/tests/attempts.rs
crates/ecra-run/tests/budgets.rs
crates/ecra-run/tests/sqlite_store.rs
crates/ecra-run/tests/migration.rs
crates/ecra-run/tests/crash_recovery.rs
crates/ecra-run/tests/archive.rs
crates/ecra-run/tests/portability.rs
crates/ecra-run/tests/boundaries.rs
```

### Contract / fixture evidence

```text
contracts/ecra-run-v1/valid/
contracts/ecra-run-v1/invalid/
contracts/ecra-run-v1/expected/
contracts/ecra-run-v1/migrations/
specs/002-durable-run-ledger/contracts/run-ledger-v1.md
```

### Architecture / dependency gates

```text
.github/workflows/ecr-002.yml
scripts/check-core-unsafe.sh
scripts/check-core-deps.sh
scripts/check-run-unsafe.sh
scripts/check-run-deps.sh
crates/ecra-run/README.md
specs/002-durable-run-ledger/threat-model.md
research/donor-license-ledger.md
```

## FR-001–FR-057 implementation traceability

| FR | Primary implementation / evidence | Result |
|---|---|---|
| FR-001 | versioned `RunEventEnvelope`, archive manifest and SQLite schema handling in `event.rs`, `archive.rs`, `migration.rs`; contract fixtures | PASS |
| FR-002 | strict unsupported major/minor dispatch in event/archive/store migration paths; invalid fixtures/tests | PASS |
| FR-003 | `EventSequence` plus `(RunId, sequence)` authoritative SQLite key/order in `event.rs`, `migration.rs`, `store.rs` | PASS |
| FR-004 | `RunReducer` and history load order only by sequence; portability/reducer/store tests | PASS |
| FR-005 | caller-supplied `EpochMillis`; reducer/archive ambient-clock source scan in `boundaries.rs` | PASS |
| FR-006 | exact `RunPhase` enum and reducer state in `state.rs` | PASS |
| FR-007 | `execution_completed` runtime-only transition plus README/threat-model non-verification assertions | PASS |
| FR-008 | fail-closed transition matrix in `state.rs`; exhaustive `reducer.rs` cases | PASS |
| FR-009 | terminal-state no-reopen validation in reducer tests | PASS |
| FR-010 | authoritative events plus rebuildable `run_heads`/RunState projection in `store.rs`; rebuild equality tests | PASS |
| FR-011 | `RunStore::prepare_attempt` returns durable `PreparedAttemptGuard` only after committed event | PASS |
| FR-012 | duplicate/conflicting attempt binding rejection in reducer/store; `attempts.rs` | PASS |
| FR-013 | `record_receipt` plus exact ActionRef/ActionAttemptRef binding in reducer/store; `attempts.rs` | PASS |
| FR-014 | recovery boundary marks prepared-without-receipt unresolved/UNKNOWN without fabricated outcome | PASS |
| FR-015 | `RecoveryResult`/durable unresolved attempt state provides ECR-004 reconciliation handoff | PASS |
| FR-016 | `ensure_retry_allowed` preserves ECR-001 idempotency/retry classes and rejects blind retry | PASS |
| FR-017 | ordered prepared-attempt map keeps multiple attempts for one action distinct; binding tests | PASS |
| FR-018 | append-only `run_events`; UPDATE/DELETE deny triggers and mutation rejection tests | PASS |
| FR-019 | domain-separated RFC8785 + SHA-256 ledger digest in `digest.rs`; golden event bytes/hash | PASS |
| FR-020 | distinct `LedgerDigest` type and explicit non-signature/non-authentication docs/tests | PASS |
| FR-021 | strict full chain validation across event/store/archive paths; mutation/gap/binding tests | PASS |
| FR-022 | README/threat model explicitly scope plain hash-chain integrity and protected-anchor limitation | PASS |
| FR-023 | `ExpectedRunHead`, Immediate transaction and two-connection race proving one append wins | PASS |
| FR-024 | bounded SQLite adapter in `sqlite.rs`/`store.rs` | PASS |
| FR-025 | WAL + `synchronous=FULL` + read-back assertions; crash persistence test | PASS |
| FR-026 | eager Immediate write transaction with append/head projection in one transaction | PASS |
| FR-027 | `user_version` schema handling, newer-version rejection and deterministic migration fixture | PASS |
| FR-028 | archive layer has no SQLite/WAL/SHM export path; archive/boundary tests | PASS |
| FR-029 | content-addressed synthetic blob put/get with ECR-001 ContentDigest and byte-size/budget checks | PASS |
| FR-030 | synthetic/non-sensitive fixture policy plus secret-marker audit and explicit sensitive-state non-goal | PASS |
| FR-031 | exact 14 `BudgetDimension` variants in `budget.rs` | PASS |
| FR-032 | I-JSON-safe `BudgetAmount` and checked arithmetic; boundary/property tests | PASS |
| FR-033 | explicit hard limit and optional soft<=hard validation | PASS |
| FR-034 | duplicate/malformed/out-of-range/overflow rejection in budget tests | PASS |
| FR-035 | durable resource-usage events and reducer-computed cumulative usage | PASS |
| FR-036 | known-upper-bound preflight refusal in budget API/tests | PASS |
| FR-037 | post-use accounting reaches hard exhaustion without rewriting external effect | PASS |
| FR-038 | durable `budget_exhausted` evidence and suspended state | PASS |
| FR-039 | no v1 ambient budget increase/revision API; docs preserve future policy owner | PASS |
| FR-040 | cancellation request and terminal cancellation are distinct events/states | PASS |
| FR-041 | explicit suspend/resume transitions retain unresolved blockers | PASS |
| FR-042 | `intervention_recorded` preserves Actor attribution only; README/boundary non-authentication checks | PASS |
| FR-043 | `recover` scans prepared-without-receipt attempts before continued scheduling | PASS |
| FR-044 | reconciliation-required unresolved attempt blocks blind retry | PASS |
| FR-045 | pure replay/recovery idempotence; 1,000x deterministic reducer evidence | PASS |
| FR-046 | deterministic `.ecra` ZIP independent of live SQLite in `archive.rs` | PASS |
| FR-047 | Stored-only writer, fixed time/permissions/system, stable names/order, no comments/encryption/symlinks/directories | PASS |
| FR-048 | strict manifest + canonical events + optional content-addressed blobs; no SQLite/WAL entries | PASS |
| FR-049 | path/profile/duplicate/version/count/size/digest fail-closed reader preflight and malicious corpus | PASS |
| FR-050 | deterministic archive golden/hash plus LF/CRLF/compact-JSON portability producing identical archive bytes | PASS |
| FR-051 | imported events use the same strict envelope/chain/reducer validation | PASS |
| FR-052 | archive tests/fixtures remain synthetic/non-sensitive; `.ecra` explicitly not a protected secret container | PASS |
| FR-053 | typed `RunErrorCategory`/`RunErrorCode`; contract tests branch on codes rather than display text | PASS |
| FR-054 | `ecra-core` remains zero-I/O/dependency-bounded/unsafe-forbidden; core scripts run in every ECR-002 CI gate | PASS |
| FR-055 | exact-pinned `rusqlite 0.40.2` bundled and `zip 8.6.0` no-default-features; donor ledger and run dependency/unsafe gates | PASS |
| FR-056 | crate-wide source/dependency scan proves no network/provider/process/telemetry execution surface | PASS |
| FR-057 | no authentication/authorization/declassification/verification aggregation/provider execution/secret protection/trust-root ownership; README/threat-model executable non-claim audit | PASS |

## SC-001–SC-016 traceability

| SC | Primary evidence | Result |
|---|---|---|
| SC-001 | valid event/archive/store fixtures + strict parse/round-trip tests | PASS |
| SC-002 | invalid event/budget/archive/migration cases assert typed fail-closed codes | PASS |
| SC-003 | `portability.rs`: same accepted history reduced 1,000 times to identical canonical state digest; formatting variants preserve output | PASS |
| SC-004 | `crash_recovery.rs` matrix A–D covers before prepare, after prepare, ambiguous external-effect window and after receipt | PASS |
| SC-005 | two independent SQLite connections race same expected head; exactly one append succeeds | PASS |
| SC-006 | event/store tests reject sequence gaps/reorder/duplicate/digest/run substitution | PASS |
| SC-007 | child-process persistence + WAL/FULL read-back assertions | PASS |
| SC-008 | projection deletion/rebuild yields byte-equivalent derived state | PASS |
| SC-009 | all 14 budget dimensions exercised at zero/soft/hard/MAX_SAFE_INTEGER/overflow boundaries | PASS |
| SC-010 | recursive/tool-loop fixture stops deterministically at hard budget with durable exhaustion | PASS |
| SC-011 | deterministic archive golden/hash and repeated export equality; formatting variants preserve archive bytes | PASS |
| SC-012 | malicious archive corpus covers traversal/absolute/backslash/NUL/duplicate/symlink/compression/encryption/count/size/malformed/digest failures before trusted materialization | PASS |
| SC-013 | `boundaries.rs` scans production source and committed fixtures; no real network/provider action or credential/secret fixture | PASS |
| SC-014 | core dependency + zero-I/O + zero-unsafe scripts remain green in full ECR-002 exact-head CI | PASS |
| SC-015 | Phase 8 ledger head `e86e1822e621c0563f2764fe784902e3204b0085`, CI `33152251783`, job `98786745867` passed the complete pre-closure feature gate; final feature and post-merge main gates remain T071–T073 | PASS_BASELINE / FINAL_REQUIRED |
| SC-016 | this artifact maps every FR/SC and G1–G15; post-implementation analyze/convergence remain T069–T070 | PASS_TRACEABILITY / CONVERGENCE_PENDING |

## Constitution v1.1.0 G1–G15 re-check

| Gate | Exact implementation disposition | Result |
|---|---|---|
| G1 Domain coherence | `ecra-run` reuses ECR-001 Actor/ActionRef/ActionAttemptRef/ActionReceipt/ContentDigest/RunId; no parallel trusted-domain model | PASS |
| G2 Authority | attempt durability, budgets and persistence never grant authority; no authorization engine or implicit scope widening | PASS |
| G3 Provenance | ECR-002 persists typed ECR-001 references and execution evidence without creating a competing Fact/provenance model | PASS |
| G4 Side effects | durable prepare-before-effect, exact attempt identity, receipts, UNKNOWN and retry refusal are explicit | PASS |
| G5 Verification | ActionReceipt/execution completion are explicitly non-verification; VerificationReceipt ownership remains ECR-004/ECR-001 domain | PASS |
| G6 Durability | event source, WAL/FULL SQLite, migration, replay, recovery, crash tests and projection rebuild are implemented | PASS |
| G7 Privacy/secrets | acceptance persistence/export is synthetic/non-sensitive; fixture audit and docs reject protected-storage claims | PASS |
| G8 Local-first | no cloud account/network/provider dependency; complete workspace gate runs offline after dependency availability | PASS |
| G9 Interoperability | `.ecra` is a bounded local interchange profile; no external auth/protocol semantics enter trusted run state | PASS |
| G10 Donor/license | exact locked dependency/license/native SQLite evidence is current; no donor source-copy claim | PASS |
| G11 Upstream/browser maintenance | no browser engine, browser bridge or privileged browser patch exists in ECR-002 | PASS-N/A |
| G12 Benchmarks | claims are limited to reproducible deterministic/crash/concurrency/archive/budget/security gates; no unsupported superlative | PASS |
| G13 Information flow / egress | ECR-002 has no network/provider sink and does not turn persistence/read access into disclosure authority | PASS |
| G14 Identity / principal binding | intervention/cancellation Actor attribution is explicitly not authenticated Principal proof | PASS |
| G15 Bounded execution | typed budgets, checked accounting, hard exhaustion and archive parser limits fail closed without widening authority or retrying UNKNOWN | PASS |

No constitutional gate requires ECR-002 to counterfeit downstream authentication, authorization, verification or provider execution.

## Platform risk re-check

| Risk | ECR-002 disposition | Result |
|---|---|---|
| R-006 ambiguous external effects duplicate after crash/retry | durable `AttemptPrepared`, exact attempt/receipt binding, UNKNOWN recovery and blind-retry refusal; independent reconciliation remains ECR-004 | MITIGATED_FOR_ECR-002 |
| R-019 persisted schemas evolve without migration | explicit event/manifest/schema versions, newer-version refusal and deterministic SQLite migration fixture | MITIGATED_FOR_ECR-002 |
| R-033 background agents race/conflict with human actions | expected-head concurrency prevents silent history forks; intervention/cancellation events preserve durable representation; product control ownership remains ECR-008 | REPRESENTATION/MUTATION_RACE_MITIGATED; PRODUCT_CONTROL_DOWNSTREAM |
| R-039 retry attempts cannot be distinguished | exact `ActionAttemptId`/`ActionAttemptRef`, multiple-attempt tests and receipt binding | MITIGATED_FOR_ECR-002 |
| R-042 unbounded agent/model/tool loops | 14 typed budget dimensions, preflight, checked usage, durable hard exhaustion and recursive-loop fixture | MITIGATED_FOR_ECR-002 |
| R-052 plain hash chain marketed as hostile tamper resistance | typed `LedgerDigest` plus executable README/threat-model non-claim checks; protected anchor remains ECR-031 | MITIGATED_FOR_ECR-002 |
| R-053 sensitive state persisted before protection design | synthetic/non-sensitive-only acceptance, fixture secret-marker audit and explicit real-sensitive persistence prohibition | MITIGATED_FOR_ECR-002; PROTECTED_STORAGE_DOWNSTREAM |

No Critical risk is implicitly accepted by ECR-002. Residual security ownership remains exactly where the platform risk register assigns it.

## Traceability decision

```text
UNOWNED_FR=0
UNOWNED_SC=0
FAILED_CONSTITUTION_GATES=0
IMPLICITLY_ACCEPTED_CRITICAL_RISKS=0
T067=COMPLETE_ON_BRANCH
T068=COMPLETE_ON_BRANCH
T069=REQUIRED
T070=REQUIRED
T071_FINAL_EXACT_HEAD=REQUIRED
POST_MERGE_MAIN_GATE=REQUIRED
```
