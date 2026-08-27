# Tasks: Trusted Domain Kernel

**Feature:** ECR-001  
**Input:** `spec.md`, `research.md`, `data-model.md`, `contracts/domain-v1.md`, `plan.md`, `quickstart.md`  
**Status:** TASKS_READY

## Format

`[ID] [P?] [Story] Description with exact target path`

- `[P]` means the task may run in parallel with other tasks in the same phase when its dependencies are complete.
- `[US#]` maps implementation work to the numbered user story in `spec.md`.
- Tests are mandatory because the feature specification explicitly requires them.

---

## Phase 1 — Repository / Rust Setup

**Goal:** Establish the smallest reproducible Rust workspace authorized by the plan.

- [ ] T001 Create root Cargo workspace with only `crates/ecra-core` as a production member in `Cargo.toml`.
- [ ] T002 Pin Rust 1.98.x stable / Edition 2024 in `rust-toolchain.toml` and `crates/ecra-core/Cargo.toml`.
- [ ] T003 Add workspace lint policy with warnings denied in CI and forbid unsafe code in `crates/ecra-core/src/lib.rs`.
- [ ] T004 Add only research-approved initial dependencies to `crates/ecra-core/Cargo.toml`; record each dependency/source/license in `research/license-ledger.md` or the canonical repository donor/license ledger before merge.
- [ ] T005 Create contract fixture directories and README at `contracts/ecra-domain-v1/{valid,invalid}/` and `contracts/ecra-domain-v1/README.md`.
- [ ] T006 Add baseline CI commands/documentation for fmt, Clippy, tests, rustdoc and dependency boundary checks in the repository CI/config location chosen by implementation; do not add unrelated release infrastructure owned by ECR-024.

**Independent gate:** `cargo build --workspace --locked` succeeds with one production crate and no prohibited dependency category.

---

## Phase 2 — Foundational Value Objects

**Goal:** Implement versioning, errors, strong IDs, caller-supplied time and canonicalization primitives used by all stories.

- [ ] T007 [P] Implement `SchemaVersion`, v1 compatibility validation and typed compatibility errors in `crates/ecra-core/src/version.rs` and `crates/ecra-core/src/error.rs`. **FR-001, FR-033, FR-040**
- [ ] T008 [P] Implement strong typed UUID ID newtypes for Actor/Run/Capability/Observation/Fact/Evidence/Artifact/Action/Receipt/Verification in `crates/ecra-core/src/id.rs` (or plan-equivalent module). **FR-002, FR-003, FR-040**
- [ ] T009 [P] Implement `EpochMillis`, `TemporalValidity`, `EvaluationContext` and structural range validation in `crates/ecra-core/src/time.rs`. **FR-013, FR-035**
- [ ] T010 [P] Implement RFC 8785 canonicalization wrapper behind Ecra-owned API in `crates/ecra-core/src/canonical.rs`. **FR-004, FR-037**
- [ ] T011 Add unit tests for unsupported versions, typed ID parsing/separation, temporal ranges and canonicalization errors in module tests / `crates/ecra-core/tests/`. **SC-001, SC-002, SC-009, SC-010**
- [ ] T012 Add RFC 8785 / Ecra canonicalization fixtures and fixed-point tests in `crates/ecra-core/tests/canonicalization.rs` and `contracts/ecra-domain-v1/`. **FR-037, SC-010**

**Independent gate:** foundational types validate without I/O and canonicalization is deterministic on committed fixtures.

---

## Phase 3 — User Story 1: Actors, Origins, Resources, and Action Attribution

**Story goal:** A human, agent, or system action can be represented without losing actor or origin context.

**Independent test:** construct/round-trip actors, origins, resources and action-attribution fixture skeletons and verify no content string changes authority semantics.

