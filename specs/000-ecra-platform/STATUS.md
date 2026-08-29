# Ecra Platform Status

**Purpose:** compact operational lifecycle view for the platform roadmap.  
**Architecture/dependency authority:** `roadmap.md`.  
**Current execution detail:** `../../EXECUTION.md`.

## Canonically closed slices

| ID | Slice | Lifecycle | Notes |
|---|---|---|---|
| ECR-001 | Trusted Domain Kernel | `CLOSED_CANONICAL` | closure-ledger head `85e4bf65…`; CI `33099434232` passed |
| ECR-002 | Durable Run, Ledger & Budgets | `CLOSED_CANONICAL` | final closure-convergence head `aadc19c9…`; ECR-002 CI `33155302100` and ECR-001 regression `33155302026` passed |

ECR-002 is fully sealed as a dependency. Its v1 durability authorization remains synthetic/non-sensitive and does not replace ECR-031/ECR-003/ECR-025 protection/policy/privacy ownership.

## Active / eligible trusted-substrate work

| ID | Slice | Lifecycle | Depends on | Eligibility / intent |
|---|---|---|---|---|
| ECR-031 | Identity, Trust Root & Sensitive Storage | `TASKS_READY_PENDING_EXACT_GREEN_HEAD` | ECR-001, ECR-002 | canonical planning is ready; live implementation state is tracked in `EXECUTION.md`/PR #4 |
| ECR-004 | Verification & Reconciliation | `TASKS_READY` | ECR-001, ECR-002 | planning package Analyze Pass 3 clean; implementation still requires this planning state to become canonical plus exact canonical dependency regressions |
| ECR-003 | Authority, Information Flow, Policy & Secrets | `PLANNED_BLOCKED` | ECR-001, ECR-002, ECR-031 | implementation remains blocked until ECR-031 is `CLOSED_CANONICAL` |

ECR-031 planning evidence:

```text
Analyze Pass 1: 44e85aa9ccd28e185a5761889aa12b50459f286e — PLANNING_REWORK_REQUIRED
Analyze Pass 2: a3c7d563c139c65886f169f9181c07a997038f1f — ZERO_BLOCKING_PLANNING_DRIFT_FOUND
FR-001–FR-058: OWNED
SC-001–SC-016: OWNED
G1–G15: PASS / explicit PASS-N/A
Pass-1 blockers remediated: 4/4
```

ECR-004 planning evidence:

```text
Planning branch: 004-verification-receipts
Planning PR: #5
Analyze Pass 1 blocker A-001: REMEDIATED by IC-001 / T011A
Analyze Pass 2 review blocker A-002: REMEDIATED by IC-002 / FR-046 / SC-013 / Phase 5 compatibility tasks
Analyze Pass 3: ZERO_BLOCKING_PLANNING_DRIFT_FOUND
FR-001–FR-046: OWNED
SC-001–SC-013: OWNED
G1–G15: PASS / explicit PASS-N/A
```

Implementation of either slice is governed by its own exact execution package and must start/continue only from an eligible exact head. ECR-004 does not bypass ECR-031 and ECR-031 is not a dependency of ECR-004.

## ECR-031 frozen planning boundaries

- local bootstrap creates an opaque Ecra-local principal; no external/legal/NIST identity-proofing claim;
- `ProtectedTrustStateV1` is authoritative for enrollment/key lifecycle/revocation; ordinary metadata cannot activate/unrevoke a key;
- assertion issuance requires `EnrolledPrincipalHandle` + `VerifiedTrustSnapshot` -> fixed process-local `IssuerSession`; no arbitrary-principal mint;
- v1 assertion/protected-anchor signing is portable Ed25519 software signing with native-backend protection at rest;
- macOS v1 does not claim Secure Enclave/hardware-backed/non-exportable signing for that path;
- Windows/Linux remain unsupported/unverified unless native evidence is added;
- authorization remains ECR-003; independent action verification remains ECR-004.

## ECR-004 frozen planning boundaries

- reuse ECR-001 `VerificationReceipt`; no parallel verification truth record;
- `ActionReceipt` remains executor-observed evidence and cannot self-verify;
- aggregate states expose conflict rather than last-write-wins;
- reconciliation preserves UNKNOWN unless explicit independent evidence confirms effect/no-effect;
- no synthetic `ActionReceipt` is created by reconciliation;
- retry disposition is safety advisory for a future new-attempt proposal, not authorization or same-run scheduling;
- every reconciliation outcome leaves ECR-002 prepared/unreceipted/unresolved state and `RunPhase` unchanged;
- ECR-002 `RunEvent` v1 remains unchanged and no ECR-004 sidecar projection represents run resolution;
- ECR-004 uses a separate append-only verification journal with rebuildable projections;
- the journal digest chain is an integrity/corruption-detection mechanism only, not hostile complete-store tamper resistance;
- v1 acceptance stores synthetic/non-sensitive evidence metadata/references/digests only;
- no browser/network/model/provider/process/policy execution dependency is admitted.

## Planned critical path

| ID | Slice | Lifecycle | Depends on |
|---|---|---|---|
| ECR-031 | Identity, Trust Root & Sensitive Storage | `TASKS_READY_PENDING_EXACT_GREEN_HEAD` | ECR-001, ECR-002 |
| ECR-004 | Verification & Reconciliation | `TASKS_READY` | ECR-001, ECR-002 |
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
   ECR-001 [CLOSED] -> ECR-002 [CLOSED] -> {ECR-031 [TASKS_READY canonical planning / live implementation tracked separately], ECR-004 [TASKS_READY planning]} -> ECR-003 -> ECR-005

B. Browser wedge
   ECR-006 -> ECR-007 -> ECR-008

C. Trusted knowledge/context
   ECR-009 -> ECR-010 -> ECR-011

D. Skills
   ECR-012 -> ECR-013 -> ECR-014 -> ECR-015

E. Ecosystem/work surfaces
   ECR-016/ECR-017 -> ECR-018/ECR-019/ECR-020/ECR-021
```

## Sensitive-state boundary

ECR-002 proved synthetic/non-sensitive local durability. ECR-031 owns the protected identity/trust/storage foundation, but planning readiness alone does not authorize downstream slices to persist real authenticated browser secrets, credentials, private workspace payloads or equivalent high-value state. ECR-004 likewise remains synthetic/non-sensitive for persisted evidence until the relevant protection/privacy owners authorize more. Downstream policy/privacy ownership remains explicit in the roadmap.

## Update rule

When a slice lifecycle changes, update this file, `../../EXECUTION.md`, and the status field in `roadmap.md` in the same convergence/closure work. This file never overrides dependency semantics in `roadmap.md`.