# Contract: Ecra Trusted Domain v1

**Feature:** ECR-001  
**Contract version:** 1.0-planning  
**Status:** NORMATIVE_FOR_IMPLEMENTATION_AFTER_TASKS_READY  
**Constitution:** v1.1.0

This contract defines externally observable semantics for the v1 trusted-domain kernel. Implementation details may vary only if these invariants, canonical bytes and fixture behavior remain unchanged.

## 1. Versioning / Strict Parsing

Every top-level normative fixture/value is wrapped:

```json
{
  "schema_version": { "major": 1, "minor": 0 },
  "value": {}
}
```

Rules:
- major != 1 → typed `unsupported_major_version`;
- unsupported newer minor → typed `unsupported_minor_version`;
- missing envelope → typed `missing_schema_version`;
- undocumented fields on security-sensitive v1 objects are rejected;
- no parser silently widens authority/information-flow semantics.

## 2. Canonical JSON

Normative canonical bytes conform to RFC 8785 JCS.

Tests MUST cover ordering, Unicode, escapes, negative zero, invalid/non-I-JSON numeric input and fixed-point behavior.

Security digests additionally use an explicit versioned/domain-separated byte domain; JCS alone is not a signature/authentication claim.

## 3. Typed Identifier Contract

All security/audit IDs are UUID strings in JSON and distinct strong Rust newtypes:

```text
ActorId
PrincipalId
IdentityAssertionId
RunId
ResourceId
WorkspaceId
BrowserSpaceId
ContainerId
TabId
SessionId
TaskId
CapabilityRequestId
CapabilityGrantId
ObservationId
FactId
EvidenceId
ArtifactId
ActionId
ActionAttemptId
ReceiptId
VerificationId
```

Invalid:
- empty/non-UUID strings;
- implicit conversion among ID categories;
- deriving authorization from ID format/display label.

Same UUID bytes in different ID categories remain different types.

## 4. Actor / Principal Contract

`ActorKind`:

```text
human
agent
system
```

Actor required:
- ActorId;
- ActorKind.

Optional label is non-authoritative.

`PrincipalRef` is a distinct PrincipalId reference. `IdentityAssertionRef` is a distinct reference containing IdentityAssertionId + PrincipalId.

**Invariants**
- ActorId is attribution, not proof of authentication.
- ActorId cannot substitute for PrincipalId by implicit conversion.
- IdentityAssertionRef existence does not imply validity; ECR-031 validates assertions/trust roots.
- no label/model/email/username/text grants authority.

## 5. Origin Contract

Origin kinds:

```text
user_input
web
local
retrieval
tool
model
memory
system_policy
```

Web origin is structured scheme/host/port or explicit opaque form. Full URL/path is not origin.

**Invariant:** origin expresses provenance/security context, never permission/instruction authority.

## 6. Resource Contract

Every `ResourceRef` contains:
- ResourceId;
- ResourceKind;
- optional non-authoritative locator;
- optional WebOrigin where applicable.

Initial kinds:

```text
web_resource
local_resource
workspace_resource
tool_resource
artifact
abstract
```

**Invariant:** locator text is not canonical security identity and MUST NOT be treated as such by core constructors.

## 7. Scope Contract

Security scope uses:

```text
ScopeConstraint<T>
not_applicable
exact(T)
one_of(non-empty list<T>)
any_explicit
```

The Scope supports explicit typed constraints for workspace, browser space, container, tab, session, task, web origins and ResourceIds, plus optional structured PurposeRef metadata.

**Mandatory rules**
1. Missing/empty MUST NOT mean unrestricted.
2. `one_of([])` is invalid.
3. `any_explicit` is the only wildcard/unrestricted value for a dimension.
4. `not_applicable` is not equivalent to wildcard.
5. Structural parsing does not claim subset/intersection authorization; ECR-003 owns that.

## 8. Capability Contract

`CapabilityRequest` and `CapabilityGrant` have distinct types and distinct IDs.

Required request fields:
- CapabilityRequestId;
- PrincipalRef;
- OperationRef;
- target ResourceRef;
- explicit Scope;
- requested_by ActorId.

Optional request fields:
- IdentityAssertionRef;
- temporal validity;
- reason.

Required grant fields:
- CapabilityGrantId;
- PrincipalRef;
- OperationRef;
- target ResourceRef;
- explicit Scope;
- issued_by ActorId.

Optional grant fields:
- temporal validity;
- parent CapabilityGrantId;
- delegation depth.

Rules:
- request never implicitly constructs/deserializes as grant;
- `not_before <= expires_at`;
- capability types contain no Cedar/MCP/browser/model policy syntax;
- parent existence/narrowing/revocation/authentication is downstream work.

