# Tasks: Identity, Trust Root & Sensitive Storage Foundations

**Feature:** ECR-031  
**Status:** PLANNING / TASKS_CANDIDATE  
**Dependencies:** ECR-001/ECR-002 `CLOSED_CANONICAL`  
**Execution rule:** `[x]` requires the linked requirement/evidence, not merely compiling code. `VERIFIED_ON_BRANCH` is not `CLOSED_CANONICAL`.

## Phase 1 — Workspace, dependency and CI boundaries

- [ ] **T001** Re-verify exact candidate dependency versions, features, licenses, advisories and MSRV immediately before adoption; record accepted/rejected candidates. **Paths:** `specs/031-identity-trust-root/research.md`, `research/donor-license-ledger.md`. **FR-052, SC-015**
- [ ] **T002** Add one `crates/ecra-identity` workspace crate depending on `ecra-core` only initially; add crate-level `#![forbid(unsafe_code)]` and architecture/misuse docs. **Paths:** `Cargo.toml`, `crates/ecra-identity/Cargo.toml`, `crates/ecra-identity/src/lib.rs`, `crates/ecra-identity/README.md`. **FR-001, FR-047, FR-048, FR-053**
- [ ] **T003** Add only accepted minimal crypto dependencies/features; do not add Windows/Linux platform crates unless the corresponding backend is implemented in this v1 branch. **Paths:** `crates/ecra-identity/Cargo.toml`, `Cargo.lock`. **FR-052**
- [ ] **T004** Add `scripts/check-identity-unsafe.sh` proving no Ecra-authored unsafe and documenting reviewed native dependency boundary. **Path:** `scripts/check-identity-unsafe.sh`. **FR-053, SC-014**
- [ ] **T005** Add `scripts/check-identity-deps.sh` allowlisting only core/serialization/crypto/native-backend dependencies and rejecting model/browser/network/protocol/policy crates. **Path:** `scripts/check-identity-deps.sh`. **FR-047, FR-048, SC-014**
- [ ] **T006** Add trusted push-only `.github/workflows/ecr-031.yml` for `031-identity-trust-root` and `main`, preserving ECR-001/ECR-002 regressions and self-hosted trust posture. **Path:** `.github/workflows/ecr-031.yml`. **SC-013, SC-014**
- [ ] **T007** Add explicit ECR-031 contract/lifecycle/envelope/anchor/redaction/backend/macOS targets to the workflow; no generic workspace-only green gate. **Path:** `.github/workflows/ecr-031.yml`. **SC-001–SC-016**
- [ ] **T008** Add dependency/toolchain evidence output and `cargo tree -p ecra-identity` to CI. **Path:** `.github/workflows/ecr-031.yml`. **FR-052, SC-015**
- [ ] **T009** Verify the first workspace/dependency head passes build/fmt/Clippy/tests/rustdoc/offline plus ECR-001/ECR-002 gates before semantic implementation. **Path:** `specs/031-identity-trust-root/STATUS.md`. **SC-013**
- [ ] **T010** Record exact first-head dependency/license/native-boundary disposition before moving to primitives. **Paths:** `research/donor-license-ledger.md`, `specs/031-identity-trust-root/STATUS.md`. **SC-015**

## Phase 2 — IDs, errors, algorithms and strict wire primitives

