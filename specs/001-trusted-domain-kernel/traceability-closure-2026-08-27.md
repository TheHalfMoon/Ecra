# ECR-001 Implementation Traceability / Closure Evidence

**Date:** 2026-08-27  
**Slice:** ECR-001 — Trusted Domain Kernel  
**Status:** T079_TRACEABILITY_COMPLETE_PENDING_T080  
**Implementation evidence baseline:** `5dfe4c09b2abceeec14bc94b8e13d2dccddfd37c`  
**Baseline CI:** `33086490495` — success  
**Convergence head reviewed before this artifact:** `788a827ae7746f93630125c8a572205265bba7e5`

`5dfe4c09… -> 788a827…` is eight fast-forward commits with changes only in `EXECUTION.md` and ECR-001 spec/status/contract/data-model/quickstart/tasks/analyze documents. No production source, tests, Cargo graph, scripts, fixtures or CI workflow changed after the Phase 10 exact-head implementation gate.

This artifact satisfies T079 by mapping FR-001–FR-055, SC-001–SC-020, constitution G1–G15 and the pre-implementation review to exact implementation/test/contract ownership. It does not declare T080, PR readiness, merge, or `CLOSED_CANONICAL`.

## Evidence index

### Production modules

```text
crates/ecra-core/src/version.rs
crates/ecra-core/src/error.rs
crates/ecra-core/src/id.rs
crates/ecra-core/src/time.rs
crates/ecra-core/src/canonical.rs
crates/ecra-core/src/digest.rs
crates/ecra-core/src/actor.rs
crates/ecra-core/src/identity.rs
crates/ecra-core/src/origin.rs
crates/ecra-core/src/resource.rs
crates/ecra-core/src/scope.rs
crates/ecra-core/src/capability.rs
crates/ecra-core/src/information.rs
crates/ecra-core/src/evidence.rs
crates/ecra-core/src/artifact.rs
crates/ecra-core/src/action.rs
crates/ecra-core/src/receipt.rs
crates/ecra-core/src/verification.rs
```

### Primary test targets

```text
crates/ecra-core/tests/valid_fixtures.rs
crates/ecra-core/tests/invalid_fixtures.rs
crates/ecra-core/tests/contract_fixtures.rs
crates/ecra-core/tests/canonicalization.rs
crates/ecra-core/tests/action_digest.rs
crates/ecra-core/tests/properties.rs
crates/ecra-core/tests/identity_scope.rs
crates/ecra-core/tests/capability.rs
crates/ecra-core/tests/information_evidence.rs
crates/ecra-core/tests/information_use.rs
crates/ecra-core/tests/portability.rs
crates/ecra-core/tests/non_authoritative_metadata.rs
```

### Architecture/security gates

```text
.github/workflows/ecr-001.yml
scripts/check-core-unsafe.sh
scripts/check-core-deps.sh
crates/ecra-core/README.md
research/donor-license-ledger.md
contracts/ecra-domain-v1/{valid,invalid}/
```

### Converged normative documents

```text
specs/001-trusted-domain-kernel/spec.md
specs/001-trusted-domain-kernel/data-model.md
specs/001-trusted-domain-kernel/contracts/domain-v1.md
specs/001-trusted-domain-kernel/quickstart.md
specs/001-trusted-domain-kernel/tasks.md
```

## FR-001–FR-055 traceability

