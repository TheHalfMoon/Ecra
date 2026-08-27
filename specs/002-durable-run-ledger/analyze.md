# Analyze: ECR-002 Durable Run, Ledger & Budgets

**Date:** 2026-08-27  
**Lifecycle reviewed:** pre-implementation planning  
**Inputs:** constitution v1.1.0, platform roadmap/architecture/threat/gap/risk artifacts, `spec.md`, `research.md`, `data-model.md`, `contracts/run-ledger-v1.md`, `threat-model.md`, `plan.md`, `tasks.md`, `quickstart.md`

## Decision

```text
ZERO_BLOCKING_PLANNING_DRIFT_FOUND
IMPLEMENTATION_AUTHORIZATION=TASKS_READY
FAILED_CONSTITUTION_GATES=0
UNOWNED_FR=0
UNOWNED_SC=0
UNRESOLVED_SECURITY_DECISION=0
UNRESOLVED_DEPENDENCY_DECISION=0
REAL_SENSITIVE_PERSISTENCE_AUTHORIZED=NO
DISTRIBUTED_EXECUTION_AUTHORIZED=NO
```

ECR-002 is implementation-eligible only on a bounded feature branch/PR after this planning package is canonical on `main`. The implementation must not expand scope beyond synthetic/non-sensitive local durability.

## 1. Dependency check

ECR-002 depends only on ECR-001. ECR-001 is `CLOSED_CANONICAL`; therefore the roadmap dependency is satisfied.

Still blocked/deferred by design:
- real sensitive/protected persistence -> ECR-031/ECR-003/ECR-025;
- independent UNKNOWN reconciliation/verification -> ECR-004;
- authorization/declassification/budget-increase policy -> ECR-003;
- provider/browser/model/tool/process execution -> later owning slices;
- multi-device/distributed workflow service -> outside ECR-002.

No later dependency is counterfeited by this slice.

## 2. Functional-requirement traceability

