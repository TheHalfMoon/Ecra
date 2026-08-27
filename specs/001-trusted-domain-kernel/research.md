# Research: Trusted Domain Kernel

**Feature:** ECR-001  
**Date:** 2026-08-27  
**Status:** COMPLETE_FOR_PLAN

This document resolves technical choices required to plan ECR-001. It intentionally avoids choices owned by later slices (SQLite ledger, Cedar policy, Firefox, MCP, model providers, cryptographic signatures, sandboxing).

## Decision R1 — Rust toolchain

**Decision:** Rust `1.98.x`, Edition 2024, stable toolchain only for ECR-001.

**Rationale:** Rust 1.98.0 was released on 2026-08-20 and is current stable at planning time. Starting greenfield on current stable avoids unnecessary legacy constraints. The repository will pin the exact patch toolchain in `rust-toolchain.toml`; `rust-version` will express the MSRV for the first release.

**Constraints:**

- No nightly-only language features.
- `#![forbid(unsafe_code)]` in trusted-core crates for this slice.
- MSRV changes after first public release require an explicit compatibility decision.

**Alternatives rejected:**

- Older MSRV solely because donors use it: donor MSRV is not an Ecra requirement.
- Nightly: unjustified trusted-core/toolchain risk.

**Primary reference:** Rust release announcements: `https://blog.rust-lang.org/releases/`.

## Decision R2 — Workspace shape

**Decision:** Start with one production crate: `crates/ecra-core`, plus repository-level contract fixtures/tests. Do not create the eight future crates from ROADMAP.md during ECR-001.

**Rationale:** The constitution requires the trusted core to remain small and rejects speculative abstractions. ECR-001 owns only zero-I/O domain semantics. Later slices earn new crates when I/O or responsibility boundaries become real.

**Initial shape:**

```text
Cargo.toml
rust-toolchain.toml
crates/
  ecra-core/
    Cargo.toml
    src/
      lib.rs
      actor.rs
      origin.rs
      capability.rs
      evidence.rs
      artifact.rs
      action.rs
      receipt.rs
      verification.rs
      version.rs
      error.rs
      canonical.rs
    tests/
      contract_fixtures.rs
      invalid_fixtures.rs
contracts/
  ecra-domain-v1/
    valid/
    invalid/
```

## Decision R3 — Serialization format

**Decision:** JSON is the normative human-inspectable v1 domain fixture/wire representation; Rust types use Serde. Security-sensitive/public objects use explicit version envelopes and strict parsing.

**Rationale:** JSON maximizes inspectability and cross-language compatibility for future MCP/ACP/A2A/browser/storage adapters. ECR-001 does not claim JSON is the final high-volume runtime transport.

**Rules:**

- `schema_version` is mandatory at contract envelopes.
- Unknown/unsupported schema versions fail with typed compatibility errors.
- Security-sensitive canonical objects do not silently ignore unknown fields.
- Enum values use stable explicit string names.
- Large integer quantities that could exceed the I-JSON exact integer range are encoded as validated decimal strings in the normative JSON contract.
- Time values used by the domain kernel are epoch milliseconds and MUST remain inside the I-JSON safe integer range; time-dependent validation uses caller-supplied evaluation context.

## Decision R4 — Canonicalization

**Decision:** Use RFC 8785 JSON Canonicalization Scheme (JCS) for deterministic fixture/digest bytes; evaluate `serde_jcs` as the minimal Rust implementation.

**Rationale:** RFC 8785 exists specifically to create invariant JSON bytes for cryptographic/digest use across implementations. It avoids inventing an Ecra-specific canonical JSON format. `serde_jcs` is MIT/Apache-2.0 dual licensed and directly targets RFC 8785.

**Scope in ECR-001:**

- deterministic canonical bytes;
- canonical fixture conformance;
- content/action reference digests may use canonical bytes where the contract requires a stable digest input.

**Out of scope:** ledger hash chains and signatures; ECR-002 owns them.

**Primary references:**

- RFC 8785: `https://www.rfc-editor.org/rfc/rfc8785.html`
- Rust implementation candidate: `https://github.com/l1h3r/serde_jcs`

## Decision R5 — Identifiers

**Decision:** Public identifiers are strongly typed Rust newtypes serialized as UUID strings. Core types parse/validate IDs but do not require ambient randomness or clocks to create them.

**Rationale:** Strong newtypes prevent accidental mixing of ActorId/ActionId/FactId/etc. UUID text is interoperable and inspectable. Generation can be provided by caller/runtime layers; deterministic fixtures use fixed IDs.

**Rules:**

- No authorization semantics from UUID version, prefix, or display name.
- No generic `String` IDs in security-sensitive public APIs.
- Conversion between ID kinds is explicit and not automatically implemented.

**Implementation candidate:** `uuid` crate with Serde support. Generation features are not required by `ecra-core` itself.

## Decision R6 — Time model

**Decision:** `EpochMillis` is a validated value object; core validation never calls the system clock.

**Rationale:** Zero-I/O determinism requires caller-supplied time. Expiry validation accepts `EvaluationContext { now }` from the policy/runtime layer.

**Rules:**

- External/source timestamps can be represented as evidence metadata but do not automatically become trusted evaluation time.
- Expiry range `not_before <= expires_at` is validated structurally.
- “Is valid now?” requires explicit `EvaluationContext`.

## Decision R7 — Web origins

**Decision:** `WebOrigin` is structured as scheme + canonical host + effective port/optional explicit port, with an explicit opaque-origin variant where browser semantics require it. Use a standards-aware URL parser rather than hand-written string splitting.

**Rationale:** Origin mistakes are security mistakes. Later Firefox/WebDriver work must not reinterpret an ad-hoc origin string differently from core policy semantics.

