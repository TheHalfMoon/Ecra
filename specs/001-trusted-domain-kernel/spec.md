# Feature Specification: Trusted Domain Kernel

**Feature ID:** ECR-001  
**Status:** SPEC_READY  
**Created:** 2026-08-27  
**Reworked:** 2026-08-27 after pre-implementation architecture review  
**Roadmap:** `specs/000-ecra-platform/roadmap.md`  
**Constitution:** `.specify/memory/constitution.md` v1.1.0

## Purpose

Define the smallest versioned, zero-I/O trusted domain kernel that every later Ecra surface can share without inventing competing representations for actor/principal identity, scope, information flow, provenance, actions/attempts, receipts, or verification.

This slice does **not** authenticate principals, evaluate policy, execute browsers/models/tools, persist runs, access secrets, perform network I/O, or run plugins. It defines the semantic value objects and fail-closed invariants those later slices must use.

The pre-implementation review identified several semantics that must exist before v1 is implemented: Actor vs Principal separation, explicit scope wildcard semantics, source-to-sink information labels, immutable action digest binding, execution-attempt identity, orthogonal mutation/reversibility semantics, and VerificationReceipt as the single authoritative verification record.

## User Scenarios & Testing

### User Story 1 — Attribution does not masquerade as authenticated identity (Priority: P1)

As an Ecra runtime/policy developer, I need Human/Agent/System actors, authenticated-principal references, origins, resources, and security scope dimensions to be distinct so that audit attribution cannot accidentally become authorization.

**Independent Test:** Construct multiple actors and principal/identity-assertion references, resources and explicit scopes; round-trip them; prove an ActorId cannot substitute for PrincipalId and omitted/empty scope cannot become unrestricted.

**Acceptance Scenarios**

1. **Given** a Human actor and an Agent actor, **when** serialized, **then** actor kind and ActorId remain distinct.
2. **Given** an agent acting under a PrincipalRef/IdentityAssertionRef, **when** represented, **then** actor attribution and security-principal reference are separate fields/types.
3. **Given** an action with a workspace/container/origin constraint, **when** a constraint is absent/not-applicable/unknown, **then** it cannot be interpreted as `ANY` unless `ANY` is explicitly encoded.
4. **Given** a `ResourceRef`, **when** inspected, **then** stable resource identity is distinct from its human/provider locator string.
5. **Given** arbitrary web/model/tool text resembling an instruction or identity, **then** its text cannot alter actor/principal/origin/scope semantics.

---

### User Story 2 — Information retains provenance, sensitivity, freshness basis, and disclosure intent (Priority: P1)

As a Search/Memory/Policy/Verifier developer, I need observations, facts, evidence and artifacts to preserve where information came from, how fresh it is, how sensitive it is, and where a proposed action intends to send/use it.

**Independent Test:** Represent web and local observations, model-inferred facts, sensitive artifacts, stale/current assessments and an external disclosure intent; prove classification and provenance survive derivation/serialization and do not self-authorize disclosure.

**Acceptance Scenarios**

1. **Given** a DOM observation and a model inference derived from it, **then** they retain different provenance classes.
2. **Given** private information derived into a summary, **then** the representation can conservatively carry the relevant information classification/policy tags; derivation alone does not declassify it.
3. **Given** a remote model/search/tool action using private information, **then** the ActionIntent can explicitly represent the source information references and destination/use class for later source-to-sink authorization.
4. **Given** a fact described as current/stale/unknown, **then** the freshness assessment includes an inspectable temporal basis rather than a naked enum only.
5. **Given** a VerificationReceipt verifying a model-inferred Fact, **then** the Fact remains `model_inferred`; verification is read from the VerificationReceipt, not an independently mutable `Fact.verified` flag.
6. **Given** contradictory evidence/facts, **then** conflict/dispute can be represented without silently declaring either one verified.

---

### User Story 3 — Action intent, immutable action identity, execution attempts, receipts and verification cannot be confused (Priority: P1)

As an Ecra run/policy/verifier developer, I need one immutable canonical action reference plus distinct attempt and receipt identities so that approvals, retries, reconciliation and audit always bind to the exact intended parameters.

**Independent Test:** Canonicalize an ActionIntent, compute its ActionDigest, create two distinct attempts for the same intent, create receipts for each, and create an independent verification record; prove none of these identities/types can substitute for another.

**Acceptance Scenarios**

