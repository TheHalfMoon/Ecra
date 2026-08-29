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
Lifecycle: IMPLEMENTING / BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE
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
Phase 6 ledger head: 64c34744dd05b9850d8c9657a87e46913bd23412
Phase 6 CI: 33200973225 — SUCCESS

Phase 7 non-native evidence head: 4f2c150d2e5fd882d8554cd32a8aea4d4c5da639
Phase 7 ECR-031 CI: 33235282966 — all non-native steps SUCCESS; native Data Protection Keychain acceptance FAILURE
Host readiness: 33235282975 and 33235454670 — signing/provisioning assets absent

Current task frontier: T064 — live trusted-macOS Data Protection Keychain acceptance — BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE
```

## Current implementation state

IC-001 prerequisite wave T043–T050/T059–T060 and the corrected Phase 4 chain T035–T042 remain complete and verified.

Phase 5 T051–T053 is complete and verified.

Phase 6 T054–T058 is complete and exact-head verified by run `33200973225` on `64c34744dd05b9850d8c9657a87e46913bd23412`.

Phase 7 currently has:

- T061 complete: concrete macOS Data Protection Keychain backend using `security-framework = 3.7.0`, `use_protected_keychain()`, and `synchronizing=false`;
- T062 complete: unavailable/locked/not-found/delete failures normalized into redacted Ecra errors with no plaintext fallback;
- T063 complete: portable v1 macOS signing assurance frozen without Secure Enclave/hardware/non-exportability/user-presence overclaim;
- T064 open and blocked: live store/open/delete + bootstrap/reopen acceptance cannot pass until the runner has provisioning-authorized app-like code signing;
- T065 complete: Windows v1 explicit unsupported/unverified DPAPI status, no fallback/cross-machine/hardware-signing claim;
- T066 complete: Linux v1 explicit unsupported/unverified Secret Service status, no fallback and no secret lookup-attribute design;
- T067 complete: architecture tests prevent assurance inflation;
- T068 open: exact-head Phase 7 gate depends on T064.

On Phase 7 evidence head `4f2c150d2e5fd882d8554cd32a8aea4d4c5da639`, permanent ECR-031 CI run `33235282966` passed stale-lock rejection, build, rustfmt, strict Clippy, workspace tests, ECR-001/ECR-002 regressions, explicit ECR-031 targets, rustdoc, offline replay, all boundary scripts, and dependency/toolchain evidence. Only `macOS Data Protection Keychain live acceptance` failed.

## External native-acceptance blocker

The repository-scoped self-hosted macOS runner has an interactive console user that matches the runner account and has `codesign` plus Xcode build tooling. It does not currently have the credentials/assets required for Data Protection Keychain acceptance:

```text
code-signing identity      ABSENT
local provisioning profile ABSENT
Xcode developer account    ABSENT
Xcode development team     ABSENT
```

Evidence:

- readiness run `33235282975`: console/user/tools succeeded; identity/profile checks failed;
- readiness run `33235454670`: Xcode account/team, identity and profile checks failed.

The runner therefore cannot use Xcode Automatic Signing to create/download a valid profile because no Apple developer account/team is configured for that macOS user.

### Exact unblock action

Configure a valid Apple developer account/team in Xcode for the same macOS user that owns the self-hosted runner, and allow Xcode to create/install an Apple Development code-signing identity plus a provisioning profile suitable for an app-like ECR-031 test host. Then rerun the permanent trusted branch gate and require both Data Protection Keychain live tests to pass on the exact feature head.

This external prerequisite cannot be substituted by repository approval alone. Do not create or infer Apple credentials, a team identity, certificate, or provisioning profile from repository data.

## Frozen ECR-031 v1 security decisions

- Local bootstrap creates only opaque Ecra-local identity; username/email/display label/path are never PrincipalId authority.
- `ProtectedTrustStateV1` is lifecycle authority; ordinary metadata is rebuildable/non-authoritative.
- Issuance is process-local/non-ambient and cannot mint for arbitrary caller-selected principals.
- Canonical assertion/protected-anchor signing suite is Ed25519 software signing under native protected custody.
- Protected envelopes use ChaCha20-Poly1305 + HKDF-SHA-256.
- macOS v1 requires Data Protection Keychain with local-only/non-synchronizing behavior.
- No Secure Enclave, hardware-backed, non-exportable or user-presence signing claim exists in portable v1.
- No universal monotonic rollback-resistance claim exists against restoration of older valid protected+native-store state.
- No plaintext/file/environment/memory production fallback is permitted.
- Legacy file-based Keychain and ad-hoc signing are not acceptable substitutes for T064.

## Hard slice boundaries

ECR-031 does not own general authorization/declassification/approval (ECR-003), independent outcome verification (ECR-004), protocol token mapping (ECR-016), browser/model/tool/provider/process execution, local-model gateway (ECR-021), sync/recovery (ECR-022), privacy/telemetry product controls (ECR-025), or general portability/export (ECR-029).

Identity evidence answers **who / on whose behalf**, never **what is authorized**.

## Current exact execution order

```text
T064 [BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE]
  ↓
T068
  ↓
T069 → T070 → T071 → T072 → T073 → T074
  ↓
T075 → T076 → T077 → T078 → T079 → T080 → T081 → T082
```

Phase 8 is not eligible before T068. ECR-003 is not eligible before ECR-031 `CLOSED_CANONICAL`. ECR-004 remains a separate dependency-eligible slice and must not be folded into this PR.

## CI architecture

The repository-scoped self-hosted macOS runner `macbook` is the trusted ECR-031 oracle. Every asserted exact head must pass stale-lock rejection, locked build, rustfmt, strict Clippy, workspace tests, ECR-001 and ECR-002 regressions, explicit ECR-031 targets, rustdoc, offline replay, boundary scripts and dependency/toolchain evidence. Phase 7 additionally requires live Data Protection Keychain acceptance.

Historical success cannot be reused after a content change to claim current-head PASS. Bookkeeping/diagnostic commits made after an evidence head remain non-canonical until their own required gate reaches the appropriate result.

## Execution rule

Follow `tasks.md` dependency order. Fix actual CI/review blockers forward-only and immediately resume. Do not weaken tests/security boundaries to make gates green. No force-push, rebase or destructive history rewriting. Never mark PASS, MERGED, `VERIFIED_ON_BRANCH`, or `CLOSED_CANONICAL` without exact evidence required by the active package.
