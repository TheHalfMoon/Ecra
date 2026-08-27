# Tasks: Trusted Domain Kernel

**Feature:** ECR-001  
**Status:** REVIEW_REMEDIATION  
**Canonical inputs:** `spec.md`, `research.md`, `data-model.md`, `contracts/domain-v1.md`, `plan.md`, `quickstart.md`, analyze/traceability artifacts  

`[x]` means satisfied on the feature branch; it does not mean `CLOSED_CANONICAL`. Exact merge/post-merge closure remains governed by `AGENTS.md` and the constitution.

## Phase 1 — Reproducible Rust Workspace

- [x] T001 Create the root Cargo workspace with `crates/ecra-core` as the sole production member.
- [x] T002 Pin Rust 1.98.x / Edition 2024.
- [x] T003 Enforce workspace lints and `#![forbid(unsafe_code)]`. **FR-052**
- [x] T004 Add only reviewed pure dependencies and record exact dependency/license provenance. **FR-050, G10**
- [x] T005 Create the v1 valid/invalid contract fixture corpus and conventions.
- [x] T006 Add locked build/fmt/Clippy/test/rustdoc/offline/unsafe/dependency CI gates. **SC-016, SC-018**

## Phase 2 — Version, Errors, IDs, Time, Canonicalization and Digests

- [x] T007 Implement `SchemaVersion` / `Versioned<T>` strict dispatch. **FR-001, FR-047**
- [x] T008 Implement machine-readable `DomainError` / `ErrorCode` / `ErrorCategory`. **FR-053**
- [x] T009 Implement all strong ID newtypes. **FR-002, FR-003**
- [x] T010 Implement I-JSON-safe `EpochMillis`, `TemporalValidity`, `EvaluationContext`. **FR-018, FR-049**
- [x] T011 Implement RFC 8785 JCS canonicalization. **FR-004, FR-051**
- [x] T012 Implement `ContentDigest`, SHA-256 `SecurityDigest`, and validation. **FR-031, FR-032**
- [x] T013 Add foundational unit/property tests. **SC-002, SC-014, SC-016**
- [x] T014 Add canonical byte/fixed-point fixtures. **SC-015**

## Phase 3 — Actors, Principals, Origins, Resources and Scope

- [x] T015 Implement `Actor` / `ActorKind` and non-authoritative label metadata. **FR-005, FR-007**
- [x] T016 Implement opaque `PrincipalRef` / `IdentityAssertionRef`. **FR-006**
- [x] T017 Implement `Origin` / `WebOrigin`, including opaque origins. **FR-008, FR-009**
- [x] T018 Implement stable `ResourceRef` identity plus non-authoritative locator/origin metadata. **FR-010**
- [x] T019 Implement fail-closed `ScopeConstraint<T>`. **FR-011, FR-012**
- [x] T020 Implement typed multi-dimensional `Scope` + `PurposeRef`. **FR-013**
- [x] T021 Add valid Actor/Principal/Origin/Resource/Scope fixtures. **SC-001, SC-005, SC-006**
- [x] T022 Add invalid origin/resource/scope/ID fixtures. **SC-002, SC-006**
- [x] T023 Prove Actor/Principal, locator, origin and scope cannot be type/text-confused. **FR-006, FR-009, FR-010, FR-054, SC-005**

## Phase 4 — Capability Request/Grant, Delegation and Time

- [x] T024 Implement `OperationRef`, distinct request/grant types, delegation and structural validation. **FR-014–FR-019**
- [x] T025 Add valid narrow request/grant/delegation/expiry fixtures. **SC-001, SC-013**
- [x] T026 Add invalid request/grant/temporal/scope fixtures. **SC-002, SC-006, SC-013**
- [x] T027 Prove no implicit Request→Grant, ID or Actor→Principal conversion. **FR-014, FR-055, SC-013**
- [x] T028 Prove temporal evaluation uses caller context and no OS clock. **FR-018, FR-049**

## Phase 5 — Information, Observation, Fact, Freshness, Evidence and Artifacts

