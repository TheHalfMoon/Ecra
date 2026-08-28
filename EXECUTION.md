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
Merge commit: 40efc8a64a9562f0f3eb2555b350cfa03d3e0675
Final closure-convergence main head: aadc19c972e619222d426674d7542dd9c00dbe44
ECR-002 closure-head CI: 33155302100 — SUCCESS
ECR-001 regression on closure head: 33155302026 — SUCCESS

Selected active slice: ECR-031 — Identity, Trust Root & Sensitive Storage Foundations
Lifecycle: IMPLEMENTING
Authorized implementation base / current canonical main: f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0
Implementation branch: 031-identity-trust-root
Implementation PR: #4 — DRAFT / NON-CANONICAL
Implementation clarification: IC-001 — Phase 4 dependency-order correction

Phase 1 verified head: 0289596bb7cdbb81d5f03c445fd324e985294143
Phase 1 CI: 33161529028 / job 98816955646 — SUCCESS
Phase 2 verified head: 4ddb6da267ebc90647e27fde382385a9d2529452
Phase 2 CI: 33163366128 / job 98822931741 — SUCCESS
Phase 3 semantic head: 35df7cab41c85cf9f0c9e6f6b7d20c0a57b18d15
Phase 3 semantic CI: 33165443131 / job 98829634574 — SUCCESS
Phase 3 closure head: 7eaede3f9f10461c307c8900c021273a4dbffa03
Phase 3 closure CI: 33165941748 / job 98831297208 — SUCCESS
Phase 4 semantic head: f4068278352a46ab1b42dba94994adf0f653f254
Phase 4 semantic CI: 33195283366 / job 98930731243 — SUCCESS
T042 verified ledger head: f05840782fab68b6360d69db912920f657102f05
T042 ECR-031 CI: 33195948025 / job 98932988529 — SUCCESS

Current task frontier: T051 after the current closure-record head is exact-green
```

## ECR-031 current implementation state

IC-001 required this prerequisite wave before T035:

```text
T043 → T044 → T045 → T046 → T047 → T048 → T049 → T050 → T059 → T060
```

The live implementation and T042 exact-head gate establish that the full prerequisite wave is complete:

- redacted/zeroizing sensitive bytes and production/test-isolated randomness;
- strict protected-envelope schema/AAD;
- domain-separated HKDF-SHA-256 derivation;
- ChaCha20-Poly1305 RFC 8439 protection and fail-closed authenticated open;
- frozen RFC/Ecra goldens and authenticated mutation corpus;
- typed minimal `TrustBackend`/capability boundary;
- compile-target-only production backend selection with no memory/plaintext/environment/file/test production fallback.

The corrected Phase 4 chain is complete and verified:

```text
T035  protected trust-root/key/enrollment schemas
T036  one-active-key-per-purpose selection
T041  authenticated protected trust-state store
T041A complete fail-closed local bootstrap transaction
T038  retirement semantics
T037  atomic rotation transition
T039  protected-state revocation transition
T040  lifecycle/bootstrap/crash/rollback-boundary coverage
T042  exact-head Phase 4 ledger gate
```

The next dependency-eligible work is T051 → T052 → T053. Because this closure record changes lifecycle docs after the T042 evidence head, require the permanent ECR-031 workflow to be green on the current record head before beginning T051.

## Frozen ECR-031 v1 security decisions

### Local identity bootstrap

```text
Ecra-local PrincipalId only
!= OS username/email/display name
!= filesystem path identity
!= legal/external identity proofing
!= NIST IAL/AAL/FAL certification
```

Bootstrap returns no usable enrolled identity until protected backend material and `ProtectedTrustStateV1` are durably published and successfully reopened/authenticated. Partial state yields typed `incomplete_bootstrap`; it never silently mints a second principal/root.

### Authoritative lifecycle state

`ProtectedTrustStateV1` is the authenticated authority for enrollment, active key generation, retirement and revocation. Ordinary metadata is rebuildable/non-authoritative. Only authenticated protected state can produce trusted snapshot material used for issuance/validation.

V1 does not claim universal monotonic rollback resistance against restoration of an older valid protected state together with equivalent authorized native-store state.

### Non-ambient issuance

No `issue(arbitrary_principal_id, ...)` production API exists. `EnrolledPrincipalHandle` + current `VerifiedTrustSnapshot` create a non-serializable process-local `IssuerSession` fixed to one principal/root/signing key. Caller-selected subject substitution is rejected. No ECR-031 IPC/network assertion issuer exists.

### Portable v1 crypto custody

```text
assertion signing       Ed25519 software key
protected-anchor sign   Ed25519 software key, purpose-separated
bounded key use         redacted/zeroizing process materialization
protected envelope      ChaCha20-Poly1305 + HKDF-SHA-256
native macOS backend    NOT YET ACCEPTED — T061–T068
NOT claimed             Secure Enclave / hardware-backed / non-exportable / user-presence signing
```

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

## Current exact execution order

After the current closure-record head is exact-green:

```text
T051 → T052 → T053
  ↓
T054 → T055 → T056 → T057 → T058
  ↓
T061 → T062 → T063 → T064 → T065 → T066 → T067 → T068
  ↓
T069 → T070 → T071 → T072 → T073 → T074
  ↓
T075 → T076 → T077 → T078 → T079 → T080 → T081 → T082
```

T061 owns the concrete macOS Data Protection Keychain backend. Do not steal that scope into T051–T058.

ECR-004 remains independently planning-eligible but must remain a separate slice. ECR-003 remains implementation-blocked until ECR-031 is `CLOSED_CANONICAL`.

## CI architecture

The repository-scoped self-hosted macOS runner `macbook` remains the trusted execution oracle. Persistent personal runners must not execute untrusted fork PR code.

The permanent ECR-031 push workflow must keep locked build, rustfmt, strict Clippy, workspace tests, ECR-001 and ECR-002 regressions, explicit ECR-031 targets, rustdoc, offline replay, boundary scripts and dependency/toolchain evidence green on the exact head being asserted.

Historical success cannot be reused after a content change to claim current-head PASS.

## Execution rule

Follow `tasks.md` dependency order. Fix actual CI/review blockers forward-only and immediately resume. Do not weaken tests/security boundaries to make gates green. No force-push, rebase or destructive history rewriting. Never mark PASS, MERGED, `VERIFIED_ON_BRANCH`, or `CLOSED_CANONICAL` without the exact evidence required by the active package.
