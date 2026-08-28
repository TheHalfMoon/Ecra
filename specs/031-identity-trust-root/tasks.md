# Tasks: Identity, Trust Root & Sensitive Storage Foundations

**Feature:** ECR-031  
**Status:** TASKS_READY  
**Dependencies:** ECR-001/ECR-002 `CLOSED_CANONICAL`  
**Analyze:** Pass 2 `ZERO_BLOCKING_PLANNING_DRIFT_FOUND`; implementation branch requires exact green synchronized planning head.  
**Execution rule:** `[x]` requires the linked requirement/evidence, not merely compiling code. `VERIFIED_ON_BRANCH` is not `CLOSED_CANONICAL`.

## Phase 1 — Workspace, dependency and CI boundaries

- [ ] **T001** Re-verify exact candidate dependency versions, features, licenses, advisories and MSRV immediately before adoption; record accepted/rejected candidates. **Paths:** `specs/031-identity-trust-root/research.md`, `research/donor-license-ledger.md`. **FR-052, SC-015**
- [ ] **T002** Add one `crates/ecra-identity` workspace crate depending on `ecra-core` only initially; add crate-level `#![forbid(unsafe_code)]` and architecture/misuse docs. **Paths:** `Cargo.toml`, `crates/ecra-identity/Cargo.toml`, `crates/ecra-identity/src/lib.rs`, `crates/ecra-identity/README.md`. **FR-001, FR-047, FR-048, FR-053**
- [ ] **T003** Add only accepted minimal crypto dependencies/features; do not add Windows/Linux platform crates unless the corresponding backend is implemented in this v1 branch. **Paths:** `crates/ecra-identity/Cargo.toml`, `Cargo.lock`. **FR-052**
- [ ] **T004** Add `scripts/check-identity-unsafe.sh` proving no Ecra-authored unsafe and documenting reviewed native dependency boundary. **Path:** `scripts/check-identity-unsafe.sh`. **FR-053, SC-014**
- [ ] **T005** Add `scripts/check-identity-deps.sh` allowlisting only core/serialization/crypto/native-backend dependencies and rejecting model/browser/network/protocol/policy crates. **Path:** `scripts/check-identity-deps.sh`. **FR-047, FR-048, SC-014**
- [ ] **T006** Add trusted push-only `.github/workflows/ecr-031.yml` for `031-identity-trust-root` and `main`, preserving ECR-001/ECR-002 regressions and self-hosted trust posture. **Path:** `.github/workflows/ecr-031.yml`. **SC-013, SC-014**
- [ ] **T007** Add explicit ECR-031 bootstrap/contract/validation/issuance/lifecycle/envelope/anchor/redaction/backend/macOS targets to the workflow; no generic workspace-only green gate. **Path:** `.github/workflows/ecr-031.yml`. **SC-001–SC-016**
- [ ] **T008** Add dependency/toolchain evidence output and `cargo tree -p ecra-identity` to CI. **Path:** `.github/workflows/ecr-031.yml`. **FR-052, SC-015**
- [ ] **T009** Verify the first workspace/dependency head passes build/fmt/Clippy/tests/rustdoc/offline plus ECR-001/ECR-002 gates before semantic implementation. **Path:** `specs/031-identity-trust-root/STATUS.md`. **SC-013**
- [ ] **T010** Record exact first-head dependency/license/native-boundary disposition before moving to primitives. **Paths:** `research/donor-license-ledger.md`, `specs/031-identity-trust-root/STATUS.md`. **SC-015**

## Phase 2 — IDs, errors, algorithms and strict wire primitives

