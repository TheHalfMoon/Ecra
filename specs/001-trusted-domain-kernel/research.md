# Research: Trusted Domain Kernel

**Feature:** ECR-001  
**Date:** 2026-08-27  
**Status:** COMPLETE_FOR_REVISED_PLAN  
**Inputs:** `spec.md`, constitution v1.1.0, `pre-implementation-review-2026-08-27.md`

This document resolves choices required to plan the revised ECR-001 contract. It deliberately does not implement authentication, policy evaluation, persistence, browser control, protocol authentication, cryptographic trust roots, or sandboxing. Those responsibilities remain in later slices.

## R1 — Rust toolchain

**Decision:** Rust `1.98.x`, Edition 2024, stable; `#![forbid(unsafe_code)]` for `ecra-core`.

Current stable avoids legacy constraints in a greenfield trusted core. No nightly feature is authorized. The exact patch is pinned at implementation.

Reference: https://blog.rust-lang.org/releases/

## R2 — Workspace shape

**Decision:** One production crate only: `crates/ecra-core`.

Later run/policy/browser/search responsibilities earn separate crates when their I/O/responsibility boundaries exist. ECR-001 does not pre-create the future workspace architecture.

## R3 — Normative representation

**Decision:** Versioned strict JSON + Serde is the normative v1 fixture/wire representation.

Rules:
- mandatory schema envelope;
- explicit stable enum names;
- security-sensitive v1 objects reject unknown fields unless a specific extension point is defined;
- unsupported major/newer unsupported minor fails typed compatibility handling;
- large integer quantities outside I-JSON exact range use validated decimal-string forms.

JSON is not declared the permanent high-volume runtime transport.

## R4 — Canonicalization

**Decision:** RFC 8785 JCS is used for deterministic canonical JSON. Ecra exposes a wrapper rather than leaking a specific crate API.

Candidate: `serde_jcs` after exact version/security/license verification.

Reference: https://www.rfc-editor.org/rfc/rfc8785.html

## R5 — Strong identifiers

**Decision:** Every identifier used in security/audit joins is a distinct Rust newtype serialized as a UUID string.

This includes ActorId, PrincipalId, IdentityAssertionId, ResourceId, workspace/browser scope IDs, request/grant IDs, ActionId, ActionAttemptId, ReceiptId, VerificationId and the remaining IDs named by `spec.md`.

No automatic cross-ID conversion. Generation remains caller/runtime-owned so the zero-I/O core requires no randomness/clock.

## R6 — Time model

**Decision:** `EpochMillis` is a validated value; validation never calls the OS clock. `EvaluationContext { now }` is caller supplied.

Source timestamps are evidence metadata, not trusted evaluation time merely because they exist.

## R7 — Origin model

**Decision:** `WebOrigin` is standards-aware scheme/host/port plus explicit opaque-origin representation where needed. Full URL/path remains a resource locator, not origin authority.

Candidate: `url` crate wrapped behind Ecra-owned types.

## R8 — Actor vs authenticated principal

**Decision:** `Actor` and `PrincipalRef` are separate domain concepts.

- Actor: runtime/audit participant (Human/Agent/System).
- PrincipalRef: opaque security subject reference.
- IdentityAssertionRef: reference to evidence/credential assertion that may bind a principal to an actor/on-behalf-of relationship later.

ECR-001 does **not** validate authentication. ECR-031 owns trust roots, assertion validity, key lifecycle, revocation and on-behalf-of proof. ECR-003 consumes validated identity context for authorization.

**Rationale:** An ActorId is not proof of identity. Keeping the references distinct prevents an agent/run object from self-authenticating by choosing an ActorId.

## R9 — Explicit scope algebra

**Decision:** Security-relevant scope dimensions use explicit constraints rather than `Option<T>`/empty-list wildcard conventions.

Conceptual shape:

```text
ScopeConstraint<T>
- not_applicable
- exact(T)
- one_of(non-empty list<T>)
- any_explicit
```

No missing/empty encoding means `ANY`. `one_of([])` is invalid. ECR-003 owns intersection/subset/narrowing semantics.

**Rationale:** Fail-closed authorization cannot rely on undocumented serializer/caller conventions for wildcard behavior.

## R10 — Resource identity vs locator

