# Implementation Plan: Trusted Domain Kernel

**Feature:** ECR-001  
**Branch (when implementation starts):** `001-trusted-domain-kernel`  
**Date:** 2026-08-27  
**Spec:** `specs/001-trusted-domain-kernel/spec.md`

## Summary

Implement Ecra's first trusted-core contract as a single zero-I/O Rust crate (`ecra-core`). The slice provides versioned, strongly typed, serializable domain objects for actors, origins, capabilities, observations/facts/evidence, artifacts, action intents, action receipts, and verification receipts, together with strict validation and RFC 8785 canonical JSON contract fixtures.

No runtime orchestration, storage, browser integration, policy engine, model provider, protocol adapter, or networking is part of this slice.

## Technical Context

**Language/Version:** Rust 1.98.x, Edition 2024, stable  
**Primary Dependencies:** `serde`, `serde_json`, `thiserror`, `uuid`, `url`, RFC 8785 implementation candidate (`serde_jcs`)  
**Dev Dependencies:** contract/property-test tooling such as `proptest` if implementation proves useful  
**Storage:** N/A — zero-I/O domain kernel  
**Testing:** `cargo test`, contract fixtures, property tests, rustdoc tests, strict Clippy, format check  
**Target Platforms:** Linux x86_64, macOS arm64/x86_64 where CI capacity exists, Windows x86_64; domain semantics must be platform-independent  
**Project Type:** Rust workspace/library  
**Performance Goals:** validation/canonicalization are deterministic and allocation-conscious; no hot-path benchmark claim in ECR-001. Contract fixture validation should be effectively instantaneous relative to later I/O workloads.  
**Constraints:** no network, filesystem, database, browser, process, model, telemetry, or async-runtime dependency in `ecra-core`; no `unsafe`; deterministic canonical fixtures; strict unsupported-version behavior  
**Scale/Scope:** ~30–40 domain types/value objects, one production crate, normative v1 fixture corpus

## Constitution Check

### Pre-implementation gate

| Gate | Result | Evidence / plan response |
|---|---|---|
| G1 Domain coherence | PASS | Slice exists specifically to create the single canonical domain representation. |
| G2 Authority | PASS | Request and grant are distinct; no authorization implementation or ambient authority exists. |
| G3 Provenance | PASS | Observation/Fact/Provenance/Trust/Freshness/Evidence are explicit and orthogonal. |
| G4 Side effects | PASS | Action side effect/idempotency/retry classes and UNKNOWN outcome are normative. |
| G5 Verification | PASS | ActionReceipt and VerificationReceipt are distinct types; self-report cannot equal verification by type. |
| G6 Durability | PASS-N/A | Persistent runs are ECR-002; ECR-001 types are serialization-ready and deterministic. |
| G7 Privacy/secrets | PASS | No secret values or secret store; payloads/metadata do not create secret-handling behavior. |
| G8 Local-first | PASS | Entire slice is local library code with no cloud dependency. |
| G9 Interoperability | PASS | JSON contract is provider-neutral; no external protocol owns internal types. |
| G10 Donor/license | PASS | Research distinguishes inspiration from source reuse; runtime dependencies require license ledger entry before merge. |
| G11 Upstream/browser maintenance | PASS-N/A | No browser patch/integration in this slice. |
| G12 Benchmarks | PASS | Only contract/correctness claims; no performance/security superlative. |

**Gate decision:** implementation may proceed once tasks are approved/generated. No constitutional violation requires Complexity Tracking.

## Phase 0 Research Decisions

Complete in `research.md`:

- Rust 1.98.x / Edition 2024;
- one `ecra-core` crate;
- JSON + Serde normative v1 representation;
- RFC 8785 JCS canonicalization;
- UUID newtype identifiers;
- caller-supplied time model;
- structured web-origin wrapper;
- capability data independent of Cedar/protocols;
- explicit side-effect/idempotency/retry semantics;
- receipt and verification separation;
- provenance and verification as orthogonal dimensions;
- artifact references rather than artifact bytes;
- typed errors;
- mandatory unit/contract/property tests;
- small dependency budget.

## Phase 1 Design Artifacts

- `data-model.md` — conceptual entities/invariants.
- `contracts/domain-v1.md` — normative externally observable contract.
- `quickstart.md` — implementation/verification path for contributors.
- contract fixtures under root `contracts/ecra-domain-v1/` during implementation.

## Project Structure

```text
.
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── crates/
│   └── ecra-core/
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs
│       │   ├── actor.rs
│       │   ├── origin.rs
│       │   ├── resource.rs
│       │   ├── time.rs
│       │   ├── capability.rs
│       │   ├── evidence.rs
│       │   ├── artifact.rs
│       │   ├── action.rs
│       │   ├── receipt.rs
│       │   ├── verification.rs
│       │   ├── version.rs
│       │   ├── canonical.rs
│       │   └── error.rs
│       └── tests/
│           ├── contract_fixtures.rs
│           ├── invalid_fixtures.rs
│           ├── canonicalization.rs
│           └── properties.rs
├── contracts/
│   └── ecra-domain-v1/
│       ├── README.md
│       ├── valid/
│       └── invalid/
└── specs/
    └── 001-trusted-domain-kernel/
```

