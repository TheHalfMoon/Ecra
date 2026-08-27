# Post-Implementation Analyze — ECR-001 Trusted Domain Kernel

**Date:** 2026-08-27  
**Mode:** `/speckit.analyze`-equivalent, repository-truth review  
**Implementation branch:** `001-trusted-domain-kernel`  
**Implementation head reviewed:** `5dfe4c09b2abceeec14bc94b8e13d2dccddfd37c`  
**Phase 10 exact-head CI:** run `33086490495` — PASS  
**Decision:** `CONVERGENCE_REQUIRED`

## Scope reviewed

- `.specify/memory/constitution.md` v1.1.0
- `spec.md`
- `research.md`
- `data-model.md`
- `contracts/domain-v1.md`
- `plan.md`
- `tasks.md`
- `quickstart.md`
- `implementation-clarifications.md`
- `STATUS.md`
- `EXECUTION.md`
- `research/donor-license-ledger.md`
- `crates/ecra-core/**`
- `contracts/ecra-domain-v1/**`
- `.github/workflows/ecr-001.yml`
- `scripts/check-core-{deps,unsafe}.sh`
- pre-implementation review findings owned by or adjacent to ECR-001

## Result

The implementation is materially stronger and more explicit than the planning-era canonical package in several areas. Phase 10 gates are green, but ECR-001 is **not eligible for closure yet** because canonical documentation has MUST-level drift from the implemented and tested v1 contract. T075 therefore activates a required Phase 12 Convergence rather than hiding these differences in a closure report.

## Blocking findings

### A-001 — Version-envelope missing-field error contract is stale

`contracts/domain-v1.md` states that a missing envelope produces machine code `missing_schema_version`.

The implemented `Versioned<T>::from_json_slice` first decodes the strict wire envelope using Serde. A missing `schema_version`/`value` field is therefore a structural serialization failure and maps to `DomainError::Serialization` / `ErrorCode::SerializationFailed`. The machine error model intentionally has no `MissingSchemaVersion` variant.

FR-047 requires typed compatibility errors for unsupported major/newer minor plus strict security-sensitive parsing; it does not require a separate missing-envelope compatibility code. The convergent resolution is to make the canonical contract say:

- unsupported major → `unsupported_major_version`;
- unsupported newer minor → `unsupported_minor_version`;
- malformed/missing strict envelope or unknown strict field → `serialization_failed`;
- valid supported envelope → typed value.

**Owner:** Phase 12.  
**Files:** `contracts/domain-v1.md`, `data-model.md`, error/compatibility traceability.

### A-002 — Planning error-category names do not match the machine API

The planning data model/contract lists conceptual names such as `IdentityReferenceError`, `InformationFlowShapeError`, `EvidenceError`, `ActionSemanticError`, and `ActionReferenceError`.

The implemented machine API exposes exactly 16 `ErrorCategory` variants and 19 `ErrorCode` variants. Phase 10 T065 now tests the complete matrix directly without parsing display strings.

Canonical docs must name the actual machine contract and may separately describe which detailed validation failures are intentionally represented by broader categories.

**Owner:** Phase 12.  
**Files:** `data-model.md`, `contracts/domain-v1.md`, implementation report.

### A-003 — Bounded implementation clarifications C1–C12 are not fully folded into the primary contract

`implementation-clarifications.md` explicitly says its resolutions must converge into the primary package before `CLOSED_CANONICAL`. The implementation and fixtures already rely on them.

Required canonical convergence includes at least:

1. exact `ObservationPayloadRef` variants and opaque external-reference rule;
2. exact `FactValue` variants and canonical decimal/I-JSON rules;
3. exact `LineageRef` stable-ID variants;
4. `EvidenceRef.external_ref` non-empty/non-authoritative rule;
5. paired freshness basis kind/time invariant;
6. artifact `byte_size_decimal` canonical non-negative decimal rule;
7. free-form metadata non-empty-when-required/non-authoritative rule, including capability-request `reason`;
8. task-order note that base `InformationRef` existed before `InformationUse` construction;
9. exact `ActionParametersRef` security-digest binding and opaque `ActionParameterRef.path`;
10. complete Effect × Idempotency × Retry compatibility matrix, not only selected examples;
11. exact `ClaimRef`, `ErrorSummary`, receipt timing and verification evidence-cardinality semantics;
12. fixture storage convention: semantic inner-body fixtures may be committed while the normative wire object remains `Versioned<T>` and the runner supplies/verifies the envelope.

**Owner:** Phase 12.  
**Files:** `data-model.md`, `contracts/domain-v1.md`, `tasks.md`, optionally `quickstart.md` where verification behavior is affected.

### A-004 — Quickstart verification guide is behind the actual gate surface

The guide still emphasizes legacy/manual unsafe and dependency checks and does not explicitly run the exhaustive `valid_fixtures`, portability, or non-authoritative-metadata targets introduced during convergence.

The repository now has dedicated fail-closed scripts and CI steps:

```text
bash scripts/check-core-unsafe.sh
bash scripts/check-core-deps.sh
```

The revised quickstart must execute the current fixture/security targets and remain consistent with CI.

**Owner:** Phase 12.  
**File:** `quickstart.md`.

### A-005 — Execution ledgers lag the verified implementation

`tasks.md`, `STATUS.md`, and `EXECUTION.md` still contain planning-era unchecked/current-phase states for work that is already exact-head verified. The repository handoff rule requires these documents to be self-explanatory from live truth.

**Owner:** Phase 12/Phase 11 closure.  
**Files:** `tasks.md`, `STATUS.md`, `EXECUTION.md`.

## Non-blocking / intentionally downstream items

The analyze review does **not** reclassify downstream enforcement as ECR-001 work:

- principal assertion validity, trust roots and protected sensitive storage remain ECR-031;
- grant narrowing, authorization decisions/leases, disclosure/declassification, approvals and secrets remain ECR-003;
- durable runs, attempt lifecycle, budgets/cancellation and persistence/integrity chain remain ECR-002;
- evidence sufficiency, verifier orchestration and reconciliation remain ECR-004;
- browser/process/model/protocol runtime execution is outside ECR-001.

Representation in the domain model is not a claim that those enforcement responsibilities are implemented.

## Implementation evidence already passing

At exact head `5dfe4c09b2abceeec14bc94b8e13d2dccddfd37c`, CI run `33086490495` passed:

```text
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --workspace --locked
cargo test --workspace --locked --offline
bash scripts/check-core-unsafe.sh
bash scripts/check-core-deps.sh
```

The Phase 9 exhaustive manifests cover 43 valid and 39 invalid committed JSON fixtures, and the normative ActionDigest golden value remains:

```text
6ccf10a5d1db36cb9637b4ed2ee6d3f0167c44eec585b543f58d86287710e3d4
```

These green gates do not waive the canonical-document drift above.

## Required Phase 12 convergence tasks

The active task ledger must append and complete:

```text
T077 fold C1–C12 and actual machine error/version semantics into canonical data model + contract
T078 revise quickstart and execution/task/status ledgers to current verified gate surface and phase truth
T079 produce FR/SC + constitution + pre-implementation-review traceability evidence against converged docs/implementation
T080 rerun analyze-equivalent + full quickstart/exact-head gates; permit PR readiness only with zero blocking drift
```

## Closure decision

`CONVERGENCE_REQUIRED`

ECR-001 remains `IMPLEMENTING`. PR #1 must remain Draft. No roadmap transition to `CLOSED_CANONICAL` is authorized by this analyze result.