**Implementation candidate:** `url` crate for parsing/normalization, wrapped behind Ecra-owned `WebOrigin` types.

**Boundary:** Full URL/location is not authority. Origin and resource URL/path remain distinct concepts.

## Decision R8 — Capability representation

**Decision:** ECR-001 defines capability data, not policy language.

`CapabilityRequest` and `CapabilityGrant` are separate. A grant contains explicit structured scope constraints and optional delegation provenance. It does not contain Cedar expressions, MCP tool schemas, browser selectors, or model instructions.

**Rationale:** This preserves provider/policy-engine independence and makes it impossible to treat a model request as authorization by type confusion.

**Deferred:**

- subset/narrowing evaluator implementation beyond structural validation → ECR-003;
- Cedar adapter → ECR-003;
- browser-origin policy enforcement → ECR-003/ECR-006.

## Decision R9 — Side-effect semantics

**Decision:** Every `ActionIntent` declares a conservative side-effect class, idempotency class, and retry class before execution.

**Side-effect classes v1:**

- `read_only`
- `local_mutation`
- `reversible_external_mutation`
- `irreversible_external_mutation`
- `unknown`

**Idempotency classes v1:**

- `naturally_idempotent`
- `idempotent_with_key`
- `non_idempotent`
- `unknown`

**Retry classes v1:**

- `safe`
- `requires_same_idempotency_key`
- `requires_external_reconciliation`
- `never_blind_retry`

**Rationale:** Retry safety must be known before ECR-002 adds crash/resume and before browser/terminal/data executors exist.

**Validation principle:** uncertain or unspecified semantics become the more conservative representation, never the more permissive one.

## Decision R10 — Receipt versus verification

**Decision:** `ActionReceipt` and `VerificationReceipt` are different domain types.

- `ActionReceipt` records what the executor knows about execution.
- `VerificationReceipt` records an independent evaluation of a claim/action/fact using evidence.

**Action outcomes:** `confirmed_success`, `confirmed_failure`, `unknown`.

**Verification outcomes:** `verified`, `rejected`, `inconclusive`, `not_evaluated`.

**Rationale:** This prevents executor self-report from becoming success evidence and makes ECR-004 possible without changing core semantics.

## Decision R11 — Provenance and trust state

**Decision:** Preserve original provenance and derived trust/verification state as orthogonal fields/records.

**Provenance classes v1:**

- `user_provided`
- `observed_web`
- `observed_local`
- `retrieved`
- `tool_provided`
- `model_inferred`
- `system_derived`

**Trust/evidence state v1:**

- `unverified`
- `verified`
- `contradicted`
- `disputed`
- `inconclusive`

**Freshness state:** current/unknown/stale is represented independently with observation/effective timestamps where available.

**Rationale:** Verification must not erase that a claim originated as model inference, and staleness must not be confused with falsity.

## Decision R12 — Artifact references

**Decision:** Core stores `ArtifactRef`, not artifact bytes.

An artifact reference contains:

- typed artifact ID;
- kind/media metadata;
- content digest when known;
- byte size as safe decimal-string value when known;
- optional logical name;
- lineage references;
- opaque storage locator owned/interpreted by storage layers, not authority logic.

**Rationale:** Large blobs and persistence belong to ECR-002. Core still needs stable lineage/evidence references.

## Decision R13 — Error model

**Decision:** Use typed error enums with machine-readable variants/codes. Human-readable messages are not API contracts.

**Initial categories:**

- compatibility/version;
- invalid identifier;
- invalid origin;
- invalid scope/capability;
- invalid temporal range;
- invalid side-effect/idempotency/retry combination;
- invalid receipt/outcome;
- invalid evidence/provenance;
- canonicalization failure.

**Candidate:** `thiserror` for display/source plumbing while preserving Ecra-owned variants.

## Decision R14 — Testing strategy

**Decision:** Tests are mandatory for ECR-001 even though generic Spec Kit templates allow optional tests.

**Layers:**

1. unit tests for constructors/invariants;
2. normative valid JSON fixtures;
3. normative invalid JSON fixtures;
4. serialization/canonicalization contract tests;
5. property tests for invariants where input space is broad;
6. dependency/architecture test asserting prohibited dependency categories are absent;
7. `cargo fmt`, strict Clippy, rustdoc tests.

**Potential dev dependency:** `proptest` for invariant/property tests; no runtime dependency requirement.

## Decision R15 — Dependency policy

**Decision:** Every runtime dependency added to `ecra-core` requires a donor/license entry and justification in the implementation PR. Initial candidate set is intentionally small:

- `serde`
- `serde_json`
- `thiserror`
- `uuid`
- `url`
- `serde_jcs` (or equivalent RFC 8785 implementation after implementation-time license/maintenance verification)

No Tokio, async runtime, reqwest, database, browser, model SDK, policy engine, tracing exporter, filesystem abstraction, or protocol SDK belongs in ECR-001.

## Open Questions Resolved by Defaults

No blocking clarification remains for PLAN_READY. The following choices are intentionally conservative defaults and can evolve through versioned contracts:

- JSON is the v1 normative contract format.
- strict unsupported-version behavior is preferred over silent forward parsing.
- capability narrowing enforcement is deferred to ECR-003; structural representation is ECR-001.
- ledger/signature cryptography is deferred to ECR-002.
- user-facing display/localization is not part of core domain strings.

## Donor/Reference Boundary

ECR-001 may learn conceptually from:

- Block Buzz: explicit actors/events/audit discipline;
- Rig: serializable agent/run semantics;
- AgentFS: portable audit/state concepts;
- Graphify: extracted/inferred provenance distinction;
- RFC 8785: canonical JSON.

No donor source code is authorized for copying by this research document. Any copied or adapted source requires a separate donor/license ledger entry before implementation.