- [ ] **T011** Implement typed `TrustRootId`, `KeyId`, `ProtectedObjectId`, enrollment ID and any required bounded nonce/delegation ID using existing typed-ID conventions. **Path:** `crates/ecra-identity/src/ids.rs`. **FR-015**
- [ ] **T012** Add compile-fail/type tests proving `ActorId`, `PrincipalId`, `TrustRootId`, `KeyId` and `ProtectedObjectId` are not interchangeable. **Paths:** `crates/ecra-identity/src/lib.rs`, `crates/ecra-identity/tests/validation.rs`. **FR-001, FR-013, SC-003**
- [ ] **T013** Implement typed `IdentityErrorCategory`, `IdentityErrorCode`, redacted error payloads and safe formatting including bootstrap/enrollment/issuer-session/trust-snapshot failures. **Path:** `crates/ecra-identity/src/error.rs`. **FR-049, FR-050**
- [ ] **T014** Implement closed `SignatureAlgorithm`, `AeadAlgorithm`, `KeyPurpose`, `KeyStatus`, `ProtectedPurpose` and backend-kind enums with unsupported-value rejection; v1 assertion/anchor signing allowlist is Ed25519. **Paths:** `crates/ecra-identity/src/algorithm.rs`, `crates/ecra-identity/src/key.rs`, `crates/ecra-identity/src/envelope.rs`, `crates/ecra-identity/src/backend.rs`. **FR-016, FR-017, FR-029, FR-041**
- [ ] **T015** Implement strict version parsing/compatibility helper for ECR-031 persisted/wire types without weakening ECR-001 version rules. **Paths:** `crates/ecra-identity/src/lib.rs`, contract tests. **FR-010, FR-031, FR-054**
- [ ] **T016** Implement gross byte/depth/count/parser limits before expensive crypto/materialization, including protected trust-state key/revocation counts. **Paths:** `crates/ecra-identity/src/assertion.rs`, `crates/ecra-identity/src/key.rs`, `crates/ecra-identity/src/envelope.rs`. **FR-031**
- [ ] **T017** Add valid/invalid primitive fixtures including unknown/duplicate fields, nil/invalid IDs, unsupported algorithms/versions and size/depth/count breaches. **Paths:** `contracts/ecra-identity-v1/valid/`, `contracts/ecra-identity-v1/invalid/`. **SC-001, SC-005**
- [ ] **T018** Implement repository-aligned RFC 8785 JCS helpers/domain-separated SHA-256 only by reusing existing canonicalization where possible; do not create competing JCS semantics. **Paths:** `crates/ecra-identity/src/assertion.rs`, `crates/ecra-identity/src/anchor.rs`. **FR-051**
- [ ] **T019** Add fixed canonical byte/digest goldens for assertion payload and protected-anchor input. **Paths:** `contracts/ecra-identity-v1/expected/`, `crates/ecra-identity/tests/assertion_contract.rs`, `crates/ecra-identity/tests/anchor.rs`. **FR-051, SC-001, SC-010**
- [ ] **T020** Exact-head Phase 2 CI and status ledger update. **Path:** `specs/031-identity-trust-root/STATUS.md`. **SC-013**

## Phase 3 — Identity assertion, bootstrap interfaces, issuance and deterministic validation