1. **Given** a proposed action, **when** its canonical security-relevant body changes, **then** its ActionDigest changes even if a caller reuses the same ActionId.
2. **Given** one ActionIntent retried twice, **then** each attempt has a different ActionAttemptId and each receipt references the exact ActionRef and attempt.
3. **Given** an executor reports success, **then** the ActionReceipt outcome is `EXECUTOR_OBSERVED_SUCCESS`, not `VERIFIED`.
4. **Given** an ambiguous non-idempotent external mutation, **then** the receipt remains `UNKNOWN` and conservative retry semantics remain representable.
5. **Given** a VerificationReceipt, **then** it is a distinct type whose outcome may be VERIFIED/REJECTED/INCONCLUSIVE/NOT_EVALUATED and cannot be manufactured by casting an ActionReceipt.

---

### User Story 4 — Policy can later authorize both action and information disclosure without redesigning v1 (Priority: P1)

As a policy developer, I need capability requests/grants to name an authenticated principal reference, operation, target and explicit scope, while action intents separately declare source-to-sink information use/disclosure.

**Independent Test:** Build narrow request/grant fixtures, explicit `ANY` fixtures, child delegation representation and data-disclosure actions; prove request/grant IDs differ, no implicit request→grant conversion exists, and no missing scope widens authorization.

**Acceptance Scenarios**

1. **Given** a capability limited to reading `docs.example.com`, **then** it cannot be represented as global browser read authority without an explicit `ANY` constraint.
2. **Given** a capability with expiry, **then** validity can be evaluated with caller-supplied time; no OS clock is required.
3. **Given** a delegated grant, **then** its parent/provenance reference is explicit; subset enforcement remains ECR-003.
4. **Given** CapabilityRequest and CapabilityGrant, **then** they have distinct types and distinct typed IDs.
5. **Given** read authority over source A and write authority to destination B, **then** no ECR-001 type implies those authorities automatically permit disclosure A→B; an explicit InformationUse/DisclosureIntent is required for later policy.

---

### User Story 5 — Mutation, reversibility, idempotency and retry are orthogonal (Priority: P1)

As an execution developer, I need to describe effect domain separately from reversibility and idempotency so that deleting a local file is not implicitly less risky than a reversible external edit.

**Independent Test:** Enumerate effect-domain × reversibility × idempotency × retry combinations and reject invalid/permissive combinations.

**Acceptance Scenarios**

1. A read-only action has mutation domain `none` and reversibility `not_applicable`.
2. A destructive local delete may be `local + irreversible`.
3. An external update may be `external + reversible`.
4. Unknown mutation/reversibility/idempotency never normalizes to a more permissive value.
5. Non-idempotent/unknown idempotency cannot use unconditional safe retry.

---

### User Story 6 — The contract is portable, deterministic and evolvable (Priority: P2)

As an Ecra storage/protocol developer, I need strong IDs, schema versions, deterministic canonicalization, explicit unknown-field behavior and typed errors so later adapters/persistence can evolve without guessing trust semantics.

**Independent Test:** Validate v1 fixtures, reject unsupported versions/unknown security-sensitive fields, canonicalize deterministic values, and prove security digest/reference behavior is portable across supported CI platforms.

## Edge Cases

- The same ActorId is presented with two different ActorKinds.
- Actor and Principal identifiers have the same UUID bytes but must remain different types.
- An actor exists without an authenticated principal assertion.
- A PrincipalRef is present but its assertion is expired/revoked; validity is ECR-031/ECR-003, not inferred by ECR-001.
- A scope dimension is `not_applicable`, `exact`, `one_of`, or explicit `any`; empty `one_of` is invalid.
- An origin is opaque/non-web/local.
- Two locators alias the same eventual provider resource; locator text is non-authoritative.
- A resource is local but contains/derives from sensitive web content.
- Information derived from multiple sources has mixed labels/tags.
- An action includes an external destination but no declared information source/use.
- A fact is verified by one VerificationReceipt and rejected by another; the Fact itself does not contain a canonical verified flag.
- A freshness basis is missing/unknown or source-supplied and untrusted.
- One ActionId is paired with a modified canonical body/different digest.
- Two attempts exist for one action intent.
- An executor crashes after external commit but before receipt persistence.
- A receipt says executor-observed success while verification is inconclusive/rejected.
- A local destructive action is irreversible.
- Idempotency requires a key but none is supplied.
- A content checksum algorithm is unknown; it must not be used as a security action digest.
- Unknown fields appear in a later schema revision.
- Arbitrary untrusted text resembles authority/policy/approval instructions.

## Functional Requirements

### Domain Identity and Versioning

