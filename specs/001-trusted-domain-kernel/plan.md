# Implementation Plan: Trusted Domain Kernel

**Feature:** ECR-001  
**Branch (when implementation starts):** `001-trusted-domain-kernel`  
**Date:** 2026-08-27  
**Replanned:** 2026-08-27  
**Spec:** `specs/001-trusted-domain-kernel/spec.md`  
**Contract:** `specs/001-trusted-domain-kernel/contracts/domain-v1.md`

## Summary

Implement Ecra's first trusted-core contract as one zero-I/O Rust crate, `ecra-core`.

The revised contract provides versioned, strongly typed value objects for:

- actor attribution vs principal/identity-assertion references;
- origins, resource identity and explicit security scope algebra;
- capability request/grant representation;
- information classification and source-to-sink use declarations;
- observations, facts, freshness, evidence and artifacts;
- action intent, immutable ActionDigest/ActionRef and ActionAttempt identity;
- orthogonal mutation/reversibility/idempotency/retry semantics;
- executor ActionReceipt vs independent VerificationReceipt;
- deterministic canonicalization and structured errors.

No authentication, authorization, persistence, browser/model/tool execution, policy engine, networking, secret access, or verification orchestration is in this slice.

## Technical Context

**Language/Version:** Rust 1.98.x, Edition 2024, stable  
**Runtime dependency candidates:** `serde`, `serde_json`, `thiserror`, `uuid`, `url`, reviewed RFC-8785/JCS implementation, reviewed SHA-256 implementation such as `sha2`  
**Dev candidates:** `proptest` where useful  
**Storage:** none  
**I/O:** none  
**Unsafe:** forbidden  
**Testing:** unit + normative valid/invalid fixtures + canonicalization/action-digest fixtures + property/type-confusion tests + rustdoc + dependency-boundary gates  
**Platforms:** platform-independent semantics on Linux/macOS/Windows CI where available  
**Scale:** one production crate; roughly 50–70 small domain/value/reference types; no service abstractions

## Constitution v1.1.0 Check

| Gate | Result | Plan response |
|---|---|---|
| G1 Domain coherence | PASS | Revised v1 is the single canonical value-object model. |
| G2 Authority | PASS | Actor/Principal separation, explicit ScopeConstraint algebra, Request/Grant separation; no policy implementation. |
| G3 Provenance | PASS | Observation/Fact/Provenance/Evidence/Freshness remain explicit; verification not stored as Fact truth flag. |
| G4 Side effects | PASS | MutationDomain/Reversibility/Idempotency/Retry are orthogonal; ActionAttempt identity exists; UNKNOWN preserved. |
| G5 Verification | PASS | VerificationReceipt is the only verification outcome record; executor outcomes use `executor_observed_*`. |
| G6 Durability | PASS-N/A | Persistence/lifecycle is ECR-002; types are serialization/digest ready. |
| G7 Privacy/secrets | PASS | InformationClassification/Use can represent sensitive flows; no secret values/store. |
| G8 Local-first | PASS | Entire feature runs offline after dependencies are available. |
| G9 Interoperability | PASS | No protocol SDK/type owns the domain model. |
| G10 Donor/license | PASS-CONDITIONAL | Each exact dependency/version/license must be recorded before implementation merge. |
| G11 Browser maintenance | PASS-N/A | No browser dependency/patch. |
| G12 Benchmarks | PASS | Only deterministic contract/correctness claims. |
| G13 Information flow / egress | PASS | Classification + InformationUse represent source-to-sink intent; enforcement explicitly ECR-003. |
| G14 Identity / principal binding | PASS | Actor/Principal/IdentityAssertion references are separate; authentication explicitly ECR-031. |
| G15 Bounded execution | PASS-N/A | No recursive/tool/process/model execution exists in ECR-001. |

**Gate decision:** no constitutional blocker remains in the revised ECR-001 plan. Implementation is still forbidden until revised `tasks.md`, checklist and analyze report are complete.

## Research Decisions Incorporated

See `research.md`. Most important pre-code corrections:

