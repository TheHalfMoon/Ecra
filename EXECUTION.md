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

## Canonically closed trusted substrate

```text
ECR-001 — Trusted Domain Kernel: CLOSED_CANONICAL
Closure ledger head: 85e4bf657b6c33e3f88d83e92e7a35279d177349
Closure CI: 33099434232 — SUCCESS

ECR-002 — Durable Run, Ledger & Budgets: CLOSED_CANONICAL
Final closure-convergence main head: aadc19c972e619222d426674d7542dd9c00dbe44
ECR-002 closure CI: 33155302100 — SUCCESS
ECR-001 regression on closure head: 33155302026 — SUCCESS

ECR-004 — Verification & Reconciliation: CLOSED_CANONICAL
Merged implementation head: 990addb79e6fe5a1ad2b16dae159c624959e2128
Canonical implementation merge: 2a95fbb4f20b1646505cb179f4822a758a546895
Closure-convergence head: c159c96061a73ead9710985d07608e2b417fe275
```

ECR-004 merge-state workflows:

```text
ECR-001  RUN 33255780673  JOB 99109106995  SUCCESS
ECR-002  RUN 33255780671  JOB 99109107144  SUCCESS
ECR-004  RUN 33255780663  JOB 99109107058  SUCCESS
```

ECR-004 closure-convergence workflows on exact head `c159c96061a73ead9710985d07608e2b417fe275`:

```text
ECR-001  RUN 33256430974  JOB 99110882402  SUCCESS
ECR-002  RUN 33256430942  JOB 99110916386  SUCCESS
ECR-004  RUN 33256430965  JOB 99110882233  SUCCESS
```

The T053 lifecycle marker marks ECR-004 `CLOSED_CANONICAL`. Do not make the external closure claim until ECR-001 + ECR-002 + ECR-004 also succeed on the exact canonical `main` head containing this marker.

## Current active trusted-substrate state

### ECR-031

```text
Slice: ECR-031 — Identity, Trust Root & Sensitive Storage Foundations
Branch: 031-identity-trust-root
PR: #4 — DRAFT / OPEN / NON-CANONICAL
State: BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE
Current blocking task: T064
```

The live ECR-031 branch/package is authoritative for exact implementation progress. Native macOS Data Protection Keychain acceptance remains externally blocked because the trusted runner user lacks a valid Apple Development signing identity, suitable provisioning profile, configured developer account and development team. Repository approval cannot create or infer those external Apple assets.

Do not bypass T064 with legacy file-based Keychain, ad-hoc signing, plaintext/file/environment/memory fallback, `synchronizing=true`, weakened acceptance, or unsupported Secure Enclave/hardware/non-exportability claims.

Canonical ECR-031 order remains:

```text
T064 [BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE]
  ↓
T068
  ↓
T069 → T070 → T071 → T072 → T073 → T074
  ↓
T075 → T076 → T077 → T078 → T079 → T080 → T081 → T082
```

## ECR-004 frozen boundaries

- ECR-001 `VerificationReceipt` is the only canonical independent verification record.
- `ActionReceipt` remains executor-observed evidence and cannot self-verify.
- UNKNOWN reconciliation never fabricates `ActionReceipt` or mutates ECR-002 run-event truth.
- aggregate/checkpoint/reconciliation views preserve conflict and grant no authority.
- `semantically_retryable*` is advisory only for a future new-attempt proposal.
- every reconciliation outcome leaves the original ECR-002 prepared/unreceipted/unresolved state and `unresolved_attempts` unchanged.
- `RunPhase` remains unchanged by ECR-004 reconciliation.
- same-run `RunResumed`, `ExecutionCompleted`, and blind-retry guards remain authoritative.
- ECR-002 `RunEvent` v1 remains unchanged; no sidecar projection represents run resolution.
- ECR-004 persistence remains a separate append-only verification journal with rebuildable projections.
- journal chaining is local integrity/corruption/substitution detection only, not hostile full-store tamper resistance.
- persisted v1 acceptance is synthetic/non-sensitive evidence metadata/references/digests only.
- no browser/network/model/provider/process/policy/authorization/identity/telemetry runtime dependency enters `ecra-verify`.
- ECR-004 exposes no provider execution, authorization, declassification, identity validation, secret-storage or same-run repair API.

## Dependency consequences after ECR-004 closure

- ECR-003 remains blocked until ECR-031 is `CLOSED_CANONICAL`.
- ECR-005 requires ECR-001, ECR-002, ECR-003, ECR-004 and ECR-031; ECR-004 closure alone does **not** make ECR-005 implementation-eligible.
- No later slice may be pulled forward merely because the ECR-031 lane is externally blocked.
- `specs/003-authority-policy-secrets/` is not an implementation-authorized package merely because its roadmap row exists; dependency closure and normal Spec Kit lifecycle still apply.

## CI architecture

The repository-scoped self-hosted macOS runner `macbook` is the trusted execution oracle for current Rust slices. Persistent personal runners must not execute untrusted fork PR code.

ECR-001, ECR-002 and ECR-004 workflows remain permanent trusted push gates on `main`. Historical green must never be reused after a content change to claim exact-head PASS.

## Execution rule

Follow the active slice `tasks.md` dependency order. Fix actual CI/review blockers forward-only and immediately resume. Do not weaken tests or security boundaries to make gates green. No force-push, rebase or destructive history rewriting. Never mark PASS, MERGED, `VERIFIED_ON_BRANCH`, or `CLOSED_CANONICAL` without the exact evidence required by the owning package.

At the current dependency frontier, ordinary repository work cannot bypass ECR-031 T064. If live GitHub/host evidence later proves the Apple signing/provisioning prerequisite is satisfied, resume ECR-031 from T064; otherwise the project is genuinely blocked on that external native-host prerequisite.
