# Post-Implementation Analyze: ECR-002 Durable Run, Ledger & Budgets

**Date:** 2026-08-28  
**Lifecycle reviewed:** post-implementation / pre-merge convergence  
**Implementation baseline:** Phase 8 ledger head `e86e1822e621c0563f2764fe784902e3204b0085`  
**Baseline CI:** `33152251783` / job `98786745867` — SUCCESS  
**Traceability artifact:** `traceability-closure.md`

## Decision

```text
IMPLEMENTATION_REQUIREMENTS_COVERED=YES
UNOWNED_FR=0
UNOWNED_SC=0
FAILED_CONSTITUTION_GATES=0
IMPLICITLY_ACCEPTED_CRITICAL_RISKS=0
MUST_LEVEL_IMPLEMENTATION_DEFECTS_FOUND=0
CONVERGENCE_DRIFT_FOUND=4
T069=COMPLETE_ON_BRANCH
T070=ACTIVE_REQUIRED
PR_READY=NO
MERGE_AUTHORIZED=NO
```

The implementation satisfies the ECR-002 behavioral/security contract on the verified Phase 8 ledger state. Four documentation/convergence defects remain and must be corrected before final exact-head readiness. None requires production semantic redesign.

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

## Convergence findings

### CVG-001 — implementation clarification C1 is not folded into primary normative documents

**Severity:** MUST_BEFORE_CLOSURE  
**Type:** numeric parser-contract drift

Implementation fixes these deterministic UTF-8 byte limits:

```text
SuspensionReason::other.code  1..=256 bytes
intervention_recorded.note    0..=4096 bytes when present
```

`implementation-clarifications.md` correctly records them, but `data-model.md` still says only “bounded non-empty string” / “Option<bounded string>” and the primary `run-ledger-v1.md` does not freeze the numeric limits.

**Convergence action:** fold exact bounds into `data-model.md` and `contracts/run-ledger-v1.md`, then mark `implementation-clarifications.md` folded/non-competing.

### CVG-002 — dependency planning text is stale relative to locked implementation truth

**Severity:** MUST_BEFORE_CLOSURE  
**Type:** dependency-version drift

Planning/research studied `rusqlite 0.40.1`; implementation exact-pinned and verified `rusqlite 0.40.2`, `libsqlite3-sys 0.38.2`, bundled SQLite `3.53.2`, and `zip 8.6.0` with default features disabled. The donor ledger and CI evidence already reflect implementation truth.

**Convergence action:** update `plan.md` to the implemented dependency state. Preserve the historical research observation that 0.40.1 was studied, but append implementation resolution selecting 0.40.2 rather than rewriting research history.

### CVG-003 — operational execution guide is stale at Phase 5

**Severity:** MUST_BEFORE_CLOSURE  
**Type:** recovery/runbook drift

`EXECUTION.md` still says the current phase is Phase 5 and lists T035–T044 as active work despite Phases 5–8 being verified and Phase 9 active.

**Convergence action:** replace active execution truth/task order with Phase 8 verified evidence and T067–T073 closure order; retain exact earlier phase evidence where useful.

### CVG-004 — verification/contract prose retains pre-implementation wording

**Severity:** MUST_BEFORE_CLOSURE  
**Type:** lifecycle/documentation drift

Examples:
- `contracts/run-ledger-v1.md` status is `NORMATIVE_PLANNING` and calls `ecra-run` a planned crate;
- `data-model.md` status is `PLAN_MODEL`;
- `quickstart.md` says “When the crate exists”; the crate and all dedicated targets now exist;
- `plan.md` still labels repository structure/dependencies as planned candidates where implementation truth is fixed.

**Convergence action:** update lifecycle wording without changing normative semantics; quickstart remains the executable exact verification surface.

## Requirements / constitution / risk result

`traceability-closure.md` demonstrates:

```text
FR-001–FR-057  PASS
SC-001–SC-014  PASS
SC-015         PASS_BASELINE / FINAL_EXACT_HEAD_AND_MAIN_REQUIRED
SC-016         PASS_TRACEABILITY / CONVERGENCE_PENDING
G1–G15         PASS or explicit PASS-N/A
R-006          mitigated for ECR-002, ECR-004 retains independent reconciliation
R-019          version/migration mitigation implemented
R-033          local history-race/representation mitigation implemented; product control downstream
R-039          exact attempt identity implemented
R-042          bounded execution implemented
R-052          hostile-tamper overclaim blocked
R-053          real-sensitive persistence remains blocked
```

No convergence action is allowed to widen ECR-002 into authentication, authorization, independent verification, provider execution, protected sensitive storage or distributed workflow infrastructure.

## T070 convergence checklist

```text
[ ] Fold C1 numeric bounds into data-model.md
[ ] Fold C1 numeric bounds into contracts/run-ledger-v1.md
[ ] Mark implementation-clarifications.md folded into primary contract
[ ] Converge plan.md dependency/lifecycle wording to exact implementation truth
[ ] Add research.md implementation-resolution note for rusqlite 0.40.2
[ ] Converge quickstart.md from future-tense to implemented verification surface
[ ] Converge EXECUTION.md to Phase 9 live truth
[ ] Re-check spec.md: no semantic drift requiring amendment
[ ] Re-check threat-model.md: implementation acceptance section current
[ ] Re-check STATUS.md/tasks.md against final convergence state
[ ] Run complete exact-head CI after convergence
```

## Analyze decision

```text
ZERO_BLOCKING_IMPLEMENTATION_SEMANTIC_DRIFT_FOUND
DOCUMENTATION_CONVERGENCE_REQUIRED=YES
T070_MUST_COMPLETE_BEFORE_T071=YES
```
