# Ecra Spec Kit Index

This directory contains Ecra's canonical Spec-Driven Development packages.

## Start here

For current execution state, read:

1. `../EXECUTION.md`
2. `000-ecra-platform/roadmap.md`
3. `000-ecra-platform/STATUS.md`
4. the active slice `STATUS.md`, `tasks.md`, and analyze/convergence artifacts
5. exact live GitHub branch/head, PR, CI, review and changed-file truth

Do not infer implementation eligibility from directory names alone. The roadmap dependency graph, active package, and exact live evidence decide what may be planned or implemented.

## Platform package

`000-ecra-platform/` is the spec-of-specs and owns immutable ECR IDs/dependencies plus cross-platform architecture, threat, risk, gap, benchmark and decision records.

## Canonically closed slices

### ECR-001 — Trusted Domain Kernel

Directory: `001-trusted-domain-kernel/`  
Lifecycle: `CLOSED_CANONICAL`.  
PR #1 merged; closure-ledger head `85e4bf657b6c33e3f88d83e92e7a35279d177349` passed CI `33099434232`.

### ECR-002 — Durable Run, Ledger & Budgets

Directory: `002-durable-run-ledger/`  
Lifecycle: `CLOSED_CANONICAL`.  
Final closure-convergence main head `aadc19c972e619222d426674d7542dd9c00dbe44` passed ECR-002 CI `33155302100` and ECR-001 regression CI `33155302026`.

ECR-002 owns synthetic/non-sensitive local run durability, budgets, recovery and deterministic `.ecra` interchange. It does not authorize real sensitive persistence, authentication/trust roots, authorization, independent verification, or provider execution.

## Active trusted-substrate slices

### ECR-031 — Identity, Trust Root & Sensitive Storage Foundations

Directory: `031-identity-trust-root/`  
Implementation PR: #4.  
Dependencies: ECR-001 + ECR-002 `CLOSED_CANONICAL`.

The implementation has progressed beyond historical planning markers, but current native macOS Data Protection Keychain acceptance remains externally blocked by the trusted runner user's missing Apple Development signing identity, suitable provisioning profile and usable developer account/team. No legacy/plaintext/ad-hoc fallback is authorized. Exact live ECR-031 package/PR/Actions truth governs its frontier.

### ECR-004 — Verification & Reconciliation

Directory: `004-verification-receipts/`  
Lifecycle: `IMPLEMENTING_FINAL_CONVERGENCE`.  
Implementation branch: `004-verification-receipts-impl`.  
Implementation PR: #6 (Draft until T050).  
Canonical implementation base: `4fb61f8b41267983fc460c666fddd7781d91653c`.  
Dependencies: ECR-001 + ECR-002 `CLOSED_CANONICAL`.

Package:

```text
STATUS.md
spec.md
research.md
data-model.md
contracts/verification-reconciliation-v1.md
threat-model.md
plan.md
tasks.md
quickstart.md
implementation-clarifications.md
analyze.md
post-implementation-analyze.md
traceability-closure.md
checklists/requirements.md
```

Verified implementation state includes Phases 1–6, hostile-input/resource ceilings, portability, complete quickstart, donor/license/dependency reconciliation and Phase 7 closure. The exact T045 Phase 7 gate succeeded on:

```text
HEAD   90ed1bbeafea72ee655bc58a96e94696096f360e
RUN    33251037913
JOB    99096645538
RESULT SUCCESS
```

T046/T047 traceability maps FR-001–FR-046, SC-001–SC-013 and G1–G15 with zero unowned MUST requirement and zero constitutional blocker. T048 post-implementation analyze found bounded documentation-only convergence work; T049 owns and corrects it before T050 final exact-head gate.

