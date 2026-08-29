# Tasks: ECR-004 Verification & Reconciliation

**Feature:** ECR-004  
**Status:** IMPLEMENTING_REVIEW_READY  
**Dependencies:** ECR-001/ECR-002 `CLOSED_CANONICAL`  
**Execution rule:** `[x]` requires implementation plus linked evidence. Planning completion does not authorize implementation until the planning package is canonical and the exact canonical planning head passes the required dependency regressions.

## Phase 1 — Workspace, dependency and CI boundary

- [x] **T001** Re-verify exact dependency versions/features/licenses/advisories/MSRV for `ecra-verify`, including whether existing repository-approved `rusqlite`, serialization, JCS/hash dependencies can be reused without widening the boundary; record accepted/rejected candidates. **Paths:** `specs/004-verification-receipts/research.md`, `research/donor-license-ledger.md`. **FR-041, FR-042; SC-010, SC-011**
- [x] **T002** Add `crates/ecra-verify` to the workspace with `#![forbid(unsafe_code)]`, dependency-minimal crate docs, and an explicit separation between pure verification logic and local journal I/O. **Paths:** `Cargo.toml`, `crates/ecra-verify/Cargo.toml`, `crates/ecra-verify/src/lib.rs`, `crates/ecra-verify/README.md`. **FR-001–FR-004, FR-041, FR-042**
- [x] **T003** Add `scripts/check-verify-unsafe.sh` and `scripts/check-verify-deps.sh` proving no Ecra-authored unsafe and rejecting browser/network/model/provider/process/policy/authorization dependencies or private ECR-002 mutation bridges. **Paths:** `scripts/check-verify-unsafe.sh`, `scripts/check-verify-deps.sh`. **FR-041, FR-042, FR-046; SC-011, SC-013**
- [x] **T004** Add permanent push-only `.github/workflows/ecr-004.yml` for the implementation branch and `main` with locked build/fmt/Clippy/workspace/rustdoc/offline, explicit ECR-004 targets, ECR-001/ECR-002 regressions, boundary scripts, dependency/toolchain evidence and `cargo tree -p ecra-verify`. **Path:** `.github/workflows/ecr-004.yml`. **SC-010, SC-011, SC-013**
- [x] **T005** Verify the first workspace/dependency implementation head passes every Phase 1 gate before semantic implementation and record exact head/run/job evidence. **Path:** `specs/004-verification-receipts/STATUS.md`. **SC-010, SC-011**

## Phase 2 — IDs, errors and strict verification request contract

- [x] **T006** Implement typed non-nil `CheckpointId` and `ReconciliationId`, strict v1 version handling, machine-readable ECR-004 error categories/codes, and bounded safe diagnostic formatting. **Paths:** `crates/ecra-verify/src/ids.rs`, `crates/ecra-verify/src/error.rs`. **FR-010, FR-036, FR-043, FR-044**
- [x] **T007** Implement strict `VerificationRequestV1` with exact canonical target, verifier, optional principal evidence, method, bounded evidence list, proposed outcome, explicit evaluated time, rule ID and notes; reject unknown fields/unsupported versions/duplicate evidence IDs/over-limit input. **Path:** `crates/ecra-verify/src/request.rs`. **FR-005–FR-010, FR-017, FR-043, FR-044**
- [x] **T008** Add valid/invalid request fixtures covering every target variant, every verification outcome/method class, missing evidence, duplicate IDs, malformed/unknown fields, unsupported versions, oversized arrays/strings and target mutations. **Paths:** `contracts/ecra-verify-v1/valid/`, `contracts/ecra-verify-v1/invalid/`, `crates/ecra-verify/tests/request_contract.rs`. **FR-002, FR-005–FR-010, FR-043; SC-001, SC-009**
- [x] **T009** Implement request validation -> canonical ECR-001 `VerificationReceipt` construction, proving output receipt ID/verifier/principal/target/method/evidence/outcome/time/notes exactly match validated request inputs and no second verification-record type exists. **Paths:** `crates/ecra-verify/src/request.rs`, `crates/ecra-verify/tests/request_contract.rs`. **FR-001–FR-009; SC-001**
- [x] **T010** Add architecture/type tests proving `ActionReceipt`, `Fact`, `Artifact`, notes and model/tool metadata cannot self-promote to `VerificationReceipt` or gain a parallel verified flag. **Paths:** `crates/ecra-verify/tests/boundaries.rs`, `crates/ecra-core/tests/non_authoritative_metadata.rs`. **FR-003, FR-004, FR-014, FR-015; SC-001, SC-011**
- [x] **T011** Run exact-head Phase 2 CI and record the immutable evidence checkpoint before evidence semantics. **Path:** `specs/004-verification-receipts/STATUS.md`. **SC-001, SC-009–SC-011**
- [x] **T011A** Add only the read-only canonical `EvidenceRef` accessors authorized by `implementation-clarifications.md` for existing artifact/observation/receipt/external-ref/content-digest/as-of fields; prove ECR-001 serialization/canonical semantics are unchanged and run the full ECR-001 regression suite before ECR-004 reads those fields. **Paths:** `crates/ecra-core/src/evidence.rs`, `crates/ecra-core/tests/information_evidence.rs`, `crates/ecra-core/tests/contract_fixtures.rs`. **FR-011–FR-016; SC-001, SC-008, SC-010**