## 9. Information Classification Contract

`InformationClass`:

```text
public
private
sensitive
secret
unknown
```

`InformationClassification` contains class + zero or more opaque structured policy tags.

**Invariants**
- classification grants no authority;
- unknown is not public;
- deriving/transcribing information does not implicitly declassify it;
- ECR-003 owns inheritance/join/declassification policy.

Observation, Fact and ArtifactRef MUST support InformationClassification.

## 10. Observation / Fact / Provenance Contract

Observation and Fact are distinct types.

Provenance:

```text
user_provided
observed_web
observed_local
retrieved
tool_provided
model_inferred
system_derived
```

Fact has no independently mutable `verified` field/state.

Conflict/dispute state MAY be represented as:

```text
undisputed
contradicted
disputed
inconclusive
unknown
```

**Invariant:** independent verification status is determined through VerificationReceipt records. A model-inferred Fact remains model-inferred after verification.

## 11. Freshness Contract

Freshness states:

```text
current
stale
unknown
```

FreshnessAssessment MUST be able to include:
- assessed_at;
- basis kind;
- basis time;
- basis evidence reference.

Basis kinds include observed/retrieved/published/effective/source-reported/other.

**Invariant:** freshness state does not rewrite provenance/verification and source-reported time is not automatically trusted.

## 12. Information Use / Disclosure Contract

`InformationRef` can reference ObservationId, FactId, ArtifactId or an ActionParameterRef.

Use kinds:

```text
local_compute
model_context
persist
log_or_diagnostic
external_disclosure
remote_provider
other
```

`InformationUse` requires a non-empty source list and can identify destination ResourceRef/WebOrigin where relevant.

**Invariants**
- InformationUse is declaration, not authorization.
- read authority over source A plus write/tool authority to B does not imply allowed A→B flow.
- external/remote use is a policy boundary owned by ECR-003.

## 13. Evidence Contract

Evidence kinds:

```text
observation
artifact
structured_tool_result
network_receipt
external_state
computation
model_judgment
other
```

EvidenceRef may include stable references plus optional content digest and `as_of` timestamp.

Evidence kind alone is not proof. Later verification policy decides evidentiary sufficiency; the domain supports immutable capture metadata for decision-grade verification.

## 14. Artifact / Digest Contract

ArtifactRef supports:
- ArtifactId/kind;
- media/logical metadata;
- InformationClassification;
- optional ContentDigest;
- byte size as safe decimal string when needed;
- non-authoritative storage locator;
- lineage.

Generic `ContentDigest { algorithm, hex }` is metadata and does not itself imply authenticity.

Security action binding uses a different `SecurityDigest` type with v1 algorithm `sha256`.

## 15. Action Intent Contract

Every ActionIntent includes:
- ActionId;
- ActorId;
- optional PrincipalRef / IdentityAssertionRef;
- OperationRef;
- target ResourceRef;
- explicit Scope;
- parameters/reference;
- zero or more InformationUse declarations;
- EffectProfile;
- IdempotencySpec;
- RetryClass;
- correlation identity;
- optional created_at.

`operation` is a requested operation, not an authority grant.

## 16. Effect / Idempotency / Retry Contract

Mutation domain:

```text
none
local
external
unknown
```

Reversibility:

```text
not_applicable
reversible
conditional
irreversible
unknown
```

Idempotency:

```text
naturally_idempotent
idempotent_with_key
non_idempotent
unknown
```

Retry:

```text
safe
requires_same_idempotency_key
requires_external_reconciliation
never_blind_retry
```

Mandatory checks:
1. mutation `none` requires reversibility `not_applicable`;
2. mutation `local`/`external` cannot use `not_applicable` reversibility;
3. keyed idempotency requires non-empty key_ref;
4. non-idempotent or unknown idempotency cannot use unconditional `safe` retry;
5. unknown mutation/reversibility does not normalize to a permissive value.

## 17. Action Digest / ActionRef Contract

Every security-binding action reference is:

```text
ActionRef
- id: ActionId
- digest: ActionDigest
```

ActionDigest v1:

```text
SHA-256(
  UTF8("ecra/action-intent/v1\0")
  || RFC8785_JCS(Versioned<ActionIntent>)
)
```

The canonical ActionIntent used for digesting contains all security-relevant fields and no derived digest field.

**Invariant:** same ActionId + different canonical security-relevant body is a digest mismatch and MUST be rejected by binding/receipt validation.

## 18. Action Attempt Contract

`ActionAttemptId` is distinct from ActionId.

```text
ActionAttemptRef
- id: ActionAttemptId
- action: ActionRef
```