- **FR-001** The kernel MUST define a versioned wire contract with explicit schema major/minor fields.
- **FR-002** Security/audit identifiers MUST be strong opaque newtypes. At minimum: ActorId, PrincipalId, IdentityAssertionId, RunId, ResourceId, WorkspaceId, BrowserSpaceId, ContainerId, TabId, SessionId, TaskId, CapabilityRequestId, CapabilityGrantId, ObservationId, FactId, EvidenceId, ArtifactId, ActionId, ActionAttemptId, ReceiptId and VerificationId.
- **FR-003** IDs MUST NOT derive authority/trust semantics from display strings, prefixes or UUID version.
- **FR-004** Public persisted/wire types MUST validate deterministically without I/O/model execution.

### Actors, Principals, Origins, Resources and Scope

- **FR-005** The kernel MUST distinguish Human, Agent and System ActorKind.
- **FR-006** Actor attribution MUST be separate from PrincipalRef/IdentityAssertionRef; ActorId MUST NOT be accepted where PrincipalId is required without explicit caller logic.
- **FR-007** Actor display metadata MUST be non-authoritative. A given ActorId MUST identify one immutable ActorKind within a validated trust domain/run context; conflicting definitions are invalid for downstream stores/runs.
- **FR-008** Origin/provenance contexts MUST distinguish at least user input, web, local, retrieval, tool/protocol provider, model, memory and system policy.
- **FR-009** External content origin/text MUST NOT imply instruction, identity, approval, or authority.
- **FR-010** ResourceRef MUST contain stable ResourceId/kind plus optional non-authoritative locator/origin metadata; policy callers MUST be able to avoid comparing raw locator strings as identity.
- **FR-011** Scope MUST use explicit constraint semantics for security-relevant dimensions. Missing/empty MUST NOT mean unrestricted. `ANY` MUST be explicit.
- **FR-012** Scope constraint representation MUST distinguish at least `not_applicable`, `exact`, non-empty `one_of`, and `any_explicit`; invalid empty sets fail closed.
- **FR-013** Scope MUST support typed workspace, browser-space, container, tab, session, task, origin and resource constraints where applicable, plus non-authoritative purpose metadata/reference.

### Capabilities, Delegation and Time

- **FR-014** CapabilityRequest and CapabilityGrant MUST be distinct types with distinct ID types.
- **FR-015** A capability request/grant MUST identify PrincipalId/PrincipalRef, operation, target ResourceRef, explicit Scope and temporal validity where applicable.
- **FR-016** Capability requests MAY record requesting ActorId/IdentityAssertionRef separately from the principal being requested for.
- **FR-017** Delegated grants MUST represent parent/delegation provenance without claiming subset validity; actual authorization/narrowing is ECR-003.
- **FR-018** Structural validation MUST reject invalid scopes/temporal ranges. Time-dependent evaluation accepts explicit caller-supplied EvaluationContext.
- **FR-019** Capability types MUST NOT embed Cedar/MCP/Firefox/model-provider policy syntax.

### Information Classification, Provenance, Facts and Freshness

- **FR-020** Observation and Fact MUST be distinct concepts.
- **FR-021** Provenance MUST distinguish user-provided, observed-web, observed-local, retrieved, tool-provided, model-inferred and system-derived information.
- **FR-022** Verification outcome MUST NOT be stored as an independently mutable Fact truth flag. VerificationReceipt is the authoritative verification record.
- **FR-023** Facts MAY represent conflict/dispute relationships without choosing a verified winner.
- **FR-024** Freshness MUST be an assessment with state plus inspectable temporal basis/assessment metadata where known; staleness does not change provenance.
- **FR-025** EvidenceRef MUST identify type and stable evidence/artifact/observation/receipt references without embedding arbitrary large blobs.
- **FR-026** EvidenceRef MUST be able to record immutable capture/content digest and `as_of`/observation metadata when available so later verification can require decision-grade evidence.
- **FR-027** Observation, Fact and ArtifactRef MUST be able to carry an InformationClassification containing a conservative InformationClass plus zero or more opaque policy tags.
- **FR-028** Initial InformationClass MUST distinguish at least public, private, sensitive, secret and unknown. Classification alone grants no authority.
- **FR-029** Derived information MUST be representable with lineage to source Fact/Observation/Artifact information; ECR-003 owns conservative inheritance/declassification decisions.

### Artifacts and Digests

- **FR-030** ArtifactRef MUST support stable ArtifactId, kind/media metadata, content size/digest when known, non-authoritative storage locator, information classification and lineage.
- **FR-031** Generic ContentDigest metadata MUST distinguish algorithm from bytes and MUST NOT automatically be treated as a security/authenticity digest.
- **FR-032** Security binding digests used for ActionRef MUST use a contract-defined strong algorithm/domain. ECR-001 v1 MUST define SHA-256 over a versioned/domain-separated RFC 8785 canonical ActionIntent representation unless research supersedes this before PLAN_READY.

