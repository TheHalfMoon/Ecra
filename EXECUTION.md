# Ecra Execution Guide

> **Operational start-here document.** Recover live work from this file, platform roadmap/status, the active slice package, and exact GitHub truth; do not depend on private chat state.

## Source-of-truth order

1. `.specify/memory/constitution.md`
2. `EXECUTION.md`
3. `specs/000-ecra-platform/roadmap.md`
4. `specs/000-ecra-platform/STATUS.md`
5. relevant platform architecture/threat/gap/risk/benchmark/decision artifacts
6. `specs/README.md`
7. active slice package
8. exact live branch/head, PR, CI, reviews and changed files

Stale prose must be updated to live evidence, never the reverse.

## Current execution truth

```text
ECR-001 — Trusted Domain Kernel: CLOSED_CANONICAL
Closure ledger head: 85e4bf657b6c33e3f88d83e92e7a35279d177349
Closure CI: 33099434232 — SUCCESS

ECR-002 — Durable Run, Ledger & Budgets: CLOSED_CANONICAL
Final feature head: 87fd9fc560bf5ca21a07a4d25473f305b4c05f05
Merge commit: 40efc8a64a9562f0f3eb2555b350cfa03d3e0675
Final closure-convergence main head: aadc19c972e619222d426674d7542dd9c00dbe44
ECR-002 closure-head CI: 33155302100 — SUCCESS
ECR-001 regression on closure head: 33155302026 — SUCCESS

Selected active slice: ECR-031 — Identity, Trust Root & Sensitive Storage Foundations
Lifecycle: TASKS_READY_PENDING_EXACT_GREEN_HEAD
Analyze Pass 1: 44e85aa9ccd28e185a5761889aa12b50459f286e — PLANNING_REWORK_REQUIRED
Analyze Pass 2: a3c7d563c139c65886f169f9181c07a997038f1f — ZERO_BLOCKING_PLANNING_DRIFT_FOUND
Requirements checklist: PASS_FOR_ANALYZE_PASS_2
Implementation branch: NOT YET CREATED
Implementation PR: NOT YET CREATED
```

ECR-031 implementation becomes authorized only after the final planning/lifecycle-convergence `main` head passes both permanent ECR-001 and ECR-002 workflows. The implementation branch MUST be created from that exact green SHA.

## ECR-031 package

Read in order:

```text
specs/031-identity-trust-root/STATUS.md
specs/031-identity-trust-root/spec.md
specs/031-identity-trust-root/research.md
specs/031-identity-trust-root/data-model.md
specs/031-identity-trust-root/contracts/identity-trust-v1.md
specs/031-identity-trust-root/threat-model.md
specs/031-identity-trust-root/plan.md
specs/031-identity-trust-root/tasks.md
specs/031-identity-trust-root/quickstart.md
specs/031-identity-trust-root/analyze.md
specs/031-identity-trust-root/checklists/requirements.md
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
PASS_1_BLOCKERS_FOUND=4
PASS_1_BLOCKERS_REMEDIATED=4
```

## Frozen ECR-031 v1 security decisions

### Local identity bootstrap

```text
Ecra-local PrincipalId only
!= OS username/email/display name
!= legal/external identity proofing
!= NIST IAL/AAL/FAL certification
```

Bootstrap returns no usable enrolled identity until native protected material and `ProtectedTrustStateV1` are durably published and successfully reopened/authenticated. A partial crash yields `incomplete_bootstrap`; it never silently mints a second principal/root.

### Authoritative lifecycle state

`ProtectedTrustStateV1` is the authenticated authority for enrollment, active key generation, retirement and revocation. Ordinary DB/file metadata is rebuildable/non-authoritative. Only authenticated protected state can produce `VerifiedTrustSnapshot` for validation or issuance.

V1 does not claim universal monotonic rollback resistance against restoration of an older valid protected state together with equivalent authorized OS trust-store state.

### Non-ambient issuance

No `issue(arbitrary_principal_id, ...)` production API exists. `EnrolledPrincipalHandle` + current `VerifiedTrustSnapshot` create a non-serializable process-local `IssuerSession` fixed to one principal/root/signing key. Caller-selected subject substitution is rejected. No ECR-031 IPC/network assertion issuer exists.

### Portable v1 crypto custody

```text
assertion signing       Ed25519 software key
protected-anchor sign   Ed25519 software key, purpose-separated
key at rest             protected by native TrustBackend
bounded key use         redacted/zeroizing process materialization
protected envelope      ChaCha20-Poly1305 + HKDF-SHA-256 direction
macOS v1 claim          Data Protection Keychain custody at rest
NOT claimed             Secure Enclave signing / hardware-backed / non-exportable signing
```

Exact dependency versions/features/licenses/advisories/MSRV remain T001 and must be re-verified immediately before adoption.

## Hard slice boundaries

ECR-031 MUST NOT absorb:
- general authorization/declassification/approval/secret-use policy — ECR-003;
- independent action outcome verification/reconciliation — ECR-004;
- protocol auth/token mapping — ECR-016;
- browser/model/tool/provider/process execution;
- local-model gateway — ECR-021;
- multi-device sync/recovery — ECR-022;
- privacy/telemetry product controls — ECR-025;
- general portability/export — ECR-029.

Identity evidence answers **who / on whose behalf**. It never means **what is authorized**.

## Next exact execution order

```text
1. Converge roadmap/platform status/spec indexes/tasks header to ECR-031 TASKS_READY planning truth.
2. Freeze resulting canonical main SHA.
3. Require ECR-001 and ECR-002 permanent workflows SUCCESS on that exact SHA.
4. Create branch 031-identity-trust-root from that exact SHA.
5. Create Draft ECR-031 implementation PR.
6. Execute T001 dependency/license/advisory/MSRV re-verification.
7. Continue T002–T082 strictly in tasks.md dependency order.
```

ECR-004 remains independently planning-eligible but must remain a separate slice. ECR-003 remains implementation-blocked until ECR-031 is `CLOSED_CANONICAL`.

## CI architecture

The repository-scoped self-hosted macOS runner `macbook` remains the trusted execution oracle. Persistent personal runners must not execute untrusted fork PR code.

Closed ECR-001/ECR-002 workflows remain push gates on `main`. ECR-031 implementation will add its own trusted push-only branch/main workflow with explicit bootstrap, validation, issuance, trust-state, envelope, anchor, redaction, boundaries and live macOS targets.

## Execution rule

Follow T001–T082 in dependency order after exact-head planning authorization. Fix actual CI/review blockers and immediately resume. Do not weaken tests/security boundaries to make gates green. No force-push, rebase or destructive history rewriting. Never mark PASS, MERGED or `CLOSED_CANONICAL` without exact-head/post-merge evidence.
