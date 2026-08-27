# ECR-001 Status — Trusted Domain Kernel

**Slice:** ECR-001  
**Lifecycle:** IMPLEMENTING  
**Branch:** `001-trusted-domain-kernel`  
**PR:** `#1` (draft until full slice closure)  
**Last fully verified implementation head before Phase 5:** `992dd31c44104aa619b0ea59429063f69e559014`

This file is the execution ledger for ECR-001. It summarizes progress; normative semantics remain in `spec.md`, `data-model.md`, `contracts/`, and approved implementation clarifications.

## Current position

```text
Phase 1  VERIFIED_ON_BRANCH
Phase 2  VERIFIED_ON_BRANCH
Phase 3  VERIFIED_ON_BRANCH
Phase 4  VERIFIED_ON_BRANCH
Phase 5  NEXT_ACTIVE_PHASE
Phase 6+ BLOCKED_BY_ORDERING
```

## Verified phase evidence

### Phase 1 — T001–T006

Outcome: `VERIFIED_ON_BRANCH`

Established:

- one production crate: `crates/ecra-core`;
- Rust 1.98.x / Edition 2024 baseline;
- `#![forbid(unsafe_code)]` and workspace lint policy;
- bounded dependency set;
- normative fixture directories;
- CI gates for locked build, fmt, Clippy, tests, rustdoc, offline replay, and dependency boundary.

### Phase 2 — T007–T014

Outcome: `VERIFIED_ON_BRANCH`

Established deterministic zero-I/O primitives:

- schema/version dispatch;
- machine-readable errors;
- strong ID types;
- caller-supplied time values;
- RFC 8785 JCS canonicalization;
- ContentDigest vs SecurityDigest separation;
- property/canonicalization tests.

### Phase 3 — T015–T023

Outcome: `VERIFIED_ON_BRANCH`

Exact-head evidence was green after adding normative valid/invalid fixtures and compile/runtime type-confusion coverage.

Key invariants:

- Actor attribution != Principal authentication;
- tuple/opaque origins are explicit;
- resource locator metadata grants nothing;
- missing/empty scope is not wildcard;
- `any_explicit` is the only unrestricted scope representation;
- ActorId cannot implicitly become PrincipalId.

### Phase 4 — T024–T028

Outcome: `VERIFIED_ON_BRANCH`

Verified head: `992dd31c44104aa619b0ea59429063f69e559014`.

Exact-head CI passed:

```text
Build locked workspace       PASS
Format                       PASS
Clippy -D warnings           PASS
Tests                        PASS
Rustdoc tests                PASS
Offline replay gate          PASS
Dependency boundary          PASS
```

Key invariants:

- `CapabilityRequest` and `CapabilityGrant` have distinct types and IDs;
- no request→grant implicit conversion;
- delegation records provenance only, not subset validity;
- temporal evaluation uses caller-supplied `EvaluationContext` only;
- capability types remain provider/policy-syntax neutral.

## Next active phase — Phase 5 / T029–T038

Goal: information trust/disclosure metadata survives derivation without becoming permission.

Required work:

- T029 `InformationClass`, `InformationPolicyTag`, `InformationClassification`.
- T030 `Observation` and payload reference.
- T031 `ArtifactRef`, `ArtifactKind`, content/size/storage metadata, lineage.
- T032 `FreshnessAssessment`, `FreshnessState`, `FreshnessBasisKind`.
- T033 `Fact`, `FactValue`, `Provenance`, `DisputeState`, derived `InformationRef` lineage.
- T034 `EvidenceRef`, `EvidenceKind`, immutable capture/as-of metadata.
- T035 valid normative fixtures.
- T036 invalid normative fixtures.
- T037 orthogonality tests and proof that Fact has no canonical VERIFIED flag.
- T038 lineage/classification round-trip properties.

### Phase 5 implementation clarification

Implementation review found two planning underspecifications that must be made explicit before relying on them:

1. `ObservationPayloadRef`, `FactValue`, and `LineageRef` were named but their exact v1 wire shapes were not fully specified.
2. Fact lineage needs the base `InformationRef` in Phase 5 even though `InformationUse` remains Phase 6.

Record the bounded resolution in `implementation-clarifications.md`, keep it provider-neutral, and converge it into the canonical contract/package before ECR-001 closure.

Required safety properties:

```text
Provenance != Classification != Freshness != Verification
unknown classification != public
Fact has no verified: bool
ContentDigest != authenticity proof
storage locator != authority
```

## Phase ordering after Phase 5

```text
Phase 5  Information / evidence / fact / artifact
  ↓
Phase 6  InformationUse / source-to-sink declaration
  ↓
Phase 7  Effects / idempotency / retry / ActionIntent / ActionDigest
  ↓
Phase 8  ActionAttempt / ActionReceipt / VerificationReceipt
  ↓
Phase 9  strict v1 fixture runner / portability / public contract convergence
  ↓
Phase 10 cross-cutting security / dependency / architecture / closure gates
```

Do not begin Phase 6 merely because some Phase 6 type is useful. If a dependency such as base `InformationRef` is required earlier, document the ordering clarification while keeping the later behavior (for example InformationUse authorization semantics) in its owning phase.

## Per-batch verification gate

Every material batch must pass on its exact head:

```bash
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --workspace --locked
cargo test --workspace --locked --offline
bash scripts/check-core-deps.sh
```

No later phase is eligible after a failing gate.

## ECR-001 closure checklist

ECR-001 may be called `CLOSED_CANONICAL` only when all are true:

- every task in `tasks.md` is satisfied by implementation + required evidence;
- every normative valid fixture passes and invalid fixture fails as specified;
- strict versioning/canonicalization/digest contracts converge;
- no prohibited dependency category or unsafe code exists;
- quickstart/full exact-head gate passes;
- rustdoc/compile-fail contract examples pass;
- final spec/research/data-model/contracts/plan/tasks/implementation traceability review passes;
- implementation clarifications are reconciled into canonical package documents;
- donor/license ledger is current;
- final analyze/convergence finds no blocking gap;
- PR is ready, reviewed as required, and merged without destructive history rewriting;
- required post-merge verification on canonical `main` is recorded;
- platform roadmap and `EXECUTION.md` are advanced to the next eligible slice.

Until then, ECR-002 and all dependent slices remain implementation-blocked.
