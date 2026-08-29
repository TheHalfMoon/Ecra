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

The latest fully observed pre-harness-remediation Phase 7 implementation gate was head `4f2c150d2e5fd882d8554cd32a8aea4d4c5da639`, ECR-031 CI run `33235282966`. On that exact head all non-native gates succeeded and the native acceptance tests failed closed with redacted `TrustRootUnavailable` results.

T064 remains the current blocking task. T068 cannot close until T064 succeeds, and Phase 8 remains dependency-blocked by T068.

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
LATEST_LEGACY_READINESS_RUN     33235609984
LATEST_LEGACY_READINESS_ATTEMPT 4
LATEST_LEGACY_READINESS_JOB     99122182202
T064_HOST_HARNESS               PROVISIONING_AUTHORIZED_APP_LIKE
```

## T064 live-acceptance state

### Repository-side harness remediation

Live investigation found a MUST-level native-acceptance harness defect: the original workflow invoked the ignored Rust library tests as a bare Cargo test executable even though Data Protection Keychain access on macOS is determined by the host process code-signing entitlements and restricted keychain entitlements require provisioning-profile authorization.

IC-002 records the correction. Repository-owned T064 acceptance now uses `scripts/run-ecr031-macos-live-acceptance.sh` and requires a signed app-like host before either live test executes. The harness:

- requires the self-hosted runner user to own an interactive macOS console session;
- requires `security`, `codesign`, `xcodebuild`, and `python3`;
- selects only an unexpired macOS provisioning profile authorizing the fixed synthetic bundle identifier `dev.ecra.identity.t064` and its concrete keychain access group;
- requires a matching `Apple Development` signing identity for the profile team;
- builds the exact `ecra-identity` library test executable without running it bare;
- wraps the test executable as `EcraT064Host.app/Contents/MacOS/EcraT064Host`;
- embeds the selected profile at `Contents/embedded.provisionprofile`;
- claims only the concrete application-identifier, team-identifier and keychain-access-groups entitlements authorized by that profile;
- signs and verifies the app-like bundle and checks the signed entitlement payload;
- runs both ignored Data Protection Keychain tests only from the signed app-like host;
- removes all temporary host material after completion.

The permanent ECR-031 workflow invokes the same harness for strict readiness and live acceptance. The dedicated host-readiness workflow now checks the same actual profile/identity authorization boundary instead of reporting overall green while asset probes fail under `continue-on-error`.

### Historical external readiness evidence

Before the app-like harness correction, trusted-runner checks established:

- run `33235282975`: interactive console session `SUCCESS`; runner matches console user `SUCCESS`; signing tools available `SUCCESS`; code-signing identity check `FAILURE`; local provisioning-profile check `FAILURE`;
- run `33235454670`: Xcode developer-account registry check `FAILURE`; Xcode development-team check `FAILURE`; code-signing identity check `FAILURE`; provisioning-profile check `FAILURE`; signing tools check `SUCCESS`;
- run `33235609984`, attempt `2`, job `99114614854`: same external Apple assets absent after ECR-004 canonical closure;
- run `33235609984`, attempt `3`, job `99121242472`: same external Apple assets absent on trusted runner `macbook`;
- run `33235609984`, attempt `4`, job `99122182202`, executed at `2026-08-29T15:37:36Z`: console user, runner-user match, `codesign`, and `xcodebuild` passed; no code-signing identity or local provisioning profile was present, and Xcode account/team registries were also absent.

Those legacy diagnostics remain evidence of the external environment at their exact times. They are not T064 acceptance evidence because the original live-test execution shape was not a provisioning-authorized app-like host.

### Exact remaining external prerequisite

The repository-side host shape is now defined. T064 still requires the trusted macOS runner to provide:

1. an unexpired macOS provisioning profile authorizing `dev.ecra.identity.t064` and the concrete keychain access group used by the signed host; and
2. a matching Apple Development code-signing identity for the same developer team.

Configuring a valid Apple developer account/team in Xcode and allowing Xcode to create/install the development certificate and suitable profile is one normal way to obtain these assets. Equivalent authorized installation of the matching certificate/profile is also acceptable. Xcode preference-registry presence is diagnostic, not the security claim.

After those assets exist, the permanent workflow must pass both ignored Data Protection Keychain tests through the signed app-like host on one exact feature head. Possession of the assets alone does not satisfy T064.

### Prohibited shortcuts

Do not unblock T064 by:

- using the legacy file-based keychain instead of Data Protection Keychain;
- setting `synchronizing=true`;
- introducing plaintext/file/environment/memory fallback;
- treating ad-hoc signing as provisioning-authorized signing;
- running the final acceptance tests from an unprovisioned bare command-line/test executable;
- weakening the live native acceptance requirement;
- claiming Secure Enclave, hardware-backed, non-exportable, user-presence, cross-machine, recovery, or unverified platform guarantees.

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

T064's live tests are present but intentionally ignored in the ordinary workspace suite. They cover Data Protection Keychain store/open/delete for all v1 secret purposes and bootstrap → durable protected-state publish → authenticated reopen of the same local identity. They are accepted only when the signed app-like T064 harness runs them successfully.

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