- [ ] **T021** Implement strict `EnrollmentRecord`, opaque `EnrolledPrincipalHandle`, `IssuerSession`, `AssertionIssuer`, `ActorBinding`, `AssertionAudience`, bounded assertion attributes and optional `OnBehalfOfBinding`. Handles/sessions are non-serializable and carry no authority. **Paths:** `crates/ecra-identity/src/bootstrap.rs`, `crates/ecra-identity/src/issuance.rs`, `crates/ecra-identity/src/assertion.rs`. **FR-003–FR-006, FR-012, FR-013, FR-058**
- [ ] **T022** Implement `IdentityAssertionV1` reusing ECR-001 `IdentityAssertionId`/`PrincipalId` and excluding signature from canonical signed payload. **Path:** `crates/ecra-identity/src/assertion.rs`. **FR-001–FR-003, FR-051**
- [ ] **T023** Implement `IdentityAssertionDigest` as an ECR-031-specific domain-separated digest distinct from generic/action/ledger digests. **Path:** `crates/ecra-identity/src/assertion.rs`. **FR-037, FR-051**
- [ ] **T024** Implement `IdentityValidationContext` with explicit evaluated time, expected actor/audience/principal and replay input; no ambient clock/environment access. **Path:** `crates/ecra-identity/src/validation.rs`. **FR-006–FR-008, FR-014**
- [ ] **T025** Implement `VerifiedTrustSnapshot` creation interface that accepts only authenticated protected trust state plus validated lifecycle invariants; pure validator must not accept ordinary unsigned metadata or make filesystem/native calls. **Paths:** `crates/ecra-identity/src/validation.rs`, `crates/ecra-identity/src/key.rs`. **FR-009, FR-014, FR-021**
- [ ] **T026** Implement v1 Ed25519 software signing/verification primitives and assertion issuance through `IssuerSession`; no API may mint for caller-selected arbitrary `PrincipalId`, and signing uses only the session's active purpose key. **Paths:** `crates/ecra-identity/src/assertion.rs`, `crates/ecra-identity/src/issuance.rs`, `crates/ecra-identity/src/validation.rs`. **FR-009–FR-012, FR-016, FR-020, FR-058**
- [ ] **T027** Implement exact principal/actor/audience/time binding and reject any mismatch. **Path:** `crates/ecra-identity/src/validation.rs`. **FR-004, FR-006, FR-007, FR-011**
- [ ] **T028** Implement bounded on-behalf-of identity binding; missing means no delegation claim, v1 issuer session cannot substitute another principal, and validation never grants capability authority. **Paths:** `crates/ecra-identity/src/assertion.rs`, `crates/ecra-identity/src/issuance.rs`, `crates/ecra-identity/src/validation.rs`. **FR-005, FR-012**
- [ ] **T029** Implement replay-mode validation from explicit caller-supplied replay state; single-use nonce reuse rejects when that assertion class is used. **Path:** `crates/ecra-identity/src/validation.rs`. **FR-008**
- [ ] **T030** Implement `ValidatedIdentityContext` containing only identity/trust evidence and no capability/approval/declassification/authorization fields. **Path:** `crates/ecra-identity/src/validation.rs`. **FR-012, FR-058**
- [ ] **T031** Add full invalid assertion/issuance/trust-snapshot corpus: wrong signature/issuer/key/subject/actor/audience/time/delegation/replay/revoked key, arbitrary-principal mint attempt, issuance without enrolled handle/session, unsigned/stale lifecycle metadata, malformed fields and unsupported version. **Paths:** `contracts/ecra-identity-v1/invalid/`, `crates/ecra-identity/tests/assertion_contract.rs`, `crates/ecra-identity/tests/validation.rs`, `crates/ecra-identity/tests/issuance.rs`. **FR-009–FR-014, FR-021, SC-001**
- [ ] **T032** Add deterministic property test validating identical assertion+verified trust snapshot+context 1,000 times to identical canonical validated-context bytes/digest. **Path:** `crates/ecra-identity/tests/validation.rs`. **FR-014, SC-002**
- [ ] **T033** Add architecture tests proving labels/usernames/emails/paths/protocol strings cannot become PrincipalId, arbitrary caller IDs cannot mint assertions, and `IssuerSession`/validated context expose no authority semantics. **Path:** `crates/ecra-identity/tests/backend_boundaries.rs`. **FR-012, FR-013, FR-058**
- [ ] **T034** Exact-head Phase 3 CI and status ledger update. **Path:** `specs/031-identity-trust-root/STATUS.md`. **SC-001–SC-003, SC-013**

## Phase 4 — Local bootstrap, protected trust state and key lifecycle