## Phase 3 — Decision-grade evidence and deterministic aggregation

- [x] **T012** Implement `DecisionGradeAssessmentV1` and rule validation for immutable content/snapshot/artifact binding, evidence uniqueness, supported evidence shape and self-attesting execution-receipt rejection. **Path:** `crates/ecra-verify/src/evidence.rs`. **FR-011, FR-012, FR-014, FR-015; SC-008**
- [x] **T013** Implement explicit freshness/as-of requirements for time-sensitive rules using supplied evidence metadata and evaluation time only; no ambient clock or remote fetch. **Path:** `crates/ecra-verify/src/evidence.rs`. **FR-013, FR-041, FR-045; SC-008**
- [x] **T014** Implement deterministic `VerificationAggregateViewV1` over one exact `VerificationTarget` with closed states `Absent`, `Verified`, `Rejected`, `Inconclusive`, `Conflicted`, retaining all immutable receipt IDs. **Path:** `crates/ecra-verify/src/aggregate.rs`. **FR-018–FR-021; SC-002, SC-003**
- [x] **T015** Add conflict/aggregation fixtures proving `Verified + Rejected` is always `Conflicted`, `NotEvaluated` never satisfies verification, and no last-write-wins behavior exists. **Paths:** `contracts/ecra-verify-v1/valid/`, `crates/ecra-verify/tests/aggregate.rs`. **FR-018–FR-021; SC-003**
- [x] **T016** Add property tests for receipt-order permutation invariance and 1,000 identical aggregate evaluations producing byte-equivalent canonical views; prove verification never mutates ECR-001 provenance/freshness/dispute state. **Paths:** `crates/ecra-verify/tests/aggregate.rs`, `crates/ecra-verify/tests/evidence.rs`. **FR-009, FR-016, FR-018; SC-002**
- [x] **T017** Run exact-head Phase 3 CI and update the branch status ledger. **Path:** `specs/004-verification-receipts/STATUS.md`. **SC-002, SC-003, SC-008, SC-010**

## Phase 4 — Critical verification checkpoints

- [x] **T018** Implement strict `VerificationRequirementV1`, `VerificationCheckpointV1` and bounded checkpoint labels/requirement counts; reject duplicate exact targets and prohibited satisfying states. **Path:** `crates/ecra-verify/src/checkpoint.rs`. **FR-022–FR-024, FR-043**
- [x] **T019** Implement deterministic `CheckpointEvaluationV1` from aggregate views with explicit satisfied/unsatisfied/conflicted target sets and no mutation of receipts or ECR-002 run phase. **Path:** `crates/ecra-verify/src/checkpoint.rs`. **FR-023, FR-025**
- [x] **T020** Add architecture tests proving checkpoints contain no `CapabilityGrant`, approval, policy decision, declassification, secret handle or executor authority surface. **Path:** `crates/ecra-verify/tests/boundaries.rs`. **FR-024, FR-041; SC-011**
- [x] **T021** Add checkpoint fixtures for all-satisfied, absent, inconclusive, rejected, conflicted, duplicate-target and over-limit cases, including critical-point false-completion scenarios. **Paths:** `contracts/ecra-verify-v1/valid/`, `contracts/ecra-verify-v1/invalid/`, `crates/ecra-verify/tests/checkpoint.rs`. **FR-022–FR-025; SC-009**
- [x] **T022** Run exact-head Phase 4 CI and record checkpoint evidence. **Path:** `specs/004-verification-receipts/STATUS.md`. **SC-003, SC-009–SC-011**