- [ ] **T011** Implement typed `TrustRootId`, `KeyId`, `ProtectedObjectId` and any required bounded nonce/delegation ID using existing typed-ID conventions. **Path:** `crates/ecra-identity/src/ids.rs`. **FR-015**
- [ ] **T012** Add compile-fail/type tests proving `ActorId`, `PrincipalId`, `TrustRootId`, `KeyId` and `ProtectedObjectId` are not interchangeable. **Paths:** `crates/ecra-identity/src/lib.rs`, `crates/ecra-identity/tests/validation.rs`. **FR-001, FR-013, SC-003**
- [ ] **T013** Implement typed `IdentityErrorCategory`, `IdentityErrorCode`, redacted error payloads and safe formatting. **Path:** `crates/ecra-identity/src/error.rs`. **FR-049, FR-050**
- [ ] **T014** Implement closed `SignatureAlgorithm`, `AeadAlgorithm`, `KeyPurpose`, `KeyStatus`, `ProtectedPurpose` and backend-kind enums with unsupported-value rejection. **Paths:** `crates/ecra-identity/src/algorithm.rs`, `crates/ecra-identity/src/key.rs`, `crates/ecra-identity/src/envelope.rs`, `crates/ecra-identity/src/backend.rs`. **FR-016, FR-017, FR-029, FR-041**
- [ ] **T015** Implement strict version parsing/compatibility helper for ECR-031 persisted/wire types without weakening ECR-001 version rules. **Paths:** `crates/ecra-identity/src/lib.rs`, contract tests. **FR-010, FR-031, FR-054**
- [ ] **T016** Implement gross byte/depth/count/parser limits before expensive crypto/materialization. **Paths:** `crates/ecra-identity/src/assertion.rs`, `crates/ecra-identity/src/envelope.rs`. **FR-031**
- [ ] **T017** Add valid/invalid primitive fixtures including unknown/duplicate fields, nil/invalid IDs, unsupported algorithms/versions and size/depth breaches. **Paths:** `contracts/ecra-identity-v1/valid/`, `contracts/ecra-identity-v1/invalid/`. **SC-001, SC-005**
- [ ] **T018** Implement repository-aligned RFC 8785 JCS helpers/domain-separated SHA-256 only by reusing existing canonicalization where possible; do not create competing JCS semantics. **Paths:** `crates/ecra-identity/src/assertion.rs`, `crates/ecra-identity/src/anchor.rs`. **FR-051**
- [ ] **T019** Add fixed canonical byte/digest goldens for assertion payload and protected-anchor input. **Paths:** `contracts/ecra-identity-v1/expected/`, `crates/ecra-identity/tests/assertion_contract.rs`, `crates/ecra-identity/tests/anchor.rs`. **FR-051, SC-001, SC-010**
- [ ] **T020** Exact-head Phase 2 CI and status ledger update. **Path:** `specs/031-identity-trust-root/STATUS.md`. **SC-013**

## Phase 3 — Identity assertion and deterministic validation

