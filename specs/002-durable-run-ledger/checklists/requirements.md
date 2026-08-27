# Requirements Checklist: ECR-002

**Feature:** Durable Run, Ledger & Budgets  
**Review point:** pre-implementation TASKS_READY gate

## Specification quality

- [x] Scope is bounded to local durable execution; provider execution/policy/verification/trust-root are explicit non-goals.
- [x] User stories cover recovery, attempt safety, budgets, append-only history, portable artifact and intervention/cancellation.
- [x] FR-001–FR-057 are testable and use stable terminology.
- [x] SC-001–SC-016 are reproducible and do not rely on marketing claims.
- [x] UNKNOWN external outcome remains first-class.
- [x] `execution_completed` and ActionReceipt are not verification.
- [x] Actor attribution is not authenticated Principal.

## Durability/state model

- [x] EventSequence is the ordering authority.
- [x] Run phases and terminal transitions are explicit.
- [x] Attempt preparation must commit before provider invocation.
- [x] Prepared-without-receipt recovery remains unresolved/UNKNOWN.
- [x] Append-only authoritative events are separate from rebuildable projections.
- [x] Event chain guarantee is scoped and does not overclaim hostile tamper resistance.
- [x] Concurrent append uses expected-head validation.
- [x] Migration/version behavior is explicit and fail-closed.

## Budgets

- [x] Exact v1 dimensions are frozen.
- [x] Amounts are I-JSON-safe integers with checked arithmetic.
- [x] Hard limits are explicit; optional soft limit cannot exceed hard.
- [x] Preflight and post-use accounting semantics are distinct.
- [x] Hard exhaustion durably blocks further governed work.
- [x] v1 has no ambient budget-increase API.

## Persistence / archive security

- [x] SQLite engine/configuration/transaction policy is decided.
- [x] WAL is recognized as part of live persistence state.
- [x] Live SQLite bytes are not the `.ecra` interchange format.
- [x] `.ecra` deterministic ZIP profile is frozen.
- [x] Path/method/feature/count/size limits are fixed before implementation.
- [x] Import validates manifest/content/event chain/reducer before trusted materialization.
- [x] No generic unsafe extract-to-directory API is planned.

## Privacy / trust boundaries

- [x] Real sensitive persistence is explicitly blocked pending ECR-031/ECR-003/ECR-025.
- [x] No network/telemetry/provider execution is part of ECR-002.
- [x] ECR-001 zero-I/O/unsafe/dependency boundary remains mandatory.
- [x] Native SQLite dependency is isolated to ECR-002 and has a dedicated review task.
- [x] Donor/source reuse is not implicitly authorized.

## Spec Kit governance

- [x] `spec.md` complete.
- [x] `research.md` resolves implementation-shaping decisions.
- [x] `data-model.md` complete.
- [x] normative `contracts/run-ledger-v1.md` complete.
- [x] slice `threat-model.md` complete.
- [x] `plan.md` passes G1–G15.
- [x] `tasks.md` contains ordered executable tasks and exact target paths.
- [x] `quickstart.md` defines exact verification surface.
- [x] `analyze.md` reports zero blocking planning drift and zero unowned FR/SC.

## Authorization result

```text
TASKS_READY
IMPLEMENTATION_MAY_BEGIN_ON_BOUNDED_FEATURE_BRANCH
REAL_SENSITIVE_PERSISTENCE=NOT_AUTHORIZED
DISTRIBUTED_EXECUTION=NOT_AUTHORIZED
```
