# Feature Specification: Trusted Domain Kernel

**Feature ID:** ECR-001  
**Status:** SPEC_READY  
**Created:** 2026-08-27  
**Roadmap:** `specs/000-ecra-platform/roadmap.md`  
**Constitution:** `.specify/memory/constitution.md`

## Purpose

Define and implement the smallest versioned, zero-I/O trusted domain kernel that every later Ecra surface can share without inventing competing representations for identity, authority, provenance, action, receipts, or verification.

This slice does **not** execute browsers, models, tools, policies, storage, networking, or plugins. It defines the semantic objects and invariants those later slices must use.

## User Scenarios & Testing

### User Story 1 — A human or agent action is represented without losing who did what or where it came from (Priority: P1)

As an Ecra runtime developer, I need a canonical representation of actors, origins, resources, and action intents so that Browser, Search, Terminal, Data, and external protocol adapters cannot create incompatible trust semantics.

**Why this priority:** Every privileged or evidence-producing subsystem depends on these distinctions. If they are ambiguous, downstream security cannot be repaired by adapters.

**Independent Test:** Construct human, agent, and system actors; construct web, local, tool, memory, and model origins; construct action intents referencing them; serialize/deserialize them; assert identity, origin, and scope survive exactly.

**Acceptance Scenarios**

1. **Given** a human actor and an agent actor, **when** each creates an action intent, **then** the serialized intents preserve distinct actor kinds and stable actor identifiers.
2. **Given** content observed from a web origin, **when** represented as an observation, **then** the observation can be distinguished from a user instruction without interpreting the text payload.
3. **Given** an action targeting a resource under an origin, **when** it is inspected, **then** actor, target resource, origin context, requested capability, and side-effect metadata are separately addressable fields.

---

### User Story 2 — Ecra can represent evidence and inference without collapsing them into generated text (Priority: P1)

As a Search/Memory/Verifier developer, I need observations, facts, provenance, evidence references, trust states, and freshness states to be explicit so that later systems can distinguish what was seen, inferred, provided, verified, contradicted, or stale.

**Independent Test:** Build facts with each provenance/trust class, link them to evidence references, mark freshness/verification state, round-trip through the versioned wire representation, and assert no class is lost or silently promoted.

**Acceptance Scenarios**

1. **Given** a DOM observation and a model inference about that observation, **when** both are represented, **then** they have different provenance classes.
2. **Given** a fact that is later independently verified, **when** verification metadata is added, **then** the original provenance remains available; verification does not rewrite history.
3. **Given** a stale fact, **when** represented, **then** staleness is explicit and cannot be confused with verified freshness.
4. **Given** contradictory evidence, **when** a fact references it, **then** the domain model can represent contradiction without forcing an arbitrary winner.

---

### User Story 3 — Consequential actions can be reasoned about before execution and described honestly afterward (Priority: P1)

As an Ecra execution/verifier developer, I need canonical side-effect, idempotency, retry, outcome, and receipt semantics so that later executors cannot treat ambiguous external outcomes as success or safely retry everything by default.

**Independent Test:** Construct action intents across read-only, reversible, destructive, and externally consequential categories; construct receipts with confirmed success, confirmed failure, and unknown outcomes; assert invalid combinations are rejected.

**Acceptance Scenarios**

1. **Given** a read-only action, **when** represented, **then** it may declare no external mutation.
2. **Given** a non-idempotent consequential action, **when** its execution outcome is ambiguous, **then** the only valid outcome is `UNKNOWN` until independent evidence resolves it.
3. **Given** an action marked unsafe for blind retry, **when** a receipt reports `UNKNOWN`, **then** the model preserves that retry restriction.
4. **Given** a receipt, **when** inspected, **then** the intended action and observed outcome are distinguishable; the receipt cannot masquerade as verification.

---

### User Story 4 — Later policy engines can express least-authority grants without redesigning the core (Priority: P1)

As a policy developer, I need capability requests and grants to express principal, action, resource/scope, origin constraints, purpose/task scope, expiry, and delegation provenance without depending on Cedar, MCP, Firefox, or any model vendor.

**Independent Test:** Construct narrowed capability grants and rejection cases; serialize/deserialize them; assert that a child grant cannot represent broader authority than its declared parent without being detectable by validation.

**Acceptance Scenarios**

1. **Given** an agent capability limited to reading `docs.example.com`, **when** it is represented, **then** the scope cannot be mistaken for global browser read authority.
2. **Given** a capability with expiry, **when** parsed, **then** expiry is explicit and validation can determine whether the grant is temporally valid using a caller-supplied time context.
3. **Given** a delegated capability, **when** represented, **then** delegation lineage is explicit.
4. **Given** a capability request and capability grant, **then** the two are distinct types; requesting authority never equals possessing authority.

---

### User Story 5 — The domain contract is portable and evolvable (Priority: P2)

As an Ecra protocol/storage developer, I need stable IDs, schema versions, deterministic validation, forward-compatible unknown-field behavior where safe, and documented compatibility rules so that persisted runs and external adapters can survive evolution.

