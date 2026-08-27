# Ecra Platform Status

**Purpose:** compact operational lifecycle view for the platform roadmap.  
**Architecture/dependency authority:** `roadmap.md`.  
**Current execution detail:** `../../EXECUTION.md`.

## Active slice

| ID | Slice | Lifecycle | Notes |
|---|---|---|---|
| ECR-001 | Trusted Domain Kernel | `IMPLEMENTING` | active branch/PR; Phases 1–4 verified on branch; Phase 5 next |

No later ECR slice is implementation-eligible until ECR-001 becomes `CLOSED_CANONICAL`, except bounded research explicitly authorized by its own spec/dependencies.

## Planned critical path

| ID | Slice | Lifecycle | Depends on |
|---|---|---|---|
| ECR-002 | Durable Run, Ledger & Budgets | `PLANNED` | ECR-001 |
| ECR-031 | Identity, Trust Root & Sensitive Storage | `PLANNED` | ECR-001, ECR-002 |
| ECR-004 | Verification & Reconciliation | `PLANNED` | ECR-001, ECR-002 |
| ECR-003 | Authority, Information Flow, Policy & Secrets | `PLANNED` | ECR-001, ECR-002, ECR-031 |
| ECR-005 | Evaluation & Threat Harness | `PLANNED` | ECR-001, ECR-002, ECR-003, ECR-004, ECR-031 |
| ECR-006 | Stock Firefox / WebDriver BiDi Prototype | `PLANNED` | trusted substrate/evaluation gates |
| ECR-007 | Browser Foundation & Upstream Strategy | `PLANNED` | ECR-006 |
| ECR-008 | Ecra Browser Wedge | `PLANNED` | policy, verification, browser foundation, identity/trust |
| ECR-009 | Search Evidence Fabric | `PLANNED` | trusted substrate/policy/verification |
| ECR-010 | Workspace & Memory | `PLANNED` | trusted substrate/search/identity |
| ECR-011 | Browser-Native Semantic Capabilities | `PLANNED` | policy/verification/browser/search |
| ECR-012 | Skill IR | `PLANNED` | trusted substrate/policy/verification |
| ECR-013 | Skill Compiler | `PLANNED` | evaluation/memory/semantic router/Skill IR |
| ECR-014 | Deterministic Replay | `PLANNED` | ECR-012, ECR-013 |
| ECR-015 | Divergence & Repair | `PLANNED` | ECR-014 |
| ECR-016 | Protocol Gateway | `PLANNED` | trusted substrate/search/memory/Skill IR/identity |
| ECR-017 | Plugin & Sandbox Runtime | `PLANNED` | policy/verification/evaluation/protocol gateway |
| ECR-018 | Terminal Execution | `PLANNED` | durable runtime/policy/verification/evaluation/sandbox/identity |
| ECR-019 | Developer Workspace | `PLANNED` | search/memory/protocol/terminal |
| ECR-020 | Data & Analytics | `PLANNED` | verification/search/memory/sandbox/identity |
| ECR-021 | Local Model Gateway | `PLANNED` | search through protocol/supply-chain foundations per roadmap |

## Deferred / cross-cutting program

Follow exact dependencies in `roadmap.md` for ECR-022 through ECR-030:

- ECR-022 Optional Sync & Multi-Device — `DEFERRED`.
- ECR-023 Extension Registry & Trust — `DEFERRED`.
- ECR-024 Release, Update & Supply Chain — `PLANNED`.
- ECR-025 Privacy, Telemetry & Diagnostics — `PLANNED`.
- ECR-026 Accessibility, Internationalization & Human UX Quality — `PLANNED`.
- ECR-027 Search/Content Compliance & Source Policy — `PLANNED`.
- ECR-028 Public Benchmark & Research Program — `PLANNED`.
- ECR-029 Migration, Import & Export — `DEFERRED`.
- ECR-030 Ecosystem Gateway — `DEFERRED`.

## Wave view

```text
A. Trusted substrate
   ECR-001 → ECR-002 → ECR-031/ECR-004 → ECR-003 → ECR-005

B. Browser wedge
   ECR-006 → ECR-007 → ECR-008

C. Trusted knowledge/context
   ECR-009 → ECR-010 → ECR-011

D. Skills
   ECR-012 → ECR-013 → ECR-014 → ECR-015

E. Ecosystem/work surfaces
   ECR-016/ECR-017 → ECR-018/ECR-019/ECR-020/ECR-021

F. Cross-cutting product maturity
   ECR-022–ECR-030 according to exact roadmap dependencies
```

## Update rule

When a slice lifecycle changes, update this file, `../../EXECUTION.md`, and the status field in `roadmap.md` in the same convergence/closure work. This file is intentionally compact; it never overrides dependency semantics in `roadmap.md`.
