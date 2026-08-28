# Ecra Execution Guide

> **Operational start-here document.** Recover live work from this file, the platform roadmap/status, the active slice package, and exact GitHub truth. Live repository/PR/Actions truth overrides stale prose.

## Source-of-truth order

1. `.specify/memory/constitution.md`
2. `EXECUTION.md`
3. `specs/000-ecra-platform/roadmap.md`
4. `specs/000-ecra-platform/STATUS.md`
5. relevant platform architecture/threat/gap/risk/benchmark/decision artifacts
6. `specs/README.md`
7. active slice package
8. exact live branch/head, PR, Actions, reviews and changed files

## Current execution truth

```text
ECR-001 — Trusted Domain Kernel: CLOSED_CANONICAL
ECR-001 closure CI: 33099434232 — SUCCESS

ECR-002 — Durable Run, Ledger & Budgets: CLOSED_CANONICAL
ECR-002 final closure main head: aadc19c972e619222d426674d7542dd9c00dbe44
ECR-002 closure CI: 33155302100 — SUCCESS

Selected active slice: ECR-031 — Identity, Trust Root & Sensitive Storage Foundations
Lifecycle: IMPLEMENTING
Canonical implementation base / current canonical main: f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0
Implementation branch: 031-identity-trust-root
Implementation PR: #4 — DRAFT / NON-CANONICAL
Implementation clarification: IC-001 — Phase 4 dependency-order correction

Phase 1 verified head: 0289596bb7cdbb81d5f03c445fd324e985294143
Phase 1 CI: 33161529028 / job 98816955646 — SUCCESS
Phase 2 verified head: 4ddb6da267ebc90647e27fde382385a9d2529452
Phase 2 CI: 33163366128 / job 98822931741 — SUCCESS
Phase 3 closure head: 7eaede3f9f10461c307c8900c021273a4dbffa03
Phase 3 closure CI: 33165941748 / job 98831297208 — SUCCESS
Phase 4 closure record: 217934d1f2c334b943349af87bcf40a4ad44b889
Phase 4 closure CI: 33196312711 / job 98934231597 — SUCCESS
T051 verified head: 2e16ec209e082d5964d176a9c79a95e7ddc907a4
T051 CI: 33197753549 / job 98939130739 — SUCCESS
T052 verified head: 16aac463d225a66c8b156e72ada9c74c30a4bf63
T052 CI: 33198215480 / job 98940733505 — SUCCESS

Current task frontier: T053 exact-head Phase 5 ledger gate
Next after T053: T054
```

## ECR-031 implementation state

IC-001 required this prerequisite wave before T035:

```text
T043 → T044 → T045 → T046 → T047 → T048 → T049 → T050 → T059 → T060
```

It is implemented and exercised by the Phase 4 gate. The corrected Phase 4 chain is also complete and exact-head verified:

```text
T035 → T036 → T041 → T041A → T038 → T037 → T039 → T040 → T042
```

Phase 5 implementation through T052 now provides:

- `SensitiveBytes` redaction + zeroizing ownership with explicit memory-secrecy non-claims;
- system CSPRNG/test-isolated deterministic randomness;
- strict `ProtectedEnvelopeV1` schema/AAD;
- HKDF-SHA-256 derived envelope keys;
- ChaCha20-Poly1305 RFC 8439 authenticated protection;
- fail-closed authenticated open with no plaintext on authentication/interpretation failure;
- frozen RFC/Ecra crypto vectors and mutation corpus;
- recursive at-rest fixture sentinel scanning;
- signing/master/private/secret sentinel exclusion from debug/display, parser-error/log-style output, backend capability structure and persisted protected-envelope metadata.

T053 remains pending until the ledger-convergence head containing this execution record passes the permanent ECR-031 workflow on its exact SHA. Historical green runs may not be reused after a content change.

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

`ProtectedTrustStateV1` is the authenticated authority for enrollment, active generation, retirement and revocation. Ordinary metadata is rebuildable/non-authoritative. Only authenticated protected state can produce trusted snapshot material used for issuance/validation.

V1 does not claim universal monotonic rollback resistance against restoration of an older valid protected state together with equivalent authorized native-store state.

### Non-ambient issuance

No generic `issue(arbitrary_principal_id, ...)` production API exists. `EnrolledPrincipalHandle` + current `VerifiedTrustSnapshot` create a non-serializable process-local `IssuerSession` fixed to one principal/root/signing key. Caller-selected subject substitution is rejected. No ECR-031 IPC/network assertion issuer exists.

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
- independent outcome verification/reconciliation — ECR-004;
- protocol auth/token mapping — ECR-016;
- browser/model/tool/provider/process execution;
- local-model gateway — ECR-021;
- multi-device sync/recovery — ECR-022;
- privacy/telemetry product controls — ECR-025;
- general portability/export — ECR-029.

Identity evidence answers **who / on whose behalf**. It never means **what is authorized**.

## Current exact execution order

```text
T053 exact-head Phase 5 gate
  ↓
T054 → T055 → T056 → T057 → T058
  ↓
T061 → T062 → T063 → T064 → T065 → T066 → T067 → T068
  ↓
T069 → T070 → T071 → T072 → T073 → T074
  ↓
T075 → T076 → T077 → T078 → T079 → T080 → T081 → T082
```

T061 owns the concrete macOS Data Protection Keychain backend. Do not steal that scope into protected-anchor work. T065/T066 may record explicit unsupported/unverified Windows/Linux status if exact dependency/native evidence is unavailable; they must not introduce a fallback or false assurance claim.

## CI architecture

The repository-scoped self-hosted macOS runner `macbook` is the trusted ECR-031 execution oracle. Persistent personal runners must not execute untrusted fork PR code.

Every asserted ECR-031 gate head must pass:

- stale-lock rejection;
- locked workspace build;
- rustfmt;
- strict Clippy (`-D warnings`);
- workspace tests;
- ECR-001 regression targets;
- ECR-002 regression targets;
- explicit ECR-031 targets;
- rustdoc;
- offline replay;
- ECR-001/ECR-002/ECR-031 boundary scripts;
- dependency/toolchain evidence.

## Execution rule

Follow `tasks.md` dependency order. Fix actual CI/review blockers forward-only and immediately resume. Do not weaken tests/security boundaries to make gates green. No force-push, rebase or destructive history rewriting. Never mark PASS, MERGED, `VERIFIED_ON_BRANCH`, or `CLOSED_CANONICAL` without the exact evidence required by the active package.