**Decision:** `ResourceRef` contains a strong `ResourceId`, kind, and optional locator/origin metadata. Locator strings are explicitly non-authoritative.

Providers later resolve canonical/native resource identity and constraints. ECR-001 does not pretend URL/path normalization solves every filesystem/tool/provider alias.

## R11 — Capability representation

**Decision:** `CapabilityRequest` and `CapabilityGrant` have distinct Rust types **and distinct typed IDs**.

They carry PrincipalRef/PrincipalId, OperationRef, ResourceRef, explicit Scope, temporal validity and delegation references. Requesting Actor/IdentityAssertion refs may be recorded separately.

No Cedar expressions, MCP schema, browser selectors or model prompts live in the canonical capability representation. Subset/narrowing/authorization is ECR-003.

## R12 — Information classification and source-to-sink representation

**Decision:** ECR-001 introduces policy-neutral information-flow value objects so later policy can distinguish read/use/disclosure.

Initial classification:

```text
InformationClass
- public
- private
- sensitive
- secret
- unknown
```

`InformationClassification` may include opaque policy tags. Observation, Fact and ArtifactRef can carry classification and lineage. `InformationUse` on ActionIntent names source InformationRef(s), use kind, and destination where relevant.

Initial use kinds:
- local_compute;
- model_context;
- persist;
- log_or_diagnostic;
- external_disclosure / remote_provider.

Classification/use does not authorize anything. ECR-003 owns conservative inheritance, declassification and source-to-sink policy.

**Rationale:** Capability to read A plus capability to write B must never imply permission to disclose A→B.

## R13 — Provenance, verification and dispute state

**Decision:** Original provenance and independent verification are separate records. `Fact` does not carry an independently mutable `verified` truth flag.

Provenance v1:
- user_provided;
- observed_web;
- observed_local;
- retrieved;
- tool_provided;
- model_inferred;
- system_derived.

Facts may carry conflict/dispute relationships/state and freshness assessment. Verification truth is represented only by `VerificationReceipt` records targeting the Fact/claim/action/etc.

**Rationale:** Two independent verification state stores eventually diverge. A model-inferred Fact remains model-inferred even after a verifier validates it.

## R14 — Freshness assessment

**Decision:** Freshness is not a naked current/stale flag. The core supports an assessment with state plus `assessed_at` and optional temporal basis (`observed_at`, `retrieved_at`, `published_at`, `effective_at`, or other explicit basis) where known.

A source timestamp remains provenance/evidence and may itself be untrusted. ECR-009 decides ranking/refresh policy.

## R15 — Artifact/evidence references and digest policy

**Decision:** Core stores references, not large bytes.

`ArtifactRef` supports type/media metadata, information classification, byte size, lineage, opaque storage locator and optional `ContentDigest`.

`ContentDigest` is metadata and must not automatically imply authenticity. Decision-grade evidence can include immutable capture/digest/as-of metadata; ECR-004 decides when it is mandatory.

## R16 — Action effect semantics are orthogonal

**Decision:** Replace the old single SideEffectClass with `EffectProfile`:

```text
MutationDomain
- none
- local
- external
- unknown

Reversibility
- not_applicable
- reversible
- conditional
- irreversible
- unknown
```

Idempotency remains separate:
- naturally_idempotent;
- idempotent_with_key;
- non_idempotent;
- unknown.

Retry remains separate:
- safe;
- requires_same_idempotency_key;
- requires_external_reconciliation;
- never_blind_retry.

**Rationale:** A destructive local delete is not inherently safer than a reversible external update. Mutation location and reversibility are different properties.

## R17 — Immutable action binding

**Decision:** A security-relevant action has both `ActionId` and deterministic `ActionDigest`; consumers bind through `ActionRef { id, digest }`.

The digest input is a domain-separated, versioned RFC-8785 canonical representation of the entire security-relevant ActionIntent body. Reusing an ActionId with changed parameters/scope/information-use/effect semantics must create a digest mismatch.

**Algorithm for v1:** SHA-256. Security binding uses a dedicated `SecurityDigest` type rather than arbitrary ContentDigest algorithms.

A minimal pure Rust SHA-256 dependency (e.g. `sha2`) is permitted as a candidate after exact license/security review.

## R18 — Intent vs execution attempt