- [ ] T013 [P] [US1] Implement `Actor` and `ActorKind` in `crates/ecra-core/src/actor.rs`. **FR-005, FR-006**
- [ ] T014 [P] [US1] Implement `WebOrigin`, `Origin`, `OriginKind` and strict validation in `crates/ecra-core/src/origin.rs`. **FR-007, FR-008**
- [ ] T015 [US1] Implement `ResourceRef`, `ResourceKind` and structured `Scope` in `crates/ecra-core/src/resource.rs`, depending on T014. **FR-010, FR-011**
- [ ] T016 [P] [US1] Add valid actor/origin/resource fixtures under `contracts/ecra-domain-v1/valid/`. **SC-001, SC-005**
- [ ] T017 [P] [US1] Add invalid actor/origin/resource/version fixtures under `contracts/ecra-domain-v1/invalid/`. **SC-002, SC-009**
- [ ] T018 [US1] Add contract tests proving Human/Agent/System and user/web/local/tool/model/memory origins remain distinct through serialization/canonicalization in `crates/ecra-core/tests/contract_fixtures.rs`. **SC-005**
- [ ] T019 [US1] Add tests proving arbitrary external text matching policy/instruction words does not alter `Origin` or actor/capability fields in `crates/ecra-core/tests/properties.rs`. **FR-008**

---

## Phase 4 — User Story 4: Capability Request/Grant Representation

**Story goal:** Later policy engines can express scoped authority without type-confusing requests and grants.

**Independent test:** create narrowed request/grant fixtures, expiry/delegation metadata, and invalid temporal/scope cases; prove there is no implicit request→grant conversion.

- [ ] T020 [US4] Implement `OperationRef`, `CapabilityRequest`, `CapabilityGrant`, `DelegationRef` and structural validation in `crates/ecra-core/src/capability.rs`. **FR-009–FR-014**
- [ ] T021 [P] [US4] Add valid scoped request/grant/delegation fixtures under `contracts/ecra-domain-v1/valid/`. **SC-001, SC-008**
- [ ] T022 [P] [US4] Add invalid temporal/scope/request-as-grant fixtures under `contracts/ecra-domain-v1/invalid/`. **SC-002, SC-008**
- [ ] T023 [US4] Add compile/runtime contract tests proving no `From<CapabilityRequest> for CapabilityGrant` or implicit equivalent exists and grant parsing is explicit in `crates/ecra-core/tests/contract_fixtures.rs`. **SC-008**
- [ ] T024 [US4] Add caller-supplied time evaluation tests for capability temporal validity without OS clock access in `crates/ecra-core/tests/properties.rs`. **FR-035**

---

## Phase 5 — User Story 2: Observations, Facts, Evidence, Artifacts, Provenance

**Story goal:** Ecra can represent observed/retrieved/inferred information and independent trust state without collapsing them.

**Independent test:** represent a web observation, a model-inferred fact derived from it, independent verification, contradiction, staleness and artifact lineage; round-trip without losing any dimension.

- [ ] T025 [P] [US2] Implement `Observation`, observation payload reference and provenance-bearing origin references in `crates/ecra-core/src/evidence.rs`. **FR-015, FR-016**
- [ ] T026 [P] [US2] Implement `ArtifactRef`, `ArtifactKind`, content digest/size representation and lineage in `crates/ecra-core/src/artifact.rs`. **FR-031, FR-032**
- [ ] T027 [US2] Implement `Fact`, `FactValue`, `Provenance`, `TrustState`, `Freshness`, `EvidenceRef`, `EvidenceKind` and derived-fact lineage in `crates/ecra-core/src/evidence.rs`, depending on T025/T026 as needed. **FR-015–FR-020**
- [ ] T028 [P] [US2] Add valid observation/fact/evidence/artifact lineage fixtures under `contracts/ecra-domain-v1/valid/`, including `model_inferred + verified` and contradicted evidence. **SC-001, SC-006**
- [ ] T029 [P] [US2] Add invalid provenance/evidence/artifact numeric/canonicalization fixtures under `contracts/ecra-domain-v1/invalid/`. **SC-002**
- [ ] T030 [US2] Add tests proving verification never erases original provenance and stale/contradicted states remain orthogonal in `crates/ecra-core/tests/contract_fixtures.rs`. **FR-017–FR-019, SC-006**
- [ ] T031 [US2] Add property tests for evidence lists/lineage round-trip and invalid byte-size/digest/time forms in `crates/ecra-core/tests/properties.rs`. **FR-020, FR-031, FR-032**

---

## Phase 6 — User Story 3: Action and Side-Effect Semantics

**Story goal:** Ecra represents risky actions conservatively before execution.

**Independent test:** construct every side-effect/idempotency/retry class combination and verify invalid permissive combinations are rejected.