| FR | Planning owner / implementation tasks | Result |
|---|---|---|
| FR-001 | versioned event/manifest + DB schema checks; T013–T017, T035–T037, T052–T055 | OWNED |
| FR-002 | strict unsupported-version rejection; T014, T017, T037, T055 | OWNED |
| FR-003 | EventSequence + SQLite PK/order; T010, T014, T040 | OWNED |
| FR-004 | sequence-only order/pure reducer; T014, T020, T040 | OWNED |
| FR-005 | caller-supplied time/no reducer clock; T014, T020, T060 | OWNED |
| FR-006 | RunPhase; T012, T019–T021 | OWNED |
| FR-007 | execution_completed != verified; T012, T021, T064–T065 | OWNED |
| FR-008 | fail-closed transitions; T020–T025, T039 | OWNED |
| FR-009 | terminal no-reopen; T021, T025 | OWNED |
| FR-010 | projection rebuild/non-authoritative; T019, T039–T041 | OWNED |
| FR-011 | durable AttemptPrepared before effect; T022, T045, T049 | OWNED |
| FR-012 | duplicate/conflicting attempt reject; T022, T045, T050 | OWNED |
| FR-013 | exact receipt binding; T022, T046, T050 | OWNED |
| FR-014 | prepared/no receipt -> UNKNOWN/unresolved; T023, T047, T049 | OWNED |
| FR-015 | durable hook for ECR-004; T023, T047–T048 | OWNED |
| FR-016 | retry guard preserves ECR-001 semantics; T023, T048–T049 | OWNED |
| FR-017 | multiple attempts stay distinct; T019, T022, T050 | OWNED |
| FR-018 | append-only history/UPDATE-DELETE guard; T036, T043 | OWNED |
| FR-019 | domain-separated ledger digest; T011, T015 | OWNED |
| FR-020 | distinct digest/no auth claim; T011, T064–T065 | OWNED |
| FR-021 | chain verification; T014, T017, T040, T043 | OWNED |
| FR-022 | scoped integrity claim; T064–T065 | OWNED |
| FR-023 | expected-head atomic append; T038–T039, T051 | OWNED |
| FR-024 | SQLite local store; T035–T044 | OWNED |
| FR-025 | WAL + FULL assertion; T035, T044 | OWNED |
| FR-026 | eager write transaction; T038–T039 | OWNED |
| FR-027 | schema version/migrations; T036–T037 | OWNED |
| FR-028 | WAL not export; T035, T059 | OWNED |
| FR-029 | content-addressed synthetic blobs; T042, T055 | OWNED |
| FR-030 | synthetic/non-sensitive gate; T042, T059, T064 | OWNED |
| FR-031 | exact 14 budget dimensions; T027, T032 | OWNED |
| FR-032 | I-JSON/checked arithmetic; T010, T027–T032 | OWNED |
| FR-033 | hard + optional soft<=hard; T027–T028 | OWNED |
| FR-034 | malformed/duplicate/overflow reject; T028, T032 | OWNED |
| FR-035 | durable usage/reducer authority; T029, T031, T039 | OWNED |
| FR-036 | preflight; T030 | OWNED |
| FR-037 | post-use charge/exhaustion; T031, T034 | OWNED |
| FR-038 | durable exhaustion evidence; T031–T033 | OWNED |
| FR-039 | no ambient budget expansion; T031, T034, T064 | OWNED |
| FR-040 | cancellation request != cancelled; T013, T021, T025 | OWNED |
| FR-041 | explicit suspend/resume; T021, T024–T025 | OWNED |
| FR-042 | intervention Actor attribution only; T013, T025, T064–T065 | OWNED |
| FR-043 | recovery scan before scheduling; T023, T047 | OWNED |
| FR-044 | unresolved blind retry blocked; T024, T048–T049 | OWNED |
| FR-045 | recovery/replay idempotence; T020, T023, T026, T047 | OWNED |
| FR-046 | deterministic ZIP `.ecra`; T052–T059 | OWNED |
| FR-047 | Stored/fixed metadata profile; T052–T053 | OWNED |
| FR-048 | manifest/events/blobs, no SQLite; T052–T055, T059 | OWNED |
| FR-049 | archive safety/limits; T052, T054, T057 | OWNED |
| FR-050 | byte-identical export; T053, T056, T063 | OWNED |
| FR-051 | same strict event/chain validation on import; T055, T058 | OWNED |
| FR-052 | synthetic archive gate; T059, T064 | OWNED |
| FR-053 | typed machine errors; T009, T018 | OWNED |
| FR-054 | core remains zero-I/O/no new run deps; T001, T004, T061 | OWNED |
| FR-055 | dependency boundary/license review; T002, T004, T007, T062, T066 | OWNED |
| FR-056 | no network/provider execution; T005, T060, T064 | OWNED |
| FR-057 | no auth/verification/trust-root ownership; T003, T060, T064–T065 | OWNED |

## 3. Success-criteria traceability

| SC | Evidence task(s) | Result |
|---|---|---|
| SC-001 valid fixtures | T015–T016, T025, T055 | OWNED |
| SC-002 invalid typed failures | T017–T018, T025, T028, T057–T058 | OWNED |
| SC-003 1,000 deterministic reductions | T026, T063 | OWNED |
| SC-004 crash boundary matrix | T045–T049 | OWNED |
| SC-005 concurrent append | T051 | OWNED |
| SC-006 chain mutations/gaps/reorder | T015, T017, T040, T043 | OWNED |
| SC-007 SQLite crash/WAL+FULL | T035, T044 | OWNED |
| SC-008 projection rebuild equality | T041 | OWNED |
| SC-009 all budget dimensions/boundaries | T027–T032 | OWNED |
| SC-010 bounded recursive/tool loop | T033 | OWNED |
| SC-011 deterministic archive | T053, T056, T063 | OWNED |
| SC-012 malicious archive corpus | T054, T057–T058 | OWNED |
| SC-013 no real remote/sensitive fixture | T059–T060, T064 | OWNED |
| SC-014 ECR-001 boundaries stay green | T004, T061–T062 | OWNED |
| SC-015 exact-head full gate | T006, T066, T071–T073 | OWNED |
| SC-016 FR/SC/G1–G15 traceability | this analyze + T067–T070 | OWNED |