- [ ] **T021** Implement strict `AssertionIssuer`, `ActorBinding`, `AssertionAudience`, bounded assertion attributes and optional `OnBehalfOfBinding`. **Path:** `crates/ecra-identity/src/assertion.rs`. **FR-003–FR-006, FR-013**
- [ ] **T022** Implement `IdentityAssertionV1` reusing ECR-001 `IdentityAssertionId`/`PrincipalId` and excluding signature from canonical signed payload. **Path:** `crates/ecra-identity/src/assertion.rs`. **FR-001–FR-003, FR-051**
- [ ] **T023** Implement `IdentityAssertionDigest` as an ECR-031-specific domain-separated digest distinct from generic/action/ledger digests. **Path:** `crates/ecra-identity/src/assertion.rs`. **FR-037, FR-051**
- [ ] **T024** Implement `IdentityValidationContext` with explicit evaluated time, expected actor/audience/principal and replay input; no ambient clock/environment access. **Path:** `crates/ecra-identity/src/validation.rs`. **FR-006–FR-008, FR-014**
- [ ] **T025** Implement immutable trust/key validation snapshot interface needed by pure validator; do not make filesystem/native backend calls inside the pure validation path. **Paths:** `crates/ecra-identity/src/validation.rs`, `crates/ecra-identity/src/key.rs`. **FR-009, FR-014**
- [ ] **T026** Implement signature verification in the normative fail-closed validation order with exact issuer/key/algorithm binding. **Paths:** `crates/ecra-identity/src/assertion.rs`, `crates/ecra-identity/src/validation.rs`. **FR-009–FR-011**
- [ ] **T027** Implement exact principal/actor/audience/time binding and reject any mismatch. **Path:** `crates/ecra-identity/src/validation.rs`. **FR-004, FR-006, FR-007, FR-011**
- [ ] **T028** Implement bounded on-behalf-of identity binding; missing means no delegation claim and validation never grants capability authority. **Paths:** `crates/ecra-identity/src/assertion.rs`, `crates/ecra-identity/src/validation.rs`. **FR-005, FR-012**
- [ ] **T029** Implement replay-mode validation from explicit caller-supplied replay state; single-use nonce reuse rejects when that assertion class is used. **Path:** `crates/ecra-identity/src/validation.rs`. **FR-008**
- [ ] **T030** Implement `ValidatedIdentityContext` containing only identity/trust evidence and no capability/approval/declassification/authorization fields. **Path:** `crates/ecra-identity/src/validation.rs`. **FR-012, FR-058**
- [ ] **T031** Add full invalid assertion corpus: wrong signature/issuer/key/subject/actor/audience/time/delegation/replay/revoked key, malformed fields and unsupported version. **Paths:** `contracts/ecra-identity-v1/invalid/`, `crates/ecra-identity/tests/assertion_contract.rs`, `crates/ecra-identity/tests/validation.rs`. **FR-009–FR-014, SC-001**
- [ ] **T032** Add deterministic property test reducing/validating identical assertion+trust snapshot+context 1,000 times to identical canonical validated-context bytes/digest. **Path:** `crates/ecra-identity/tests/validation.rs`. **FR-014, SC-002**
- [ ] **T033** Add architecture test proving labels/usernames/emails/paths/protocol strings cannot enter principal binding API except as non-authoritative display data where explicitly typed. **Path:** `crates/ecra-identity/tests/backend_boundaries.rs`. **FR-013**
- [ ] **T034** Exact-head Phase 3 CI and status ledger update. **Path:** `specs/031-identity-trust-root/STATUS.md`. **SC-001–SC-003, SC-013**

## Phase 4 — Key lifecycle and trust metadata

- [ ] **T035** Implement strict `TrustRootRecord`, `KeyRecord`, generation and lifecycle invariants with no serialized private/root/symmetric key material. **Path:** `crates/ecra-identity/src/key.rs`. **FR-015–FR-018, FR-024**
- [ ] **T036** Implement one-active-key-per-trust-root/purpose selection and reject ambiguous/duplicate active generations. **Path:** `crates/ecra-identity/src/key.rs`. **FR-018**
- [ ] **T037** Implement rotate transition: create next generation, activate it, retire previous according to purpose; no deletion required for rotation. **Path:** `crates/ecra-identity/src/key.rs`. **FR-019**
- [ ] **T038** Implement retirement semantics blocking new signing/protection while permitting only contract-authorized historical verification/decryption. **Path:** `crates/ecra-identity/src/key.rs`. **FR-017–FR-020**
- [ ] **T039** Implement revocation semantics blocking new use and current identity assertion validation; distinguish revocation from unavailable/destroyed key. **Paths:** `crates/ecra-identity/src/key.rs`, `crates/ecra-identity/src/validation.rs`. **FR-020, FR-021**
- [ ] **T040** Add exhaustive transition-table tests including invalid reactivation, generation collision, stale-key issuance and revoked-key validation. **Path:** `crates/ecra-identity/tests/key_lifecycle.rs`. **SC-004**
- [ ] **T041** Decide from implementation evidence whether ECR-031 needs its own persistent metadata store; if yes, converge plan/contract first, then implement versioned crash-safe store/migrations without reusing ECR-002 tables as identity authority. **Paths if authorized:** `crates/ecra-identity/src/store.rs`, `contracts/ecra-identity-v1/migrations/`, `crates/ecra-identity/tests/migration.rs`, `implementation-clarifications.md`. **FR-021, FR-054**
- [ ] **T042** Exact-head Phase 4 CI and status ledger update. **Path:** `specs/031-identity-trust-root/STATUS.md`. **SC-004, SC-013**