### Structure Decision

Use a Cargo workspace immediately, but only one production crate for ECR-001. This makes later crate separation possible without creating speculative packages now. Normative JSON fixtures live outside the crate so future language/protocol/storage implementations can reuse the same corpus.

## Implementation Design

### 1. Base value objects

Implement in dependency order:

```text
SchemaVersion / errors
→ typed IDs
→ EpochMillis / evaluation context
→ Actor
→ WebOrigin / Origin
→ ResourceRef / Scope
```

All constructors perform structural validation. Deserialization must route through validated representations rather than permit invalid internal state.

### 2. Capabilities

Implement distinct request/grant structs and structured scope metadata. ECR-001 validates shape/time ranges only; it does not decide whether a grant authorizes a request.

Avoid convenience APIs that accidentally grant authority, such as `From<CapabilityRequest> for CapabilityGrant`.

### 3. Evidence domain

Implement Observation, Fact, Provenance, TrustState, Freshness, EvidenceRef and ArtifactRef. Preserve lineage and evidence references without embedding large data.

### 4. Action semantics

Implement ActionIntent and semantic classes with cross-field validation:

```text
side effect × idempotency × retry
```

Invalid permissive combinations fail at construction/deserialization.

### 5. Receipts and verification

ActionReceipt records executor-known outcome. VerificationReceipt records independent evaluation. No blanket conversion exists between them.

### 6. Serialization and canonicalization

Each normative top-level fixture uses a version envelope. Contract parser rejects unsupported versions and undocumented fields.

RFC 8785 canonicalization is exposed through a narrow Ecra-owned function so the chosen crate can be replaced without changing callers.

### 7. Errors

Expose structured error categories/variants. Tests match variants/codes, not display strings.

## Validation Strategy

### Unit tests

Each constructor and cross-field invariant.

### Contract tests

Load all files under:

```text
contracts/ecra-domain-v1/valid
contracts/ecra-domain-v1/invalid
```

Valid fixtures must parse, validate, serialize and round-trip. Invalid fixtures must fail with the expected category/code documented in fixture metadata or naming convention.

### Canonicalization tests

- byte equality against committed expected JCS fixtures;
- canonicalization fixed point;
- cross-field digest input stability where applicable;
- RFC 8785 edge cases included by the chosen dependency and Ecra-specific wrapper tests.

### Property tests

Focus on:

- temporal range validation;
- Action semantic combinations;
- typed ID separation/round-trip;
- canonicalize(parse(canonical)) fixed-point behavior within supported values.

### Architecture/dependency test

CI/automation checks that `ecra-core` does not acquire prohibited dependency categories. This may use a small script/cargo metadata assertion rather than a runtime test.

## Compatibility Policy

Before first public release, ECR-001 defines v1 semantics. Once dependent slices merge:

- removing/renaming fields or enum values, changing validation meaning, or changing canonical representation is a major contract change unless a migration envelope preserves old behavior;
- additive changes still require explicit minor-version support and fixtures;
- no reader silently accepts a schema minor greater than it understands in v1;
- digest/canonicalization semantics are especially migration-sensitive.

## Security Considerations

- No `unsafe` authorized.
- Strict deserialization for normative security-sensitive types.
- No text field can become an instruction/authority token by parsing its contents.
- `storage_locator`, `reason`, `label`, `notes`, and other free-form strings are non-authoritative metadata.
- Origin parser must reject malformed/ambiguous origin forms rather than normalize unsafe strings ad hoc.
- Large untrusted payload bytes are out of the core; later storage/parsing layers own resource limits.

## Donor / License Plan

Before implementation dependency merge, create/update a donor/license ledger entry for each runtime/dev dependency and any copied/adapted source. ECR-001 authorizes conceptual inspiration only from existing donor research; no direct source-copy approval is implied.

## Observability

No telemetry/exporter in ECR-001. Errors and validation results are structured so later tracing can record category/code without adding tracing into the core.

## Documentation

Public types need rustdoc explaining authority/provenance semantics where misuse could become a security bug. Contract semantics remain canonical in `contracts/domain-v1.md`; rustdoc should link/reference them rather than invent conflicting rules.

## Definition of Done for ECR-001

1. All tasks in `tasks.md` complete.
2. Contract fixture corpus meets `contracts/domain-v1.md` minimums.
3. `cargo fmt --check` passes.
4. strict Clippy passes with warnings denied for Ecra-owned code, subject to documented lint exceptions if any.
5. unit/contract/property/rustdoc tests pass on exact head.
6. zero prohibited runtime dependency categories.
7. zero `unsafe` in `ecra-core`.
8. donor/license records current.
9. no unresolved spec/plan/task traceability gap.
10. README/spec references reflect actual implemented type names and contract version.
11. exact implementation state demonstrates SC-001 through SC-015 or records a spec amendment before closure.

## Complexity Tracking

No constitutional complexity violation is currently justified or authorized.

If implementation requires a second production crate, async runtime, database, network/browser dependency, code generation service, or `unsafe`, the plan MUST be amended before that change lands.