## Phase 5 — UNKNOWN reconciliation and retry safety

- [x] **T023** Implement strict `ReconciliationOutcomeV1`, `ReconciliationRecordV1`, bounded support receipt IDs/notes and append-only typed identity. **Path:** `crates/ecra-verify/src/reconcile.rs`. **FR-026, FR-027, FR-032, FR-035, FR-036, FR-043**
- [x] **T024** Validate exact `RunId` + durable ECR-002 `ActionAttemptRef` + underlying `ActionRef` binding against supplied read-only `RunState`; reject cross-run/cross-attempt/cross-action evidence before reconciliation. **Path:** `crates/ecra-verify/src/reconcile.rs`. **FR-026, FR-046; SC-004, SC-013**
- [x] **T025** Resolve supporting canonical verification receipt IDs and reject missing/duplicate/irrelevant/cross-target receipts; preserve all supporting IDs in the reconciliation record. **Path:** `crates/ecra-verify/src/reconcile.rs`. **FR-032, FR-035; SC-004**
- [x] **T026** Implement fail-closed reconciliation rules: explicit conclusive effect evidence -> `effect_confirmed`; explicit conclusive no-effect evidence -> `no_effect_confirmed`; absent/insufficient/conflicting evidence -> `still_unknown`; absence of provider receipt alone is never no-effect proof. **Path:** `crates/ecra-verify/src/reconcile.rs`. **FR-027–FR-030, FR-035; SC-004, SC-005**
- [x] **T027** Add type/source/state tests proving reconciliation cannot construct/synthesize `ActionReceipt`, cannot construct/append ECR-002 `RunEvent`, cannot mutate ECR-002 `RunState`/`PreparedAttemptState`, cannot remove the prior attempt from `unresolved_attempts`, and cannot make the same run resumable/completable. Compare pre/post canonical/semantic run state for all reconciliation outcomes. **Paths:** `crates/ecra-verify/tests/boundaries.rs`, `crates/ecra-verify/tests/reconcile.rs`. **FR-031, FR-037, FR-046; SC-006, SC-011, SC-013**
- [x] **T028** Implement `RetryDispositionV1` derived from exact ECR-001 retry/idempotency semantics plus reconciliation state: duplicate block, reconciliation required, semantically retryable, same-key-only, or explicit nonblind path. `semantically_retryable*` MUST be documented/typed as advisory for a future new-attempt proposal only and expose no same-run resume/schedule/execution/authorization method. **Path:** `crates/ecra-verify/src/reconcile.rs`. **FR-028–FR-034, FR-046; SC-004, SC-005, SC-013**
- [x] **T029** Add exhaustive reconciliation/retry matrix including unreceipted/unresolved, provider-ambiguous, conflicting evidence, effect/no-effect confirmation, all `RetryClass`/`IdempotencyClass` combinations, same-key mutation and duplicate-effect prevention. For every outcome, prove ECR-002 unresolved membership is unchanged and the canonical `RunResumed`, `ExecutionCompleted`, and blind-retry guards still reject the unresolved prior attempt where applicable. **Paths:** `crates/ecra-verify/tests/reconcile.rs`, `crates/ecra-run/tests/attempts.rs`, `crates/ecra-run/tests/crash_recovery.rs`. **FR-026–FR-035, FR-046; SC-004–SC-006, SC-013**
- [x] **T030** Run exact-head Phase 5 CI including explicit ECR-002 unresolved-state compatibility targets and update status before persistence work. **Path:** `specs/004-verification-receipts/STATUS.md`. **SC-004–SC-006, SC-010, SC-011, SC-013**

## Phase 6 — Append-only verification journal and local persistence

