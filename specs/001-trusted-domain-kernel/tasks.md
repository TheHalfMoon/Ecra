# Tasks: Trusted Domain Kernel

**Feature:** ECR-001  
**Input:** revised `spec.md`, `research.md`, `data-model.md`, `contracts/domain-v1.md`, `plan.md`, `quickstart.md`, `analyze.md`  
**Status:** TASKS_READY

## Format

`[ID] [P?] [Story] Description with exact target path — requirement coverage`

- `[P]` means safe to parallelize only after stated dependencies are complete.
- `[US#]` maps to the numbered user story in `spec.md`.
- All tests are mandatory because the spec makes security-sensitive contract behavior normative.

---

## Phase 1 — Reproducible Rust Workspace

**Goal:** establish the smallest authorized build surface.

- [ ] T001 Create root Cargo workspace with only `crates/ecra-core` as a production member in `Cargo.toml`.
- [ ] T002 Pin Rust 1.98.x stable and Edition 2024 in `rust-toolchain.toml` and `crates/ecra-core/Cargo.toml`.
- [ ] T003 Add workspace lint policy and `#![forbid(unsafe_code)]` in `crates/ecra-core/src/lib.rs`. **FR-052**
- [ ] T004 Add only research-approved pure dependency candidates to `crates/ecra-core/Cargo.toml` with minimal features; update `research/donor-license-ledger.md` for exact versions/licenses before merge. **FR-050, G10**
- [ ] T005 Create `contracts/ecra-domain-v1/{valid,invalid}/` plus `contracts/ecra-domain-v1/README.md` documenting fixture naming/error-code conventions.
- [ ] T006 Add baseline CI/script commands for fmt, Clippy, tests, rustdoc, offline test, unsafe check and dependency-boundary check without adding ECR-024 release infrastructure. **SC-016, SC-018**

**Gate:** workspace builds with exactly one production crate and no prohibited dependency category.

---

## Phase 2 — Version, Errors, IDs, Time, Canonicalization and Digests

**Goal:** provide deterministic primitives needed by every story.

- [ ] T007 [P] Implement `SchemaVersion`, `Versioned<T>` and strict supported-version dispatch in `crates/ecra-core/src/version.rs`. **FR-001, FR-047**
- [ ] T008 [P] Implement machine-readable error categories/codes in `crates/ecra-core/src/error.rs`, including compatibility/identity/scope/information/digest/action/attempt/receipt/verification errors. **FR-053**
- [ ] T009 [P] Implement all strong ID newtypes named by FR-002 in `crates/ecra-core/src/id.rs`, with no implicit cross-ID conversion. **FR-002, FR-003**
- [ ] T010 [P] Implement `EpochMillis`, `TemporalValidity`, `EvaluationContext` and I-JSON/range validation in `crates/ecra-core/src/time.rs`. **FR-018, FR-049**
- [ ] T011 [P] Implement RFC 8785 JCS wrapper in `crates/ecra-core/src/canonical.rs`. **FR-004, FR-051**
- [ ] T012 [P] Implement `ContentDigest`, `SecurityDigest` (`sha256`) and validation in `crates/ecra-core/src/digest.rs`. **FR-031, FR-032**
- [ ] T013 Add unit/property tests for ID separation, version strictness, temporal ranges, digest encoding and canonicalization error behavior in `crates/ecra-core/tests/properties.rs`. **SC-002, SC-014, SC-016**
- [ ] T014 Add RFC 8785 canonical fixture expectations/fixed-point tests in `crates/ecra-core/tests/canonicalization.rs` and `contracts/ecra-domain-v1/`. **SC-015**

**Gate:** deterministic foundational values validate with zero I/O; security and generic digest types are distinct.

---

## Phase 3 — US1: Actors, Principals, Origins, Resources and Explicit Scope

**Goal:** attribution, authentication references and scope cannot be type-confused.