- [ ] **T035** Implement strict `TrustRootRecord`, `KeyRecord`, `ProtectedTrustStateV1` and local enrollment invariants with no serialized private/root/symmetric key material; local principal IDs are generated, never derived from OS/user labels. **Paths:** `crates/ecra-identity/src/bootstrap.rs`, `crates/ecra-identity/src/key.rs`. **FR-015–FR-018, FR-021, FR-024**
- [ ] **T036** Implement one-active-key-per-trust-root/purpose selection inside authenticated protected trust state and reject ambiguous/duplicate active generations. **Path:** `crates/ecra-identity/src/key.rs`. **FR-018, FR-021**
- [ ] **T037** Implement rotate transition: create/protect next generation, activate it and atomically publish new protected trust state; prior active becomes retired according to purpose. **Paths:** `crates/ecra-identity/src/key.rs`, `crates/ecra-identity/src/store.rs`. **FR-019, FR-021**
- [ ] **T038** Implement retirement semantics blocking new signing/protection while permitting only contract-authorized historical verification/decryption. **Path:** `crates/ecra-identity/src/key.rs`. **FR-017–FR-020**
- [ ] **T039** Implement revocation semantics in protected trust state blocking new use/current assertion validation; distinguish revocation from unavailable/destroyed key and reject ordinary metadata attempts to unrevoke/reactivate. **Paths:** `crates/ecra-identity/src/key.rs`, `crates/ecra-identity/src/validation.rs`, `crates/ecra-identity/src/store.rs`. **FR-020, FR-021**
- [ ] **T040** Add exhaustive lifecycle/bootstrap tests including first enrollment, crash before/after backend secret creation and before/after protected-state publish, incomplete-bootstrap recovery, invalid second bootstrap, generation collision, stale-key issuance, revoked-key validation, stale/unsigned metadata and explicit no-monotonic-rollback-overclaim fixture. **Paths:** `crates/ecra-identity/tests/bootstrap.rs`, `crates/ecra-identity/tests/key_lifecycle.rs`. **SC-004, SC-008**
- [ ] **T041** Implement the ECR-031-owned versioned protected trust-state store using authenticated envelope + crash-safe atomic replacement; ordinary metadata is rebuildable/non-authoritative. Add migration/corruption fixtures; do not reuse ECR-002 tables as identity authority. **Paths:** `crates/ecra-identity/src/store.rs`, `contracts/ecra-identity-v1/migrations/`, `crates/ecra-identity/tests/migration.rs`. **FR-021, FR-054**
- [ ] **T042** Exact-head Phase 4 CI and status ledger update. **Path:** `specs/031-identity-trust-root/STATUS.md`. **SC-004, SC-008, SC-013**

## Phase 5 — Sensitive byte handling and protected envelopes

- [ ] **T043** Implement redacted/zeroizing `SensitiveBytes` wrapper for backend-protected software signing/master secrets without claiming process/OS-wide memory secrecy. **Paths:** `crates/ecra-identity/src/envelope.rs`, `crates/ecra-identity/tests/redaction.rs`. **FR-032, SC-009**
- [ ] **T044** Implement production `SecureRandom` boundary using accepted system CSPRNG dependency; deterministic provider remains test-only and supplies bootstrap IDs, signing seeds and nonces. **Paths:** `crates/ecra-identity/src/backend.rs`, tests. **FR-025, FR-029**
- [ ] **T045** Implement strict `ProtectedEnvelopeV1`, `EnvelopeKeyRef`, purpose/classification and exact AAD generation. **Path:** `crates/ecra-identity/src/envelope.rs`. **FR-026–FR-028, FR-031**
- [ ] **T046** Implement HKDF-SHA-256 domain-separated derivation over the native-backend-protected v1 master secret materialized only for bounded crypto use; no hardware-non-exportability claim. **Path:** `crates/ecra-identity/src/envelope.rs`. **FR-027–FR-029**
- [ ] **T047** Implement ChaCha20-Poly1305 RFC 8439 protection with 96-bit unique nonce ownership and full tag. **Path:** `crates/ecra-identity/src/envelope.rs`. **FR-029**
- [ ] **T048** Implement authenticated open returning no plaintext on version/algorithm/key/AAD/nonce/ciphertext/tag failure. **Path:** `crates/ecra-identity/src/envelope.rs`. **FR-030, FR-031, FR-050**
- [ ] **T049** Add RFC/dependency vectors plus Ecra canonical envelope goldens with deterministic injected test randomness. **Paths:** `contracts/ecra-identity-v1/expected/`, `crates/ecra-identity/tests/envelope.rs`. **SC-005, SC-006**
- [ ] **T050** Add mutation/property corpus for every authenticated envelope component and wrong-key/AAD/nonce/tag behavior. **Path:** `crates/ecra-identity/tests/envelope.rs`. **SC-005, SC-006**
- [ ] **T051** Add synthetic at-rest sentinel scan proving ordinary ECR-031 persisted fixture bytes do not contain plaintext secret outside intentional test input source. **Paths:** `crates/ecra-identity/tests/envelope.rs`, `crates/ecra-identity/tests/redaction.rs`. **FR-033, SC-007**
- [ ] **T052** Add redaction/log/error tests proving sentinel signing/master/private/secret bytes absent from Debug/Display/errors/backend capability/persisted metadata. **Path:** `crates/ecra-identity/tests/redaction.rs`. **FR-032, FR-049, FR-050, SC-009**
- [ ] **T053** Exact-head Phase 5 CI and status ledger update. **Path:** `specs/031-identity-trust-root/STATUS.md`. **SC-005–SC-009, SC-013**