Frozen v1 boundaries:
- ECR-001 `VerificationReceipt` remains the only canonical independent verification record;
- `ActionReceipt` remains executor-observed evidence and cannot self-verify;
- Fact/artifact/run metadata gain no competing verified truth flag;
- deterministic aggregates preserve conflict;
- checkpoints are exact-target requirements, not authority;
- UNKNOWN reconciliation never fabricates an `ActionReceipt` or mutates ECR-002 run-event truth;
- `effect_confirmed`, `no_effect_confirmed`, and `still_unknown` are independent effect-evidence outcomes only;
- every reconciliation outcome leaves the original ECR-002 prepared/unreceipted/unresolved state and `unresolved_attempts` unchanged;
- `semantically_retryable*` is advisory for a future new-attempt proposal only, not same-run resume/schedule/execution authorization;
- ECR-002 `RunEvent` v1 remains unchanged and no sidecar projection represents run resolution;
- a separate append-only ECR-004 journal stores synthetic/non-sensitive evidence metadata/references/digests only;
- journal chaining is local integrity/corruption/substitution detection, not hostile complete-store tamper resistance;
- no browser/network/model/provider/process/policy/authorization/identity/telemetry runtime dependency enters `ecra-verify`;
- IC-001 permits only read-only access to already-existing canonical EvidenceRef metadata;
- IC-002 prohibits ECR-004 from clearing ECR-002 unresolved state or counterfeiting run repair;
- IC-003 permits empty reconciliation support only for evidence-absent `still_unknown`, never for conclusive effect/no-effect outcomes.

Remaining lifecycle order:

```text
T049 package/platform/index/EXECUTION convergence
  ↓
T050 final exact-head ECR-004 + ECR-001 + ECR-002 gate
  ↓
T051 PR #6 review-ready + zero actionable review blockers
  ↓
T052 exact expected-head non-rebase merge + canonical-main workflows
  ↓
T053 post-merge lifecycle closure
```

ECR-004 is not `CLOSED_CANONICAL` before T052/T053 evidence.

## Dependency boundary

ECR-004 is independently eligible from ECR-001/ECR-002 and may finish while ECR-031 remains externally blocked. That independence does not authorize ECR-031 identity/trust/sensitive-storage scope or real sensitive evidence persistence.

ECR-003 remains blocked until ECR-031 is `CLOSED_CANONICAL`. ECR-005 remains blocked by its complete dependency set, including ECR-003, ECR-004 and ECR-031.

## Future slices

The full program reserves ECR-001 through ECR-031 in the platform roadmap. A roadmap row does not imply implementation authorization or package completion.

## Package lifecycle

```text
SPEC_READY
  ↓
PLAN_READY
  ↓
TASKS_READY
  ↓
IMPLEMENTING
  ↓
exact-head verification
  ↓
convergence / analyze
  ↓
merge + required post-merge verification
  ↓
CLOSED_CANONICAL
```

A successful individual task, commit, CI run, review or merge does not by itself close a slice.

## Required package contents

A normal stateful implementation slice should contain, as applicable:
- `STATUS.md` — execution ledger and next eligible work;
- `spec.md` — requirements and success criteria;
- `research.md` — primary-source/donor/dependency decisions;
- `data-model.md` — domain/wire/state model;
- `contracts/` — normative interfaces/wire contracts;
- threat model;
- `implementation-clarifications.md` when implementation discovers a real underspecification;
- `plan.md` — architecture and constitutional gates;
- `tasks.md` — ordered executable tasks;
- `quickstart.md` — verification workflow;
- analyze/traceability/convergence artifacts;
- checklists.

## Status vocabulary

- `PLANNED` — roadmap decomposition only.
- `PLANNING_REWORK` — blocking planning defect; implementation forbidden.
- `SPEC_READY` — requirements complete enough for planning.
- `PLAN_READY` — research/data model/contracts/plan complete.
- `TASKS_READY` — executable tasks and latest planning analyze clean.
- `IMPLEMENTING` — bounded branch/PR active, including final convergence/review work.
- `VERIFIED_ON_BRANCH` — phase/task group has exact-head evidence on the active branch.
- `BLOCKED` — dependency/evidence gate prevents continuation.
- `CLOSED_CANONICAL` — implementation, tests, docs, convergence, merge and required post-merge evidence complete.
- `DEFERRED` — intentionally outside current critical path.

`VERIFIED_ON_BRANCH` is deliberately narrower than `CLOSED_CANONICAL`.