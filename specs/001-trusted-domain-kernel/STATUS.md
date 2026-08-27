# ECR-001 Status — Trusted Domain Kernel

**Slice:** ECR-001  
**Lifecycle:** IMPLEMENTING  
**Branch:** `001-trusted-domain-kernel`  
**PR:** `#1` (draft until full slice closure)  
**Latest fully verified implementation head:** `d29f700ca314e067f5912815b2bd5a1047a63602`

This file is the execution ledger for ECR-001. It summarizes progress; normative semantics remain in `spec.md`, `data-model.md`, `contracts/`, and approved implementation clarifications.

## Current position

```text
Phase 1  VERIFIED_ON_BRANCH
Phase 2  VERIFIED_ON_BRANCH
Phase 3  VERIFIED_ON_BRANCH
Phase 4  VERIFIED_ON_BRANCH
Phase 5  VERIFIED_ON_BRANCH
Phase 6  NEXT_ACTIVE_PHASE
Phase 7+ BLOCKED_BY_ORDERING
```

## Verified phase evidence

### Phase 1 — T001–T006

Outcome: `VERIFIED_ON_BRANCH`

Established one production crate, Rust 1.98.x / Edition 2024, `#![forbid(unsafe_code)]`, bounded dependencies, normative fixture directories, and locked CI gates.

### Phase 2 — T007–T014

Outcome: `VERIFIED_ON_BRANCH`

Established schema/version dispatch, machine-readable errors, strong IDs, caller-supplied time, RFC 8785 JCS canonicalization, ContentDigest/SecurityDigest separation, and property/canonicalization tests.

### Phase 3 — T015–T023

Outcome: `VERIFIED_ON_BRANCH`

Established Actor/Principal separation, tuple/opaque origins, stable resource identity, explicit scope algebra, normative fixtures, and compile/runtime type-confusion coverage.

### Phase 4 — T024–T028

Outcome: `VERIFIED_ON_BRANCH`

Verified head: `992dd31c44104aa619b0ea59429063f69e559014`.

Established distinct `CapabilityRequest`/`CapabilityGrant`, structural delegation provenance, caller-supplied temporal evaluation, and no request→grant shortcut.

### Phase 5 — T029–T038

Outcome: `VERIFIED_ON_BRANCH`

Verified head: `d29f700ca314e067f5912815b2bd5a1047a63602`.

Exact-head CI run `33074624203` passed:

```text
Build locked workspace       PASS
Format                       PASS
Clippy -D warnings           PASS
Tests                        PASS
Rustdoc tests                PASS
Offline replay gate          PASS
Dependency boundary          PASS
```

Established:

- explicit public/private/sensitive/secret/unknown information classification;
- typed information references used by Fact lineage;
- Observation payload references rather than arbitrary embedded blobs;
- EvidenceRef and freshness metadata;
- Fact provenance/classification/freshness/dispute as independent axes;
- no canonical `Fact.verified` truth flag;
- deterministic FactValue v1 values and I-JSON integer bounds;
- classified ArtifactRef, stable lineage, canonical byte-size text, and non-authoritative storage locator;
- valid/invalid normative fixtures and round-trip/property coverage.

The bounded clarifications in `implementation-clarifications.md` remain normative for this PR and MUST converge into the primary contract/data model/tasks before ECR-001 closure.

## Next active phase — Phase 6 / T039–T042

Goal: represent information use/source-to-sink intent without turning it into authorization.

Required work:

- keep the base `InformationRef` introduced in Phase 5 because Fact lineage depends on it;
- implement `InformationUseKind` and `InformationUse` in `crates/ecra-core/src/information.rs`;
- require a non-empty source set;
- allow explicit optional destination ResourceRef and WebOrigin plus declared output classification;
- add valid local-compute/model-context/persist/log/external-disclosure/remote-provider fixtures;
- add invalid empty-source/malformed-destination fixtures;
- prove InformationUse cannot become CapabilityGrant/authorization and separate read/write capabilities do not imply A→B disclosure.

Required safety property:

```text
information-use declaration != information-flow authorization
read(A) + write(B) != disclose(A → B)
```

ECR-003 owns actual source-to-sink policy, declassification, and authorization decisions.

## Phase ordering after Phase 6

```text
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
