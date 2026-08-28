# ECR-031 Requirements Quality Checklist

**Purpose:** pre-implementation quality gate for Identity, Trust Root & Sensitive Storage Foundations.  
**Rule:** every item must be `[x]` before `TASKS_READY`; checklist success does not itself authorize implementation.

## Scope and ownership

- [x] ECR-031 purpose is bounded to identity assertions, actor/principal/on-behalf-of binding, trust root, key lifecycle, protected local storage and protected authenticity anchors.
- [x] ECR-003 owns general authorization/declassification/approval/secret-use mediation.
- [x] ECR-004 owns independent action outcome verification/reconciliation.
- [x] ECR-016 owns protocol authentication/token mapping; no token passthrough is ambient authority here.
- [x] ECR-021 local-model gateway is not pulled forward.
- [x] ECR-022/ECR-029 cross-device sync/recovery/export are excluded.
- [x] ECR-001 Actor/Principal/IdentityAssertion IDs and references are reused rather than replaced.

## Identity semantics

- [x] Actor attribution is explicitly non-authenticating.
- [x] Existence of `IdentityAssertionRef` is explicitly non-validating.
- [x] Assertion subject, actor, issuer, signing key, audience and validity are exact typed fields.
- [x] On-behalf-of binding is explicit and missing never means ANY/unrestricted.
- [x] Validation context includes explicit evaluation time and expected relying context.
- [x] Replay semantics are explicit where single-use assertions apply.
- [x] `ValidatedIdentityContext` is defined without capability/approval/authorization semantics.
- [x] Labels/usernames/emails/paths/protocol strings cannot become canonical principal identity.

## Cryptographic contract

- [x] Signed payload canonicalization is explicit and domain-separated.
- [x] Assertion digest is type/domain distinct from existing generic/action/ledger digests.
- [x] Signature algorithm is a closed allowlist, not attacker-selected arbitrary text.
- [x] Protected-envelope AEAD algorithm, key size, nonce size and full-tag requirement are explicit candidates.
- [x] Interpretation-critical envelope metadata is authenticated as AAD.
- [x] Nonce uniqueness ownership is explicit.
- [x] HKDF use is domain-separated and conditional on legitimate key-custody access to IKM.
- [x] Contract explicitly forbids extracting hardware-protected key material merely to fit the preferred KDF formula.
- [x] Protected anchor is distinct from `LedgerDigest` and `VerificationReceipt`.
- [x] Unsupported algorithms/versions fail closed.

## Key lifecycle

- [x] Key purpose and key status are typed and distinct.
- [x] One active key per trust-root/purpose invariant is explicit.
- [x] Rotation, retirement and revocation semantics are distinct.
- [x] Revocation is not conflated with destruction/unavailability.
- [x] Retired keys cannot create new material.
- [x] Current assertion validation rejects revoked assertion-signing keys.
- [x] Generic raw private/root key export is excluded from trusted v1 API.

## Native backend and platform honesty

- [x] Production backend unavailable/locked/unsupported fails closed.
- [x] Plaintext/file/environment/process-memory fallback is prohibited.
- [x] Test backend is required to be unreachable through ordinary production configuration.
- [x] Common backend capability metadata cannot overstate platform guarantees.
- [x] macOS Data Protection Keychain is the v1 native acceptance baseline.
- [x] Secure Enclave/user-presence is conditional on exact operation/evidence, not assumed for all Keychain data.
- [x] Windows default DPAPI is not described as cross-machine protection.
- [x] Linux Secret Service 0.2 is labeled DRAFT.
- [x] Linux lookup attributes are treated as non-secret and prohibited from carrying sensitive values.
- [x] Windows/Linux are not marked verified without native evidence.

## Secrets, privacy and persistence

- [x] Raw private/root/derived key bytes are excluded from serializable key metadata.
- [x] Sensitive plaintext and keys are excluded from Debug/Display/log/error/run-artifact paths.
- [x] At-rest synthetic sentinel scan is an acceptance criterion.
- [x] ECR-031 protected envelope is not blanket authorization for other slices to persist sensitive data.
- [x] Real disclosure/remote egress remains gated by ECR-003.
- [x] No hidden telemetry/cloud requirement is introduced.
- [x] Cross-device recovery and team governance are excluded from v1.