| FR | Owning tasks | Primary implementation / evidence | Result |
|---|---|---|---|
| FR-001 | T007, T077 | `version.rs`; strict `Versioned<T>` contract; `invalid_fixtures.rs` | PASS |
| FR-002 | T009 | `id.rs`; typed-ID compile/runtime fixtures/tests | PASS |
| FR-003 | T009, T023, T066 | `id.rs`; `non_authoritative_metadata.rs` | PASS |
| FR-004 | T007–T014 | deterministic constructors/Serde/JCS across core; fixture/property tests | PASS |
| FR-005 | T015 | `actor.rs`; actor fixtures | PASS |
| FR-006 | T016, T023, T027 | `identity.rs`; compile-fail/type-separation tests | PASS |
| FR-007 | T015, T066 | Actor kind + non-authoritative label; downstream conflicting-kind rule documented | PASS — store/run enforcement intentionally downstream |
| FR-008 | T017 | `origin.rs`; origin fixtures | PASS |
| FR-009 | T017, T023, T066 | origin/context types + authority-looking text tests | PASS |
| FR-010 | T018, T023, T066 | `resource.rs`; stable ResourceId vs locator tests/audit | PASS |
| FR-011 | T019–T020 | `scope.rs`; identity/scope and invalid fixtures | PASS |
| FR-012 | T019, T022 | explicit constraint variants; empty-one-of rejection | PASS |
| FR-013 | T020, T066 | typed Scope dimensions + PurposeRef; non-authority audit | PASS |
| FR-014 | T024, T027 | `capability.rs`; distinct request/grant types and IDs; compile-fail docs/tests | PASS |
| FR-015 | T024–T028 | capability principal/operation/target/scope/temporal shape | PASS |
| FR-016 | T024 | requested_by ActorId + optional IdentityAssertionRef separate from PrincipalRef | PASS |
| FR-017 | T024–T027 | DelegationRef parent/depth shape; no subset-validity claim | PASS — authorization downstream ECR-003 |
| FR-018 | T010, T024, T028 | `time.rs`, capability temporal evaluation with caller context | PASS |
| FR-019 | T024, T068 | `OperationRef`; crate docs prohibit provider/policy syntax | PASS |
| FR-020 | T030, T033 | distinct Observation/Fact types in `evidence.rs` | PASS |
| FR-021 | T033 | exact Provenance enum + fixtures/tests | PASS |
| FR-022 | T033, T037, T054, T057 | Fact has no verified field; VerificationReceipt is separate authority | PASS |
| FR-023 | T033, T035, T037 | DisputeState + conflict fixtures | PASS |
| FR-024 | T032, T035, T077 | FreshnessAssessment with paired basis kind/time | PASS |
| FR-025 | T034, T077 | bounded EvidenceRef typed links; no arbitrary evidence blob | PASS |
| FR-026 | T034, T077 | EvidenceRef ContentDigest + `as_of`; snapshot-capable representation | PASS — sufficiency policy ECR-004 |
| FR-027 | T029–T031, T035 | classification on Observation/Fact/ArtifactRef | PASS |
| FR-028 | T029, T035, T038 | five InformationClass values; unknown != public | PASS |
| FR-029 | T031, T033, T038, T077 | InformationRef + LineageRef stable-ID lineage | PASS — inheritance/declassification ECR-003 |
| FR-030 | T031, T036, T077 | `artifact.rs`: kind/media/digest/size/locator/classification/lineage | PASS |
| FR-031 | T012, T031, T036 | ContentDigest distinct from security digest | PASS |
| FR-032 | T012, T046, T050, T060 | SHA-256 domain-separated RFC8785 ActionDigest | PASS |
| FR-033 | T045, T077 | `ActionIntent` exact fields; bound parameters; flat effect/idempotency/retry wire | PASS |
| FR-034 | T039–T041 | `InformationUse` non-empty sources/use kind/destination | PASS |
| FR-035 | T039, T042 | InformationUse declaration-only compile/runtime evidence | PASS — authorization ECR-003 |
| FR-036 | T043–T044, T049 | MutationDomain + Reversibility independent axes | PASS |
| FR-037 | T043–T044, T049 | IdempotencyClass/Spec exact validation | PASS |
| FR-038 | T043–T049, T077 | explicit RetryClass and fail-closed compatibility matrix | PASS |
| FR-039 | T045–T051 | pre-execution ActionIntent + deterministic ActionRef exact-body binding | PASS |
| FR-040 | T052, T055, T057 | distinct ActionAttemptId + ActionAttemptRef | PASS — lifecycle ECR-002 |
| FR-041 | T052–T057 | receipt binds exact ActionRef + ActionAttemptId | PASS |
| FR-042 | T053, T055, T057 | executor_observed_success/failure/unknown only | PASS |
| FR-043 | T053, T057, T066 | bounded ErrorSummary/evidence; receipt != verification | PASS |
| FR-044 | T054–T057, T077 | exact VerificationTarget/verifier/method/evidence/outcome | PASS |
| FR-045 | T054–T057 | verified/rejected/inconclusive/not_evaluated | PASS |
| FR-046 | T034, T054–T057 | EvidenceRef immutable capture metadata usable by verification | PASS — mandatory sufficiency ECR-004 |
| FR-047 | T007, T058–T060, T077 | strict v1 Serde; unsupported version codes; malformed/unknown -> serialization_failed | PASS |
| FR-048 | T013, T026, T036, T041, T048, T056 | constructor/deserializer cross-field rejection + invalid corpus | PASS |
| FR-049 | T010, T028, T062, T069 | caller clock + portability/static scan + offline workspace test | PASS |
| FR-050 | T004, T063, T067 | `check-core-deps.sh`; direct allowlist/transitive prohibited scan; donor ledger | PASS |
| FR-051 | T011, T014, T046, T050, T060 | JCS fixed point + domain-separated digest golden bytes | PASS |
| FR-052 | T003, T064 | `#![forbid(unsafe_code)]` + `check-core-unsafe.sh` + CI | PASS |
| FR-053 | T008, T065, T077 | exact 16 ErrorCategory / 19 ErrorCode matrix; no display parsing | PASS |
| FR-054 | T023, T066, T077 | free-form authority audit + rustdoc + `non_authoritative_metadata.rs` | PASS |
| FR-055 | T027, T042, T057, T061 | no implicit scope widening/request->grant/Actor->Principal/receipt->verification conversions | PASS |