- [x] **T031** Implement strict `VerificationJournalEntryV1`, body variants, positive sequence, exact previous-digest rule and domain-separated canonical SHA-256 entry digest using repository-aligned JCS semantics. **Path:** `crates/ecra-verify/src/journal.rs`. **FR-036, FR-038; SC-007**
- [x] **T032** Add fixed canonical journal material/digest goldens and mutation tests for version/sequence/previous digest/body/entry digest. **Paths:** `contracts/ecra-verify-v1/expected/`, `crates/ecra-verify/tests/journal.rs`. **FR-038; SC-007**
- [x] **T033** Implement transactional SQLite v1 initialization/migration and authoritative append-only `verification_journal` plus rebuildable receipt/checkpoint/reconciliation indexes, separate from ECR-002 run storage. **Path:** `crates/ecra-verify/src/store.rs`. **FR-036–FR-040**
- [x] **T034** Enforce store API/schema append-only behavior so ordinary update/delete of canonical journal truth is rejected while projection indexes remain rebuildable; no projection may represent or mutate ECR-002 run resolution. **Path:** `crates/ecra-verify/src/store.rs`. **FR-036, FR-037, FR-046; SC-006, SC-007, SC-013**
- [x] **T035** Implement expected-head compare-and-append concurrency; two writers competing on one expected head allow exactly one canonical append. **Paths:** `crates/ecra-verify/src/store.rs`, `crates/ecra-verify/tests/sqlite_store.rs`. **FR-036, FR-038; SC-007**
- [x] **T036** Add corruption/migration/projection tests for malformed JSON, sequence gaps, previous-digest mismatch, entry-digest mismatch, duplicate IDs, newer schema, failed migration rollback, projection deletion/rebuild and index poisoning. **Paths:** `crates/ecra-verify/tests/sqlite_store.rs`, `crates/ecra-verify/tests/migration.rs`. **FR-038–FR-040, FR-044; SC-007**
- [x] **T037** Add restart/reopen/replay test proving identical receipts produce byte-equivalent aggregate/checkpoint/reconciliation views after store reopen. **Paths:** `crates/ecra-verify/tests/sqlite_store.rs`, `crates/ecra-verify/tests/aggregate.rs`, `crates/ecra-verify/tests/checkpoint.rs`, `crates/ecra-verify/tests/reconcile.rs`. **FR-040; SC-007**
- [x] **T038** Add synthetic/non-sensitive sentinel scans proving raw secret/private payload strings are absent from journal rows, fixtures, errors and Debug/Display; persist references/digests only. **Paths:** `crates/ecra-verify/tests/boundaries.rs`, `crates/ecra-verify/tests/sqlite_store.rs`. **FR-011, FR-017, FR-039; SC-011**
- [x] **T039** Run exact-head Phase 6 CI and explicitly record the integrity-only/non-hostile-tamper claim boundary. **Paths:** `specs/004-verification-receipts/STATUS.md`, `crates/ecra-verify/README.md`. **SC-007, SC-010, SC-011**

## Phase 7 — Hostile input, portability and documentation

- [x] **T040** Add arbitrary bounded-input/property tests for request/evidence/checkpoint/reconciliation/journal parsing and exact maxima for evidence refs, receipts per target, checkpoint requirements, support receipt IDs, notes/rule IDs, journal bytes and query materialization; over-limit input fails typed and does not panic. **Paths:** `crates/ecra-verify/tests/request_contract.rs`, `crates/ecra-verify/tests/evidence.rs`, `crates/ecra-verify/tests/checkpoint.rs`, `crates/ecra-verify/tests/reconcile.rs`, `crates/ecra-verify/tests/journal.rs`. **FR-043, FR-044; SC-009**
- [x] **T041** Add portability tests proving semantically equivalent strict JSON formatting/order/line-ending variants yield identical canonical digest/aggregate/reconciliation behavior where allowed. **Path:** `crates/ecra-verify/tests/portability.rs`. **FR-009, FR-038; SC-002**
- [x] **T042** Document exact v1 usage, decision-grade evidence rules, checkpoint semantics, reconciliation/retry non-authority, unchanged ECR-002 unresolved-state boundary, future-new-attempt-only meaning of `semantically_retryable*`, synthetic-only persistence, offline operation, journal integrity claim and explicit non-claims about verifier infallibility/full-store tamper resistance/provider authenticity/exactly-once effects. **Path:** `crates/ecra-verify/README.md`. **FR-011–FR-017, FR-039, FR-041, FR-045, FR-046; SC-013**
- [x] **T043** Execute the complete `quickstart.md` exact-head gate and record toolchain/dependency/test evidence. **Path:** `specs/004-verification-receipts/STATUS.md`. **SC-010, SC-011, SC-013**
- [x] **T044** Reconcile donor/license/dependency ledger with actual implementation and prove no unrecorded donor source or new runtime/provider dependency entered the slice. **Paths:** `research/donor-license-ledger.md`, `specs/004-verification-receipts/research.md`. **FR-041, FR-042; SC-011**
- [x] **T045** Run exact-head Phase 7 closure gate with ECR-001/ECR-002 regressions and update status only on exact success. **Path:** `specs/004-verification-receipts/STATUS.md`. **SC-009–SC-011, SC-013**

