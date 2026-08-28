# ECR-031 Quickstart / Verification Guide

**Status:** PLANNING_VERIFICATION_CONTRACT / PASS_1_REMEDIATED  
**Target:** exact implementation head only; branch success is not canonical closure.

## 1. Preconditions

Before implementation verification:
- ECR-031 package is `TASKS_READY` and analyze-clean;
- implementation is on the bounded ECR-031 branch/PR;
- exact `Cargo.lock` and donor/license delta are reviewed;
- no unresolved MUST-level implementation clarification exists;
- test fixtures are synthetic/non-sensitive;
- no test backend can be selected through production configuration.

## 2. Baseline workspace regression

```bash
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --workspace --locked
cargo test --workspace --locked --offline
```

## 3. Existing trusted-substrate gates

```bash
bash scripts/check-core-unsafe.sh
bash scripts/check-core-deps.sh
bash scripts/check-run-unsafe.sh
bash scripts/check-run-deps.sh
cargo test -p ecra-core --locked
cargo test -p ecra-run --locked
```

ECR-031 cannot weaken ECR-001/ECR-002.

## 4. ECR-031 boundary gates

```bash
bash scripts/check-identity-unsafe.sh
bash scripts/check-identity-deps.sh
cargo tree -p ecra-identity
```

Required properties:
- Ecra-authored trusted Rust forbids unsafe;
- no browser/model/network/protocol/policy engine dependencies;
- only reviewed crypto/native platform dependencies;
- native FFI remains in external reviewed bindings, not Ecra-authored unsafe blocks.

## 5. Bootstrap / enrollment target

```bash
cargo test -p ecra-identity --test bootstrap --locked
```

Must prove:
- initial `PrincipalId`, `TrustRootId`, enrollment and key IDs are generated as opaque Ecra-local IDs rather than derived from username/email/Actor labels;
- v1 claims only an Ecra-local installation principal, not external/legal/NIST identity proofing;
- bootstrap creates backend secret material + authenticated `ProtectedTrustStateV1` and is complete only after durable publish + authenticated reopen;
- crash before protected-state publication yields typed `incomplete_bootstrap` and never silently creates a second principal/root;
- orphan backend material is not accepted as enrollment identity;
- `EnrolledPrincipalHandle` can be obtained only from authenticated local enrollment state.

## 6. Contract and validation targets

```bash
cargo test -p ecra-identity --test assertion_contract --locked
cargo test -p ecra-identity --test validation --locked
cargo test -p ecra-identity --test portability --locked
```

Must prove:
- strict JSON/duplicate/unknown/version/limit rejection;
- canonical JCS payload bytes and fixed digest/signature fixture;
- actor/principal/audience/on-behalf-of/time/replay mismatch rejection;
- revoked/unknown/wrong signing key rejection;
- validation accepts only `VerifiedTrustSnapshot`, not ordinary unsigned lifecycle metadata;
- stale/unsigned/replaced metadata cannot reactivate/unrevoke a key;
- 1,000-run deterministic validation output;
- validated context contains no authority semantics.

## 7. Assertion issuance target

```bash
cargo test -p ecra-identity --test issuance --locked
```

Must prove:
- no public production API issues assertions from caller-provided arbitrary `PrincipalId`;
- `IssuerSession` requires an authenticated `EnrolledPrincipalHandle` plus current `VerifiedTrustSnapshot`;
- session subject principal/root are immutable and non-serializable;
- caller may request actor/audience binding but cannot substitute subject principal;
- v1 on-behalf-of issuance cannot mint a different principal;
- retired/revoked signing key cannot issue;
- no ECR-031 IPC/network assertion-minting service exists;
- `IssuerSession` is identity issuance context, not CapabilityGrant/authorization.

## 8. Key lifecycle / authoritative trust-state target

```bash
cargo test -p ecra-identity --test key_lifecycle --locked
cargo test -p ecra-identity --test migration --locked
```

Must prove exhaustive v1 transition table:
- one active key per purpose in authenticated protected state;
- rotate activates next generation and atomically publishes new protected trust state;
- retired key cannot create new material;
- revoked assertion signing key rejects current assertions;
- unavailable/destroyed is not fabricated revocation;
- ordinary DB/file metadata cannot activate/unrevoke a key;
- trust-state authentication failure returns no `VerifiedTrustSnapshot`;
- unsupported/corrupt trust-state versions fail closed;
- no accidental reactivation;
- restoring an older valid protected state is documented as outside universal monotonic rollback resistance rather than falsely detected by an unsigned counter.

## 9. Portable v1 signing custody target

The v1 assertion/protected-anchor suite is Ed25519 software signing with key material protected at rest by the selected native backend.

Tests must prove:
- Ed25519 seed/key generated through approved production CSPRNG;
- ordinary persisted metadata contains public material only;
- software signing secret is opened only through the backend and wrapped in redacted/zeroizing sensitive bytes for bounded use;
- formatted errors/logging do not expose it;
- portable v1 capability metadata does **not** claim Secure Enclave, hardware-backed signing or non-exportability;
- any future native non-exportable signing path must be a separately versioned/evidenced algorithm suite.

## 10. Protected-envelope target

```bash
cargo test -p ecra-identity --test envelope --locked
```

Must prove:
- RFC 8439/selected dependency vectors where applicable;
- exact AAD binding;
- wrong key/AAD/nonce/ciphertext/tag/version/algorithm fail;
- no plaintext on authentication failure;
- nonce length/uniqueness ownership contract;
- parser byte/depth/count limits;
- deterministic test vectors only via injected test randomness;
- production path sources randomness through approved CSPRNG boundary;
- native-backend-protected master secret uses bounded/redacted process materialization and is not described as hardware non-exportable.