### Actions, Information Use, Effects and Attempts

- **FR-033** ActionIntent MUST identify ActionId, ActorId, optional Principal/IdentityAssertion reference, operation (not a CapabilityGrant), target ResourceRef, explicit Scope, parameter reference/body, InformationUse declarations, EffectProfile, IdempotencySpec, RetryClass and correlation identity.
- **FR-034** InformationUse MUST be able to identify source InformationRef(s), use kind and destination when relevant. Initial use kinds MUST include local_compute, model_context, persist, log_or_diagnostic and external_disclosure/remote_provider.
- **FR-035** InformationUse representation MUST NOT itself authorize disclosure; ECR-003 owns source-to-sink policy/declassification.
- **FR-036** EffectProfile MUST represent mutation domain separately from reversibility. MutationDomain MUST distinguish none, local, external and unknown. Reversibility MUST distinguish not_applicable, reversible, conditional, irreversible and unknown.
- **FR-037** Idempotency MUST distinguish naturally_idempotent, idempotent_with_key, non_idempotent and unknown.
- **FR-038** Retry semantics MUST be explicit and include safe, requires_same_idempotency_key, requires_external_reconciliation and never_blind_retry; permissive invalid combinations fail closed.
- **FR-039** ActionIntent MUST be representable before authorization/execution and MUST have a deterministic ActionDigest/ActionRef binding the exact canonical security-relevant body.
- **FR-040** ActionAttemptId MUST be a distinct type. ECR-001 defines attempt identity/reference; ECR-002 owns lifecycle/state transitions.

### Receipts and Verification

- **FR-041** ActionReceipt MUST bind exact ActionRef (ActionId + ActionDigest) and exact ActionAttemptId.
- **FR-042** ActionReceipt outcome MUST distinguish EXECUTOR_OBSERVED_SUCCESS, EXECUTOR_OBSERVED_FAILURE and UNKNOWN; it MUST NOT use `VERIFIED` or equivalent independent-verification terminology.
- **FR-043** ActionReceipt MUST describe executor-known evidence/errors without implying independent verification.
- **FR-044** VerificationReceipt MUST be distinct and reference the exact action/attempt/receipt/fact/artifact/claim target, verifier identity/class, method, evidence and result.
- **FR-045** VerificationOutcome MUST support VERIFIED, REJECTED, INCONCLUSIVE and NOT_EVALUATED.
- **FR-046** Mutable/live evidence materially used for consequential verification MUST be representable with immutable snapshot/digest/as-of metadata; ECR-004 decides when this is mandatory.

### Compatibility and Safety

- **FR-047** Unsupported major versions MUST fail with typed compatibility errors; unknown-field/forward-compatibility rules MUST be explicit and strict for security-sensitive v1 objects.
- **FR-048** Invalid enum/cross-field combinations MUST fail closed.
- **FR-049** Core validation MUST not require wall clock, filesystem, network, database, browser, model, process, secret store or environment access.
- **FR-050** `ecra-core` runtime dependencies MUST exclude async runtime, network, database, browser-control, model/provider, policy-engine, protocol SDK, process/filesystem execution framework and telemetry exporter.
- **FR-051** Normative canonicalization MUST be deterministic and domain-separated for security binding/digest purposes.
- **FR-052** The core MUST contain no `unsafe` code; this slice authorizes no exception.
- **FR-053** Error types MUST expose machine-readable compatibility/validation/invariant categories/codes without display-string parsing.
- **FR-054** Free-form fields (`label`, `reason`, `purpose`, `notes`, locators, provider metadata) MUST be documented non-authoritative and MUST NOT be parsed for permissions.
- **FR-055** No constructor/Serde conversion may implicitly widen scope, convert CapabilityRequest→CapabilityGrant, treat Actor→Principal as authentication, or treat ActionReceipt→VerificationReceipt as verification.

## Key Entities