**Independent Test:** Validate version-1 fixtures, reject unsupported breaking versions, preserve documented optional-field behavior, and demonstrate deterministic fixture serialization used by contract tests.

**Acceptance Scenarios**

1. **Given** a supported v1 fixture, **when** loaded, **then** it validates deterministically.
2. **Given** an unsupported major schema version, **when** loaded, **then** it fails with a typed compatibility error rather than best-effort interpretation.
3. **Given** a malformed enum/value violating invariants, **when** parsed, **then** it fails closed.
4. **Given** two semantically identical canonical fixture values, **when** normalized for digesting, **then** canonical bytes are identical.

## Edge Cases

- Actor IDs collide across actor kinds.
- Origin is absent for a purely local action.
- A browser action transitions from one origin to another.
- A resource is local but was derived from web content.
- A fact has multiple evidence references with conflicting trust/freshness states.
- A model inference cites another model inference rather than primary evidence.
- A capability expires exactly at evaluation time.
- A delegated grant references a missing parent.
- An action is reversible in theory but the inverse operation is not currently available.
- An action is idempotent only when an idempotency key is present.
- An executor crashes before knowing whether an external mutation committed.
- A receipt exists but no verifier has yet examined the external state.
- A timestamp is missing or generated by an untrusted external source.
- A payload contains unknown fields from a later minor schema revision.
- Arbitrary untrusted text contains strings that resemble authority/policy instructions.

## Functional Requirements

### Domain Identity and Versioning

- **FR-001** The kernel MUST define a versioned wire contract with explicit schema major/minor version fields.
- **FR-002** The kernel MUST define stable opaque identifiers for at least actors, runs, actions, artifacts, facts, evidence items, capabilities, receipts, and verification records.
- **FR-003** IDs MUST NOT derive authorization semantics from human-readable prefixes or display names.
- **FR-004** Public persisted/wire types MUST support deterministic validation independent of I/O or model execution.

### Actors and Origins

- **FR-005** The kernel MUST distinguish Human, Agent, and System actor kinds.
- **FR-006** Actor identity MUST be separable from display metadata.
- **FR-007** The kernel MUST represent origin/provenance contexts for at least user input, web origin, local resource, tool/protocol provider, model output, memory/retrieval, and system policy.
- **FR-008** External content origin MUST NOT imply instruction authority in the type model.

### Capabilities and Authority

- **FR-009** CapabilityRequest and CapabilityGrant MUST be distinct types.
- **FR-010** A capability MUST identify principal, operation/action class, target scope/resource constraints, and temporal validity when applicable.
- **FR-011** The model MUST support optional origin, workspace/task/purpose, browser space/container/tab/session, and delegation constraints without requiring all surfaces to populate all fields.
- **FR-012** Delegated grants MUST be able to reference delegation provenance/parentage.
- **FR-013** Validation MUST reject structurally invalid scopes and invalid temporal ranges.
- **FR-014** Capability types MUST NOT contain policy-engine-specific expressions as their canonical representation.

### Observation, Fact, Evidence, and Provenance

- **FR-015** Observation and Fact MUST be distinct concepts.
- **FR-016** The kernel MUST define provenance classes sufficient to distinguish user-provided, observed, retrieved, tool-provided, model-inferred, and system-derived information.
- **FR-017** Verification state MUST be represented separately from original provenance.
- **FR-018** Freshness/staleness MUST be representable without changing the underlying provenance.
- **FR-019** Facts MUST support zero or more evidence references and explicit contradiction/dispute state.
- **FR-020** Evidence references MUST identify evidence type and stable evidence/artifact identifiers without embedding arbitrary large blobs in core types.

### Actions and Side Effects

- **FR-021** ActionIntent MUST identify actor, requested capability/operation, target, parameters or parameter reference, side-effect classification, and correlation identity.
- **FR-022** Side-effect classification MUST distinguish at least read-only, local mutation, reversible external mutation, irreversible/destructive mutation, and unknown/unspecified mutation risk.
- **FR-023** Idempotency semantics MUST distinguish naturally idempotent, idempotent-with-key, non-idempotent, and unknown.
- **FR-024** Retry semantics MUST be explicit and MUST support “never blind retry”.
- **FR-025** ActionIntent MUST be representable before policy authorization or execution.

### Receipts and Verification

- **FR-026** ActionReceipt MUST identify the exact ActionIntent or its stable digest/reference.
- **FR-027** ActionReceipt outcome MUST distinguish CONFIRMED_SUCCESS, CONFIRMED_FAILURE, and UNKNOWN.
- **FR-028** A receipt MUST describe execution evidence without implying independent verification.
- **FR-029** VerificationReceipt MUST be a distinct type referencing the claim/action/fact it verifies, evidence examined, method, result, and verifier identity/class.
- **FR-030** Verification results MUST support at least VERIFIED, REJECTED, INCONCLUSIVE, and NOT_EVALUATED states.

### Artifacts

