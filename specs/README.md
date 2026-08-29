# Ecra Spec Kit Index

This directory contains Ecra's canonical Spec-Driven Development packages.

## Start here

For current execution state, read:

1. `../EXECUTION.md`
2. `000-ecra-platform/roadmap.md`
3. `000-ecra-platform/STATUS.md`
4. the active slice `STATUS.md`, `tasks.md`, and `analyze.md`
5. exact live GitHub branch/head, PR, CI, review and changed-file truth

Do not infer implementation eligibility from directory names alone. The roadmap dependency graph, active package, and exact live evidence decide what may be planned or implemented.

## Platform package

`000-ecra-platform/` is the spec-of-specs and contains cross-platform planning:

- `roadmap.md` — immutable `ECR-###` IDs and dependencies.
- `architecture.md` — platform layers, dependency direction, providers, trust zones.
- `threat-model.md` — assets, adversaries, trust boundaries, attack classes.
- `gap-audit.md` — planning coverage and explicit gaps/deferrals.
- `risk-register.md` — persistent program risks and owning slices.
- `benchmark-matrix.md` — acceptance/evaluation program.
- `decision-log.md` — accepted architecture decisions and revisit triggers.
- `pre-implementation-review-2026-08-27.md` — pre-code review and remediation record.

## Canonically closed slices

### ECR-001 — Trusted Domain Kernel

Directory: `001-trusted-domain-kernel/`  
Lifecycle: `CLOSED_CANONICAL`.  
PR #1 merged; closure-ledger head `85e4bf657b6c33e3f88d83e92e7a35279d177349` passed CI `33099434232`.

### ECR-002 — Durable Run, Ledger & Budgets

Directory: `002-durable-run-ledger/`  
Lifecycle: `CLOSED_CANONICAL`.  
Final feature head `87fd9fc560bf5ca21a07a4d25473f305b4c05f05` merged as `40efc8a64a9562f0f3eb2555b350cfa03d3e0675`.  
Final closure-convergence main head `aadc19c972e619222d426674d7542dd9c00dbe44` passed ECR-002 CI `33155302100` and ECR-001 regression CI `33155302026`.

ECR-002 owns synthetic/non-sensitive local run durability, budgets, recovery and deterministic `.ecra` interchange. It does not authorize real sensitive persistence, authentication/trust roots, authorization, independent verification, or provider execution.

## Active / independently ready trusted-substrate slices

### ECR-031 — Identity, Trust Root & Sensitive Storage Foundations

Directory: `031-identity-trust-root/`  
Canonical planning lifecycle: `TASKS_READY`.  
Dependencies: ECR-001 + ECR-002 `CLOSED_CANONICAL`.  
Analyze Pass 2: `a3c7d563c139c65886f169f9181c07a997038f1f` — `ZERO_BLOCKING_PLANNING_DRIFT_FOUND`.

Package:

```text
STATUS.md
spec.md
research.md
data-model.md
contracts/identity-trust-v1.md
threat-model.md
plan.md
tasks.md
quickstart.md
analyze.md
checklists/requirements.md
```

Planning result:

```text
FR-001–FR-058 OWNED
SC-001–SC-016 OWNED
G1–G15 PASS / explicit PASS-N/A
UNOWNED_FR=0
UNOWNED_SC=0
MUST_LEVEL_PLANNING_GAPS=0
FAILED_CONSTITUTION_GATES=0
PASS_1_BLOCKERS_REMEDIATED=4/4
```

Frozen v1 boundaries include:
- opaque Ecra-local Principal bootstrap; no username/email/external identity-proofing shortcut;
- authenticated `ProtectedTrustStateV1` as lifecycle/revocation authority;
- `EnrolledPrincipalHandle` + `VerifiedTrustSnapshot` -> fixed, non-serializable `IssuerSession`; no arbitrary-principal mint;
- portable Ed25519 software signing key protected by native backend at rest, without Secure Enclave/hardware-backed/non-exportable signing claim;
- ChaCha20-Poly1305/HKDF protected-envelope direction subject to exact dependency verification;
- authorization remains ECR-003; independent outcome verification remains ECR-004.

Live implementation state is not represented by the old canonical planning snapshot alone. PR #4 / branch `031-identity-trust-root` is active; exact live branch `STATUS.md`, `EXECUTION.md`, Actions and PR truth govern its current task frontier. At the latest verified live state, T064/T068 are blocked by the trusted macOS runner's missing Apple code-signing identity/provisioning profile/developer account/team; no legacy/plaintext/ad-hoc fallback is authorized.

### ECR-004 — Verification & Reconciliation

