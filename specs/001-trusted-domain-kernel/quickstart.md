# Quickstart / Verification Guide: ECR-001 Trusted Domain Kernel

This guide verifies ECR-001 once implementation exists. It requires no browser, model API key, cloud account, database, secret store, network service, or external process runtime.

## Prerequisites

- repository-pinned Rust toolchain;
- Git;
- dependencies already fetched/cached for offline test;
- no external credentials.

## Expected Slice

```text
Cargo.toml
Cargo.lock
rust-toolchain.toml
crates/ecra-core/
contracts/ecra-domain-v1/
specs/001-trusted-domain-kernel/
```

## Build / Formatting / Lints

```bash
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
```

Expected: one ECR-001 production crate, zero Ecra-owned warnings unless an explicit reviewed lint exception exists.

## Test Suite

```bash
cargo test --workspace --locked
cargo test -p ecra-core --test contract_fixtures --locked
cargo test -p ecra-core --test invalid_fixtures --locked
cargo test -p ecra-core --test canonicalization --locked
cargo test -p ecra-core --test action_digest --locked
cargo test -p ecra-core --test properties --locked
cargo test --doc --workspace --locked
```

Expected:
- all valid fixtures round-trip;
- all invalid fixtures fail with expected machine-readable category/code;
- canonical fixtures are byte-stable under RFC 8785;
- ActionDigest fixtures are exact/stable;
- property/type-confusion tests preserve constitutional invariants.

## Offline / Zero-I/O Gate

After dependencies are available:

```bash
cargo test --workspace --locked --offline
```

Expected: tests require no network, browser, model, DB, filesystem service, keychain/secret store, remote clock or spawned external process.

## Unsafe Gate

```bash
rg -n "unsafe\s*\{" crates/ecra-core
rg -n "allow\(unsafe_code\)|unsafe_code" crates/ecra-core
```

Expected: no unsafe block or allowance; crate root forbids unsafe code.

## Dependency Boundary Gate

```bash
cargo tree -p ecra-core
```

Plus the repository dependency-policy script/check created by tasks.

Production graph MUST NOT include async runtime, HTTP/network, database/storage driver, browser/CDP/WebDriver, model SDK, policy engine, MCP/ACP/A2A, process/filesystem execution abstraction or telemetry exporter.

Pure serialization/UUID/URL/JCS/SHA-256/error dependencies must match reviewed donor/license records.

## Normative Fixture Review

Minimum fixture classes are defined by:

```text
specs/001-trusted-domain-kernel/contracts/domain-v1.md
```

Reviewers must sample actual JSON and expected canonical/digest outputs. Fixtures are contract artifacts, not disposable test data.

## Security-Sensitive Manual Checks

Before closure, demonstrate from tests/fixtures:

1. `ActorId` cannot implicitly become `PrincipalId`; actor attribution is not authentication.
2. `CapabilityRequestId`/`CapabilityRequest` cannot implicitly become CapabilityGrantId/CapabilityGrant.
3. `ScopeConstraint.one_of([])` is rejected and missing/empty scope does not become `any_explicit`.
4. unrestricted scope is represented explicitly in canonical JSON.
5. Resource locator/free-form text is documented/tested non-authoritative.
6. web/model/tool/memory text cannot grant permission or alter scope/origin/identity by content.
7. private/sensitive/secret classifications round-trip and `unknown` never normalizes to public.
8. a source→remote-provider `InformationUse` is representable but cannot become a CapabilityGrant/authorization object.
9. Fact has no independent verified truth flag; a model-inferred Fact remains model-inferred while a separate VerificationReceipt can be VERIFIED.
10. freshness includes an inspectable assessment/basis when provided.
11. local irreversible mutation and external reversible mutation are representable without conflating location/reversibility.
12. keyed idempotency without a key is rejected.
13. non-idempotent or unknown idempotency + unconditional safe retry is rejected.
14. exact normative `ActionDigest` matches expected SHA-256/JCS/domain-separated fixture.
15. changing every security-relevant ActionIntent field tested changes ActionDigest.
16. `ActionRef` with a wrong digest is rejected.
17. two attempts for one ActionIntent have distinct ActionAttemptIds.
18. every ActionReceipt binds exact ActionRef + ActionAttemptId.
19. executor-observed success is not VERIFIED.
20. ActionReceipt cannot deserialize/cast as VerificationReceipt.
21. UNKNOWN receipt outcome round-trips unchanged.
22. unsupported version/unknown security-sensitive field fails typed compatibility handling.
23. canonicalization is a fixed point.
24. ContentDigest metadata is not accepted as an ActionDigest/security-binding type by implicit conversion.

## Constitution v1.1.0 Review

Confirm ECR-001's applicable G1–G15 plan results still match implementation. In particular:
- G2 explicit scope/no ambient authority representation;
- G5 single authoritative verification path;
- G13 information-flow/egress representation;
- G14 Actor/Principal separation;
- G15 N/A is still true because ECR-001 contains no recursive/runtime execution.

## Pre-Implementation Review Closure

Review:

```text
specs/000-ecra-platform/pre-implementation-review-2026-08-27.md
```

For every ECR-001-owned finding, the implementation report must identify exact code/test/fixture evidence. Findings owned by later slices must remain assigned/deferred; do not claim they are solved by ECR-001 types alone.

## Closure Evidence

Implementation PR/report includes:
- exact feature head SHA;
- toolchain version;
- changed-file list;
- fmt/Clippy results;
- test totals/results by test target;
- offline result;
- dependency-boundary evidence;
- unsafe-code evidence;
- valid/invalid fixture counts/classes;
- canonical ActionDigest fixture result(s);
- donor/license delta;
- FR-001–FR-055 + SC-001–SC-020 traceability matrix/result;
- constitution G1–G15 re-check;
- pre-implementation review remediation mapping;
- Spec Kit analyze-equivalent result;
- convergence tasks if any.

`CLOSED_CANONICAL` requires exact repository evidence; a local green statement is insufficient.