- [ ] T015 [P] [US1] Implement `Actor`, `ActorKind` and non-authoritative display metadata in `crates/ecra-core/src/actor.rs`. **FR-005, FR-007**
- [ ] T016 [P] [US1] Implement `PrincipalRef` and `IdentityAssertionRef` as opaque references in `crates/ecra-core/src/identity.rs`; provide no assertion-validity/authentication behavior. **FR-006**
- [ ] T017 [P] [US1] Implement `Origin`, `OriginKind`, standards-aware `WebOrigin` and opaque-origin support in `crates/ecra-core/src/origin.rs`. **FR-008, FR-009**
- [ ] T018 [P] [US1] Implement `ResourceRef`, `ResourceKind` with strong ResourceId and explicit non-authoritative locator/origin metadata in `crates/ecra-core/src/resource.rs`. **FR-010**
- [ ] T019 [US1] Implement generic `ScopeConstraint<T>` (`not_applicable`, `exact`, non-empty `one_of`, `any_explicit`) in `crates/ecra-core/src/scope.rs`. **FR-011, FR-012**
- [ ] T020 [US1] Implement `Scope` with typed Workspace/BrowserSpace/Container/Tab/Session/Task/Origin/Resource constraints and structured PurposeRef in `crates/ecra-core/src/scope.rs`. **FR-013**
- [ ] T021 [P] [US1] Add valid fixtures for Actor/Principal/IdentityAssertion/origins/resources and every ScopeConstraint variant under `contracts/ecra-domain-v1/valid/`. **SC-001, SC-005, SC-006**
- [ ] T022 [P] [US1] Add invalid fixtures for malformed origin/resource, empty `one_of`, implicit wildcard attempts and ID-type mismatch representations under `contracts/ecra-domain-v1/invalid/`. **SC-002, SC-006**
- [ ] T023 [US1] Add compile/runtime tests proving ActorId cannot implicitly become PrincipalId, Resource locator text grants nothing, and arbitrary external strings do not change origin/scope semantics in `crates/ecra-core/tests/contract_fixtures.rs` and `properties.rs`. **FR-006, FR-009, FR-010, FR-054, SC-005**

**Gate:** unrestricted scope is visible/explicit and actor attribution never equals authentication.

---

## Phase 4 — US4: Capability Request/Grant, Delegation and Temporal Shape

**Goal:** later policy can authorize least authority without type confusion.

- [ ] T024 [US4] Implement `OperationRef`, `CapabilityRequest`, `CapabilityGrant`, `DelegationRef` and structural validation in `crates/ecra-core/src/capability.rs`. **FR-014–FR-019**
- [ ] T025 [P] [US4] Add valid narrow request/grant/delegation/expiry fixtures using distinct CapabilityRequestId/CapabilityGrantId under `contracts/ecra-domain-v1/valid/`. **SC-001, SC-013**
- [ ] T026 [P] [US4] Add invalid request-as-grant, invalid temporal, invalid scope and empty-wildcard fixtures under `contracts/ecra-domain-v1/invalid/`. **SC-002, SC-006, SC-013**
- [ ] T027 [US4] Add compile/runtime tests proving no `From<CapabilityRequest> for CapabilityGrant`, no generic ID conversion and no Actor→Principal authentication shortcut in `crates/ecra-core/tests/contract_fixtures.rs`. **FR-014, FR-055, SC-013**
- [ ] T028 [US4] Add caller-supplied temporal evaluation tests proving no OS-clock access is needed in `crates/ecra-core/tests/properties.rs`. **FR-018, FR-049**

---

## Phase 5 — US2: Information Classification, Observation, Fact, Freshness, Evidence and Artifacts

**Goal:** information trust and disclosure metadata survive derivation without becoming permission.

