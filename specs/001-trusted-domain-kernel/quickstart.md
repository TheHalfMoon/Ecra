# Quickstart / Verification Guide: ECR-001 Trusted Domain Kernel

This guide describes how a contributor or reviewer verifies the ECR-001 slice once implementation exists. It is intentionally executable without a browser, model API key, cloud account, database, or network service.

## Prerequisites

- Rust toolchain from repository `rust-toolchain.toml`.
- Git.
- No external service credentials.

## Expected Repository Slice

```text
Cargo.toml
rust-toolchain.toml
crates/ecra-core/
contracts/ecra-domain-v1/
specs/001-trusted-domain-kernel/
```

## Build

```bash
cargo build --workspace --locked
```

Expected: success with only ECR-001 production crate(s) authorized by `plan.md`.

## Formatting and Lints

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
```

Expected: zero warnings in Ecra-owned code unless a lint exception is explicitly documented in the feature plan/PR.

## Tests

```bash
cargo test --workspace --locked
cargo test -p ecra-core --test contract_fixtures --locked
cargo test -p ecra-core --test invalid_fixtures --locked
cargo test -p ecra-core --test canonicalization --locked
cargo test -p ecra-core --test properties --locked
cargo test --doc --workspace --locked
```

Expected:

- every valid fixture parses/validates/round-trips;
- every invalid fixture fails with the expected typed category/code;
- canonical fixtures are byte-stable under RFC 8785 JCS;
- property tests preserve security-sensitive invariants.

## Offline Gate

The core must build/test without requiring network access after dependencies are already present in the Cargo cache/vendor environment.

Example validation environment:

```bash
cargo test --workspace --locked --offline
```

Expected: no test attempts network, browser, model, database, filesystem service, or external process integration.

## Unsafe Gate

```bash
rg -n "unsafe\s*\{" crates/ecra-core
rg -n "allow\(unsafe_code\)|unsafe_code" crates/ecra-core
```

Expected: no authorized unsafe block or unsafe-code allowance for ECR-001. The crate root should forbid unsafe code.

## Dependency Boundary Gate

Review:

```bash
cargo tree -p ecra-core
```

The production dependency graph must not contain prohibited categories from `contracts/domain-v1.md`, including async runtime, HTTP/network, database, browser automation, model SDK, Cedar, MCP/ACP/A2A, process/filesystem execution framework, or telemetry exporter.

Any runtime dependency not listed/justified in `research.md` requires plan amendment or explicit documented approval before closure.

## Contract Fixture Review

Minimum valid and invalid fixture classes are normative in:

```text
specs/001-trusted-domain-kernel/contracts/domain-v1.md
```

Reviewers should sample the committed JSON files directly. The fixture corpus is part of the public contract, not only test data.

## Security-Sensitive Manual Checks

Before closure, demonstrate these cases from tests/fixtures:

1. `CapabilityRequest` cannot implicitly become `CapabilityGrant`.
2. Web/model/tool/memory origin text cannot become authority based on content.
3. `model_inferred + verified` preserves both states simultaneously.
4. `non_idempotent + safe retry` is rejected.
5. `unknown idempotency + safe retry` is rejected.
6. executor `ActionReceipt` is not `VerificationReceipt`.
7. unsupported schema version fails typed compatibility handling.
8. completion-before-start time is rejected.
9. `UNKNOWN` action outcome round-trips without coercion.
10. canonicalization is a fixed point on normative fixtures.

## Closure Evidence

The ECR-001 implementation PR/report should include:

- exact head SHA;
- toolchain version;
- changed file list;
- `cargo fmt` result;
- Clippy result;
- test totals/results;
- dependency boundary evidence;
- unsafe-code evidence;
- contract fixture count by valid/invalid category;
- donor/license ledger delta;
- Spec Kit traceability/analyze result;
- any convergence tasks and their completion evidence.

Do not mark ECR-001 `CLOSED_CANONICAL` from a local green claim alone; closure requires exact repository state and the Definition of Done from `plan.md`.
