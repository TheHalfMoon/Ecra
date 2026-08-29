# ECR-031 Status — Identity, Trust Root & Sensitive Storage Foundations

**Slice:** ECR-031  
**Lifecycle:** IMPLEMENTING / BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE  
**Dependencies:** ECR-001 `CLOSED_CANONICAL`, ECR-002 `CLOSED_CANONICAL`  
**Canonical implementation base:** `f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0`  
**Current canonical main:** `0e2ff8c687c93e6f158da6984a7a6915339b5f3f`  
**Implementation branch:** `031-identity-trust-root`  
**Draft implementation PR:** #4  
**Constitution:** v1.1.0

ECR-031 remains branch-only and non-canonical. `CLOSED_CANONICAL` is forbidden until T081 merges the exact expected feature head by an allowed non-rebase method and T082 records required post-merge `main` evidence.

## Current execution frontier

Phase 6 is closed on the branch. T058's ledger head `64c34744dd05b9850d8c9657a87e46913bd23412` passed permanent ECR-031 CI run `33200973225` with result `SUCCESS`.

Phase 7 implementation has advanced through T061–T063 and T065–T067. The macOS backend is concrete Data Protection Keychain integration, Windows and Linux are explicit unsupported/unverified v1 statuses with no fallback, and architecture tests prevent assurance inflation.

The latest fully observed Phase 7 implementation gate was head `4f2c150d2e5fd882d8554cd32a8aea4d4c5da639`, ECR-031 CI run `33235282966`. On that exact head all non-native gates succeeded:

- stale-lock rejection and locked workspace build;
- rustfmt and strict Clippy;
- complete workspace tests;
- ECR-001 regression targets;
- ECR-002 regression targets;
- explicit ECR-031 phase targets;
- rustdoc and offline replay;
- ECR-001/ECR-002/ECR-031 boundary and dependency checks;
- ECR-031 dependency/toolchain evidence;
- macOS host-readiness diagnostics.

The only failing step was `macOS Data Protection Keychain live acceptance`. Both native acceptance tests failed closed at `macos_keychain_store` with the redacted Ecra `TrustRootUnavailable` result. No implementation regression was observed before that step.

T064 is therefore the current blocking task. T068 cannot close until T064 succeeds, and Phase 8 remains dependency-blocked by T068.

Canonical repository truth has advanced independently while this branch remains blocked: ECR-004 is now `CLOSED_CANONICAL` and canonical `main` is `0e2ff8c687c93e6f158da6984a7a6915339b5f3f`. That advancement does not satisfy T064 and does not authorize ECR-003/ECR-005 or any later slice to bypass ECR-031.

```text
CURRENT_TASK                    T064_LIVE_MACOS_ACCEPTANCE
CURRENT_STATE                   BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE
NEXT_IF_UNBLOCKED               T064 → T068 → T069
FEATURE_BRANCH                  031-identity-trust-root
DRAFT_PR                        #4
CANONICAL_IMPLEMENTATION_BASE   f6d8eb6ff6a60aa0ad8a6f52686a62f12cd374b0
CURRENT_CANONICAL_MAIN          0e2ff8c687c93e6f158da6984a7a6915339b5f3f
PHASE6_LEDGER_HEAD              64c34744dd05b9850d8c9657a87e46913bd23412
PHASE6_ECR031_CI_RUN            33200973225
PHASE6_ECR031_RESULT            SUCCESS
PHASE7_NON_NATIVE_EVIDENCE_HEAD 4f2c150d2e5fd882d8554cd32a8aea4d4c5da639
PHASE7_ECR031_CI_RUN            33235282966
PHASE7_NON_NATIVE_GATES         SUCCESS
PHASE7_NATIVE_ACCEPTANCE        FAILURE
LATEST_READINESS_RUN            33235609984
LATEST_READINESS_ATTEMPT        4
LATEST_READINESS_JOB            99122182202
```

## T064 external host blocker

The trusted self-hosted `macbook` runner has the required interactive user context and build/signing tools, but it lacks the Apple signing/provisioning material required for Data Protection Keychain access by an app-like host.

Readiness evidence:

- run `33235282975`: interactive console session `SUCCESS`; runner matches console user `SUCCESS`; signing tools available `SUCCESS`; code-signing identity check `FAILURE`; local provisioning-profile check `FAILURE`;
- run `33235454670`: Xcode developer-account registry check `FAILURE`; Xcode development-team check `FAILURE`; code-signing identity check `FAILURE`; provisioning-profile check `FAILURE`; signing tools check `SUCCESS`;
- run `33235609984`, attempt `2`, job `99114614854`: same external Apple assets absent after ECR-004 canonical closure;
- run `33235609984`, attempt `3`, job `99121242472`: same external Apple assets absent on trusted runner `macbook`;
- run `33235609984`, attempt `4`, job `99122182202`, executed at `2026-08-29T15:37:36Z`: console user, runner-user match, `codesign`, and `xcodebuild` passed; code-signing identity, local provisioning profile, Xcode developer-account registry, and Xcode development-team registry remained absent.