## Phase 5 — Sensitive byte handling and protected envelopes

- [ ] **T043** Implement redacted/zeroizing `SensitiveBytes` wrapper using the accepted dependency without claiming process/OS-wide memory secrecy. **Paths:** `crates/ecra-identity/src/envelope.rs`, `crates/ecra-identity/tests/redaction.rs`. **FR-032, SC-009**
- [ ] **T044** Implement production `SecureRandom` boundary using accepted system CSPRNG dependency; deterministic provider remains test-only. **Paths:** `crates/ecra-identity/src/backend.rs`, tests. **FR-029, FR-025**
- [ ] **T045** Implement strict `ProtectedEnvelopeV1`, `EnvelopeKeyRef`, purpose/classification and exact AAD generation. **Path:** `crates/ecra-identity/src/envelope.rs`. **FR-026–FR-028, FR-031**
- [ ] **T046** Implement candidate HKDF-SHA-256 domain-separated derivation only if accepted key-custody design can legitimately expose IKM; otherwise stop and converge contract before alternative wrapping design. **Path:** `crates/ecra-identity/src/envelope.rs`. **FR-027–FR-029**
- [ ] **T047** Implement ChaCha20-Poly1305 RFC 8439 protection with 96-bit unique nonce ownership and full tag. **Path:** `crates/ecra-identity/src/envelope.rs`. **FR-029**
- [ ] **T048** Implement authenticated open returning no plaintext on version/algorithm/key/AAD/nonce/ciphertext/tag failure. **Path:** `crates/ecra-identity/src/envelope.rs`. **FR-030, FR-031, FR-050**
- [ ] **T049** Add RFC/dependency vectors plus Ecra canonical envelope goldens with deterministic injected test randomness. **Paths:** `contracts/ecra-identity-v1/expected/`, `crates/ecra-identity/tests/envelope.rs`. **SC-005, SC-006**
- [ ] **T050** Add mutation/property corpus for every authenticated envelope component and wrong-key/AAD/nonce/tag behavior. **Path:** `crates/ecra-identity/tests/envelope.rs`. **SC-005, SC-006**
- [ ] **T051** Add synthetic at-rest sentinel scan proving ordinary ECR-031 persisted fixture bytes do not contain plaintext secret outside intentional test input source. **Paths:** `crates/ecra-identity/tests/envelope.rs`, `crates/ecra-identity/tests/redaction.rs`. **FR-033, SC-007**
- [ ] **T052** Add redaction/log/error tests proving sentinel private/secret bytes absent from Debug/Display/errors/backend capability/persisted metadata. **Path:** `crates/ecra-identity/tests/redaction.rs`. **FR-032, FR-049, FR-050, SC-009**
- [ ] **T053** Exact-head Phase 5 CI and status ledger update. **Path:** `specs/031-identity-trust-root/STATUS.md`. **SC-005–SC-009, SC-013**

## Phase 6 — Protected authenticity anchor

- [ ] **T054** Implement strict `ProtectedAnchorV1`, purpose enum and domain-separated canonical input. **Path:** `crates/ecra-identity/src/anchor.rs`. **FR-036–FR-038**
- [ ] **T055** Implement accepted signing/MAC path using exact key-purpose/lifecycle checks and backend/software test fixture as authorized by dependency research. **Paths:** `crates/ecra-identity/src/anchor.rs`, `crates/ecra-identity/src/backend.rs`. **FR-036–FR-038**
- [ ] **T056** Add mutation/golden tests and compile/type tests proving anchor != generic digest != `VerificationReceipt`. **Path:** `crates/ecra-identity/tests/anchor.rs`. **FR-037, FR-038, FR-040, SC-010**
- [ ] **T057** Add bounded ECR-002 ledger-head digest fixture as an anchor payload example without changing `LedgerDigest` bytes/store semantics. **Paths:** `crates/ecra-identity/tests/anchor.rs`, `contracts/ecra-identity-v1/valid/`. **FR-039, FR-040**
- [ ] **T058** Exact-head Phase 6 CI and status ledger update. **Path:** `specs/031-identity-trust-root/STATUS.md`. **SC-010, SC-013**

