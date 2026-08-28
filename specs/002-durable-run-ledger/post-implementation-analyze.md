# Post-Implementation Analyze: ECR-002 Durable Run, Ledger & Budgets

**Date:** 2026-08-28  
**Lifecycle reviewed:** post-implementation / pre-merge convergence  
**Implementation baseline:** Phase 8 ledger head `e86e1822e621c0563f2764fe784902e3204b0085`  
**Baseline CI:** `33152251783` / job `98786745867` — SUCCESS  
**Traceability artifact:** `traceability-closure.md`  
**Convergence state:** T070 documentation corrections applied; final exact-head convergence gate pending

## Decision

```text
IMPLEMENTATION_REQUIREMENTS_COVERED=YES
UNOWNED_FR=0
UNOWNED_SC=0
FAILED_CONSTITUTION_GATES=0
IMPLICITLY_ACCEPTED_CRITICAL_RISKS=0
MUST_LEVEL_IMPLEMENTATION_DEFECTS_FOUND=0
CONVERGENCE_DRIFT_FOUND=4
CONVERGENCE_DRIFT_REMEDIATED=4
T069=COMPLETE_ON_BRANCH
T070=ACTIVE_PENDING_FINAL_EXACT_HEAD_GATE
PR_READY=NO
MERGE_AUTHORIZED=NO
```

The implementation satisfies the ECR-002 behavioral/security contract on the verified Phase 8 ledger state. Post-implementation analysis found four documentation/convergence defects and no production semantic defect. All four documentation defects have now been remediated in the branch; T070 still requires one complete exact-head ECR-002 gate on the final converged head before it can be marked complete.

## Inputs reviewed

```text
.specify/memory/constitution.md
specs/000-ecra-platform/risk-register.md
specs/002-durable-run-ledger/spec.md
specs/002-durable-run-ledger/research.md
specs/002-durable-run-ledger/data-model.md
specs/002-durable-run-ledger/contracts/run-ledger-v1.md
specs/002-durable-run-ledger/implementation-clarifications.md
specs/002-durable-run-ledger/threat-model.md
specs/002-durable-run-ledger/plan.md
specs/002-durable-run-ledger/tasks.md
specs/002-durable-run-ledger/quickstart.md
specs/002-durable-run-ledger/STATUS.md
specs/002-durable-run-ledger/analyze.md
specs/002-durable-run-ledger/traceability-closure.md
EXECUTION.md
research/donor-license-ledger.md
crates/ecra-run/src/*
crates/ecra-run/tests/*
.github/workflows/ecr-002.yml
scripts/check-core-*.sh
scripts/check-run-*.sh
```

## Convergence findings and disposition

### CVG-001 — implementation clarification C1 was not folded into primary normative documents

**Severity:** MUST_BEFORE_CLOSURE  
**Type:** numeric parser-contract drift  
**Disposition:** REMEDIATED

Implementation fixes these deterministic UTF-8 byte limits:

```text
SuspensionReason::other.code  1..=256 bytes
intervention_recorded.note    0..=4096 bytes when present
```

The exact bounds are now normative in `data-model.md` and `contracts/run-ledger-v1.md`. `implementation-clarifications.md` is retained only as historical convergence evidence and explicitly has no competing normative authority.

### CVG-002 — dependency planning text was stale relative to locked implementation truth

**Severity:** MUST_BEFORE_CLOSURE  
**Type:** dependency-version drift  
**Disposition:** REMEDIATED

Planning/research studied `rusqlite 0.40.1`; implementation exact-pinned and verified `rusqlite 0.40.2`, `libsqlite3-sys 0.38.2`, bundled SQLite `3.53.2`, and `zip 8.6.0` with default features disabled. `plan.md` now reflects the implemented dependency state. `research.md` preserves the historical 0.40.1 study and separately records the implementation resolution selecting 0.40.2.

### CVG-003 — operational execution guide was stale at Phase 5

**Severity:** MUST_BEFORE_CLOSURE  
**Type:** recovery/runbook drift  
**Disposition:** REMEDIATED

`EXECUTION.md` now records Phases 1–8 as verified, T067–T069 complete, T070 active, the latest green convergence baseline, exact dependency/toolchain evidence and the T070→T073 closure order.

### CVG-004 — verification/contract prose retained pre-implementation wording

**Severity:** MUST_BEFORE_CLOSURE  
**Type:** lifecycle/documentation drift  
**Disposition:** REMEDIATED

Convergence now records:
- `contracts/run-ledger-v1.md` as `NORMATIVE_IMPLEMENTED` and `crates/ecra-run` as implemented;
- `data-model.md` as `IMPLEMENTED_NORMATIVE_MODEL`;
- `plan.md` as `IMPLEMENTED_PENDING_CLOSURE` with exact dependency truth;
- `quickstart.md` as the implemented exact verification surface;
- `STATUS.md` and `EXECUTION.md` as Phase 9 pre-merge convergence truth.

The primary `spec.md` was re-read against implementation/traceability and no semantic amendment was required. The threat model was re-read against the implemented Phase 8 security acceptance suite and no missing security control or new Critical-risk acceptance was found.

## Requirements / constitution / risk result

`traceability-closure.md` demonstrates:

```text
FR-001–FR-057  PASS
SC-001–SC-014  PASS
SC-015         PASS_BASELINE / FINAL_EXACT_HEAD_AND_MAIN_REQUIRED
SC-016         PASS_TRACEABILITY / CONVERGENCE_FINAL_GATE_PENDING
G1–G15         PASS or explicit PASS-N/A
R-006          mitigated for ECR-002, ECR-004 retains independent reconciliation
R-019          version/migration mitigation implemented
R-033          local history-race/representation mitigation implemented; product control downstream
R-039          exact attempt identity implemented
R-042          bounded execution implemented
R-052          hostile-tamper overclaim blocked
R-053          real-sensitive persistence remains blocked
```

No convergence action widens ECR-002 into authentication, authorization, independent verification, provider execution, protected sensitive storage or distributed workflow infrastructure.

## T070 convergence checklist

```text
[x] Fold C1 numeric bounds into data-model.md
[x] Fold C1 numeric bounds into contracts/run-ledger-v1.md
[x] Mark implementation-clarifications.md folded into primary contract
[x] Converge plan.md dependency/lifecycle wording to exact implementation truth
[x] Add research.md implementation-resolution note for rusqlite 0.40.2
[x] Converge quickstart.md from future-tense to implemented verification surface
[x] Converge EXECUTION.md to Phase 9 live truth
[x] Re-check spec.md: no semantic drift requiring amendment
[x] Re-check threat-model.md: implementation acceptance section current
[x] Re-check STATUS.md/tasks.md against final convergence state
[ ] Run complete exact-head CI after convergence
```

## Analyze decision

```text
ZERO_BLOCKING_IMPLEMENTATION_SEMANTIC_DRIFT_FOUND
DOCUMENTATION_CONVERGENCE_REMEDIATED=YES
FINAL_CONVERGENCE_EXACT_HEAD_GATE=REQUIRED
T070_MUST_COMPLETE_BEFORE_T071=YES
```