## SC-001–SC-020 traceability

| SC | Evidence | Result |
|---|---|---|
| SC-001 | `valid_fixtures.rs` exhaustive manifest + round-trip; 43 valid JSON fixtures at Phase 10 baseline | PASS |
| SC-002 | `invalid_fixtures.rs` exhaustive manifest/code-category assertions; 39 invalid JSON fixtures at baseline | PASS |
| SC-003 | `scripts/check-core-deps.sh` + CI Dependency boundary | PASS |
| SC-004 | `crates/ecra-core/README.md`, crate rustdoc, source module map | PASS |
| SC-005 | typed ActorId/PrincipalId + compile/runtime separation + non-authority tests | PASS |
| SC-006 | ScopeConstraint tests/fixtures: explicit `any_explicit`, empty one_of rejected | PASS |
| SC-007 | information/evidence/property tests keep provenance/classification/freshness/verification orthogonal | PASS |
| SC-008 | `information_use.rs` + contract fixtures prove no implicit A->B disclosure | PASS |
| SC-009 | `action_digest.rs` golden/mutation/wrong-ref tests | PASS |
| SC-010 | two-attempt fixtures + receipt exact binding tests | PASS |
| SC-011 | effect/idempotency/retry matrix + UNKNOWN conservative fixtures | PASS |
| SC-012 | receipt/verification type-confusion rejection + executor success != verified | PASS |
| SC-013 | CapabilityRequest/Grant distinct typed APIs/IDs + compile-fail coverage | PASS |
| SC-014 | version/unknown-field full-envelope fixtures and machine errors | PASS |
| SC-015 | `canonicalization.rs`, `action_digest.rs`, fixed canonical bytes/hex | PASS |
| SC-016 | offline workspace test + zero-unsafe script/lint | PASS |
| SC-017 | dependency gate + portability/static source scan; no provider/runtime public dependency | PASS |
| SC-018 | exact-head Phase 10 CI passed build/fmt/Clippy/tests/rustdoc/offline/unsafe/deps; revised full quickstart rerun reserved for T080 | PASS_BASELINE / T080_FINAL_REQUIRED |
| SC-019 | pre-code critical ECR-001 findings were replanned; this matrix records implementation resolution/downstream ownership | PASS_PENDING_T080_ANALYZE |
| SC-020 | T074 found drift and activated convergence; T077–T079 eliminate documented drift; final zero-blocker analyze is T080 | PENDING_T080 |

## Constitution v1.1.0 G1–G15 re-check

