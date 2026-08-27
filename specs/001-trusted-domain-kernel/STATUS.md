# ECR-001 Status — Trusted Domain Kernel

**Slice:** ECR-001  
**Lifecycle:** IMPLEMENTING_CONVERGENCE  
**Branch:** `001-trusted-domain-kernel`  
**PR:** `#1` (Draft; mergeable at last live check)  
**Latest fully verified implementation head:** `5dfe4c09b2abceeec14bc94b8e13d2dccddfd37c`  
**Phase 10 exact-head CI:** `33086490495` — `success`  
**Post-implementation analyze:** `CONVERGENCE_REQUIRED`

This is the active-slice execution ledger required by `AGENTS.md`. Normative semantics live in `spec.md`, `data-model.md`, `contracts/domain-v1.md`, and implementation truth. `VERIFIED_ON_BRANCH` is not `CLOSED_CANONICAL`.

## Current position

```text
Phase 1   VERIFIED_ON_BRANCH
Phase 2   VERIFIED_ON_BRANCH
Phase 3   VERIFIED_ON_BRANCH
Phase 4   VERIFIED_ON_BRANCH
Phase 5   VERIFIED_ON_BRANCH
Phase 6   VERIFIED_ON_BRANCH
Phase 7   VERIFIED_ON_BRANCH
Phase 8   VERIFIED_ON_BRANCH
Phase 9   VERIFIED_ON_BRANCH
Phase 10  VERIFIED_ON_BRANCH
Phase 11  PARTIAL — T074/T075 complete; T070–T073/T076 remain closure work
Phase 12  ACTIVE_CONVERGENCE — T077 complete on branch; T078 active
```

PR #1 remains Draft until T077–T080 convergence and the remaining Phase 11 closure requirements pass on exact repository evidence.

## Exact-head implementation evidence through Phase 10

### Phase 6 — T039–T042

Verified head: `b0f4ae4cb15d4ccb38cdd8cdbc0b764689fd29f5`  
CI: `33075545972` — `success`

Established declaration-only source-to-sink `InformationUse` semantics. Read authority over A plus write authority to B does not create disclosure authority A -> B.

### Phase 7 — T043–T051

Verified head: `ea17736310f661149026153a90a202e36396ba45`  
CI: `33078470973` — `success`

Established fail-closed effect/idempotency/retry semantics, exact SecurityDigest-bound action parameters, ActionIntent, domain-separated RFC 8785 ActionDigest, ActionRef validation and action fixtures/property coverage.

Golden ActionDigest:

```text
6ccf10a5d1db36cb9637b4ed2ee6d3f0167c44eec585b543f58d86287710e3d4
```

### Phase 8 — T052–T057

Verified head: `0b273f41f853f61e3dd691d4dcd5c2149c28f166`  
CI: `33080355344` — `success`

Established distinct action attempts, executor-only receipts, UNKNOWN preservation, bounded ErrorSummary/ClaimRef, independent VerificationReceipt outcomes and fail-closed verification evidence cardinality.

### Phase 9 — T058–T062

Verified head: `946e95366ed681c724192cd01ece199d5e8f55a7`  
CI: `33083362584` — `success`

Established strict tagged/public v1 parsing, exhaustive valid/invalid fixture manifests with drift detection, `Versioned<T>` runner coverage, canonical byte/digest fixtures, executable rustdoc and portability/static-source checks.

### Phase 10 — T063–T069

Latest fully verified implementation head: `5dfe4c09b2abceeec14bc94b8e13d2dccddfd37c`  
CI run: `33086490495` — `success`

All exact-head gates passed:

```text
cargo build --workspace --locked                                      PASS
cargo fmt --all --check                                               PASS
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings  PASS
cargo test --workspace --locked                                       PASS
cargo test --doc --workspace --locked                                 PASS
cargo test --workspace --locked --offline                             PASS
bash scripts/check-core-unsafe.sh                                     PASS
bash scripts/check-core-deps.sh                                       PASS
```

Phase 10 additionally established:
- fail-closed direct dependency allowlist + prohibited transitive dependency scan;
- crate lint plus static zero-unsafe boundary;
- exhaustive 16-category/19-code machine error matrix tests;
- repository-wide free-form non-authority audit, including `CapabilityRequest.reason`;
- locked dependency/license/no-source-copy donor ledger;
- `ecra-core` architecture/requirement map and seven misuse warnings;
- exact-head offline/no-service-access evidence.

## Phase 11 analyze result

T074 produced:

```text
specs/001-trusted-domain-kernel/post-implementation-analyze-2026-08-27.md
Decision: CONVERGENCE_REQUIRED
```

T075 activated Phase 12 because primary planning documents lagged the implemented/tested contract. The analyze found no authorization to weaken implementation; it required documentation/traceability convergence.

## Phase 12 — active convergence

### T077 — COMPLETE_ON_BRANCH

Primary `data-model.md` and `contracts/domain-v1.md` now fold implementation clarifications C1–C12 and the actual machine/wire behavior, including:
- unsupported-version vs malformed-envelope machine errors;
- exact 16 ErrorCategory / 19 ErrorCode API;
- ObservationPayloadRef, FactValue, LineageRef and freshness pairing;
- artifact canonical byte-size rules;
- free-form metadata authority boundary;
- exact ActionParametersRef security binding;
- full effect/idempotency/retry matrix;
- ClaimRef/ErrorSummary/receipt/verification evidence semantics;
- fixture inner-body vs public `Versioned<T>` wire convention.

These docs-only convergence commits still require final exact-head verification in T080.

### T078 — ACTIVE

Required updates:
- revised `quickstart.md` matching the actual CI/security/fixture targets;
- `tasks.md` phase/task completion truth;
- this active `STATUS.md`;
- root `EXECUTION.md`.

Note: there is no root `STATUS.md`. `AGENTS.md` requires the active slice status at `specs/001-trusted-domain-kernel/STATUS.md`; convergence follows that live repository structure.

### T079 — NEXT

Produce one exact traceability artifact covering:
- FR-001–FR-055;
- SC-001–SC-020;
- constitution G1–G15;
- every ECR-001-owned pre-implementation-review finding;
- explicit downstream deferrals to ECR-002/ECR-003/ECR-004/ECR-031.

### T080 — BLOCKED_BY_T078_T079

Run revised quickstart + full exact-head CI + analyze-equivalent review. PR readiness is allowed only if zero blocking drift remains.

## Current verification gate

For material convergence heads:

```bash
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --workspace --locked
cargo test --workspace --locked --offline
bash scripts/check-core-unsafe.sh
bash scripts/check-core-deps.sh
```

Dedicated contract/security targets are listed in the converged `quickstart.md` and must be recorded by T073/T080.

## ECR-001 closure checklist

ECR-001 may be called `CLOSED_CANONICAL` only when all are true:
- every task is satisfied by code/docs/evidence;
- T077–T080 convergence is complete;
- valid/invalid fixture manifests and contract behavior pass;
- strict versioning/canonicalization/digest contracts agree with implementation;
- no prohibited dependency category or unsafe code exists;
- revised quickstart passes on exact final feature head;
- FR/SC/G1–G15/pre-review traceability has no blocking gap;
- donor/license ledger remains current;
- final analyze reports zero blocking drift;
- PR is no longer Draft only when repository readiness criteria are met;
- PR merges without force-push/rebase/destructive history rewriting;
- required post-merge verification on canonical `main` is recorded;
- platform roadmap/status and `EXECUTION.md` advance truthfully.

Until then, ECR-002 and all dependent implementation slices remain blocked by roadmap ordering.