## Phase 6 — Protected authenticity anchor

- [ ] **T054** Implement strict `ProtectedAnchorV1`, purpose enum and domain-separated canonical input. **Path:** `crates/ecra-identity/src/anchor.rs`. **FR-036–FR-038**
- [ ] **T055** Implement v1 Ed25519 protected-anchor signing using purpose-specific software key material protected by `TrustBackend`, active lifecycle checks and bounded redacted secret materialization. **Paths:** `crates/ecra-identity/src/anchor.rs`, `crates/ecra-identity/src/backend.rs`. **FR-036–FR-038**
- [ ] **T056** Add mutation/golden tests and compile/type tests proving anchor != generic digest != `VerificationReceipt`. **Path:** `crates/ecra-identity/tests/anchor.rs`. **FR-037, FR-038, FR-040, SC-010**
- [ ] **T057** Add bounded ECR-002 ledger-head digest fixture as an anchor payload example without changing `LedgerDigest` bytes/store semantics. **Paths:** `crates/ecra-identity/tests/anchor.rs`, `contracts/ecra-identity-v1/valid/`. **FR-039, FR-040**
- [ ] **T058** Exact-head Phase 6 CI and status ledger update. **Path:** `specs/031-identity-trust-root/STATUS.md`. **SC-010, SC-013**

## Phase 7 — Native TrustBackend and macOS v1 acceptance

- [ ] **T059** Implement minimal `TrustBackend` trait and `TrustBackendCapabilities`; no public raw private-key export and no platform-native type leakage. Bounded protected-secret open is internal to trusted crypto operations. **Path:** `crates/ecra-identity/src/backend.rs`. **FR-022–FR-025, FR-041, FR-045**
- [ ] **T060** Implement explicit production backend selection so memory/plaintext/environment/file-key are impossible production choices; test backend is test-only. **Paths:** `crates/ecra-identity/src/backend.rs`, `crates/ecra-identity/tests/backend_boundaries.rs`. **FR-023, FR-025, SC-008**
- [ ] **T061** Implement macOS Data Protection Keychain backend using accepted minimal binding dependency; local-only/non-synchronizing protection for root/master/Ed25519 software signing secrets. **Path:** `crates/ecra-identity/src/platform/macos.rs`. **FR-022, FR-041, FR-042**
- [ ] **T062** Implement macOS unavailable/locked/not-found/delete normalization with redacted typed errors; never plaintext fallback. **Paths:** `crates/ecra-identity/src/platform/macos.rs`, `crates/ecra-identity/src/error.rs`. **FR-022, FR-023, FR-049, FR-050**
- [ ] **T063** Freeze/report portable v1 macOS signing assurance: software Ed25519 secret protected by Keychain at rest; `hardware_backed_private_operations=false` and `non_exportable_private_key=false`; no Secure Enclave/user-presence signing claim. **Paths:** `crates/ecra-identity/src/platform/macos.rs`, `crates/ecra-identity/tests/backend_boundaries.rs`, `specs/031-identity-trust-root/STATUS.md`. **FR-042, FR-045, FR-046**
- [ ] **T064** Add live trusted-macOS backend tests using unique synthetic test namespace: store/open/delete root/master/signing secret, complete bootstrap/reopen, local-only configuration and exact capability assertions. **Path:** `crates/ecra-identity/tests/macos_backend.rs`. **SC-008, SC-011**
- [ ] **T065** Add Windows backend contract/status: implement DPAPI only if exact dependency/native tests become available in this slice; otherwise compile/runtime status explicit unsupported/unverified with no fallback/cross-machine/hardware-signing claim. **Paths:** `crates/ecra-identity/src/platform/windows.rs`, `specs/031-identity-trust-root/STATUS.md`. **FR-043, FR-045, SC-012**
- [ ] **T066** Add Linux backend contract/status: implement Secret Service only if exact dependency/live tests become available; otherwise explicit unsupported/unverified. Any implementation keeps secret material out of lookup attributes. **Paths:** `crates/ecra-identity/src/platform/linux.rs`, `crates/ecra-identity/tests/backend_boundaries.rs`, `STATUS.md`. **FR-034, FR-044, FR-045, SC-012**
- [ ] **T067** Add source/architecture tests ensuring Windows/Linux unverified status cannot render as verified/hardware-backed and portable macOS Ed25519 cannot render as Secure Enclave/non-exportable signing. **Path:** `crates/ecra-identity/tests/backend_boundaries.rs`. **FR-045, FR-046, SC-012**
- [ ] **T068** Exact-head Phase 7 CI including live macOS native target and status ledger update. **Path:** `specs/031-identity-trust-root/STATUS.md`. **SC-008, SC-011–SC-014**