- **FR-031** Artifact references MUST support typed media/data kinds, content digest, size when known, and storage/location indirection without binding the core to a filesystem or database.
- **FR-032** Artifacts derived from other artifacts or observations MUST be able to preserve lineage references.

### Compatibility and Safety

- **FR-033** Unsupported major schema versions MUST fail with a typed compatibility error.
- **FR-034** Invalid enum/state combinations MUST fail closed.
- **FR-035** Core validation MUST not require wall-clock, filesystem, network, database, browser, model, or environment access; time-dependent validation accepts an explicit caller-supplied evaluation context.
- **FR-036** Core library code MUST contain no network, database, browser-control, process-spawning, secret-store, telemetry, or model-provider dependency.
- **FR-037** Serialized fixtures used for contract tests MUST be deterministic under the chosen canonicalization rules.
- **FR-038** The domain contract MUST document unknown-field/forward-compatibility behavior rather than relying on serializer defaults.
- **FR-039** The core MUST avoid `unsafe` code unless a later explicit exception is approved through constitution governance; this feature authorizes none.
- **FR-040** Error types MUST be structured enough for callers/tests to distinguish validation, compatibility, and invariant failures without parsing display strings.

## Key Entities

- `SchemaVersion`
- `ActorId`, `Actor`, `ActorKind`
- `Origin`, `OriginKind`, `WebOrigin`
- `ResourceRef`, `Scope`
- `CapabilityId`, `CapabilityRequest`, `CapabilityGrant`, `DelegationRef`
- `ObservationId`, `Observation`
- `FactId`, `Fact`, `Provenance`, `Freshness`, `TrustState`
- `EvidenceId`, `EvidenceRef`, `EvidenceKind`
- `ArtifactId`, `ArtifactRef`, `ArtifactKind`, `LineageRef`
- `ActionId`, `ActionIntent`, `SideEffectClass`, `IdempotencyClass`, `RetryClass`
- `ReceiptId`, `ActionReceipt`, `ActionOutcome`
- `VerificationId`, `VerificationReceipt`, `VerificationOutcome`, `VerificationMethod`
- Structured error/validation types

## Success Criteria

- **SC-001** 100% of normative domain fixtures round-trip without semantic loss on all supported CI platforms.
- **SC-002** 100% of invalid security-sensitive fixture combinations defined by the contract suite are rejected deterministically.
- **SC-003** The `ecra-core` dependency graph contains no I/O/runtime/model/browser/database dependency category prohibited by FR-036.
- **SC-004** A reviewer can trace every public domain type to at least one FR, entity definition, or contract invariant.
- **SC-005** Human, Agent, and System actor attribution remains distinct across serialization and canonical fixture normalization.
- **SC-006** Observed/retrieved/model-inferred/user-provided provenance and independent verification state can be represented simultaneously without overwriting one another.
- **SC-007** An ambiguous non-idempotent action can be represented as `UNKNOWN` with `never blind retry` semantics without contradictory fields.
- **SC-008** CapabilityRequest cannot deserialize or convert implicitly into CapabilityGrant without an explicit constructor/authorization path in caller code.
- **SC-009** Unsupported major schema fixtures fail with a typed compatibility error in 100% of contract tests.
- **SC-010** Canonicalization produces byte-identical output for semantically identical normative fixtures used for digesting.
- **SC-011** The core compiles and all tests pass with network access disabled.
- **SC-012** No public type requires a model-provider, browser-engine, external-protocol, policy-engine, or storage-specific type.
- **SC-013** `cargo fmt --check`, strict `clippy` configuration, unit tests, contract tests, documentation tests, and dependency policy checks pass on the exact feature head before closure.
- **SC-014** The feature contains zero unresolved `[NEEDS CLARIFICATION]` markers before PLAN_READY.
- **SC-015** A Spec Kit traceability analysis finds no MUST/SHOULD requirement without an owning task or explicit deferred rationale.

## Assumptions

- Rust is the trusted-core implementation language.
- The repository remains greenfield at this slice; no persisted compatibility obligations exist before ECR-001 v1 is released.
- Exact policy evaluation, storage, browser integration, cryptographic ledger hashing, and external protocols are intentionally delegated to later roadmap slices.
- The core may depend on small pure libraries for serialization, identifiers, time-value representation, hashing/canonicalization helpers, or error derivation if research/plan shows they preserve the zero-I/O boundary.
- “Canonical serialization” in this slice means a deterministic representation used by fixtures/digests; wire/storage containers may add additional envelopes later without changing domain semantics.

## Out of Scope

- Browser execution or Firefox integration.
- SQLite or any persistent ledger.
- Cedar or another policy engine.
- Model API calls.
- MCP/ACP/A2A/WebMCP adapters.
- Secret storage or OS keychain integration.
- Plugin execution/sandboxing.
- Search ranking/indexing.
- Workspace/memory persistence.
- Skill execution/compiler implementation.
- Public SDK stability guarantees beyond the version-1 domain contract required by this slice.
