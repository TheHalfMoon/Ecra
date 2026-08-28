# Ecra Execution Guide

> **Operational start-here document.** Recover live work from this file, platform roadmap/status, active slice package, and exact GitHub truth. Live repository/PR/Actions truth overrides stale prose.

## Source-of-truth order

1. `.specify/memory/constitution.md`
2. `EXECUTION.md`
3. `specs/000-ecra-platform/roadmap.md`
4. `specs/000-ecra-platform/STATUS.md`
5. relevant architecture/threat/gap/risk/benchmark/decision artifacts
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
Phase 5 verified ledger head: bd066fa501476ff4f7fe43d0f4153de1e8d2fc60
Phase 5 CI: 33198508505 / job 98941727727 — SUCCESS

Current task frontier: T054 after current Phase 5 closure-record head is exact-green
```

## Current implementation state

IC-001 prerequisite wave T043–T050/T059–T060 and the corrected Phase 4 chain T035–T042 are complete and exact-head verified.

Phase 5 is now verified through T053. It provides:

- zeroizing/redacted sensitive-byte ownership;
- system CSPRNG with test-only deterministic provider;
- strict authenticated protected envelope with exact AAD;
- HKDF-SHA-256 derived key separation;
- ChaCha20-Poly1305 RFC 8439 protection and fail-closed open;
- frozen RFC/Ecra vectors plus authenticated-component mutation tests;
- committed-fixture at-rest secret sentinel scanning;
- signing/master/private/secret sentinel exclusion from debug/display, errors/log-style rendering, backend capability structure and persisted envelope metadata.

The current lifecycle-documentation closure record must itself pass the permanent ECR-031 workflow before T054 starts. Historical green evidence cannot be reused after a content change.

## Frozen ECR-031 v1 security decisions

- Local bootstrap creates only opaque Ecra-local identity; username/email/display label/path are never PrincipalId authority.
- `ProtectedTrustStateV1` is lifecycle authority; ordinary metadata is rebuildable/non-authoritative.
- Issuance is process-local/non-ambient and cannot mint for arbitrary caller-selected principals.
- Canonical assertion/protected-anchor signing suite is Ed25519 software signing under native protected custody.
- Protected envelopes use ChaCha20-Poly1305 + HKDF-SHA-256.
- No Secure Enclave, hardware-backed, non-exportable or user-presence signing claim exists in portable v1.
- No universal monotonic rollback-resistance claim exists against restoration of older valid protected+native-store state.
- No plaintext/file/environment/memory production fallback is permitted.

## Hard slice boundaries

ECR-031 does not own general authorization/declassification/approval (ECR-003), independent outcome verification (ECR-004), protocol token mapping (ECR-016), browser/model/tool/provider/process execution, local-model gateway (ECR-021), sync/recovery (ECR-022), privacy/telemetry product controls (ECR-025), or general portability/export (ECR-029).

Identity evidence answers **who / on whose behalf**, never **what is authorized**.

## Current exact execution order

```text
T054 → T055 → T056 → T057 → T058
  ↓
T061 → T062 → T063 → T064 → T065 → T066 → T067 → T068
  ↓
T069 → T070 → T071 → T072 → T073 → T074
  ↓
T075 → T076 → T077 → T078 → T079 → T080 → T081 → T082
```

T061 owns the concrete macOS Data Protection Keychain backend. T065/T066 may explicitly record Windows/Linux as unsupported/unverified if exact dependency/native verification is unavailable; they may not fabricate an implementation or fallback.

## CI architecture

The repository-scoped self-hosted macOS runner `macbook` is the trusted ECR-031 oracle. Every asserted exact head must pass stale-lock rejection, locked build, rustfmt, strict Clippy, workspace tests, ECR-001/ECR-002 regressions, explicit ECR-031 targets, rustdoc, offline replay, boundary scripts and dependency/toolchain evidence.

## Execution rule

Follow `tasks.md` dependency order. Fix actual CI/review blockers forward-only and immediately resume. Do not weaken tests/security boundaries to make gates green. No force-push, rebase or destructive history rewriting. Never mark PASS, MERGED, `VERIFIED_ON_BRANCH`, or `CLOSED_CANONICAL` without exact evidence required by the active package.