- [ ] T029 [P] [US2] Implement `InformationClass`, `InformationPolicyTag`, `InformationClassification` and validation in `crates/ecra-core/src/information.rs`. **FR-027, FR-028**
- [ ] T030 [P] [US2] Implement `Observation`, payload reference and classified provenance-bearing observation data in `crates/ecra-core/src/evidence.rs`. **FR-020, FR-021, FR-027**
- [ ] T031 [P] [US2] Implement `ArtifactRef`, `ArtifactKind`, classification, ContentDigest/size/storage locator and lineage in `crates/ecra-core/src/artifact.rs`. **FR-029–FR-031**
- [ ] T032 [US2] Implement `FreshnessAssessment`, `FreshnessState`, `FreshnessBasisKind` with assessed/basis metadata in `crates/ecra-core/src/evidence.rs`. **FR-024**
- [ ] T033 [US2] Implement `Fact`, `FactValue`, `Provenance`, `DisputeState` and derived InformationRef lineage **without any `verified` truth flag** in `crates/ecra-core/src/evidence.rs`. **FR-020–FR-024, FR-029**
- [ ] T034 [US2] Implement `EvidenceRef`, `EvidenceKind` with optional immutable capture digest/as-of metadata in `crates/ecra-core/src/evidence.rs`. **FR-025, FR-026**
- [ ] T035 [P] [US2] Add valid fixtures for public/private/sensitive/secret/unknown classifications, classified observation/artifact, model-inferred Fact, conflict/dispute and freshness basis under `contracts/ecra-domain-v1/valid/`. **SC-001, SC-007**
- [ ] T036 [P] [US2] Add invalid classification/tag/freshness/evidence/digest/byte-size fixtures under `contracts/ecra-domain-v1/invalid/`. **SC-002**
- [ ] T037 [US2] Add tests proving provenance/classification/freshness remain orthogonal and no Fact contains or derives a canonical VERIFIED state without a VerificationReceipt in `crates/ecra-core/tests/contract_fixtures.rs`. **FR-022–FR-029, SC-007**
- [ ] T038 [US2] Add lineage/classification round-trip property tests, including derived sensitive information remaining representable as sensitive and `unknown` never normalizing to public in `crates/ecra-core/tests/properties.rs`. **FR-027–FR-029**

---

## Phase 6 — US2/US4: Information Use / Source-to-Sink Intent

**Goal:** read authority and disclosure intent are representationally distinct.

- [ ] T039 [US2] Implement `InformationRef`, `InformationUseKind`, `InformationUse` with non-empty sources and optional destination ResourceRef/WebOrigin in `crates/ecra-core/src/information.rs`. **FR-034, FR-035**
- [ ] T040 [P] [US2] Add valid local-compute/model-context/persist/log/external-disclosure/remote-provider fixtures under `contracts/ecra-domain-v1/valid/`. **SC-008**
- [ ] T041 [P] [US2] Add invalid empty-source and malformed destination InformationUse fixtures under `contracts/ecra-domain-v1/invalid/`. **SC-002, SC-008**
- [ ] T042 [US4] Add tests proving InformationUse is declaration only: no conversion to CapabilityGrant/authorization object and no implicit A→B disclosure created by separate read/write capabilities in `crates/ecra-core/tests/contract_fixtures.rs`. **FR-035, FR-055, SC-008**

---

## Phase 7 — US5: Action Effect, Idempotency, Retry and Immutable Action Digest

**Goal:** exact action identity and safe effect semantics exist before execution.

- [ ] T043 [US5] Implement `MutationDomain`, `Reversibility`, `EffectProfile`, `IdempotencyClass`, `IdempotencySpec`, `RetryClass` in `crates/ecra-core/src/action.rs`. **FR-036–FR-038**
- [ ] T044 [US5] Implement cross-field conservative effect/idempotency/retry validation in `crates/ecra-core/src/action.rs`. **FR-036–FR-038, FR-048**
- [ ] T045 [US3] Implement `ActionIntent`, `ActionParametersRef`, principal/identity references, explicit operation/scope, InformationUse list and correlation fields in `crates/ecra-core/src/action.rs`. **FR-033–FR-039**
- [ ] T046 [US3] Implement Ecra-owned ActionDigest calculation and `ActionRef { id, digest }` in `crates/ecra-core/src/digest.rs` / `action.rs` using the normative domain-separated JCS+SHA-256 contract. **FR-032, FR-039, FR-051**
- [ ] T047 [P] [US5] Add valid fixtures for read-only, irreversible local, reversible external, keyed-idempotent and conservative non-idempotent actions under `contracts/ecra-domain-v1/valid/`. **SC-001, SC-011**
- [ ] T048 [P] [US5] Add invalid mutation/reversibility contradictions, missing idempotency key and unsafe retry combinations under `contracts/ecra-domain-v1/invalid/`. **SC-002, SC-011**
- [ ] T049 [US5] Add exhaustive/table/property tests for effect × reversibility × idempotency × retry combinations in `crates/ecra-core/tests/properties.rs`. **FR-036–FR-038**
- [ ] T050 [US3] Add fixed ActionDigest fixtures and field-mutation tests proving every security-relevant ActionIntent change changes digest in `crates/ecra-core/tests/action_digest.rs`. **FR-039, SC-009, SC-015**
- [ ] T051 [US3] Add invalid ActionRef wrong-digest fixture/tests under `contracts/ecra-domain-v1/invalid/` and `crates/ecra-core/tests/action_digest.rs`. **SC-002, SC-009**