## Phase 8 — Cross-cutting hostile input, compatibility and documentation

- [ ] **T069** Add arbitrary bounded-input property/fuzz-style tests for assertion/protected-trust-state/envelope parsers: no panic, limits before expensive work where practical. **Paths:** `crates/ecra-identity/tests/assertion_contract.rs`, `crates/ecra-identity/tests/key_lifecycle.rs`, `crates/ecra-identity/tests/envelope.rs`. **FR-031, SC-001, SC-005**
- [ ] **T070** Add portability tests for LF/CRLF/property order/whitespace around semantically equivalent JSON and identical JCS/digest/signature validation behavior. **Path:** `crates/ecra-identity/tests/portability.rs`. **FR-051, SC-002**
- [ ] **T071** Add architecture scan proving no model/browser/network/protocol/policy execution surface, no ECR-031 IPC/network issuer, and no `CapabilityGrant`/authorization output type in `ecra-identity`. **Path:** `crates/ecra-identity/tests/backend_boundaries.rs`. **FR-012, FR-047, SC-014**
- [ ] **T072** Add committed-fixture synthetic/non-sensitive audit and ensure generic `.ecra` artifacts do not receive raw ECR-031 secret/root/private-key values. **Paths:** `crates/ecra-identity/tests/redaction.rs`, ECR-031 README/threat model. **FR-032, FR-035, FR-055**
- [ ] **T073** Document exact backend assurance/non-claims, Ecra-local-vs-external identity boundary, rollback boundary, key compromise/revocation behavior, no sync/recovery, no NIST-certification claim and no compromised-OS guarantee. **Path:** `crates/ecra-identity/README.md`. **FR-046, FR-056, FR-057**
- [ ] **T074** Run full quickstart exact-head gate and capture toolchain/dependency/backend evidence. **Paths:** `specs/031-identity-trust-root/STATUS.md`, `research/donor-license-ledger.md`. **SC-013–SC-015**

## Phase 9 — Traceability, convergence, review and canonical closure

- [ ] **T075** Map FR-001–FR-058 and SC-001–SC-016 plus Pass-1 C1–C4 to implementation/tests/contracts; zero unowned requirements. **Path:** `specs/031-identity-trust-root/traceability-closure.md`. **SC-016**
- [ ] **T076** Re-check constitution G1–G15 and platform risks R-018/R-036/R-052/R-053/R-054 plus discovered ECR-031 bootstrap/issuance/rollback risks. **Path:** `traceability-closure.md`. **SC-016**
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
T021–T034 assertion/bootstrap interfaces/issuance/validation
        ↓
T035–T042 protected bootstrap/trust state/key lifecycle
        ↓
T043–T053 protected envelope + secret custody
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