## 11. Protected-anchor target

```bash
cargo test -p ecra-identity --test anchor --locked
```

Must prove:
- exact domain separation;
- key/purpose/payload mutation rejection;
- type distinction from `ContentDigest`, `ActionDigest`, `LedgerDigest`, and `VerificationReceipt`;
- no ECR-004 outcome-verification claim;
- v1 Ed25519 signing custody follows the same native-backend-protected software-key rule.

## 12. Secret/redaction target

```bash
cargo test -p ecra-identity --test redaction --locked
```

Use unique synthetic sentinel bytes and prove they do not appear in:
- `Debug`/`Display` of secret wrappers;
- formatted public/internal errors intended for logging;
- backend capability output;
- persisted ordinary metadata;
- ordinary run artifacts;
- fixture files except intentional source/input literals explicitly audited by the test.

Zeroization tests may prove wrapper/drop behavior under the selected dependency, but documentation MUST NOT claim complete memory erasure against a compromised process/OS.

## 13. Backend-boundary target

```bash
cargo test -p ecra-identity --test backend_boundaries --locked
```

Must prove:
- production backend selection contains no memory/plaintext/environment/file-key fallback;
- unavailable/locked native backend returns typed failure;
- test backend cannot be selected by ordinary production configuration;
- capabilities are assurance metadata, not authority;
- platform SDK/native types do not leak into canonical assertion/envelope types;
- portable Ed25519 v1 reports `hardware_backed_private_operations=false` and `non_exportable_private_key=false` unless a different versioned native suite is explicitly selected and evidenced.

## 14. macOS native live acceptance

On the trusted repository-scoped macOS runner:

```bash
cargo test -p ecra-identity --test macos_backend --locked
```

Live test fixture requirements:
- unique test namespace/item;
- create/store/open/delete protected synthetic root/master/Ed25519 seed through Data Protection Keychain;
- assert local-only/non-synchronizing configuration used by v1;
- verify unavailable/lookup/deletion behavior;
- verify backend protection of portable software-key material at rest;
- do **not** describe this as Secure Enclave signing;
- clean up test items even on recoverable failure where possible.

Do not run persistent personal self-hosted native-key tests on untrusted fork PR code.

## 15. Windows/Linux acceptance truth

ECR-031 v1 does not permit a blanket `VERIFIED` claim for Windows/Linux without native evidence.

If an implementation is included:
- Windows: compile/fixture tests plus native DPAPI test evidence are required before verified status.
- Linux: compile/fixture tests plus live Secret Service test evidence are required before verified status.

Without such evidence, runtime/backend status is explicit `unsupported` or `unverified`; it MUST NOT silently fall back.

## 16. At-rest plaintext scan

For synthetic known secrets written through protected trust state/envelopes/native store, inspect ECR-031-owned ordinary persisted bytes and prove plaintext sentinels are absent.

This does not claim resistance to process-memory/kernel compromise or monotonic rollback resistance.

## 17. Hostile-input/fuzz/property gate

Required before closure:
- assertion/envelope/trust-state JSON parser property tests;
- malformed base64/length/depth/count corpus;
- signature/envelope/trust-state byte mutation corpus;
- no panic on arbitrary bounded input;
- no allocation/crypto work before configured gross input limits where practical.

## 18. Platform-source policy test

Linux backend test must reject attempts to place synthetic secret sentinel values in Secret Service lookup attributes.

Windows docs/tests must not represent default DPAPI as cross-machine protection.

macOS capability tests must not mark portable software Ed25519 signing as Secure Enclave/hardware-backed/non-exportable.

## 19. Dependency/provenance evidence

Record exact versions/features/licenses/advisories and `Cargo.lock` digest in ECR-031 status/closure ledger.

Candidate research versions as of 2026-08-28, to verify again immediately before adoption:

```text
ed25519-dalek        3.0.0
chacha20poly1305     0.11.0
hkdf                 0.13.0
sha2                 0.11.0
zeroize              1.9.0
getrandom            0.4.3
security-framework   3.7.0       macOS candidate
windows              0.62.2      Windows candidate, only if implemented
secret-service       5.1.0       Linux candidate, only if implemented
```

These are candidates, not authorization to add every dependency. T001 re-verifies current truth before adoption.

## 20. Review/convergence gate

Before merge:
1. map FR-001–FR-058 and SC-001–SC-016 to implementation/tests/contracts;
2. explicitly map C1 bootstrap, C2 authoritative trust snapshot/rollback, C3 issuance boundary and C4 software-signing custody to evidence;
3. re-run G1–G15;
4. run post-implementation analyze-equivalent review;
5. fold all implementation clarifications into primary spec/data model/contract/plan;
6. update `STATUS.md`, `EXECUTION.md`, platform status/roadmap, donor ledger;
7. run this entire quickstart on exact converged feature head;
8. process all actionable PR review threads/checks;
9. only then mark PR Ready.

## 21. Canonical closure gate

`CLOSED_CANONICAL` requires:
- exact expected feature head merged using non-rebase method;
- complete ECR-031 workflow success on canonical `main` merge state;
- ECR-001/ECR-002 regression workflows green;
- exact merge/post-merge evidence in closure ledger;
- no overclaim of unsupported platform backend assurance, external identity proofing, or rollback resistance.