Directory: `004-verification-receipts/`  
Lifecycle: `TASKS_READY` in this planning convergence; implementation remains unauthorized until this package is canonical on `main` and the exact resulting canonical planning head passes required ECR-001/ECR-002 regressions.  
Dependencies: ECR-001 + ECR-002 `CLOSED_CANONICAL`.  
Planning PR: #5.

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
checklists/requirements.md
```

Planning result:

```text
ANALYZE_PASS=3
PASS_1_BLOCKER_A-001=REMEDIATED
PASS_2_REVIEW_BLOCKER_A-002=REMEDIATED
FR-001–FR-046 OWNED
SC-001–SC-013 OWNED
G1–G15 PASS / explicit PASS-N/A
UNOWNED_FR=0
UNOWNED_SC=0
MUST_LEVEL_PLANNING_GAPS=0
FAILED_CONSTITUTION_GATES=0
ANALYZE=ZERO_BLOCKING_PLANNING_DRIFT_FOUND
```

Frozen v1 boundaries include:
- ECR-001 `VerificationReceipt` remains the only canonical independent verification record;
- `ActionReceipt` remains executor-observed evidence and cannot self-verify;
- `Fact`/artifact/run metadata do not receive a competing verified flag;
- deterministic aggregate states preserve verification conflict;
- critical verification checkpoints are exact-target requirements, not authority;
- UNKNOWN reconciliation never fabricates an `ActionReceipt` or mutates ECR-002 run-event truth;
- `effect_confirmed`, `no_effect_confirmed`, and `still_unknown` are ECR-004 effect evidence only;
- every reconciliation outcome leaves ECR-002 prepared/unreceipted/unresolved state, `unresolved_attempts`, and `RunPhase` unchanged;
- `semantically_retryable*` is fail-closed advisory metadata for a future new-attempt proposal only, not same-run resume/schedule/execution authorization;
- ECR-002 `RunEvent` v1 wire contract remains unchanged and no sidecar projection represents ECR-002 run resolution;
- a separate append-only ECR-004 journal stores synthetic/non-sensitive evidence metadata/references/digests only;
- journal digest chaining is normal integrity/corruption detection, not hostile complete-store tamper resistance;
- no browser/network/model/provider/process/policy execution dependency enters ECR-004 v1;
- IC-001 permits only read-only accessors for already-existing canonical ECR-001 `EvidenceRef` metadata, with unchanged wire/canonical semantics and mandatory ECR-001 regressions;
- IC-002 explicitly prohibits ECR-004 from clearing ECR-002 unresolved attempts or counterfeiting a run-repair protocol.

## Dependency boundary

ECR-004 is independently eligible from ECR-001/ECR-002 and therefore may progress even while ECR-031 has a separate native-host blocker. This independence must not be used to absorb ECR-031 scope or persist real sensitive evidence. ECR-003 remains blocked until ECR-031 is `CLOSED_CANONICAL`; ECR-005 remains blocked by its complete dependency set.

## Future slices

The full program reserves ECR-001 through ECR-031 in the platform roadmap. A roadmap row does not imply implementation authorization or that every package already exists.

Capability families:

```text
Trusted domain / durability / identity / policy / verification
Evaluation and threat harness
Browser prototype / browser foundation / browser product
Search evidence / workspace / memory / semantic capability routing
Skill IR / compile / replay / repair
MCP / ACP / A2A / Agent Skills gateway
Plugin and sandbox runtime
Terminal / developer workspace / data analytics
Local model gateway
Sync / registry / supply chain / privacy / accessibility
Source compliance / public benchmark program / portability / ecosystem gateway
```

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
- threat model for security/state boundaries;
- `implementation-clarifications.md` only when a real implementation underspecification is discovered, and converged before closure;
- `plan.md` — implementation architecture and constitutional gates;
- `tasks.md` — ordered executable tasks with exact paths;
- `quickstart.md` — verification workflow;
- `analyze.md` — consistency/traceability review;
- checklists.

## Status vocabulary

- `PLANNED` — roadmap decomposition only.
- `PLANNING_REWORK` — blocking planning defect; implementation forbidden.
- `SPEC_READY` — requirements complete enough for planning.
- `PLAN_READY` — research/data model/contracts/plan complete.
- `TASKS_READY` — executable tasks and latest analyze pass clean.
- `IMPLEMENTING` — bounded branch/PR active.
- `VERIFIED_ON_BRANCH` — phase/task group has exact-head evidence on active branch.
- `BLOCKED` — dependency/evidence gate prevents continuation.
- `CLOSED_CANONICAL` — implementation, tests, docs, convergence, merge and required post-merge evidence complete.
- `DEFERRED` — intentionally outside current critical path.

`VERIFIED_ON_BRANCH` is deliberately narrower than `CLOSED_CANONICAL`.