## 4. Constitution G1–G15

| Gate | Result | Evidence |
|---|---|---|
| G1 Domain coherence | PASS | ECR-001 types reused; no parallel Actor/Action/Receipt/Verification types |
| G2 Authority | PASS | no execution authorization or grant synthesis; attempt durability is not authority |
| G3 Provenance | PASS/N/A | run events preserve typed refs; no new Fact provenance model |
| G4 Side effects | PASS | exact attempt-before-effect, UNKNOWN and retry semantics |
| G5 Verification | PASS | execution/receipt remain non-verification; ECR-004 named owner |
| G6 Durability | PASS | event source, SQLite, migration, replay, recovery, projection rebuild |
| G7 Privacy/secrets | PASS | explicit synthetic/non-sensitive-only gate |
| G8 Local-first | PASS | no cloud/network dependency |
| G9 Interoperability | PASS | strict local `.ecra`; no external auth/protocol semantics |
| G10 Donor/license | PASS_PLANNING | candidates identified; T007/T066 require exact lock review before readiness/closure |
| G11 Browser maintenance | N/A | no browser patch/bridge |
| G12 Benchmarks | PASS | deterministic/crash/concurrency/archive/budget tests are reproducible |
| G13 Information flow/egress | PASS | no network/remote sink; storage does not imply future disclosure authority |
| G14 Identity/principal | PASS | Actor attribution explicitly non-authenticating; ECR-031 owner preserved |
| G15 Bounded execution | PASS | run budgets + archive parser limits are binding requirements |

No failed constitutional gate blocks implementation.

## 5. Platform risk ownership check

Directly affected risks:

```text
R-006 ambiguous external effects duplicate after crash/retry
R-019 persisted schema evolution without migration
R-033 background/human conflict durable events (representation only)
R-039 retry attempts indistinguishable
R-042 unbounded loops/cost/resource exhaustion
R-052 hash-chain hostile-tamper overclaim
R-053 sensitive state persisted before protection design
```

All have explicit prevention/evidence tasks or named downstream boundaries. No Critical risk is implicitly accepted.

## 6. Storage choice consistency

SQLite/rusqlite is compatible with architecture Layer B and does not leak into ECR-001. `redb` was reviewed and rejected for a stated complexity reason, not ignored. Native SQLite introduces dependency/supply-chain cost, which is contained by T002/T004/T007/T062/T066.

No current evidence requires a platform-level architecture amendment because the engine is an ECR-002 implementation detail behind the durable-run boundary.

## 7. Archive choice consistency

`.ecra` is independent of live SQLite/WAL, preserving portability and avoiding database-byte-format coupling. Strict ZIP profile limits parser scope. No encryption/signature claim is made. ECR-029 may later broaden portability without redefining ECR-002 run history semantics.

## 8. Open questions audit

Resolved before TASKS_READY:
- database engine: SQLite;
- transaction mode: Immediate write transaction;
- durability pragmas: WAL + FULL;
- event ordering: EventSequence only;
- integrity algorithm/preimage: domain-separated JCS + SHA-256 including schema version;
- run phases/terminal semantics: frozen v1 matrix;
- budget dimensions/amount representation: frozen v1 enum + integer microunits;
- archive format/profile/metadata/limits: frozen v1;
- sensitive persistence: not authorized;
- whole-store authenticity: not claimed;
- distributed execution: non-goal.

Implementation-time facts that remain intentionally evidence tasks, not design ambiguity:
- exact transitive lockfile versions;
- implementation-time advisories/licenses;
- exact bundled SQLite transitive package hashes;
- actual CI timings/performance.

## 9. Pre-implementation authorization

The next legal repository action after canonicalizing this package is:

```text
create branch 002-durable-run-ledger from exact canonical main
open Draft PR
mark roadmap/platform/active status IMPLEMENTING in the branch
execute T001, then continue only by dependency order
```

No ECR-031/ECR-004/ECR-003 implementation is authorized by this analyze.
