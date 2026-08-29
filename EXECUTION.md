# Ecra Execution Guide

> **Operational start-here document.** Recover live work from this file, platform roadmap/status, active slice packages, and exact GitHub truth; do not depend on private chat state. Live repository/PR/Actions truth overrides stale prose.

## Source-of-truth order

1. `.specify/memory/constitution.md`
2. `EXECUTION.md`
3. `specs/000-ecra-platform/roadmap.md`
4. `specs/000-ecra-platform/STATUS.md`
5. relevant platform architecture/threat/gap/risk/benchmark/decision artifacts
6. `specs/README.md`
7. active slice package(s)
8. exact live branch/head, PR, CI, reviews and changed files

Stale prose must be updated to live evidence, never the reverse.

## Canonically closed foundation

```text
ECR-001 — Trusted Domain Kernel: CLOSED_CANONICAL
Closure ledger head: 85e4bf657b6c33e3f88d83e92e7a35279d177349
Closure CI: 33099434232 — SUCCESS

ECR-002 — Durable Run, Ledger & Budgets: CLOSED_CANONICAL
Final closure-convergence main head: aadc19c972e619222d426674d7542dd9c00dbe44
ECR-002 closure CI: 33155302100 — SUCCESS
ECR-001 regression on closure head: 33155302026 — SUCCESS
```

## Active trusted-substrate state

### ECR-031

```text
Slice: ECR-031 — Identity, Trust Root & Sensitive Storage Foundations
Branch: 031-identity-trust-root
PR: #4 — DRAFT / OPEN / NON-CANONICAL
State: BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE
```

Non-native work has advanced, but native macOS Data Protection Keychain acceptance remains externally blocked because the trusted runner user has no valid Apple Development signing identity, suitable provisioning profile, configured developer account or development team. Repository approval cannot create or infer those credentials. No legacy file-based Keychain, ad-hoc signing, plaintext/file/environment/memory fallback, or weakened acceptance is authorized.

### ECR-004

```text
Slice: ECR-004 — Verification & Reconciliation
Implementation PR: #7 — MERGED
Superseded review PR: #6 — CLOSED / NOT MERGED
Expected merged feature head: 990addb79e6fe5a1ad2b16dae159c624959e2128
Canonical merge head: 2a95fbb4f20b1646505cb179f4822a758a546895
Lifecycle: IMPLEMENTING_CLOSURE_CONVERGENCE
Current frontier: T053
```

T051 review processing completed with all 19 Cubic threads resolved and the governance-converged exact merge candidate passed the complete ECR-004 branch gate:

```text
HEAD 990addb79e6fe5a1ad2b16dae159c624959e2128
RUN  33255653083
JOB  99108796794
RESULT SUCCESS
```

T052 then merged PR #7 using the allowed non-rebase `merge` method with exact expected head `990addb79e6fe5a1ad2b16dae159c624959e2128`. Canonical `main` became `2a95fbb4f20b1646505cb179f4822a758a546895` and all required post-merge workflows passed on that exact state:

```text
ECR-001  RUN 33255780673  JOB 99109106995  SUCCESS
ECR-002  RUN 33255780671  JOB 99109107144  SUCCESS
ECR-004  RUN 33255780663  JOB 99109107058  SUCCESS
```

T052 is complete. T053 is the only remaining ECR-004 task.

## ECR-004 frozen boundaries

- ECR-001 `VerificationReceipt` is the only canonical independent verification record.
- `ActionReceipt` remains executor-observed evidence and cannot self-verify, including with its own immutable digest/artifact binding.
- UNKNOWN reconciliation never fabricates `ActionReceipt` or mutates ECR-002 run-event truth.
- aggregate/checkpoint/reconciliation views preserve conflict and grant no authority.
- `semantically_retryable*` is advisory only for a future new-attempt proposal and revalidates supplied reconciliation records against canonical supporting receipts.
- every reconciliation outcome leaves the original ECR-002 prepared/unreceipted/unresolved state and `unresolved_attempts` unchanged.
- `RunPhase` is unchanged by every ECR-004 reconciliation outcome.
- same-run `RunResumed`, `ExecutionCompleted`, and blind-retry guards remain authoritative.
- ECR-002 `RunEvent` v1 remains unchanged; no sidecar projection represents run resolution.
- ECR-004 uses a separate append-only verification journal with rebuildable projections.
- journal chaining is local integrity/corruption/substitution detection only, not hostile full-store tamper resistance.
- persisted v1 acceptance is synthetic/non-sensitive evidence metadata/references/digests only.
- no browser/network/model/provider/process/policy/authorization/identity/telemetry runtime dependency enters `ecra-verify`.
- ECR-004 exposes no provider execution, authorization, declassification, identity validation, secret-storage or same-run repair API.

## T053 closure sequence

```text
record T052 evidence and converge the active T053 execution ledger
  ↓
run ECR-001 + ECR-002 + ECR-004 on the exact closure-convergence main head
  ↓
if all succeed, atomically mark T053 complete and ECR-004 CLOSED_CANONICAL
     across tasks/status/roadmap/platform status/spec index/EXECUTION
  ↓
run ECR-001 + ECR-002 + ECR-004 on the exact final closure-marker main head
  ↓
only then claim CLOSED_CANONICAL externally
```

Historical green must not be reused after a content change to claim exact-head PASS.

## Dependency consequences

- ECR-003 remains blocked until ECR-031 is `CLOSED_CANONICAL`.
- ECR-005 requires ECR-001, ECR-002, ECR-003, ECR-004 and ECR-031; completing ECR-004 alone does **not** make ECR-005 implementation-eligible.
- ECR-004 closure can complete despite the ECR-031 external native-host blocker because ECR-031 is not an ECR-004 dependency.
- No later slice may be pulled forward merely because one lane is blocked.

## CI architecture

The repository-scoped self-hosted macOS runner `macbook` is the trusted execution oracle for current Rust slices. Persistent personal runners must not execute untrusted fork PR code.

ECR-001 and ECR-002 workflows remain permanent push gates on `main`. ECR-004 is also a permanent trusted push gate on `main` and includes explicit quickstart targets, ECR-001/ECR-002 regressions, unresolved-state compatibility, rustdoc/offline, boundary checks and dependency evidence.

## Execution rule

Follow each active slice's `tasks.md` dependency order. Fix actual CI/review blockers forward-only and immediately resume. Do not weaken tests/security boundaries to make gates green. No force-push, rebase or destructive history rewriting. Never mark PASS, MERGED, `VERIFIED_ON_BRANCH`, or `CLOSED_CANONICAL` without the exact evidence required by the owning package.