## Phase 8 — Traceability, convergence, review and canonical closure

- [x] **T046** Map FR-001–FR-046 and SC-001–SC-013 to implementation/tests/contracts with zero unowned MUST requirement. **Path:** `specs/004-verification-receipts/traceability-closure.md`. **SC-012, SC-013**
- [x] **T047** Re-check constitution G1–G15 and platform verification risks/gaps including executor self-verification, UNKNOWN retry, duplicate effects, mutable evidence, malicious evidence, verifier conflict/capture and same-run unresolved-state bypass. **Path:** `specs/004-verification-receipts/traceability-closure.md`. **SC-012, SC-013**
- [x] **T048** Run post-implementation analyze-equivalent review; append explicit convergence tasks for any MUST-level drift rather than hiding it. **Path:** `specs/004-verification-receipts/post-implementation-analyze.md`. **SC-012, SC-013**
- [x] **T049** Converge spec/research/data-model/contract/threat-model/plan/tasks/quickstart/status/platform lifecycle docs with exact implementation truth. **Paths:** `specs/004-verification-receipts/`, `specs/000-ecra-platform/`, `specs/README.md`, `EXECUTION.md` as applicable.
- [x] **T050** Run complete exact-head final ECR-004 CI on the final feature head and require ECR-001/ECR-002 regression success, including unresolved-state compatibility acceptance. **SC-010–SC-013**
- [ ] **T051** Move implementation PR out of Draft only after T050; process all review/check/thread findings and require zero actionable blocker. **SC-012, SC-013**
- [ ] **T052** Merge the exact expected implementation head by an allowed non-rebase method and require canonical-main ECR-004 + ECR-001 + ECR-002 workflows to succeed on the resulting canonical state.
- [ ] **T053** Mark ECR-004 `CLOSED_CANONICAL` only after post-merge evidence; update roadmap/status/index/EXECUTION and re-evaluate dependency eligibility for ECR-005 and other slices from live canonical truth.

## Dependency graph

```text
T001 → T002 → T003 → T004 → T005
  ↓
T006 → T007 → T008 → T009 → T010 → T011 → T011A
  ↓
T012 → T013 → T014 → T015 → T016 → T017
  ↓
T018 → T019 → T020 → T021 → T022
  ↓
T023 → T024 → T025 → T026 → T027 → T028 → T029 → T030
  ↓
T031 → T032 → T033 → T034 → T035 → T036 → T037 → T038 → T039
  ↓
T040 → T041 → T042 → T043 → T044 → T045
  ↓
T046 → T047 → T048 → T049 → T050 → T051 → T052 → T053
```

## Scope guard

ECR-004 remains separate from ECR-031 and ECR-003. It may progress from ECR-001/ECR-002 alone, but it must not persist real sensitive evidence, validate identity assertions, authorize/retry provider actions, fetch live external evidence, change ECR-002's canonical run-event v1 contract, clear ECR-002 unresolved attempts, or claim that reconciliation resumes/completes the existing run. Completing ECR-004 alone does not unblock ECR-005 while ECR-003/ECR-031 dependencies remain open.