## Phase 7 — Native TrustBackend and macOS v1 acceptance

- [ ] **T059** Implement minimal `TrustBackend` trait and `TrustBackendCapabilities`; no raw private-key export and no platform-native type leakage. **Path:** `crates/ecra-identity/src/backend.rs`. **FR-022–FR-025, FR-041, FR-045**
- [ ] **T060** Implement explicit production backend selection so memory/plaintext/environment/file-key are impossible production choices; test backend is test-only. **Paths:** `crates/ecra-identity/src/backend.rs`, `crates/ecra-identity/tests/backend_boundaries.rs`. **FR-023, FR-025, SC-008**
- [ ] **T061** Implement macOS Data Protection Keychain backend using accepted minimal binding dependency; local-only/non-synchronizing item configuration. **Path:** `crates/ecra-identity/src/platform/macos.rs`. **FR-022, FR-041, FR-042**
- [ ] **T062** Implement macOS unavailable/locked/not-found/delete normalization with redacted typed errors; never plaintext fallback. **Paths:** `crates/ecra-identity/src/platform/macos.rs`, `crates/ecra-identity/src/error.rs`. **FR-022, FR-023, FR-049, FR-050**
- [ ] **T063** If Secure Enclave/user-presence signing is feasible under the frozen v1 algorithm/product contract, implement it as a separate capability path; otherwise explicitly report capability false and do not weaken the contract. **Paths:** `crates/ecra-identity/src/platform/macos.rs`, `specs/031-identity-trust-root/STATUS.md`. **FR-042, FR-045, FR-046**
- [ ] **T064** Add live trusted-macOS backend tests using unique synthetic test namespace, create/open/protect/delete cleanup and exact capability assertions. **Path:** `crates/ecra-identity/tests/macos_backend.rs`. **SC-011**
- [ ] **T065** Add Windows backend contract/status: implement DPAPI only if exact dependency/native tests become available in this slice; otherwise compile/runtime status is explicit unsupported/unverified with no fallback or cross-machine claim. **Paths:** `crates/ecra-identity/src/platform/windows.rs`, `specs/031-identity-trust-root/STATUS.md`. **FR-043, FR-045, SC-012**
- [ ] **T066** Add Linux backend contract/status: implement Secret Service only if exact dependency/live tests become available; otherwise explicit unsupported/unverified. Any implementation must keep secret material out of lookup attributes. **Paths:** `crates/ecra-identity/src/platform/linux.rs`, `crates/ecra-identity/tests/backend_boundaries.rs`, `STATUS.md`. **FR-034, FR-044, FR-045, SC-012**
- [ ] **T067** Add source/architecture tests ensuring Windows/Linux unverified status cannot be rendered as verified/hardware-backed and macOS capability claims match exact backend configuration. **Path:** `crates/ecra-identity/tests/backend_boundaries.rs`. **FR-045, FR-046, SC-012**
- [ ] **T068** Exact-head Phase 7 CI including live macOS native target and status ledger update. **Path:** `specs/031-identity-trust-root/STATUS.md`. **SC-008, SC-011–SC-014**

## Phase 8 — Cross-cutting hostile input, compatibility and documentation