## Threat model

- [x] malicious model/content identity injection is addressed.
- [x] local unrelated/same-user process boundaries are distinguished.
- [x] filesystem/database tamper is addressed.
- [x] stale/revoked assertion/key threat is addressed.
- [x] assertion replay/audience confusion is addressed.
- [x] delegation escalation is addressed.
- [x] nonce reuse is identified as a critical protocol violation.
- [x] crypto-oracle error detail is addressed.
- [x] migration/downgrade attacks are addressed.
- [x] fully compromised same-user OS/kernel/debugger is outside general containment and not hidden.
- [x] platform-specific stronger guarantees require exact evidence.

## Bounded input/execution

- [x] strict assertion/envelope byte/depth/count limits are specified.
- [x] arbitrary recursive delegation/certificate chains are excluded from v1.
- [x] parser validation precedes expensive cryptography/materialization where practical.
- [x] no recursive model/tool/process loop exists in this slice.

## Testability and success criteria

- [x] every assertion mismatch family has a named negative test/fixture target.
- [x] deterministic validation repeated 1,000 times is required.
- [x] key lifecycle transition-table tests are exhaustive.
- [x] envelope mutation/wrong-key/AAD/tag/version tests are required.
- [x] redaction/no-secret-log tests are required.
- [x] production no-fallback tests are required.
- [x] protected-anchor mutation/type-distinction tests are required.
- [x] macOS live native test is required on trusted runner.
- [x] Windows/Linux verification status is evidence-dependent.
- [x] ECR-001/ECR-002 regression gates remain mandatory.
- [x] dependency/unsafe/source-I/O boundary checks are required.

## Dependency/provenance planning

- [x] NIST SP 800-63-4/B-4/C-4 primary references are recorded.
- [x] NIST SP 800-57 Part 1 Rev.5 is recorded for key management.
- [x] RFC 8439/5869/8032 references are recorded.
- [x] Apple Keychain/Secure Enclave primary docs are recorded.
- [x] Microsoft DPAPI primary docs are recorded.
- [x] Freedesktop Secret Service current draft is recorded.
- [x] candidate Rust dependency versions are listed in quickstart for re-verification before adoption.
- [x] no donor source-copy claim is made.
- [x] exact license/advisory/features review remains T001 before dependency adoption.

## Constitution G1–G15

- [x] G1 Domain coherence — one identity model; ECR-001 IDs reused.
- [x] G2 Authority — identity output carries no authority.
- [x] G3 Provenance — issuer/root/key/digest explicit.
- [x] G4 Side effects — local key/storage mutations identified and testable.
- [x] G5 Verification — crypto validation is not ECR-004 outcome verification.
- [x] G6 Durability — lifecycle/protected metadata persistence rules explicit if store is needed.
- [x] G7 Privacy/secrets — no fallback, redacted paths, protected storage.
- [x] G8 Local-first — no cloud account/provider.
- [x] G9 Interoperability — external/native systems remain adapters.
- [x] G10 Donor/license — references recorded; exact implementation dependency lock is gated before adoption.
- [x] G11 Browser/upstream — N/A, no browser patch.
- [x] G12 Benchmark claims — reproducible security acceptance only; no superiority claim.
- [x] G13 Information flow — no remote egress; later disclosure remains gated.
- [x] G14 Identity/principal binding — explicit owning slice with fail-closed semantics.
- [x] G15 Bounded execution — strict limits/no unbounded chain/loops.

## Quality conclusion

```text
AMBIGUOUS_MUST_REQUIREMENTS=0
UNOWNED_SECURITY_BOUNDARIES=0
IMPLICIT_AUTHORITY_SHORTCUTS=0
PLAINTEXT_FALLBACK_PATHS_AUTHORIZED=0
UNBOUNDED_ASSERTION_OR_DELEGATION_STRUCTURES=0
PLATFORM_ASSURANCE_OVERCLAIMS_AUTHORIZED=0
CHECKLIST_RESULT=PASS_FOR_ANALYZE
```
