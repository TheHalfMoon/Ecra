# ECR-001 Status — Trusted Domain Kernel

**Slice:** ECR-001  
**Lifecycle:** IMPLEMENTING  
**Branch:** `001-trusted-domain-kernel`  
**PR:** `#1` (draft until full slice closure)  
**Latest fully verified implementation head:** `946e95366ed681c724192cd01ece199d5e8f55a7`

This file is the execution ledger for ECR-001. It summarizes progress; normative semantics remain in `spec.md`, `data-model.md`, `contracts/`, and approved implementation clarifications.

## Current position

```text
Phase 1  VERIFIED_ON_BRANCH
Phase 2  VERIFIED_ON_BRANCH
Phase 3  VERIFIED_ON_BRANCH
Phase 4  VERIFIED_ON_BRANCH
Phase 5  VERIFIED_ON_BRANCH
Phase 6  VERIFIED_ON_BRANCH
Phase 7  VERIFIED_ON_BRANCH
Phase 8  VERIFIED_ON_BRANCH
Phase 9  VERIFIED_ON_BRANCH
Phase 10 NEXT_ACTIVE_PHASE
Phase 11 BLOCKED_BY_ORDERING
```

`VERIFIED_ON_BRANCH` is not `CLOSED_CANONICAL`.

## Exact-head evidence

### Phase 6 — T039–T042

Verified head: `b0f4ae4cb15d4ccb38cdd8cdbc0b764689fd29f5`  
CI run: `33075545972` — `success`

Established declaration-only source-to-sink information use semantics. `InformationUse` remains structurally distinct from authorization, and read authority over A plus write authority to B does not grant disclosure A → B.

### Phase 7 — T043–T051

Verified head: `ea17736310f661149026153a90a202e36396ba45`  
CI run: `33078470973` — `success`

Established:

- `MutationDomain`, `Reversibility`, `EffectProfile`;
- `IdempotencyClass`, `IdempotencySpec`, `RetryClass` with fail-closed cross-field validation;
- `ActionParametersRef` with SHA-256 `SecurityDigest` binding for every non-empty parameter payload reference;
- `ActionParameterRef` for information lineage without authority semantics;
- pre-authorization/pre-execution `ActionIntent`;
- domain-separated `ActionDigest` over JCS `Versioned<ActionIntent>`;
- immutable `ActionRef { id, digest }` validation;
- valid/invalid action fixtures;
- exhaustive effect × reversibility × idempotency × retry coverage;
- fixed ActionDigest golden/mutation tests;
- wrong-digest ActionRef rejection.

The verified golden digest for the committed Phase 7 golden fixture is:

```text
6ccf10a5d1db36cb9637b4ed2ee6d3f0167c44eec585b543f58d86287710e3d4
```

### Phase 8 — T052–T057

Verified head: `0b273f41f853f61e3dd691d4dcd5c2149c28f166`  
CI run: `33080355344` — `success`

Established:

- `ActionAttemptRef` with distinct `ActionAttemptId` and exact `ActionRef` binding;
- `ActionReceipt` with executor-only `ActionOutcome` and fail-closed timing validation;
- bounded `ErrorSummary` diagnostic metadata;
- independent `VerificationReceipt`, `VerificationTarget`, `VerificationMethod`, and `VerificationOutcome`;
- bounded `ClaimRef` target shape;
- fail-closed evidence cardinality: verified/rejected/inconclusive require evidence, `not_evaluated` may have none;
- two distinct attempts for one immutable action fixture;
- UNKNOWN, executor-observed-success, and executor-observed-failure receipt fixtures;
- all four verification-outcome fixtures;
- invalid wrong-binding, timing, receipt→verification type-confusion, missing target/verifier/evidence, and empty-evidence verification coverage;
- round-trip proof that UNKNOWN remains UNKNOWN;
- explicit proof that `executor_observed_success != verified`.

### Phase 9 — T058–T062

Verified head: `946e95366ed681c724192cd01ece199d5e8f55a7`  
CI run: `33083362584` — `success`

All exact-head gates passed:

```text
cargo build --workspace --locked                                      PASS
cargo fmt --all --check                                               PASS
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings  PASS
cargo test --workspace --locked                                       PASS
cargo test --doc --workspace --locked                                 PASS
cargo test --workspace --locked --offline                             PASS
bash scripts/check-core-deps.sh                                       PASS
```

Established:

- strict unknown-field rejection for the remaining tagged public reference enums (`Origin`, `InformationRef`, `LineageRef`);
- exhaustive typed manifests for every committed valid and invalid JSON fixture, with directory/manifest drift detection;
- valid fixtures round-trip both as semantic values and through supported `Versioned<T>` v1 dispatch;
- invalid fixtures assert machine-readable `ErrorCode`/category without parsing display strings;
- explicit unsupported-major, unsupported-minor, version-envelope unknown-field, and tagged-enum unknown-field fixtures;
- fixed byte-exact JCS artifact for `Versioned<ActionIntent>` plus fixed domain-separated SHA-256 output;
- executable rustdoc examples for Actor/Principal, explicit Scope, request/grant, classification, action/attempt/receipt, and independent verification construction;
- portability tests proving LF/CRLF/pretty JSON produce identical typed value, JCS, and ActionDigest;
- compile-time production-source scan proving the trusted core does not inspect OS/environment/network/process/time-service APIs.

The fixture corpus stores most inner semantic `T` values for human readability; the exhaustive runner wraps them as `Versioned<T>`. Compatibility fixtures that test dispatch are complete envelopes. This distinction is documented in `contracts/ecra-domain-v1/README.md` and must remain consistent with the final canonical contract wording.

The bounded implementation clarifications remain normative for this PR and MUST converge into the primary contract/data model/tasks before ECR-001 closure.

## Next active phase — Phase 10 / T063–T069

Goal: complete cross-cutting security, dependency, documentation, provenance, and zero-I/O architecture evidence without widening ECR-001.

Required work:

```text
T063 dependency-boundary automation and prohibited-category proof
T064 zero-unsafe lint + explicit static/CI evidence
T065 complete structured ErrorCode/ErrorCategory matrix without display-string parsing
T066 audit/document/test every free-form field as non-authoritative
T067 update donor/license ledger for exact locked dependencies and no-source-copy provenance
T068 crate README architecture/type-to-requirement map + seven misuse warnings
T069 exact-head offline/no-service-access evidence
```

Existing branch evidence already includes `scripts/check-core-deps.sh`, the CI dependency gate, `#![forbid(unsafe_code)]`, and an offline replay gate. Phase 10 must verify these satisfy the canonical task wording, strengthen only proven gaps, and record exact evidence rather than duplicating controls.

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
- every normative valid fixture passes and every invalid fixture fails as specified;
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
