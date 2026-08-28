# Ecra Platform Status

**Purpose:** compact operational lifecycle view for the platform roadmap.  
**Architecture/dependency authority:** `roadmap.md`.  
**Current execution detail:** `../../EXECUTION.md`.

## Canonically closed slices

| ID | Slice | Lifecycle | Notes |
|---|---|---|---|
| ECR-001 | Trusted Domain Kernel | `CLOSED_CANONICAL` | closure-ledger head `85e4bf65…`; CI `33099434232` passed |
| ECR-002 | Durable Run, Ledger & Budgets | `CLOSED_CANONICAL` pending final closure-head recheck | feature head `87fd9fc5…` CI `33153413462` passed; PR #2 merged as `40efc8a6…`; post-merge ECR-002 CI `33154108410` passed |

ECR-002 closure documentation is being converged on canonical `main`; the last T073 documentation head must itself pass the permanent ECR-002 workflow before the closure is treated as fully sealed for downstream implementation authorization.

## Next planning selection

| ID | Slice | Lifecycle | Depends on | Eligibility / intent |
|---|---|---|---|---|
| ECR-031 | Identity, Trust Root & Sensitive Storage | `PLANNED_NEXT` | ECR-001, ECR-002 | selected next critical-path planning slice after final ECR-002 closure-head CI |
| ECR-004 | Verification & Reconciliation | `PLANNED_ELIGIBLE` | ECR-001, ECR-002 | independently planning-eligible in parallel after ECR-002 closure |
| ECR-003 | Authority, Information Flow, Policy & Secrets | `PLANNED_BLOCKED` | ECR-001, ECR-002, ECR-031 | remains blocked until ECR-031 is `CLOSED_CANONICAL` |

No ECR-031 or ECR-004 implementation is authorized merely by this table. Each slice must independently complete its Spec Kit planning/analyze gates before an implementation branch/PR exists.

## Planned critical path

| ID | Slice | Lifecycle | Depends on |
|---|---|---|---|
| ECR-031 | Identity, Trust Root & Sensitive Storage | `PLANNED_NEXT` | ECR-001, ECR-002 |
| ECR-004 | Verification & Reconciliation | `PLANNED_ELIGIBLE` | ECR-001, ECR-002 |
| ECR-003 | Authority, Information Flow, Policy & Secrets | `PLANNED_BLOCKED` | ECR-001, ECR-002, ECR-031 |
| ECR-005 | Evaluation & Threat Harness | `PLANNED` | ECR-001, ECR-002, ECR-003, ECR-004, ECR-031 |
| ECR-006 | Stock Firefox / WebDriver BiDi Prototype | `PLANNED` | ECR-001–ECR-005, ECR-031 |
| ECR-007 | Browser Foundation & Upstream Strategy | `PLANNED` | ECR-006 |
| ECR-008 | Ecra Browser Wedge | `PLANNED` | ECR-003, ECR-004, ECR-006, ECR-007, ECR-031 |
| ECR-009 | Search Evidence Fabric | `PLANNED` | ECR-001, ECR-002, ECR-003, ECR-004 |
| ECR-010 | Workspace & Memory | `PLANNED` | ECR-001–ECR-004, ECR-009, ECR-031 |
| ECR-011 | Browser-Native Semantic Capabilities | `PLANNED` | ECR-003, ECR-004, ECR-006, ECR-009 |
| ECR-012 | Skill IR | `PLANNED` | ECR-001–ECR-004 |
| ECR-013 | Skill Compiler | `PLANNED` | ECR-005, ECR-010, ECR-011, ECR-012 |
| ECR-014 | Deterministic Replay | `PLANNED` | ECR-012, ECR-013 |
| ECR-015 | Divergence & Repair | `PLANNED` | ECR-014 |
| ECR-016 | Protocol Gateway | `PLANNED` | ECR-001–ECR-004, ECR-009, ECR-010, ECR-012, ECR-031 |
| ECR-017 | Plugin & Sandbox Runtime | `PLANNED` | ECR-003, ECR-004, ECR-005, ECR-016 |
| ECR-018 | Terminal Execution | `PLANNED` | ECR-002–ECR-005, ECR-017, ECR-031 |
| ECR-019 | Developer Workspace | `PLANNED` | ECR-009, ECR-010, ECR-016, ECR-018 |
| ECR-020 | Data & Analytics | `PLANNED` | ECR-004, ECR-009, ECR-010, ECR-017, ECR-031 |
| ECR-021 | Local Model Gateway | `PLANNED` | ECR-009–ECR-017, ECR-024 |

## Deferred / cross-cutting program

Follow exact dependencies in `roadmap.md` for ECR-022 through ECR-030. Deferred items remain deferred unless governance explicitly changes them.

## Wave view

```text
A. Trusted substrate
   ECR-001 [CLOSED] → ECR-002 [CLOSED] → {ECR-031 [NEXT], ECR-004 [ELIGIBLE]} → ECR-003 → ECR-005

B. Browser wedge
   ECR-006 → ECR-007 → ECR-008

C. Trusted knowledge/context
   ECR-009 → ECR-010 → ECR-011

D. Skills
   ECR-012 → ECR-013 → ECR-014 → ECR-015

E. Ecosystem/work surfaces
   ECR-016/ECR-017 → ECR-018/ECR-019/ECR-020/ECR-021
```

## Sensitive-state boundary

ECR-002 proved synthetic/non-sensitive local durability. It did **not** authorize persistence of real authenticated browser secrets, credentials, private workspace payloads or equivalent high-value state. That progression remains blocked on ECR-031 plus the later policy/privacy owners named in the roadmap.

## Update rule

When a slice lifecycle changes, update this file, `../../EXECUTION.md`, and the status field in `roadmap.md` in the same convergence/closure work. This file never overrides dependency semantics in `roadmap.md`.
