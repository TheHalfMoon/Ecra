# Requirements Quality Checklist — ECR-001

**Purpose:** validate revised planning quality before first implementation. This is not an implementation test checklist.  
**Review basis:** constitution v1.1.0 + `pre-implementation-review-2026-08-27.md` + `analyze.md`.

## Scope and Independence

- [x] One bounded purpose: canonical zero-I/O trusted domain value/reference semantics.
- [x] Authentication, authorization, persistence, browser/model/tool execution, secret storage and protocol execution are explicitly downstream.
- [x] ECR-001 remains independently testable without any downstream process/service.
- [x] No requirement silently depends on cloud/browser/database/keychain/process execution.
- [x] Review remediation did not create a second production crate/service abstraction.

## Identity / Authority / Scope

- [x] Actor attribution is explicitly distinct from PrincipalRef/IdentityAssertionRef.
- [x] ActorId and PrincipalId are distinct typed IDs.
- [x] CapabilityRequest and CapabilityGrant are distinct types with distinct IDs.
- [x] Missing/empty scope cannot imply wildcard; `any_explicit` is explicit.
- [x] `not_applicable` is distinct from unrestricted.
- [x] strong IDs exist for security-relevant workspace/browser/container/tab/session/task/resource dimensions.
- [x] Resource locator/free-form text is explicitly non-authoritative.
- [x] no subset/authentication/policy decision is falsely claimed by structural types.

## Information Flow / Provenance

- [x] Observation and Fact are distinct.
- [x] provenance is distinct from verification outcome.
- [x] Fact has no independent canonical `verified` truth flag.
- [x] information classification is representable without becoming permission.
- [x] InformationUse/source-to-sink intent is representable separately from read/write capability.
- [x] remote-provider/model-context/persist/log/external-disclosure use classes are addressable.
- [x] derived information can retain lineage/classification for later conservative policy.
- [x] freshness has inspectable assessment/basis metadata.
- [x] evidence can carry immutable capture digest/as-of metadata where available.

## Actions / Attempts / Side Effects

- [x] ActionIntent is distinct from authorization/grant.
- [x] MutationDomain is distinct from Reversibility.
- [x] Idempotency and RetryClass are orthogonal and conservative.
- [x] ActionRef binds ActionId + deterministic ActionDigest.
- [x] ActionDigest domain/algorithm are versioned and explicit.
- [x] ActionAttemptId is distinct from ActionId.
- [x] receipts bind exact ActionRef + attempt.
- [x] UNKNOWN is explicit and never coerced.

## Verification

- [x] ActionReceipt is distinct from VerificationReceipt.
- [x] executor outcome names do not claim independent confirmation.
- [x] VerificationReceipt is the authoritative verification record.
- [x] verification targets can bind exact action/attempt/receipt/fact/artifact/claim references.
- [x] verification does not rewrite Fact provenance/classification/freshness.

## Functional Requirements / Success Criteria

- [x] FR-001 through FR-055 are individually testable/reviewable.
- [x] SC-001 through SC-020 are measurable on exact repository state.
- [x] each new critical review finding owned by ECR-001 has requirement/contract/task coverage.
- [x] downstream-only enforcement is explicitly assigned rather than counterfeited in ECR-001.
- [x] normative valid/invalid fixture classes are a first-class acceptance artifact.
- [x] exact ActionDigest fixtures are first-class contract evidence.
- [x] architecture/dependency purity is measurable.

## Constitution v1.1.0

- [x] G1–G15 are explicitly addressed in revised `plan.md`.
- [x] G13 information-flow/egress representation is covered without implementing policy.
- [x] G14 identity/principal binding is covered by distinct references and ECR-031 ownership.
- [x] G15 is truthfully N/A to ECR-001 because no recursive/runtime execution exists.
- [x] no ambient authority, raw secret handling, hidden telemetry, unsafe code or external protocol core dependency is authorized.
- [x] no donor source reuse is authorized merely by reference.

## Planning Artifacts

- [x] revised `spec.md` reflects review blockers.
- [x] revised `research.md` resolves ECR-001-owned blocking design choices.
- [x] revised `data-model.md` is consistent with spec ownership boundaries.
- [x] revised `contracts/domain-v1.md` is normative and strict.
- [x] revised `plan.md` maps constitution v1.1.0 and exact project structure.
- [x] revised `tasks.md` maps FR/SC groups to executable paths/tasks.
- [x] revised `quickstart.md` verifies the new identity/scope/egress/action-digest/attempt/verification invariants.
- [x] final `analyze.md` finds no critical planning defect.
- [x] no `[NEEDS CLARIFICATION]` marker is accepted as unresolved blocking work.

## Pre-Implementation Review Status

- [x] P-001 information-flow representation: remediated in ECR-001 planning; enforcement remains ECR-003.
- [x] P-002 Actor/Principal confusion: remediated; authentication remains ECR-031.
- [x] P-003 scope wildcard ambiguity: remediated.
- [x] P-004 action digest binding: remediated.
- [x] P-005 action-attempt identity: remediated.
- [x] P-006 duplicate verification truth: remediated.
- [x] P-007–P-012 relevant ECR-001 model weaknesses: remediated/assigned.
- [x] P-028–P-030 relevant domain-contract weaknesses: remediated/assigned.
- [x] all browser/search/memory/runtime/local-model findings retain explicit downstream owners.

## Result

**REQUIREMENTS_CHECKLIST_PASS — FINAL_ANALYZE_PASS — ECR-001 TASKS_READY.**

The revised planning package is implementation-eligible for ECR-001 only. This does not claim implementation success or unlock dependent slices. Exact-head implementation evidence and closure analysis remain mandatory before `CLOSED_CANONICAL`.