- [ ] T032 [US3] Implement `ActionIntent`, parameter reference, `SideEffectClass`, `IdempotencyClass`, `IdempotencySpec`, and `RetryClass` in `crates/ecra-core/src/action.rs`. **FR-021–FR-025**
- [ ] T033 [US3] Implement cross-field action semantic validation required by `contracts/domain-v1.md` in `crates/ecra-core/src/action.rs`. **FR-022–FR-024**
- [ ] T034 [P] [US3] Add valid read-only/keyed-idempotent/non-idempotent-conservative action fixtures under `contracts/ecra-domain-v1/valid/`. **SC-001, SC-007**
- [ ] T035 [P] [US3] Add invalid keyed-without-key, non-idempotent+safe, unknown-idempotency+safe and unsafe irreversible combinations under `contracts/ecra-domain-v1/invalid/`. **SC-002, SC-007**
- [ ] T036 [US3] Add exhaustive/table-driven or property tests across action semantic combinations in `crates/ecra-core/tests/properties.rs`. **FR-023, FR-024, SC-007**

---

## Phase 7 — User Story 3: Action Receipts and Independent Verification Receipts

**Story goal:** Executor-known outcome is represented honestly and independently verifiable later.

**Independent test:** unknown/confirmed receipts and verification receipts parse independently; invalid timing and type-confusion cases fail.

- [ ] T037 [P] [US3] Implement `ActionReceipt`, `ActionOutcome`, structured error summary and receipt validation in `crates/ecra-core/src/receipt.rs`. **FR-026–FR-028**
- [ ] T038 [P] [US3] Implement `VerificationReceipt`, target/method/outcome and validation in `crates/ecra-core/src/verification.rs`. **FR-029, FR-030**
- [ ] T039 [P] [US3] Add valid unknown/success/failure action receipt and verified/rejected/inconclusive/not-evaluated verification fixtures under `contracts/ecra-domain-v1/valid/`. **SC-001, SC-007**
- [ ] T040 [P] [US3] Add invalid receipt timing/missing target/missing verifier/type-confusion fixtures under `contracts/ecra-domain-v1/invalid/`. **SC-002**
- [ ] T041 [US3] Add tests proving `ActionReceipt` cannot be deserialized/cast as `VerificationReceipt` and UNKNOWN round-trips unchanged in `crates/ecra-core/tests/contract_fixtures.rs`. **FR-028–FR-030, SC-007**

---

## Phase 8 — User Story 5: Versioned Contract and Portability

**Story goal:** The contract is deterministic, strict, portable and ready for later persisted/protocol adapters.

**Independent test:** run the complete normative fixture corpus; unsupported versions and undocumented fields fail with typed errors; valid fixtures canonicalize deterministically.

- [ ] T042 [US5] Implement top-level `Versioned<T>` parsing/serialization helpers and strict schema dispatch in `crates/ecra-core/src/version.rs`. **FR-001, FR-033, FR-038**
- [ ] T043 [US5] Ensure every public normative type uses explicit Serde names/strict unknown-field behavior required by v1; add regression tests in `crates/ecra-core/tests/contract_fixtures.rs`. **FR-034, FR-038**
- [ ] T044 [US5] Build fixture runner that discovers every committed valid/invalid fixture and asserts expected behavior in `crates/ecra-core/tests/contract_fixtures.rs` and `invalid_fixtures.rs`. **SC-001, SC-002, SC-009**
- [ ] T045 [US5] Add canonical byte expectations/digests for normative fixtures and fixed-point assertions in `crates/ecra-core/tests/canonicalization.rs`. **FR-037, SC-010**
- [ ] T046 [US5] Add rustdoc examples for safe construction/validation of actors, capabilities, facts, actions and receipts in the relevant `crates/ecra-core/src/*.rs` modules. **SC-004**

---

## Phase 9 — Cross-Cutting Architecture and Security Gates

- [ ] T047 Add an automated dependency-boundary check using `cargo metadata`/`cargo tree` or a small repository script that fails if `ecra-core` acquires prohibited runtime dependency categories; place it under the repository CI/scripts path selected in T006. **FR-036, SC-003, SC-012**
- [ ] T048 Verify and enforce no `unsafe` in `ecra-core`, with `forbid(unsafe_code)` and CI/static evidence. **FR-039**
- [ ] T049 Add structured error-code/category tests proving callers never need to parse display strings in `crates/ecra-core/tests/invalid_fixtures.rs`. **FR-040**
- [ ] T050 [P] Update canonical donor/license ledger with all implementation dependencies and confirm no donor source was copied without an explicit entry. **Constitution G10**
- [ ] T051 [P] Review all public free-form string fields (`label`, `reason`, `notes`, locators) and document/test that they are non-authoritative metadata. **FR-008, Constitution II/IV**
- [ ] T052 [P] Add architecture documentation linking each public module/type to its owning FR/entity/contract section in `crates/ecra-core/README.md` or crate-level rustdoc. **SC-004**