- [ ] **T069** Add arbitrary bounded-input property/fuzz-style tests for assertion/envelope parsers: no panic, limits before expensive work where practical. **Paths:** `crates/ecra-identity/tests/assertion_contract.rs`, `crates/ecra-identity/tests/envelope.rs`. **FR-031, SC-001, SC-005**
- [ ] **T070** Add portability tests for LF/CRLF/property order/whitespace around semantically equivalent JSON and identical JCS/digest/signature validation behavior. **Path:** `crates/ecra-identity/tests/portability.rs`. **FR-051, SC-002**
- [ ] **T071** Add architecture scan proving no model/browser/network/protocol/policy execution surface and no `CapabilityGrant`/authorization output type in `ecra-identity`. **Path:** `crates/ecra-identity/tests/backend_boundaries.rs`. **FR-012, FR-047, SC-014**
- [ ] **T072** Add committed-fixture synthetic/non-sensitive audit and ensure generic `.ecra` artifacts do not receive raw ECR-031 secret/root/private-key values. **Paths:** `crates/ecra-identity/tests/redaction.rs`, ECR-031 README/threat model. **FR-032, FR-035, FR-055**
- [ ] **T073** Document exact backend assurance/non-claims, key compromise/revocation behavior, no sync/recovery, no NIST-certification claim and no compromised-OS guarantee. **Path:** `crates/ecra-identity/README.md`. **FR-046, FR-056, FR-057**
- [ ] **T074** Run full quickstart exact-head gate and capture toolchain/dependency/backend evidence. **Paths:** `specs/031-identity-trust-root/STATUS.md`, `research/donor-license-ledger.md`. **SC-013–SC-015**

## Phase 9 — Traceability, convergence, review and canonical closure

- [ ] **T075** Map FR-001–FR-058 and SC-001–SC-016 to implementation/tests/contracts; zero unowned requirements. **Path:** `specs/031-identity-trust-root/traceability-closure.md`. **SC-016**
- [ ] **T076** Re-check constitution G1–G15 and platform risks R-018/R-036/R-052/R-053/R-054 plus discovered ECR-031 risks. **Path:** `traceability-closure.md`. **SC-016**
- [ ] **T077** Run post-implementation analyze-equivalent review; append convergence tasks for MUST drift instead of hiding it. **Path:** `specs/031-identity-trust-root/post-implementation-analyze.md`. **SC-016**
- [ ] **T078** Converge spec/research/data-model/contract/threat-model/plan/quickstart/tasks/status/EXECUTION/platform status/roadmap with exact implementation truth. **Paths:** ECR-031 package + platform lifecycle docs.
- [ ] **T079** Run complete exact-head ECR-031 CI on final feature head; require ECR-001/ECR-002 regressions green. **SC-013, SC-016**
- [ ] **T080** Move PR out of Draft only after exact-head gate; process all review/check/thread findings and require zero actionable blocker. **SC-016**
- [ ] **T081** Merge exact expected head with non-rebase method and require canonical-main ECR-031 + ECR-001/ECR-002 CI SUCCESS.
- [ ] **T082** Mark ECR-031 `CLOSED_CANONICAL` only after merge/post-merge evidence; update roadmap/status/EXECUTION and identify next dependency-eligible slice (expected ECR-003 plus already-eligible ECR-004 subject to live truth).

## Dependency graph

```text
T001–T010 foundation/dependencies/CI
        ↓
T011–T020 strict primitives/contracts
        ↓
T021–T034 assertion validation
        ↓
T035–T042 key lifecycle
        ↓
T043–T053 protected envelope
        ↓
T054–T058 protected anchor
        ↓
T059–T068 native backend/macOS acceptance
        ↓
T069–T074 cross-cutting gates
        ↓
T075–T082 convergence/closure
```

## Scope guard

Completing a partial ECR-031 phase does not unlock ECR-003. ECR-003 requires ECR-031 `CLOSED_CANONICAL`. ECR-004 is separately dependency-eligible from ECR-001/ECR-002 and must remain a distinct slice.
