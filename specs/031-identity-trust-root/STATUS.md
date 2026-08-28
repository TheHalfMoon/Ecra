# ECR-031 Status — Identity, Trust Root & Sensitive Storage Foundations

**Slice:** ECR-031  
**Lifecycle:** IMPLEMENTING  
**Dependencies:** ECR-001 `CLOSED_CANONICAL`, ECR-002 `CLOSED_CANONICAL`  
**Canonical implementation base / current canonical main:** `f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0`  
**Implementation branch:** `031-identity-trust-root`  
**Draft implementation PR:** #4  
**Constitution:** v1.1.0

ECR-031 remains branch-only and non-canonical. `CLOSED_CANONICAL` is forbidden until T081 merges the exact expected feature head by an allowed non-rebase method and T082 records the required post-merge `main` evidence.

## Current execution frontier

T051 is exact-head verified on `2e16ec209e082d5964d176a9c79a95e7ddc907a4` by permanent ECR-031 CI run `33197753549`, job `98939130739`, result `SUCCESS`.

T052 is exact-head verified on `16aac463d225a66c8b156e72ada9c74c30a4bf63` by permanent ECR-031 CI run `33198215480`, job `98940733505`, result `SUCCESS`.

This record is the T053 Phase 5 ledger-convergence candidate. Phase 5 is **not** complete merely because T051/T052 passed. Require the permanent ECR-031 workflow to complete `SUCCESS` on the exact head containing this record before creating the T053 closure record or beginning T054.

```text
CURRENT_TASK               T053_PHASE5_EXACT_HEAD_GATE
NEXT_AFTER_T053            T054
FEATURE_BRANCH              031-identity-trust-root
DRAFT_PR                    #4
CANONICAL_MAIN_BASE         f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0
T051_VERIFIED_HEAD          2e16ec209e082d5964d176a9c79a95e7ddc907a4
T051_ECR031_CI_RUN          33197753549
T051_ECR031_CI_JOB          98939130739
T051_ECR031_CI_RESULT       SUCCESS
T052_VERIFIED_HEAD          16aac463d225a66c8b156e72ada9c74c30a4bf63
T052_ECR031_CI_RUN          33198215480
T052_ECR031_CI_JOB          98940733505
T052_ECR031_CI_RESULT       SUCCESS
T053_RESULT                 PENDING_EXACT_HEAD_GATE
```

## Phase 5 convergence candidate

The Phase 5 chain is now implemented through T052:

```text
T043 → T044 → T045 → T046 → T047 → T048 → T049 → T050 → T051 → T052 → T053
```

Evidence represented by the current implementation:

- T043 — redacted/zeroizing `SensitiveBytes`; no process/OS-wide memory-secrecy overclaim.
- T044 — production system CSPRNG boundary; deterministic randomness remains test-only.
- T045 — strict `ProtectedEnvelopeV1`, typed key reference, closed purpose/classification and exact AAD binding.
- T046 — domain-separated HKDF-SHA-256 derived envelope keys over bounded backend-opened master material.
- T047 — ChaCha20-Poly1305 RFC 8439 protection with 96-bit nonce ownership and full authentication tag.
- T048 — fail-closed authenticated open; no plaintext on version/algorithm/key/AAD/nonce/ciphertext/tag failure.
- T049 — RFC 8439 dependency vector plus frozen Ecra HKDF/envelope goldens.
- T050 — mutation corpus for authenticated envelope components and wrong key/AAD/nonce/tag behavior.
- T051 — recursive synthetic at-rest sentinel scan over committed ECR-031 persisted fixtures; plaintext signing-secret sentinel is forbidden outside intentional test input.
- T052 — signing/master/private/secret sentinels are absent from `SensitiveBytes` Debug/Display output, parser-error/log-style rendering, backend capability structure and persisted protected-envelope metadata.

T053 remains a gate task, not an implementation shortcut. Do not mark it complete until this ledger-convergence head itself passes locked build, rustfmt, strict Clippy, workspace tests, ECR-001/ECR-002 regressions, explicit ECR-031 targets, rustdoc, offline replay, boundary checks and dependency/toolchain evidence.

## Phase 4 verified closure evidence

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
PHASE4_CLOSURE_RECORD 217934d1f2c334b943349af87bcf40a4ad44b889  CI 33196312711  SUCCESS
```

IC-001 prerequisite tasks T043–T050 and T059–T060 were executed before T035 as required and were reconciled into the Phase 4 ledger. T061–T068 still own concrete native-backend/macOS acceptance.

## Prior phase evidence

```text
PHASE1_HEAD          0289596bb7cdbb81d5f03c445fd324e985294143
PHASE1_CI            33161529028 / job 98816955646 / SUCCESS
PHASE2_HEAD          4ddb6da267ebc90647e27fde382385a9d2529452
PHASE2_CI            33163366128 / job 98822931741 / SUCCESS
PHASE3_SEMANTIC_HEAD 35df7cab41c85cf9f0c9e6f6b7d20c0a57b18d15
PHASE3_SEMANTIC_CI   33165443131 / job 98829634574 / SUCCESS
PHASE3_CLOSURE_HEAD  7eaede3f9f10461c307c8900c021273a4dbffa03
PHASE3_CLOSURE_CI    33165941748 / job 98831297208 / SUCCESS
```

## Security and assurance boundaries

- `ProtectedTrustStateV1` is authoritative for enrollment and key lifecycle; ordinary metadata cannot mint, activate or unrevoke identity state.
- Bootstrap creates fresh opaque Ecra-local identifiers and never derives PrincipalId from username, email, display label or filesystem path.
- Identity evidence answers **who / on whose behalf** and grants no capability, approval, declassification, authorization or execution lease.
- Portable v1 Ed25519 signing material is software key material protected by the selected native backend at rest and bounded/zeroizing in process. No Secure Enclave, hardware-backed, non-exportable or user-presence signing claim is made.
- No universal monotonic rollback resistance is claimed against restoration of an older valid protected state together with equivalent authorized native-store state.
- No plaintext/environment/file/memory production fallback is permitted.
- Concrete macOS Data Protection Keychain implementation/acceptance is still T061–T068.
- Windows/Linux remain unverified until their explicit Phase 7 contract/status tasks are completed.
- ECR-031 introduces no browser/model/network/provider/protocol/process execution surface.

## Remaining canonical order

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

ECR-003 remains implementation-blocked until ECR-031 is `CLOSED_CANONICAL`. ECR-004 remains separately dependency-eligible and must stay outside this slice.