**Decision:** `ActionAttemptId` is distinct from ActionId. ECR-001 defines the identity/reference; ECR-002 owns attempt lifecycle.

Every execution receipt for side-effect-capable work binds exact ActionRef + ActionAttemptId. This makes retries/reconciliation/duplicate-effect analysis possible without changing the action intent.

## R19 — Receipt vs verification terminology

**Decision:** Keep separate ActionReceipt and VerificationReceipt types and remove the ambiguous `confirmed_success/confirmed_failure` executor terminology.

ActionReceipt outcomes:
- `executor_observed_success`;
- `executor_observed_failure`;
- `unknown`.

Verification outcomes:
- `verified`;
- `rejected`;
- `inconclusive`;
- `not_evaluated`.

**Rationale:** “confirmed success” is too easy for UI/caller code to mistake for independent verification.

## R20 — Security digest domain separation

**Decision:** Security hashes do not reuse arbitrary content-checksum semantics.

Conceptual digest input:

```text
"ecra/action-intent/v1\0" || JCS(Versioned<ActionIntent>)
```

Exact byte fixture is normative in `contracts/domain-v1.md`. Ledger chaining/signatures/MACs are later work; action digest proves identity of canonical content, not who authorized/signed it.

## R21 — Errors and validation

**Decision:** Typed machine-readable error categories/codes; callers never parse display strings.

Add categories for identity/principal mismatch, explicit-scope violations, information-flow shape, action-reference digest mismatch and attempt/receipt mismatch in addition to the existing compatibility/origin/time/evidence/action/receipt errors.

Candidate: `thiserror` for display/source plumbing only.

## R22 — Testing strategy

Mandatory layers:
1. constructor/invariant unit tests;
2. valid and invalid normative JSON fixtures;
3. strict schema/unknown-field tests;
4. canonicalization + fixed ActionDigest fixtures;
5. property tests for scope algebra, temporal values, effect/idempotency/retry matrix, classification/lineage and typed IDs;
6. type-confusion compile/runtime tests (Actor/Principal, request/grant, receipt/verification);
7. dependency boundary check;
8. fmt/strict Clippy/rustdoc;
9. exact-head Spec Kit traceability/analyze.

Potential dev dependency: `proptest`.

## R23 — Dependency policy

Initial runtime candidates remain intentionally small:
- `serde`;
- `serde_json`;
- `thiserror`;
- `uuid`;
- `url`;
- `serde_jcs` or equivalent reviewed RFC 8785 implementation;
- `sha2` or equivalent reviewed SHA-256 implementation for ActionDigest.

No Tokio/async runtime, HTTP, database, browser, model SDK, Cedar, MCP/ACP/A2A, filesystem/process abstraction or telemetry exporter belongs in ECR-001.

## R24 — Deferred responsibilities are named, not implicit

- identity assertion validation / trust root / key lifecycle / sensitive storage → ECR-031;
- capability narrowing, authorization decision/lease, source-to-sink policy/declassification, approvals, secrets → ECR-003;
- ActionAttempt lifecycle, run budgets/cancellation, persistence/integrity chain → ECR-002;
- independent verification orchestration/evidence sufficiency/reconciliation → ECR-004;
- browser origin/permission/IPC execution → ECR-006–ECR-008;
- source ranking/freshness/source independence → ECR-009;
- persistent memory classification/deletion propagation → ECR-010.

## Donor / Reference Boundary

Conceptual references remain:
- Block Buzz: explicit identity/event/audit discipline;
- Rig: serializable run/state discipline;
- AgentFS: portable audit/state concepts;
- Graphify: provenance distinction;
- RFC 8785: canonical JSON;
- NIST agent identity/authorization work: explicit authentication, delegation, on-behalf-of and revocation concepts;
- OWASP agent security guidance: privilege/tool abuse, data exfiltration and bounded consumption concerns.

This research authorizes no donor source copying. Source reuse requires exact provenance/license approval in the canonical ledger.

## Planning Conclusion

All blocking **ECR-001-owned** design questions found by the pre-implementation review now have a conservative planned answer. Their exact wire model is normative in `data-model.md` and `contracts/domain-v1.md`. Downstream enforcement remains intentionally owned by ECR-002/ECR-003/ECR-004/ECR-031 rather than smuggled into this zero-I/O crate.
