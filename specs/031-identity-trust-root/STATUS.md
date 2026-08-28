# ECR-031 Status — Identity, Trust Root & Sensitive Storage Foundations

**Slice:** ECR-031  
**Lifecycle:** IMPLEMENTING  
**Dependencies:** ECR-001 `CLOSED_CANONICAL`, ECR-002 `CLOSED_CANONICAL`  
**Canonical implementation base / current canonical main:** `f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0`  
**Implementation branch:** `031-identity-trust-root`  
**Draft implementation PR:** #4  
**Constitution:** v1.1.0

ECR-031 remains branch-only and non-canonical. `CLOSED_CANONICAL` is forbidden until T081 merges the exact expected feature head by an allowed non-rebase method and T082 records required post-merge `main` evidence.

## Current execution frontier

Phase 6 semantic work T054–T057 is exact-head verified on `a668df317d1718008c8008ee35a40ebb83c038a4` by permanent ECR-031 CI run `33200534586`, job `98948594548`, result `SUCCESS`. Locked build, rustfmt, strict Clippy, workspace tests, ECR-001/ECR-002 regressions, explicit ECR-031 targets, rustdoc, offline replay, all boundary checks and dependency/toolchain evidence succeeded on that exact head.

T058 is the current task. This ledger-convergence record must itself pass the permanent ECR-031 workflow before T058 can be closed and the frontier can move to T061.

```text
CURRENT_TASK               T058_LEDGER_GATE
NEXT_PHASE7_ORDER           T061 → T062 → T063 → T064 → T065 → T066 → T067 → T068
FEATURE_BRANCH              031-identity-trust-root
DRAFT_PR                    #4
CANONICAL_MAIN_BASE         f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0
PHASE5_VERIFIED_HEAD        bd066fa501476ff4f7fe43d0f4153de1e8d2fc60
PHASE5_ECR031_CI_RUN        33198508505
PHASE5_ECR031_CI_JOB        98941727727
PHASE5_ECR031_CI_RESULT     SUCCESS
T054_T055_VERIFIED_HEAD     11628134acbe91bbb81e0c3073bc0462ff22ecc9
T054_T055_ECR031_CI_RUN     33199925996
T054_T055_ECR031_CI_JOB     98946520761
T056_T057_VERIFIED_HEAD     a668df317d1718008c8008ee35a40ebb83c038a4
T056_T057_ECR031_CI_RUN     33200534586
T056_T057_ECR031_CI_JOB     98948594548
T056_T057_ECR031_RESULT     SUCCESS
```

## Phase 6 verified semantic behavior

```text
T054 → T055 → T056 → T057
```

Verified behavior includes:

- strict bounded `ProtectedAnchorV1` wire with closed purpose/algorithm values and strict lowercase SHA-256 payload digest representation;
- domain-separated canonical protected-anchor signing input matching the frozen v1 contract;
- Ed25519 protected-anchor signing through the purpose-specific active `KeyRecord` and bounded backend-opened seed material copied into zeroizing memory;
- no public raw signing-secret export API and no hardware/non-exportability overclaim;
- verification bound to exact trust root, key ID, purpose, algorithm, lifecycle state and signature;
- new signing denied for retired/revoked keys; historical verification follows the explicit lifecycle contract;
- deterministic signed mutation corpus covering digest, purpose, key and signature changes;
- type-level distinction between `ProtectedAnchorV1`, generic `ContentDigest`, and ECR-004 `VerificationReceipt`;
- ECR-002 `run-created-golden.sha256` reused byte-for-byte as a bounded ledger-head anchor payload example without adding an `ecra-run` dependency or changing `LedgerDigest`/store semantics.

## Phase 5 verified closure

Phase 5 remains verified through T053 and provides protected-envelope cryptography, redacted/zeroizing sensitive-byte handling, deterministic vectors/mutation coverage, and committed-fixture secret sentinel scanning. T061–T068 still own concrete native backend/macOS acceptance.

## Earlier phase checkpoints

```text
PHASE1_HEAD           0289596bb7cdbb81d5f03c445fd324e985294143  CI 33161529028  SUCCESS
PHASE2_HEAD           4ddb6da267ebc90647e27fde382385a9d2529452  CI 33163366128  SUCCESS
PHASE3_CLOSURE_HEAD   7eaede3f9f10461c307c8900c021273a4dbffa03  CI 33165941748  SUCCESS
PHASE4_CLOSURE_RECORD 217934d1f2c334b943349af87bcf40a4ad44b889  CI 33196312711  SUCCESS
PHASE5_LEDGER_HEAD    bd066fa501476ff4f7fe43d0f4153de1e8d2fc60  CI 33198508505  SUCCESS
```

## Security and assurance boundaries

- `ProtectedTrustStateV1` is authoritative for enrollment/key lifecycle; ordinary metadata cannot mint, activate or unrevoke identity state.
- Bootstrap IDs are opaque CSPRNG-generated Ecra-local identifiers, never username/email/display-label/path-derived identity.
- Identity evidence answers **who / on whose behalf** and grants no capability, approval, declassification, authorization or execution lease.
- Portable v1 signing uses Ed25519 software key material protected by the selected native backend at rest; no Secure Enclave, hardware-backed, non-exportable or user-presence signing claim is made.
- No universal monotonic rollback resistance is claimed against restoration of an older valid protected state plus equivalent authorized native-store state.
- No plaintext/environment/file/memory production fallback is permitted.
- Concrete macOS Data Protection Keychain implementation/acceptance remains T061–T068; Windows/Linux remain unverified until their Phase 7 status tasks.
- ECR-031 adds no browser/model/network/provider/protocol/process execution surface.

## Remaining canonical order

```text
T058
  ↓
T061 → T062 → T063 → T064 → T065 → T066 → T067 → T068
  ↓
T069 → T070 → T071 → T072 → T073 → T074
  ↓
T075 → T076 → T077 → T078 → T079 → T080 → T081 → T082
```

ECR-003 remains implementation-blocked until ECR-031 is `CLOSED_CANONICAL`. ECR-004 remains separately dependency-eligible and outside this slice.
