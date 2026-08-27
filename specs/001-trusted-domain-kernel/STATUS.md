# ECR-001 Status — Trusted Domain Kernel

**Slice:** ECR-001  
**Lifecycle:** IMPLEMENTING  
**Branch:** `001-trusted-domain-kernel`  
**PR:** `#1` (draft until full slice closure)  
**Latest fully verified implementation head:** `0b273f41f853f61e3dd691d4dcd5c2149c28f166`

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
Phase 9  NEXT_ACTIVE_PHASE
Phase 10+ BLOCKED_BY_ORDERING
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

The bounded C11 clarification remains normative for this PR and MUST converge into the primary contract/data model/tasks before ECR-001 closure.

## Next active phase — Phase 9 / T058–T062

Goal: make the entire v1 contract strict, deterministic, portable, and fixture-complete as one API rather than a collection of individually tested types.

Required work:

```text
T058 strict explicit Serde names / unknown-field behavior across public v1 types
T059 valid/invalid fixture runners covering every committed normative fixture
T060 canonical byte + ActionDigest expected outputs for normative fixtures
T061 rustdoc examples for safe construction and type separation
T062 portability / zero-environment-behavior tests
```

Phase 9 must preserve the zero-I/O production boundary. Any fixture discovery mechanism must not introduce runtime service, network, clock, environment, or external-process dependencies into `ecra-core`.

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