One action may later have multiple attempts. Attempt lifecycle belongs to ECR-002.

## 19. Action Receipt Contract

Required:
- ReceiptId;
- exact ActionAttemptRef (therefore exact ActionId + ActionDigest + ActionAttemptId);
- executor ActorId;
- ActionOutcome.

Action outcomes:

```text
executor_observed_success
executor_observed_failure
unknown
```

Optional:
- start/completion timestamps;
- evidence refs;
- external reference;
- structured error summary.

Checks:
- completion >= start;
- UNKNOWN remains valid;
- receipt is executor-known evidence only and never independent verification.

## 20. Verification Receipt Contract

Verification outcomes:

```text
verified
rejected
inconclusive
not_evaluated
```

Methods:

```text
structured_external_state
api_or_tool_result
network_receipt
artifact_validation
dom_or_accessibility_state
deterministic_computation
independent_model_judgment
other
```

Required:
- VerificationId;
- verifier ActorId;
- target;
- method;
- outcome;
- evidence list subject to not-evaluated/inconclusive rules.

Verification target supports exact ActionRef, ActionAttemptRef, ReceiptId, FactId, ArtifactId or typed/opaque ClaimRef.

**Invariants**
- ActionReceipt cannot cast/deserialize as VerificationReceipt;
- executor-observed success is not VERIFIED;
- VerificationReceipt is the canonical verification record.

## 21. Time Contract

EpochMillis is caller-provided and within I-JSON exact integer range. No core validation reads the OS clock.

Time-sensitive checks use explicit `EvaluationContext { now }`.

## 22. Error Contract

Callers can distinguish at least:

```text
compatibility
identifier
identity_reference
origin
resource
scope
capability
temporal
information_flow_shape
evidence
digest
action_semantics
action_reference
attempt
receipt
verification
canonicalization
```

Display messages are not machine contracts.

## 23. Minimum Valid Fixtures

Before closure, include at least:
- Human/Agent/System Actors;
- separate PrincipalRef/IdentityAssertionRef;
- user/web/local/tool/model/memory origins;
- ResourceRef with stable ID + locator;
- scope fixtures for not_applicable/exact/one_of/any_explicit;
- scoped capability request/grant using distinct IDs;
- delegated grant representation;
- classified DOM/web observation;
- private/sensitive artifact with lineage;
- model-inferred Fact without verified flag;
- Fact + separate VERIFIED VerificationReceipt;
- contradicted/disputed Facts/evidence;
- freshness assessment with basis;
- ActionIntent with private source→remote provider InformationUse;
- non-mutating action;
- irreversible local mutation;
- reversible external mutation;
- keyed-idempotent external mutation;
- non-idempotent mutation + never_blind_retry;
- fixed ActionDigest/ActionRef;
- two distinct attempts for one action;
- UNKNOWN receipt;
- executor-observed-success receipt;
- independent verification receipts.

## 24. Minimum Invalid Fixtures

Include at least:
- unsupported version / unknown strict field;
- malformed/cross-type ID case;
- Actor/Principal implicit substitution fixture rejected by typed API/contract tests;
- malformed origin/resource;
- empty `one_of`;
- implicit/missing wildcard attempt;
- request parsed as grant;
- reversed temporal range;
- classification enum invalid;
- InformationUse with empty source list;
- invalid freshness basis/time form;
- keyed idempotency without key;
- non-idempotent/unknown + safe retry;
- mutation/reversibility contradictions;
- ActionRef with wrong digest;
- receipt attempt referencing wrong ActionRef;
- receipt completion before start;
- receipt parsed as VerificationReceipt;
- non-I-JSON/canonicalization invalid values;
- VerificationReceipt missing required target/verifier.

## 25. Dependency Boundary

`ecra-core` runtime dependencies MUST NOT include:
- async runtime;
- network/HTTP;
- database/storage driver;
- browser automation;
- model SDK;
- Cedar/policy engine;
- MCP/ACP/A2A SDK;
- process/filesystem execution abstraction;
- telemetry exporter.

Pure dependencies for Serde/JSON/UUID/URL/JCS/errors/SHA-256 are permitted only after exact version/license/security review and donor-ledger update.

## 26. Enforcement Ownership

This contract defines structure, not policy truth:

- ECR-002: attempts, run state, budgets/cancellation, persistence/integrity chain;
- ECR-031: identity assertion validation, trust roots, key lifecycle, protected sensitive storage;
- ECR-003: authorization decision/lease, grant narrowing, disclosure/declassification, approvals/secrets;
- ECR-004: verifier orchestration, evidence sufficiency, reconciliation.