- [x] T029 Implement information classification and structured policy tags. **FR-027, FR-028**
- [x] T030 Implement classified provenance-bearing Observation + bounded payload reference. **FR-020, FR-021, FR-027**
- [x] T031 Implement ArtifactRef identity/classification/digest/size/locator/lineage. **FR-029–FR-031**
- [x] T032 Implement inspectable FreshnessAssessment. **FR-024**
- [x] T033 Implement Fact/FactValue/Provenance/Dispute without `Fact.verified`. **FR-020–FR-024, FR-029**
- [x] T034 Implement bounded EvidenceRef with digest/as-of metadata. **FR-025, FR-026**
- [x] T035 Add valid classification/observation/artifact/fact/conflict/freshness fixtures. **SC-001, SC-007**
- [x] T036 Add invalid information/freshness/evidence/digest/byte-size fixtures. **SC-002**
- [x] T037 Prove provenance/classification/freshness/verification remain orthogonal. **FR-022–FR-029, SC-007**
- [x] T038 Add lineage/classification property tests including unknown != public. **FR-027–FR-029**

## Phase 6 — Information Use / Source-to-Sink Intent

- [x] T039 Implement `InformationRef`, `InformationUseKind`, `InformationUse`. **FR-034, FR-035**
- [x] T040 Add valid local/model/persist/log/external/remote use fixtures. **SC-008**
- [x] T041 Add invalid InformationUse fixtures. **SC-002, SC-008**
- [x] T042 Prove InformationUse is declaration only and cannot synthesize A→B authorization. **FR-035, FR-055, SC-008**

## Phase 7 — Action Effects, Idempotency, Retry and Immutable Binding

- [x] T043 Implement mutation/reversibility/idempotency/retry value types. **FR-036–FR-038**
- [x] T044 Implement fail-closed cross-field compatibility. **FR-036–FR-038, FR-048**
- [x] T045 Implement pre-authorization `ActionIntent` and exact bound parameters. **FR-033–FR-039**
- [x] T046 Implement domain-separated RFC8785+SHA-256 `ActionDigest` / `ActionRef`. **FR-032, FR-039, FR-051**
- [x] T047 Add valid action semantic fixtures. **SC-001, SC-011**
- [x] T048 Add invalid effect/idempotency/retry fixtures. **SC-002, SC-011**
- [x] T049 Add exhaustive/table/property compatibility tests. **FR-036–FR-038**
- [x] T050 Add fixed ActionDigest golden + security-field mutation tests. **FR-039, SC-009, SC-015**
- [x] T051 Reject wrong-digest ActionRef. **SC-002, SC-009**

## Phase 8 — Attempts, Executor Receipts and Independent Verification

- [x] T052 Implement distinct `ActionAttemptRef`. **FR-040**
- [x] T053 Implement executor-only `ActionReceipt` / `ActionOutcome`. **FR-041–FR-043**
- [x] T054 Implement independent `VerificationReceipt` target/method/outcome validation. **FR-044–FR-046**
- [x] T055 Add valid multi-attempt/receipt/all-verification-outcome fixtures. **SC-010, SC-012**
- [x] T056 Add invalid binding/timing/type-confusion/evidence fixtures. **SC-002, SC-010, SC-012**
- [x] T057 Prove attempts remain distinct, UNKNOWN round-trips, receipt != verification, executor success != VERIFIED. **FR-040–FR-046, SC-010–SC-012**

## Phase 9 — Strict Versioned Contract, Fixture Runner and Portability

- [x] T058 Apply explicit Serde names and strict unknown-field handling. **FR-047**
- [x] T059 Build exhaustive valid/invalid fixture manifests and typed runners. **SC-001, SC-002, SC-014**
- [x] T060 Add canonical-byte and ActionDigest expected outputs. **FR-051, SC-015**
- [x] T061 Add executable/compile-fail rustdoc construction/type-safety examples. **SC-004**
- [x] T062 Add portability/static-source evidence with no OS/service dependence. **FR-049, SC-016, SC-017**

## Phase 10 — Cross-Cutting Security / Architecture Gates

- [x] T063 Enforce direct dependency allowlist + prohibited transitive dependency categories. **FR-050, SC-003, SC-017**
- [x] T064 Enforce zero unsafe by crate lint + static CI script. **FR-052, SC-016**
- [x] T065 Test all 16 ErrorCategory / 19 ErrorCode mappings without display parsing. **FR-053**
- [x] T066 Audit `label`, `reason`, purpose, notes, locators/provider/external metadata as non-authoritative. **FR-054**
- [x] T067 Reconcile exact locked dependency licenses/provenance/no-source-copy ledger. **G10**
- [x] T068 Add crate architecture map + seven misuse warnings. **SC-004**
- [x] T069 Prove offline/no-service-access behavior. **FR-049, FR-050, SC-016**