1. Actor != authenticated Principal.
2. `ScopeConstraint<T>` makes wildcard semantics explicit.
3. ResourceId is distinct from locator text.
4. CapabilityRequestId != CapabilityGrantId.
5. InformationClass/InformationUse enable later source-to-sink authorization.
6. Fact contains no canonical `verified` flag.
7. Freshness has a temporal assessment basis.
8. EffectProfile separates MutationDomain from Reversibility.
9. ActionRef binds ActionId + SHA-256 ActionDigest over domain-separated canonical ActionIntent.
10. ActionAttemptId separates intent from retry attempts.
11. ActionReceipt uses executor-observed outcomes; VerificationReceipt alone owns verification outcome.
12. ContentDigest metadata is distinct from security-binding digest semantics.

## Project Structure

```text
.
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── crates/
│   └── ecra-core/
│       ├── Cargo.toml
│       ├── README.md
│       ├── src/
│       │   ├── lib.rs
│       │   ├── id.rs
│       │   ├── version.rs
│       │   ├── time.rs
│       │   ├── actor.rs
│       │   ├── identity.rs
│       │   ├── origin.rs
│       │   ├── resource.rs
│       │   ├── scope.rs
│       │   ├── capability.rs
│       │   ├── information.rs
│       │   ├── evidence.rs
│       │   ├── artifact.rs
│       │   ├── action.rs
│       │   ├── digest.rs
│       │   ├── receipt.rs
│       │   ├── verification.rs
│       │   ├── canonical.rs
│       │   └── error.rs
│       └── tests/
│           ├── contract_fixtures.rs
│           ├── invalid_fixtures.rs
│           ├── canonicalization.rs
│           ├── action_digest.rs
│           └── properties.rs
├── contracts/
│   └── ecra-domain-v1/
│       ├── README.md
│       ├── valid/
│       └── invalid/
└── specs/001-trusted-domain-kernel/
```

The module split is for reviewability, not separate services/crates.

## Implementation Order

### 1. Contract substrate

```text
SchemaVersion / Versioned<T> / errors
→ typed IDs
→ EpochMillis / temporal values
→ canonicalization wrapper
→ security digest wrapper
```

No security-sensitive type accepts invalid internal state through public constructors/deserialization.

### 2. Attribution, identity references and scope

```text
Actor
PrincipalRef / IdentityAssertionRef
Origin / WebOrigin
ResourceRef
ScopeConstraint<T> / Scope
```

Key tests:
- Actor and Principal IDs non-interchangeable;
- `one_of([])` invalid;
- missing/empty never means ANY;
- `any_explicit` visible in canonical JSON;
- locator/free-form strings remain non-authoritative.

### 3. Capabilities

Implement distinct request/grant types and IDs, OperationRef, TemporalValidity and delegation references.

No `From<CapabilityRequest> for CapabilityGrant`, no Actor→Principal authentication shortcut, no subset/authorization evaluator.

### 4. Information/evidence/artifacts

Implement InformationClassification/InformationUse references, Observation, Fact, Provenance, FreshnessAssessment, DisputeState, EvidenceRef and ArtifactRef.

Do **not** add `Fact.verified`. Verification lookup/aggregation is downstream.

### 5. Action semantics

Implement ActionIntent, InformationUse, EffectProfile, idempotency/retry matrix and exact canonical ActionDigest.

The ActionDigest test corpus is security-sensitive API contract. Any field later declared security-relevant must be part of the canonical digest domain or require a versioned contract migration.

### 6. Attempt/receipt/verification

Implement ActionAttemptRef, ActionReceipt and VerificationReceipt.

Receipts bind ActionRef + ActionAttemptId. Verification target can bind exact ActionRef/attempt/receipt/fact/artifact/claim.

### 7. Full strict versioned fixture layer

Run every valid/invalid fixture through:

```text
parse → structural validation → serialize → parse → semantic equality
```

Canonical/digest fixtures additionally assert exact bytes/hex.

## ActionDigest Design

Normative input:

```text
UTF8("ecra/action-intent/v1\0")
|| RFC8785_JCS(Versioned<ActionIntent>)
```

Normative v1 algorithm: SHA-256.

The implementation MUST have a single Ecra-owned function/API for this calculation. Callers must not independently assemble digest bytes.

The digest is a content-binding identity; it is not a signature, authorization decision, MAC, or trust proof.

## Serialization Strategy

- derive/implement Serde with explicit stable names;
- strict `deny_unknown_fields`-equivalent behavior for normative security-sensitive v1 objects;
- constructors/`TryFrom` validation for cross-field invariants;
- do not rely on `Option` to encode wildcard authority;
- canonical list/set ordering rules must be documented where semantically set-like values affect JCS/digests.

