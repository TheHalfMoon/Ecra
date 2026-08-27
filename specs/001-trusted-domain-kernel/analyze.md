# ECR-001 Final Analyze — 2026-08-27

**Feature:** ECR-001 — Trusted Domain Kernel  
**Review type:** post-remediation Spec Kit analyze-equivalent  
**Base reviewed:** `48eb1796b0b8b3d9898c6537822eba021b8a04ba` on `main`  
**Constitution:** `.specify/memory/constitution.md` v1.1.0  
**Decision:** **PASS_FOR_TASKS_READY**

## Scope

This review re-checks the revised ECR-001 package after the pre-implementation architecture review. It evaluates consistency between the constitution, roadmap, `spec.md`, `research.md`, `data-model.md`, `contracts/domain-v1.md`, `plan.md`, `tasks.md`, `quickstart.md`, requirements checklist, platform threat model, gap audit, risk register and benchmark matrix.

This PASS authorizes implementation of ECR-001 only. It does not close ECR-001, authorize later roadmap slices, or claim any implemented security property.

## Analyze Summary

| Area | Result | Notes |
|---|---|---|
| Constitution coherence | PASS | G1–G15 are represented or truthfully N/A at the zero-I/O value-object layer. |
| Actor / Principal separation | PASS | Actor attribution and Principal/IdentityAssertion references are distinct typed concepts. Authentication validity remains ECR-031. |
| Scope semantics | PASS | `ScopeConstraint<T>` makes wildcard explicit; missing/empty cannot mean unrestricted. |
| Information flow representation | PASS | Information classification and source-to-sink `InformationUse` are represented without pretending to authorize disclosure. Enforcement remains ECR-003. |
| Resource identity | PASS | stable ResourceId is distinct from non-authoritative locator/origin metadata. |
| Capability request/grant separation | PASS | distinct types and distinct typed IDs; no implicit conversion is authorized. |
| Verification truth | PASS | Fact has no independent verified truth flag; VerificationReceipt is authoritative for verification state. |
| Side-effect semantics | PASS | mutation domain, reversibility, idempotency and retry are orthogonal and fail closed. |
| Action binding | PASS | ActionRef binds ActionId to deterministic domain-separated SHA-256 ActionDigest. |
| Attempt identity | PASS | ActionAttemptId is distinct from ActionId; lifecycle belongs to ECR-002. |
| Receipt / verification separation | PASS | executor outcomes do not use VERIFIED terminology; ActionReceipt and VerificationReceipt remain distinct. |
| Freshness/evidence | PASS | freshness basis and immutable capture/as-of metadata are representable without overstating trust. |
| Versioning/canonicalization | PASS | strict v1 parsing and RFC 8785 JCS plus domain separation are normative. |
| Dependency boundary | PASS_PLANNING | plan/tasks require one zero-I/O Rust crate and prohibit runtime/browser/network/model/policy/protocol dependencies. Implementation evidence still required. |
| Donor/license boundary | PASS_PLANNING | no donor source copying is authorized; dependency review remains a task before merge. |
| Review blocker remediation | PASS | all ECR-001-owned P-001..P-012 and P-028..P-030 concerns are either represented in ECR-001 or explicitly assigned downstream without counterfeit enforcement. |
| Downstream findings | PASS_ASSIGNED | browser/search/memory/runtime/local-model/protocol findings have explicit roadmap owners and do not block ECR-001 implementation. |

## Requirement / Task Coverage

### Functional requirements

- FR-001–FR-004: T007–T014 plus T058–T069.
- FR-005–FR-013: T015–T023.
- FR-014–FR-019: T024–T028.
- FR-020–FR-032: T029–T038 plus T012/T034.
- FR-033–FR-040: T039–T052.
- FR-041–FR-046: T053–T057.
- FR-047–FR-055: T058–T069 plus closure tasks.

No MUST-level FR is left without an implementation/test/review owner.

### Success criteria

- SC-001/002/014/015: normative valid/invalid/canonical/action-digest fixture runners and expectations.
- SC-003/016/017/018: dependency, offline, unsafe, fmt/Clippy/test/rustdoc gates.
- SC-004: crate architecture map/rustdoc traceability.
- SC-005/006/013: typed Actor/Principal/Scope/Capability separation tests.
- SC-007/008: provenance/classification/freshness/InformationUse tests.
- SC-009/010/011/012: ActionDigest/attempt/effect/receipt/verification tests.
- SC-019: this post-remediation analyze plus pre-implementation review ownership mapping.
- SC-020: T070/T074 exact-head traceability at closure.

No success criterion is implemented yet; this section proves planning ownership only.

## Consistency Findings

### Resolved

1. The pre-review ECR-001 contract incorrectly risked Actor-as-principal confusion. Revised types separate attribution from authentication references.
2. Implicit wildcard ambiguity is removed through explicit `ScopeConstraint<T>` variants.
3. Read authority and disclosure intent are now different concepts.
4. Action approval/receipt identity can bind exact canonical content through ActionDigest.
5. Retry/audit can distinguish repeated attempts.
6. Verification has one authoritative record family rather than a mutable Fact-level VERIFIED flag.
7. Effect domain and reversibility are no longer conflated.
8. Generic content digests are distinct from security binding digests.
9. Executor-reported success is no longer named as independent confirmation.
10. Freshness has an inspectable basis.

### Non-blocking downstream assignments

The following remain intentionally outside ECR-001 and are not planning defects:

- authentication assertion validation, trust roots, key lifecycle and sensitive storage: ECR-031;
- run/attempt lifecycle, budgets and persistence: ECR-002;
- authorization, declassification, disclosure decisions, approval leases and secrets: ECR-003;
- verification orchestration/reconciliation: ECR-004;
- browser permission broker/IPC/extension trust/trusted chrome: ECR-006–ECR-008;
- search egress/source independence/snapshots: ECR-009/ECR-027;
- memory deletion propagation: ECR-010/ECR-029;
- skill non-transferable authority: ECR-012/ECR-013;
- protocol identity/audience binding: ECR-016;
- untrusted code/parser/model sandboxing: ECR-017–ECR-021/ECR-027;
- release provenance/signing/reproducibility targets: ECR-024.

## Residual Risks Before ECR-001 Closure

These are implementation risks, not planning blockers:

- incorrect Serde strictness/unknown-field handling;
- canonicalization or SHA-256 domain-input mistakes;
- URL/origin normalization edge cases;
- accidentally permissive effect/retry validation;
- implicit conversions introduced for ergonomics;
- dependency feature creep;
- free-form strings accidentally used as authority;
- platform-dependent fixture behavior.

Tasks T013–T014, T022–T023, T026–T028, T036–T038, T041–T042, T048–T051, T055–T069 specifically exist to catch these classes.

## Status Transition

The revised package satisfies the planning lifecycle through analyze:

```text
SPEC_READY
  ↓
PLAN_READY
  ↓
TASKS_READY
```

Implementation may now begin on the bounded ECR-001 branch. The roadmap and planning index should be updated from `PLANNING_REWORK` to `TASKS_READY` before the first implementation commit, then to `IMPLEMENTING` once implementation changes exist.

## Final Decision

**PASS_FOR_TASKS_READY**

No critical or high planning inconsistency remains that requires redesign before implementing ECR-001. Any new MUST-level conflict discovered during implementation must stop the affected task, amend the planning package explicitly, and append convergence work rather than being silently worked around.
