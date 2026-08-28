# ECR-031 Status — Identity, Trust Root & Sensitive Storage Foundations

**Slice:** ECR-031  
**Lifecycle:** IMPLEMENTING  
**Dependencies:** ECR-001 `CLOSED_CANONICAL`, ECR-002 `CLOSED_CANONICAL`  
**Canonical implementation base:** `f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0`  
**Implementation branch:** `031-identity-trust-root`  
**Draft implementation PR:** #4  
**Constitution:** v1.1.0

ECR-031 remains branch-only implementation work. It is not canonical until the exact expected feature head is merged by an allowed non-rebase method and the required post-merge `main` gates succeed.

## Current execution frontier

T042 is complete. The Phase 4 ledger-convergence head `f05840782fab68b6360d69db912920f657102f05` passed permanent ECR-031 CI run `33195948025`, job `98932988529`, with result `SUCCESS` and every required gate green.

T051 is the next dependency-eligible task. Because this closure-record commit changes lifecycle documentation, begin T051 only after the permanent ECR-031 workflow also completes `SUCCESS` on this record head; live GitHub exact-head evidence remains authoritative.

```text
CURRENT_TASK               T051_AFTER_RECORD_HEAD_GREEN
NEXT_PHASE5_ORDER           T051 → T052 → T053
FEATURE_BRANCH              031-identity-trust-root
DRAFT_PR                    #4
CANONICAL_MAIN_BASE         f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0
PHASE4_SEMANTIC_HEAD        f4068278352a46ab1b42dba94994adf0f653f254
PHASE4_SEMANTIC_CI_RUN      33195283366
PHASE4_SEMANTIC_CI_JOB      98930731243
PHASE4_SEMANTIC_CI_RESULT   SUCCESS
T042_VERIFIED_HEAD          f05840782fab68b6360d69db912920f657102f05
T042_ECR031_CI_RUN          33195948025
T042_ECR031_CI_JOB          98932988529
T042_ECR031_CI_RESULT       SUCCESS
```

## Corrected prerequisite wave — reconciled implementation truth

IC-001 made the following order authoritative before T035:

```text
T043 → T044 → T045 → T046 → T047 → T048 → T049 → T050 → T059 → T060
```

The source, tests, fixtures and exact-head Phase 4/T042 CI establish that this entire prerequisite wave is implemented and exercised:

- T043 — redacted/zeroizing `SensitiveBytes` boundary.
- T044 — production system CSPRNG with deterministic provider restricted to tests.
- T045 — strict `ProtectedEnvelopeV1`, `EnvelopeKeyRef`, purpose/classification and exact AAD binding.
- T046 — domain-separated HKDF-SHA-256 envelope-key derivation.
- T047 — ChaCha20-Poly1305 RFC 8439 protection with 96-bit nonce ownership and full authentication tag.
- T048 — authenticated fail-closed open; no plaintext is returned when version/algorithm/key/AAD/nonce/ciphertext/tag validation fails.
- T049 — RFC 8439 dependency vector plus frozen Ecra HKDF/envelope goldens.
- T050 — authenticated-component mutation corpus including wrong key/AAD/nonce/tag behavior.
- T059 — typed crate-private `TrustBackend`, `TrustBackendSecretRef` and fail-closed `TrustBackendCapabilities` boundary with no raw private-key export API.
- T060 — production backend selection is compile-target-native only; memory/plaintext/environment/file/test substitutes are absent from the production selector and the in-memory marker is `cfg(test)` only.

This reconciliation does not claim that a concrete native backend exists yet. T061–T068 still own native backend/macOS acceptance.

Historical prerequisite evidence retained:

```text
IC001_VERIFIED_HEAD       21bce89f2e77bc2a54e74c37d349e9b53aa7631b
IC001_ECR031_CI_RUN       33168062289
IC001_ECR031_CI_RESULT    SUCCESS
T043_VERIFIED_HEAD        62048d9061dc1b74a9b5e0fed7376fe0ae08f2c3
T043_ECR031_CI_RUN        33168253618
T043_ECR031_CI_RESULT     SUCCESS
T044_VERIFIED_HEAD        0f84b2215529442cf7efbd1d3fa2892f224e6e6e
T044_ECR031_CI_RUN        33168674153
T044_ECR031_CI_RESULT     SUCCESS
PREREQ_RECONCILIATION     f05840782fab68b6360d69db912920f657102f05 / run 33195948025 / SUCCESS
```

## Phase 4 verified closure evidence

The corrected Phase 4 chain is complete:

```text
T035 → T036 → T041 → T041A → T038 → T037 → T039 → T040 → T042
```

Verified checkpoints:

```text
T035  0c2f9bc1cde6e33dcb36c34bb8068452f49b99bd  CI 33187589061  SUCCESS
T036  e552671c60cc7d406d01787059a9f0d093ab89ca  CI 33188491867  SUCCESS
T041  0e2406bcd2933ac9bbb01cf1904a2b774e80ba91  CI 33189579594  SUCCESS
T041A 86843c85b7114d0a318599144af4f5d82470ad6f  CI 33192605345  SUCCESS
T038  f5c229e05b23d8dc616d8abd81f543400f3307c5  CI 33193152128  SUCCESS
T037  ad1f3732b5b34e577904ab71a0cf669029295234  CI 33193746774  SUCCESS
T039  7abfcbccc3da0cba1841e939860011bf40e6b495  CI 33194274996  SUCCESS
T040  f4068278352a46ab1b42dba94994adf0f653f254  CI 33195283366  SUCCESS
T042  f05840782fab68b6360d69db912920f657102f05  CI 33195948025  SUCCESS
```

The T042 exact-head workflow passed locked build, rustfmt, strict Clippy, workspace tests, ECR-001 regression targets, ECR-002 regression targets, explicit ECR-031 phase targets, rustdoc, offline replay, ECR-001/ECR-002/ECR-031 boundary checks and dependency/toolchain evidence.

Phase 4 provides, on the feature branch:

- strict protected trust-root/key/enrollment state with opaque generated identifiers and no serialized private/root/symmetric secret material;
- one-active-key-per-purpose enforcement;
- ECR-031-owned authenticated protected trust-state persistence with bounded parsing and crash-safe atomic replacement on the accepted Unix/macOS durability path;
- fail-closed local bootstrap with durable in-progress marker, protected backend material, atomic state publish, authenticated reopen and no silent identity remint;
- retirement that blocks new material use while preserving only contract-authorized historical use;
- rotation that protects the next generation before atomically publishing the new active state and retiring the prior generation;
- protected-state revocation that blocks current use/validation without conflating revocation with backend-material destruction or backend unavailability;
- lifecycle/crash coverage for orphan backend material, incomplete bootstrap, post-publish stale marker recovery, generation collision, stale issuance, revoked validation, stale metadata and atomic write ordering;
- an explicit rollback-boundary fixture stating `no_monotonic_rollback_resistance`.

## Security and assurance boundaries

- `ProtectedTrustStateV1` is authoritative for local enrollment and key lifecycle. Ordinary metadata cannot mint, reactivate or unrevoke identity state.
- Only authenticated protected state can produce trusted snapshot material for issuance/validation.
- Bootstrap IDs are fresh opaque random identifiers and are not derived from usernames, email addresses, display labels or filesystem paths.
- ECR-031 identity evidence answers **who / on whose behalf**; it grants no authorization, capability, approval, declassification or execution lease.
- V1 does not claim universal monotonic rollback resistance against restoration of an older valid protected state together with equivalent authorized native-store state.
- Portable Ed25519 custody does not claim Secure Enclave, hardware-backed private operations, non-exportable signing keys or user-presence signing.
- Windows/Linux native backends are not verified by Phase 4.
- No browser/model/network/provider/protocol/process execution surface is introduced.

## Prior phase evidence

```text
PHASE1_HEAD               0289596bb7cdbb81d5f03c445fd324e985294143
PHASE1_CI                 33161529028 / job 98816955646 / SUCCESS
CARGO_LOCK_SHA256         5bd1b14d1643ff59492bafb7c6195b270cfc1424832788ad8078e62f22d907bc
RUST_TOOLCHAIN            1.98.0

PHASE2_HEAD               4ddb6da267ebc90647e27fde382385a9d2529452
PHASE2_CI                 33163366128 / job 98822931741 / SUCCESS

PHASE3_SEMANTIC_HEAD      35df7cab41c85cf9f0c9e6f6b7d20c0a57b18d15
PHASE3_SEMANTIC_CI        33165443131 / job 98829634574 / SUCCESS
PHASE3_CLOSURE_HEAD       7eaede3f9f10461c307c8900c021273a4dbffa03
PHASE3_CLOSURE_CI         33165941748 / job 98831297208 / SUCCESS
```

## Remaining canonical order

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

T061 owns the concrete macOS Data Protection Keychain backend. Do not represent Phase 4 or T051–T058 as satisfying native-backend acceptance.

## Closure rule

ECR-031 remains `IMPLEMENTING`, PR #4 remains draft/non-canonical, and `CLOSED_CANONICAL` remains forbidden until T081 merge evidence and T082 post-merge closure evidence are both satisfied. Historical CI success may not be reused to claim a changed current head green; the closure-record head must pass before implementation resumes at T051.