The latest live readiness result therefore remains fail-closed at the external host prerequisite boundary. The workflow's diagnostic job concludes `SUCCESS` because the asset-detection probes are evidence-producing diagnostics; the individual probe logs are authoritative for asset presence and continue to show the four required Apple assets as absent.

This means repository automation cannot create the missing assets by Xcode Automatic Signing because the runner user has no configured Apple developer account/team from which Xcode could obtain a development certificate and provisioning profile.

### Exact unblock condition

On the same macOS user that owns the self-hosted runner, configure a valid Apple developer account/team in Xcode and allow Xcode to create/install an Apple Development code-signing identity plus a provisioning profile suitable for the app-like T064 test host. After those assets exist, rerun the trusted branch workflow and require both ignored Data Protection Keychain tests to pass on the exact feature head.

The unblock condition is an external host credential/provisioning prerequisite. User approval to modify Ecra does not itself provide an Apple developer identity, certificate, team, or provisioning profile.

### Prohibited shortcuts

Do not unblock T064 by:

- using the legacy file-based keychain instead of Data Protection Keychain;
- setting `synchronizing=true`;
- introducing plaintext/file/environment/memory fallback;
- treating ad-hoc signing as provisioning-authorized signing;
- weakening the live native acceptance requirement;
- claiming Secure Enclave, hardware-backed, non-exportable, user-presence, cross-machine, or unverified platform guarantees.

## Phase 7 implemented behavior

T061–T063 provide:

- `security-framework = 3.7.0` Data Protection Keychain access through `use_protected_keychain()`;
- explicit `synchronizing=false` configuration;
- local protected storage for envelope-root and Ed25519 software-signing secret material;
- normalized unavailable/locked/not-found/delete failures with no raw OSStatus or secret leakage;
- portable v1 assurance with `hardware_backed_private_operations=false`, `non_exportable_private_key=false`, no Secure Enclave/user-presence claim.

T065–T067 provide:

- Windows DPAPI status explicitly `unsupported/unverified` until a trusted native implementation/test path exists;
- Linux Secret Service status explicitly `unsupported/unverified` until a trusted native implementation/test path exists;
- no fallback for either platform;
- architecture tests preventing Windows/Linux from rendering as verified/hardware-backed and preventing portable macOS Ed25519 from rendering as Secure Enclave/non-exportable signing.

T064's live tests are present but intentionally ignored in the ordinary workspace suite and run explicitly by the native acceptance gate. They cover Data Protection Keychain store/open/delete for all v1 secret purposes and bootstrap → durable protected-state publish → authenticated reopen of the same local identity.

## Earlier verified checkpoints

```text
PHASE1_HEAD           0289596bb7cdbb81d5f03c445fd324e985294143  CI 33161529028  SUCCESS
PHASE2_HEAD           4ddb6da267ebc90647e27fde382385a9d2529452  CI 33163366128  SUCCESS
PHASE3_CLOSURE_HEAD   7eaede3f9f10461c307c8900c021273a4dbffa03  CI 33165941748  SUCCESS
PHASE4_CLOSURE_RECORD 217934d1f2c334b943349af87bcf40a4ad44b889  CI 33196312711  SUCCESS
PHASE5_LEDGER_HEAD    bd066fa501476ff4f7fe43d0f4153de1e8d2fc60  CI 33198508505  SUCCESS
PHASE6_LEDGER_HEAD    64c34744dd05b9850d8c9657a87e46913bd23412  CI 33200973225  SUCCESS
```

## Security and assurance boundaries

- `ProtectedTrustStateV1` is authoritative for enrollment/key lifecycle; ordinary metadata cannot mint, activate or unrevoke identity state.
- Bootstrap IDs are opaque CSPRNG-generated Ecra-local identifiers, never username/email/display-label/path-derived identity.
- Identity evidence answers **who / on whose behalf** and grants no capability, approval, declassification, authorization or execution lease.
- Portable v1 signing uses Ed25519 software key material protected by the selected native backend at rest; no Secure Enclave, hardware-backed, non-exportable or user-presence signing claim is made.
- No universal monotonic rollback resistance is claimed against restoration of an older valid protected state plus equivalent authorized native-store state.
- No plaintext/environment/file/memory production fallback is permitted.
- Windows/Linux remain explicitly unsupported/unverified in ECR-031 v1.
- ECR-031 adds no browser/model/network/provider/protocol/process execution surface.

## Remaining canonical order

```text
T064 [BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE]
  ↓
T068
  ↓
T069 → T070 → T071 → T072 → T073 → T074
  ↓
T075 → T076 → T077 → T078 → T079 → T080 → T081 → T082
```

ECR-003 remains implementation-blocked until ECR-031 is `CLOSED_CANONICAL`. ECR-004 is `CLOSED_CANONICAL`. ECR-005 remains blocked by ECR-003/ECR-031 despite ECR-004 closure. No Phase 8/9 work or later slice may be represented as eligible merely to bypass T064/T068.