---

## Phase 8 — US3: Action Attempts, Executor Receipts and Independent Verification

**Goal:** intent, attempt, executor observation and independent verification are all distinct.

- [ ] T052 [US3] Implement `ActionAttemptRef` using distinct ActionAttemptId + exact ActionRef in `crates/ecra-core/src/action.rs`. **FR-040**
- [ ] T053 [US3] Implement `ActionReceipt`, `ActionOutcome` (`executor_observed_success`, `executor_observed_failure`, `unknown`) and validation in `crates/ecra-core/src/receipt.rs`. **FR-041–FR-043**
- [ ] T054 [US3] Implement `VerificationReceipt`, target/method/outcome and validation in `crates/ecra-core/src/verification.rs`. **FR-044–FR-046**
- [ ] T055 [P] [US3] Add valid fixtures with two attempts for one ActionRef, UNKNOWN receipt, executor-observed success/failure and verified/rejected/inconclusive/not-evaluated verification under `contracts/ecra-domain-v1/valid/`. **SC-010, SC-012**
- [ ] T056 [P] [US3] Add invalid wrong ActionRef-attempt binding, receipt timing, type-confusion, missing verification target/verifier/evidence fixtures under `contracts/ecra-domain-v1/invalid/`. **SC-002, SC-010, SC-012**
- [ ] T057 [US3] Add tests proving two attempts remain distinct, receipts bind exact ActionRef+attempt, UNKNOWN round-trips, ActionReceipt cannot deserialize/cast to VerificationReceipt and executor success never equals VERIFIED in `crates/ecra-core/tests/contract_fixtures.rs`. **FR-040–FR-046, SC-010–SC-012**

---

## Phase 9 — US6: Strict Versioned Contract, Fixture Runner and Portability

**Goal:** the entire v1 contract behaves deterministically as one portable API.

- [ ] T058 [US6] Ensure all normative public v1 objects use explicit Serde names and strict unknown-field behavior where required in `crates/ecra-core/src/*.rs`. **FR-047**
- [ ] T059 [US6] Build valid/invalid fixture runners that discover every committed fixture and assert expected type/error code in `crates/ecra-core/tests/contract_fixtures.rs` and `invalid_fixtures.rs`. **SC-001, SC-002, SC-014**
- [ ] T060 [US6] Add canonical byte + ActionDigest expected outputs for normative fixtures in `crates/ecra-core/tests/canonicalization.rs`, `action_digest.rs` and `contracts/ecra-domain-v1/`. **FR-051, SC-015**
- [ ] T061 [US6] Add rustdoc examples for safe Actor/Principal, explicit Scope, capability request/grant, classified information, ActionRef/attempt/receipt and verification construction in relevant `src/*.rs`. **SC-004**
- [ ] T062 [US6] Add portability tests proving supported contract behavior is identical across platform-independent fixture inputs and does not inspect environment/OS services in `crates/ecra-core/tests/contract_fixtures.rs`. **FR-049, SC-016, SC-017**

---

## Phase 10 — Cross-Cutting Security / Architecture Gates

