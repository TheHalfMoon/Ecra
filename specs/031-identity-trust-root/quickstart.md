# ECR-031 Quickstart / Verification Guide

**Status:** PLANNING_VERIFICATION_CONTRACT  
**Target:** exact implementation head only; branch success is not canonical closure.

## 1. Preconditions

Before implementation verification:
- ECR-031 package is `TASKS_READY` and analyze-clean;
- implementation is on the bounded ECR-031 branch/PR;
- exact `Cargo.lock` and donor/license delta are reviewed;
- no unresolved MUST-level implementation clarification exists;
- test fixtures are synthetic/non-sensitive.

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

## 5. Contract and validation targets

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
- 1,000-run deterministic validation output;
- validated context contains no authority semantics.

## 6. Key lifecycle target

```bash
cargo test -p ecra-identity --test key_lifecycle --locked
```

Must prove exhaustive v1 transition table:
- one active key per purpose;
- rotate activates next generation;
- retired key cannot create new material;
- revoked assertion signing key rejects current assertions;
- unavailable/destroyed is not fabricated revocation;
- no accidental reactivation.

## 7. Protected-envelope target

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
- production path sources randomness through approved CSPRNG boundary.

## 8. Protected-anchor target

```bash
cargo test -p ecra-identity --test anchor --locked
```

Must prove:
- exact domain separation;
- key/purpose/payload mutation rejection;
- type distinction from `ContentDigest`, `ActionDigest`, `LedgerDigest`, and `VerificationReceipt`;
- no ECR-004 outcome-verification claim.

## 9. Secret/redaction target

```bash
cargo test -p ecra-identity --test redaction --locked
```

Use unique synthetic sentinel bytes and prove they do not appear in:
- `Debug`/`Display` of secret wrappers;
- formatted public/internal errors intended for logging;
- backend capability output;
- persisted metadata;
- ordinary run artifacts;
- fixture files except intentional source/input literals explicitly audited by the test.

Zeroization tests may prove wrapper/drop behavior under the selected dependency, but documentation MUST NOT claim complete memory erasure against a compromised process/OS.

## 10. Backend-boundary target

```bash
cargo test -p ecra-identity --test backend_boundaries --locked
```

Must prove:
- production backend selection contains no memory/plaintext/environment/file-key fallback;
- unavailable/locked native backend returns typed failure;
- test backend cannot be selected by ordinary production configuration;
- capabilities are data/assurance metadata, not authority;
- platform SDK/native types do not leak into canonical assertion/envelope types.

## 11. macOS native live acceptance

On the trusted repository-scoped macOS runner:

```bash
cargo test -p ecra-identity --test macos_backend --locked
```

Live test fixture requirements:
- use a unique test namespace/item;
- create/store/open/delete protected synthetic root/blob through the selected Data Protection Keychain path;
- assert local-only/non-synchronizing configuration used by v1;
- verify unavailable/lookup/deletion behavior;
- if Secure Enclave/user-presence signing is implemented, test and report it separately from ordinary Keychain protection;
- clean up test items even on recoverable failure where possible.

Do not run persistent personal self-hosted native-key tests on untrusted fork PR code.

## 12. Windows/Linux acceptance truth

ECR-031 v1 planning does not permit a blanket `VERIFIED` claim for Windows/Linux without native evidence.

If an implementation is included:
- Windows: compile/fixture tests plus native DPAPI test evidence are required before marking Windows backend verified.
- Linux: compile/fixture tests plus live Secret Service test evidence are required before marking Linux backend verified.

Without such evidence, the runtime/backend capability status must be explicit `unsupported` or `unverified` according to the frozen implementation contract; it MUST NOT silently fall back.

## 13. At-rest plaintext scan

For a synthetic known secret written through a protected envelope/native store, inspect the ECR-031-owned ordinary persisted bytes/files/SQLite fixtures and prove the plaintext sentinel is absent.

This does not claim resistance to process-memory/kernel compromise.

## 14. Hostile-input/fuzz/property gate

Required before closure:
- assertion/envelope JSON parser property tests;
- malformed base64/length/depth/count corpus;
- signature/envelope byte mutation corpus;
- no panic on arbitrary bounded input;
- no allocation/crypto work before configured gross input limits where practical.

## 15. Platform-source policy test

Linux backend test must explicitly reject attempts to place synthetic secret sentinel values in Secret Service lookup attributes.

Windows backend docs/tests must not represent default DPAPI as cross-machine protection.

macOS capability tests must not set hardware-backed/non-exportable flags true unless the exact live key path proves those properties.

## 16. Dependency/provenance evidence

Record exact versions/features/licenses and `Cargo.lock` digest in the ECR-031 status/closure ledger.

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

These are candidates, not authorization to add every dependency. Minimize features and omit platform crates not implemented in v1.

## 17. Review/convergence gate

Before merge:
1. map FR-001–FR-058 and SC-001–SC-016 to implementation/tests/contracts;
2. re-run G1–G15;
3. run post-implementation analyze-equivalent review;
4. fold all implementation clarifications into primary spec/data model/contract/plan;
5. update `STATUS.md`, `EXECUTION.md`, platform status/roadmap, donor ledger;
6. run this entire quickstart on the exact converged feature head;
7. process all actionable PR review threads/checks;
8. only then mark PR Ready.

## 18. Canonical closure gate

`CLOSED_CANONICAL` requires:
- exact expected feature head merged using non-rebase method;
- complete ECR-031 workflow success on canonical `main` merge state;
- ECR-001/ECR-002 regression workflows green;
- exact merge/post-merge evidence in closure ledger;
- no overclaim of unsupported platform backend assurance.