## Validation Strategy

### Unit tests
- every constructor/value invariant;
- explicit scope algebra;
- time range;
- information-use shape;
- effect/idempotency/retry matrix;
- receipt timing/action-reference binding.

### Contract fixtures

Every valid fixture parses/validates/round-trips. Every invalid fixture fails with expected code/category.

### Canonicalization + digest

- RFC 8785 edge cases;
- canonicalization fixed point;
- exact ActionDigest expected hex;
- mutation testing style table: changing actor/principal/operation/target/scope/parameters/information-use/effect/idempotency/retry changes ActionDigest;
- changing excluded/non-security display metadata only behaves according to the explicit contract (prefer including full ActionIntent except derived digest; any exclusion is documented).

### Property/type-confusion tests

- distinct typed IDs;
- ScopeConstraint normalization/invariants;
- classification/tag/lineage round-trip;
- EffectProfile/idempotency/retry combinations;
- request cannot grant;
- receipt cannot verify;
- Actor cannot authenticate as Principal through generic conversion.

### Architecture/dependency gate

Use `cargo metadata`/`cargo tree` plus repository script/check to fail if prohibited dependency categories enter `ecra-core`.

## Compatibility Policy

Once a dependent slice closes against v1:

- changes to field/enums/scope semantics/ActionDigest canonical domain/ID categories/verification ownership are compatibility-sensitive;
- semantic changes require versioned migration rather than silent parser changes;
- additive fields still require explicit supported minor version and strict reader behavior;
- ActionDigest domain changes require a new action contract version/domain separator;
- persisted/wire adapters must preserve exact v1 meaning.

## Security Review Notes

The revised type model directly remedies review findings P-001 through P-012 and P-028–P-030 where they are ECR-001-owned:

- data disclosure can be represented independently of read authority;
- Actor is not Principal;
- wildcard scope is explicit;
- exact ActionDigest exists;
- attempts are unique;
- verification has one authoritative record path;
- local vs external mutation is orthogonal to reversibility;
- scope/resource IDs are strongly typed;
- Resource locator is non-authoritative;
- freshness has a basis;
- ContentDigest is not automatically security authenticity.

Authorization leases/TOCTOU, trust-root identity validity, real sensitive persistence, browser permissions, search provider egress enforcement and runtime budgets are downstream owners and MUST NOT be implemented accidentally here.

## Donor / Dependency Plan

Before the implementation PR can merge:

- verify exact versions/licenses/security posture for all runtime/dev dependencies;
- add canonical donor/license ledger entries;
- minimize features/default features;
- record why `sha2`/JCS implementation is used rather than copied donor source;
- copy no donor source without exact file/commit/notice provenance.

## Documentation

Rustdoc/crate README must explicitly warn:
- Actor != authenticated Principal;
- classification != permission;
- InformationUse != authorization;
- locator != resource security identity;
- ActionDigest != signature/approval;
- ActionReceipt != verification;
- UNKNOWN remains UNKNOWN.

## Definition of Done

1. All revised tasks complete.
2. Contract minimum valid/invalid fixture classes complete.
3. `cargo fmt --all --check` PASS.
4. strict Clippy PASS with documented exceptions only.
5. unit/contract/property/canonicalization/action-digest/rustdoc tests PASS on exact head.
6. dependency boundary PASS.
7. zero `unsafe` in `ecra-core`.
8. offline test PASS after dependency availability.
9. donor/license ledger current.
10. FR-001–FR-055 and SC-001–SC-020 traceability complete.
11. pre-implementation review ECR-001 blockers are marked resolved with evidence.
12. analyze-equivalent post-implementation review has no critical drift.
13. exact-head evidence is recorded before `CLOSED_CANONICAL`.

## Complexity Tracking

The review added several domain value types, but no second crate, I/O, policy engine, service abstraction or runtime. This complexity is accepted because each new type prevents a concrete security ambiguity found before code: identity confusion, implicit wildcard scope, cross-source data disclosure, action/attempt confusion or verification state duplication.

If implementation requires a second production crate, async runtime, database/network/browser/process/model dependency, code generation service or unsafe code, this plan MUST be amended before that change lands.