| Gate | Exact implementation disposition | Result |
|---|---|---|
| G1 Domain coherence | one production `ecra-core` trusted-domain representation; converged primary contract/data model | PASS |
| G2 Authority | Actor/Principal separation, explicit ScopeConstraint, request/grant distinction, no authorization engine | PASS |
| G3 Provenance | Observation/Fact/Provenance/Evidence/Freshness explicit; verification separate | PASS |
| G4 Side effects | mutation/reversibility/idempotency/retry + ActionAttempt identity + UNKNOWN semantics explicit | PASS |
| G5 Verification | VerificationReceipt is sole verification-result representation; executor outcomes are not VERIFIED | PASS |
| G6 Durability | no persistence/run lifecycle exists in ECR-001; versioned/digest-ready values only | PASS-N/A; ECR-002 owns enforcement |
| G7 Privacy/secrets | classification/use representation exists; no secret store/value/network/logging/telemetry runtime | PASS |
| G8 Local-first | full core tests work offline after dependency availability; no cloud account | PASS |
| G9 Interoperability | no MCP/ACP/A2A/browser/model protocol SDK in trusted model/deps | PASS |
| G10 Donor/license | locked runtime/dev dependency ledger + no-source-copy provenance; dependency gate reviewed | PASS |
| G11 Browser maintenance | no browser engine dependency or privileged patch in ECR-001 | PASS-N/A |
| G12 Benchmarks | ECR-001 claims deterministic contract/correctness only; no unsupported superlative | PASS |
| G13 Information flow / egress | InformationClassification + InformationUse source-to-sink declaration; enforcement explicitly ECR-003 | PASS |
| G14 Identity / principal binding | Actor, PrincipalRef, IdentityAssertionRef distinct; validity explicitly ECR-031 | PASS |
| G15 Bounded execution | ECR-001 executes no recursive/model/tool/process workload; zero-I/O value layer only | PASS-N/A; runtime budgets ECR-002 |

No constitution gate requires ECR-001 to counterfeit downstream runtime enforcement.

## Pre-implementation review disposition

The original platform review contained P-001–P-035. The table below names the ECR-001 remediation or preserves the downstream owner instead of claiming a type definition solved a runtime problem.