- `SchemaVersion`, `Versioned<T>`
- `ActorId`, `Actor`, `ActorKind`
- `PrincipalId`, `PrincipalRef`, `IdentityAssertionId`, `IdentityAssertionRef`
- `Origin`, `OriginKind`, `WebOrigin`
- `ResourceId`, `ResourceRef`, `ResourceKind`
- `WorkspaceId`, `BrowserSpaceId`, `ContainerId`, `TabId`, `SessionId`, `TaskId`
- `Scope`, `ScopeConstraint<T>`
- `CapabilityRequestId`, `CapabilityGrantId`, `CapabilityRequest`, `CapabilityGrant`, `DelegationRef`
- `ObservationId`, `Observation`
- `FactId`, `Fact`, `Provenance`, `FreshnessAssessment`, `DisputeState`
- `InformationClass`, `InformationClassification`, `InformationRef`, `InformationUse`, `InformationUseKind`
- `EvidenceId`, `EvidenceRef`, `EvidenceKind`
- `ArtifactId`, `ArtifactRef`, `ArtifactKind`, `LineageRef`
- `ContentDigest`, `SecurityDigest`
- `ActionId`, `ActionDigest`, `ActionRef`, `ActionIntent`
- `MutationDomain`, `Reversibility`, `EffectProfile`, `IdempotencyClass`, `RetryClass`
- `ActionAttemptId`
- `ReceiptId`, `ActionReceipt`, `ActionOutcome`
- `VerificationId`, `VerificationReceipt`, `VerificationOutcome`, `VerificationMethod`
- structured errors/validation types

## Success Criteria

- **SC-001** 100% of normative valid fixtures round-trip without semantic loss on supported CI platforms.
- **SC-002** 100% of contract invalid security-sensitive fixtures fail deterministically with expected machine-readable category/code.
- **SC-003** `ecra-core` has zero prohibited I/O/runtime/model/browser/database/policy/protocol dependency categories.
- **SC-004** Every public domain type is traceable to FR/entity/contract semantics.
- **SC-005** ActorId and PrincipalId are non-interchangeable; actor kind/display text cannot create principal authority.
- **SC-006** No absent/empty scope encoding widens to `ANY`; unrestricted authority requires explicit `any_explicit` fixture representation.
- **SC-007** Provenance, information classification, freshness assessment and verification outcome can coexist without overwriting one another.
- **SC-008** Read/source information plus destination capability cannot form an implicit disclosure; ActionIntent must explicitly represent InformationUse for later policy.
- **SC-009** Modifying any security-relevant canonical ActionIntent field changes ActionDigest; ActionRef mismatches are rejected.
- **SC-010** Multiple attempts for one ActionIntent are distinguishable and receipts bind exact ActionRef + ActionAttemptId.
- **SC-011** An ambiguous non-idempotent action can round-trip UNKNOWN + conservative retry without contradictory fields.
- **SC-012** ActionReceipt cannot deserialize/cast as VerificationReceipt and executor-observed success does not equal VERIFIED.
- **SC-013** CapabilityRequest cannot deserialize/convert implicitly into CapabilityGrant and uses a distinct ID type.
- **SC-014** Unsupported versions/unknown security-sensitive fields fail typed compatibility handling.
- **SC-015** Canonicalization and ActionDigest output are byte-identical for semantically identical normative fixtures.
- **SC-016** The core builds/tests offline after dependency availability and contains zero authorized unsafe code.
- **SC-017** No public type requires a specific model, browser engine, protocol, policy engine, database or secret store.
- **SC-018** `cargo fmt`, strict Clippy, unit/contract/property/rustdoc/dependency-policy gates pass on exact feature head.
- **SC-019** No unresolved blocking clarification or pre-implementation review finding owned by ECR-001 remains before TASKS_READY.
- **SC-020** Spec Kit analyze-equivalent traceability finds no MUST/SHOULD requirement without an owning task/test/explicit deferred rationale.

## Assumptions

- Rust is the trusted-core language.
- ECR-001 defines identity/principal **references**, not authentication proof validation. ECR-031 owns trust roots, assertion validation, key lifecycle and sensitive storage envelopes.
- ECR-001 represents information classification/use; ECR-003 decides disclosure/declassification/authorization.
- ECR-001 defines ActionAttemptId/reference but not attempt lifecycle; ECR-002 owns execution state.
- ECR-001 defines receipt and verification types; ECR-004 owns verification orchestration/reconciliation.
- JSON is the human-inspectable v1 normative contract; high-volume transports may differ behind versioned adapters later.

## Out of Scope

- authentication/identity assertion validation or OS keychain/trust-root implementation;
- browser/Firefox execution;
- persistence/SQLite/run ledger;
- Cedar/policy/declassification evaluation;
- model API/local inference;
- MCP/ACP/A2A/WebMCP adapters;
- secret storage;
- plugin/sandbox execution;
- search ranking/indexing;
- workspace/memory persistence;
- Skill execution/compiler;
- public SDK stability beyond the ECR-001 v1 domain contract.