---

## Phase 10 — Spec Kit Traceability, Verification, and Closure

- [ ] T053 Run all commands from `specs/001-trusted-domain-kernel/quickstart.md` on exact feature head and record results in the implementation PR/report. **SC-011, SC-013**
- [ ] T054 Perform requirement traceability review: map FR-001–FR-040 and SC-001–SC-015 to tasks/tests/contracts; amend `tasks.md` only through normal task generation/convergence rules if a gap is found. **SC-015**
- [ ] T055 Perform constitution gate re-check against exact implementation, including zero I/O, zero ambient authority semantics, receipt/verification separation, dependency boundary and donor provenance.
- [ ] T056 Run Spec Kit analyze-equivalent drift review across `spec.md`, `research.md`, `data-model.md`, `contracts/`, `plan.md`, implementation and `tasks.md`; treat MUST-level conflicts as blockers.
- [ ] T057 If implementation leaves unmet requirements or acceptance criteria, append a `Phase 11: Convergence` section with traceable tasks; complete them before closure rather than rewriting completed task history.
- [ ] T058 Update `specs/000-ecra-platform/roadmap.md` status for ECR-001 to `CLOSED_CANONICAL` only after exact-head evidence satisfies the Definition of Done; otherwise use the truthful intermediate status.

---

## Dependencies

### Phase dependency graph

```text
Phase 1 Setup
   ↓
Phase 2 Foundations
   ↓
┌───────────────┬─────────────────┐
│               │                 │
Phase 3 US1   Phase 4 US4      Phase 5 US2
│               │                 │
└───────┬───────┴────────┬────────┘
        ↓                ↓
    Phase 6 US3 Action Semantics
        ↓
    Phase 7 Receipts/Verification
        ↓
    Phase 8 US5 Contract/Portability
        ↓
    Phase 9 Cross-cutting Gates
        ↓
    Phase 10 Closure
```

Notes:
- US1/US2/US4 work can partially parallelize after foundational types because modules differ.
- ActionIntent depends on actor/resource/capability foundations.
- Receipt/verification depends on action/evidence entities.
- Full versioned fixture runner waits until all normative entities exist.

## User Story Completion Order

1. **US1 P1** — actor/origin/action attribution primitives.
2. **US4 P1** — authority request/grant representation can proceed alongside US2.
3. **US2 P1** — provenance/evidence model.
4. **US3 P1** — action/receipt/verification semantics after shared prerequisites.
5. **US5 P2** — finalizes portable versioned contract over the completed model.

## Parallel Execution Examples

### After Phase 2

```text
Engineer/agent A: T013–T019 (Actor/Origin/Resource)
Engineer/agent B: T020–T024 (Capability)
Engineer/agent C: T025–T031 (Evidence/Artifact)
```

Merge/integrate those before T032+.

### Receipt layer

```text
T037 ActionReceipt        [parallel]
T038 VerificationReceipt  [parallel]
T039/T040 fixture authoring can parallelize by separate files
```

## Requirement Traceability Summary

| Requirement group | Primary tasks |
|---|---|
| FR-001–004 version/identity/validation | T007–T012, T042–T045 |
| FR-005–008 actors/origins | T013–T019 |
| FR-009–014 capabilities | T020–T024 |
| FR-015–020 observation/fact/evidence | T025–T031 |
| FR-021–025 actions | T032–T036 |
| FR-026–030 receipts/verification | T037–T041 |
| FR-031–032 artifacts | T026, T028–T031 |
| FR-033–040 compatibility/safety | T042–T052 |
| SC-001–015 closure evidence | T011–T058 collectively; final proof T053–T058 |

## MVP Strategy

ECR-001 itself is foundational and should be implemented as one bounded slice, but review may be staged:

1. foundation + actor/origin/capability types;
2. evidence/artifacts;
3. actions/receipts/verifications;
4. complete contract fixtures and closure gates.

Do not begin ECR-002 implementation merely because a partial ECR-001 API compiles. ECR-002 depends on ECR-001 `CLOSED_CANONICAL` unless its own spec explicitly authorizes fixture-only parallel research.