| Finding | Disposition at ECR-001 closure review |
|---|---|
| P-001 Information flow | **RESOLVED_FOR_ECR-001 REPRESENTATION:** InformationClassification/InformationRef/InformationUse; disclosure authorization remains ECR-003. |
| P-002 Identity | **RESOLVED_FOR_ECR-001 REPRESENTATION:** Actor != PrincipalRef/IdentityAssertionRef; authentication/trust roots remain ECR-031. |
| P-003 Scope semantics | **RESOLVED:** ScopeConstraint has not_applicable/exact/non-empty one_of/any_explicit and fail-closed tests. |
| P-004 Action binding | **RESOLVED:** ActionDigest + ActionRef exact canonical intent binding. |
| P-005 Attempt identity | **RESOLVED:** distinct ActionAttemptId/ActionAttemptRef; lifecycle remains ECR-002. |
| P-006 Verification truth | **RESOLVED:** no Fact.verified; VerificationReceipt is authoritative representation. |
| P-007 Side effects | **RESOLVED:** MutationDomain and Reversibility are separate; idempotency/retry are separately validated. |
| P-008 Typed identity | **RESOLVED:** security-relevant scope/actor/principal/resource/action/attempt/etc. IDs are strong newtypes. |
| P-009 Resource identity | **RESOLVED_FOR_ECR-001:** stable ResourceId + explicitly non-authoritative locator; provider alias resolution remains later adapters/policy. |
| P-010 Authorization TOCTOU | **DEFERRED AS DESIGNED:** immutable AuthorizationDecision/lease is ECR-003, which can bind ECR-001 ActionRef. |
| P-011 Outcome naming | **RESOLVED:** executor_observed_success / executor_observed_failure / unknown; VERIFIED reserved for VerificationReceipt. |
| P-012 Freshness | **RESOLVED:** FreshnessAssessment carries inspectable basis kind/time/evidence. |
| P-013 Ledger integrity | **DEFERRED:** ECR-002 ledger semantics + ECR-031 protected trust anchor; ECR-001 makes no tamper-proof claim. |
| P-014 Sensitive persistence | **DEFERRED:** ECR-002/ECR-031; ECR-001 has no persistence or secret access. |
| P-015 Browser isolation | **DEFERRED:** ECR-006/ECR-008/ECR-003; no browser implementation here. |
| P-016 Browser permissions | **DEFERRED:** browser permission broker in ECR-006/ECR-008/ECR-003. |
| P-017 Browser extensions | **DEFERRED:** ECR-007/ECR-008. |
| P-018 Browser IPC | **DEFERRED:** ECR-007 research/runtime boundary. |
| P-019 Resource budgets | **DEFERRED:** ECR-002 RunBudget/ResourceBudget; ECR-001 G15 is N/A because it executes nothing. |
| P-020 Search privacy | **DEFERRED WITH ECR-001 PREREQUISITE PROVIDED:** InformationUse/classification representation exists; ECR-009/ECR-003/ECR-025 enforce query egress. |
| P-021 Search trust | **DEFERRED WITH ECR-001 PREREQUISITE PROVIDED:** EvidenceRef digest/as_of and lineage shapes exist; ECR-009 owns ranking/independence/snapshot policy. |
| P-022 Memory lifecycle | **DEFERRED:** ECR-010/ECR-029. |
| P-023 Skill authority | **DEFERRED:** ECR-012/ECR-013; ECR-001 Capability/Scope/ActionRef types are prerequisites only. |
| P-024 Local model security | **DEFERRED:** ECR-021/ECR-024/ECR-017. |
| P-025 Developer execution | **DEFERRED:** ECR-019/ECR-018/ECR-017. |
| P-026 Protocol identity | **DEFERRED WITH ECR-001 PREREQUISITE PROVIDED:** Principal/IdentityAssertion/Capability types exist; ECR-016 pins protocol/auth mappings. |
| P-027 Local adversary boundary | **PLATFORM THREAT-MODEL OWNER:** no ECR-001 runtime/secret guarantee is claimed. |
| P-028 Actor kind collision | **RESOLVED AS DOMAIN INVARIANT:** ActorId has one ActorKind in a validated run/store context; downstream state enforces cross-record conflict. |
| P-029 Digest agility | **RESOLVED:** generic ContentDigest separated from SHA-256-only v1 SecurityDigest/ActionDigest. |
| P-030 Evidence integrity | **RESOLVED FOR REPRESENTATION:** EvidenceRef supports ContentDigest/as_of; ECR-004/ECR-009 decide when immutable capture is mandatory. |
| P-031 UI trust | **DEFERRED:** ECR-008 trusted chrome/control surface. |
| P-032 Background effects | **DEFERRED:** ECR-008 browser effect policy. |
| P-033 Parsing attack surface | **DEFERRED:** ECR-009/ECR-017/ECR-019/ECR-027 sandbox/resource limits. |
| P-034 Release provenance | **DEFERRED:** ECR-024 artifact-specific provenance/reproducibility. |
| P-035 Spec Kit quality | **RESOLVED:** constitution/spec/plan/tasks were reworked before code; implementation analyze later found canonical drift and correctly activated Phase 12 instead of hiding it. |

## Explicit downstream boundary

ECR-001 does **not** claim completion of:

```text
ECR-002 — durable run/attempt lifecycle, budgets/cancellation, persistence/integrity chain
ECR-031 — authentication assertion validity, trust roots, key lifecycle, protected sensitive storage
ECR-003 — authorization decision/lease, grant narrowing/revocation, disclosure/declassification, approvals/secrets
ECR-004 — verifier orchestration, evidence sufficiency/independence, reconciliation/UNKNOWN resolution
browser/search/memory/skill/protocol/runtime slices — their execution-specific controls
```

The ECR-001 contract is intentionally the typed prerequisite those slices must reuse.

## T079 decision

`T079 COMPLETE_ON_BRANCH`

Traceability finds no FR-001–FR-055 or constitution G1–G15 requirement that lacks an implementation owner, test/evidence owner, or explicit downstream enforcement owner. SC-020 remains intentionally pending until T080 reruns the analyze-equivalent review against the exact converged head.