- [ ] T063 Add dependency-boundary automation using `cargo metadata`/`cargo tree` or a small repository script; fail on prohibited categories from FR-050. **SC-003, SC-017**
- [ ] T064 Enforce/verify zero unsafe code in `ecra-core` via crate lint plus CI/static evidence. **FR-052, SC-016**
- [ ] T065 Add structured error-code tests covering all contract categories without display-string parsing in `crates/ecra-core/tests/invalid_fixtures.rs`. **FR-053**
- [ ] T066 [P] Audit all free-form fields (`label`, `reason`, `purpose`, `notes`, locator, provider metadata, external refs) and add rustdoc/tests proving they are non-authoritative. **FR-054**
- [ ] T067 [P] Update canonical donor/license ledger for exact Serde/JSON/UUID/URL/JCS/SHA-256/error/property-test dependencies and verify no donor source was copied without exact provenance. **G10**
- [ ] T068 Add `crates/ecra-core/README.md` / crate-level architecture map linking each module/type to owning FR/entity/contract and explicitly documenting seven misuse warnings from `plan.md`. **SC-004**
- [ ] T069 Run offline tests after dependency availability and demonstrate no network/browser/model/database/process/secret-service access. **FR-049, FR-050, SC-016**

---

## Phase 11 — Pre-Closure Spec Kit Traceability / Review Remediation

- [ ] T070 Map FR-001–FR-055 and SC-001–SC-020 to tasks/tests/contracts; record matrix in implementation PR/report or a feature traceability artifact. **SC-020**
- [ ] T071 Re-check constitution v1.1.0 gates G1–G15 against exact implementation; any failed gate blocks closure.
- [ ] T072 Resolve every ECR-001-owned blocker from `specs/000-ecra-platform/pre-implementation-review-2026-08-27.md` with exact code/test evidence; do not mark downstream-only findings as implemented. **SC-019**
- [ ] T073 Run all commands/manual cases in revised `quickstart.md` on exact feature head and record toolchain/changed files/test totals/dependency/unsafe/fixture/digest evidence. **SC-018**
- [ ] T074 Run `/speckit.analyze`-equivalent review across spec/research/data-model/contract/plan/tasks/implementation; treat MUST-level drift as blocker. **SC-020**
- [ ] T075 If implementation reveals unmet requirements, append a `Phase 12 — Convergence` section with new traceable tasks; complete it before closure rather than hiding/reclassifying the gap.
- [ ] T076 Update `specs/000-ecra-platform/roadmap.md` to `CLOSED_CANONICAL` only after all exact-head Definition-of-Done evidence passes; otherwise use truthful intermediate status.

---

## Dependency Graph

```text
Phase 1 Workspace
  ↓
Phase 2 Foundations
  ↓
Phase 3 Actor/Identity/Origin/Resource/Scope
  ├───────────────┐
  ↓               ↓
Phase 4 Capability   Phase 5 Information/Evidence/Artifacts
  └──────────┬─────────────┘
             ↓
Phase 6 InformationUse
             ↓
Phase 7 Action/Effect/Digest
             ↓
Phase 8 Attempt/Receipt/Verification
             ↓
Phase 9 Versioned Contract/Portability
             ↓
Phase 10 Architecture/Security Gates
             ↓
Phase 11 Traceability/Exact-Head Closure
```

## Parallel Lanes

After Phase 2:
- Actor/identity/origin/resource work can partially parallelize by module.
- Capability shape and evidence/artifact primitives can parallelize after Scope/ID prerequisites.
- Fixture creation marked `[P]` can parallelize with implementation once the normative type is stable.

Do not parallelize edits to the same normative contract/digest code or files whose semantics are still changing.

## MVP / Incremental Delivery

ECR-001 is a contract slice, so partial production release is not the objective. Review milestones can still converge incrementally:

1. typed foundations + Actor/Principal/Scope;
2. capability + information/provenance;
3. action/digest/attempt/receipt/verification;
4. full fixture/contract/architecture gates;
5. exact-head analyze and closure.

No later ECR slice becomes implementation-eligible from a partially complete ECR-001 milestone.