**Phase 10 exact-head evidence:** `5dfe4c09b2abceeec14bc94b8e13d2dccddfd37c`; CI `33086490495` — success.

## Phase 11 — Pre-Closure Traceability / Review Remediation

- [x] T070 Map FR-001–FR-055 and SC-001–SC-020 to implementation/test/contract evidence in `traceability-closure-2026-08-27.md`. **SC-020**
- [x] T071 Re-check constitution v1.1.0 G1–G15 in the traceability artifact; downstream/N/A ownership remains explicit.
- [x] T072 Map all P-001–P-035 pre-implementation findings to exact ECR-001 remediation or named downstream owner. **SC-019**
- [x] T073 Record revised quickstart/full exact-head final results after the last repository convergence mutation. **SC-018**
- [x] T074 Run post-implementation analyze; `post-implementation-analyze-2026-08-27.md` returned `CONVERGENCE_REQUIRED`. **SC-020**
- [x] T075 Activate Phase 12 rather than hiding MUST-level drift.
- [ ] T076 Mark roadmap `CLOSED_CANONICAL` only after PR merge + required post-merge evidence; before then keep truthful intermediate status.

## Phase 12 — Convergence

- [x] T077 Fold C1–C12 plus actual version-envelope/machine-error semantics into primary `data-model.md` and `contracts/domain-v1.md`. **FR-001, FR-020–FR-054, SC-020**
- [x] T078 Converge `quickstart.md`, this task ledger, active `STATUS.md`, and `EXECUTION.md` to current gate/phase truth. **SC-018, SC-020**
- [x] T079 Produce `traceability-closure-2026-08-27.md` covering FRs, SCs, G1–G15 and pre-review findings with downstream deferrals. **SC-019, SC-020**
- [x] T080 Run final analyze-equivalent review and the revised quickstart/CI on the exact converged feature head; zero blocking drift is required before PR readiness. **SC-018, SC-020**

### T073/T080 final branch-gate evidence

Head `20a56b10257609426e5b66ec0c2ba2f884822039`, CI `33095158577`, runner `macbook`, Rust `1.98.0-aarch64-apple-darwin`, passed checkout, build, fmt, strict Clippy, full workspace tests, all eight dedicated contract/security targets, rustdoc, offline replay, unsafe boundary, dependency boundary and `cargo tree -p ecra-core`. The workflow checked out and logged the exact branch SHA before executing the gate.

The ledger-finalization head `12c7029dbde30d2d860fe70447f79b6432ff2f96` also passed the full exact-head gate in CI `33095782152` before PR #1 was marked Ready.

## Phase 13 — Ready-Review Remediation

PR #1 review on `12c7029d…` found three actionable defects. The PR returned to Draft while these tasks are remediated.

- [ ] T081 Make every public `Versioned<T>` Serde deserialization path reject unsupported major/newer minor versions while preserving typed compatibility errors from `Versioned::from_json_slice`. **FR-001, FR-047, SC-002**
- [ ] T082 Make Fact integer and canonical-decimal construction fail closed so API-created values cannot serialize wire data that strict deserialization rejects; add construction/round-trip regression coverage. **FR-020, FR-049, SC-002, SC-014**
- [ ] T083 Synchronize ECR-001 lifecycle truth between platform `STATUS.md`, platform `roadmap.md`, active `STATUS.md`, `EXECUTION.md`, and this ledger. **SC-020**
- [ ] T084 Run the complete exact-head gate after remediation, re-read reviews/threads, resolve only findings actually remediated, and return PR #1 to Ready only with zero actionable review blockers. **SC-018, SC-020**

## Remaining dependency graph

```text
T081–T083 review remediation
  ↓
T084 exact-head gate + review-thread closure
  ↓
PR Ready / final review
  ↓
merge + post-merge main verification
  ↓
T076 roadmap/status canonical closure
  ↓
CLOSED_CANONICAL
```

No dependent ECR implementation becomes eligible before ECR-001 is `CLOSED_CANONICAL`.
