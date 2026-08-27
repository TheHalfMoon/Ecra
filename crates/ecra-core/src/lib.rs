#![forbid(unsafe_code)]

//! Ecra's zero-I/O trusted domain kernel.
//!
//! ECR-001 intentionally contains only provider-neutral value objects,
//! validation, serialization and canonical security-binding helpers.
//! Runtime execution, authentication, authorization, persistence,
//! browser/model integration, protocols, secrets and telemetry belong to
//! downstream slices.
//!
//! # Non-authoritative text boundary
//!
//! Free-form labels, capability-request reasons, purposes, notes, locators,
//! provider/external references, diagnostic text, logical names and similar
//! metadata are data only. Their contents must never be parsed as authentication,
//! authorization, approval, policy syntax, identity proof or verification.
//! Stable typed identifiers, explicit scope/capability structures and independent
//! verification records remain authoritative for their respective semantics.
//!
//! # Construction examples
//!
//! Actor attribution and authenticated-principal references stay distinct:
//!
//! ```
//! use ecra_core::{Actor, ActorId, ActorKind, PrincipalId, PrincipalRef};
//!
//! # fn main() -> Result<(), ecra_core::DomainError> {
//! let actor = Actor::new(
//!     ActorId::parse_str("00000000-0000-0000-0000-000000000001")?,
//!     ActorKind::Agent,
//!     Some("research-agent".to_owned()),
//! );
//! let principal = PrincipalRef::new(PrincipalId::parse_str(
//!     "00000000-0000-0000-0000-000000000002",
//! )?);
//! assert_ne!(actor.id().to_string(), principal.id().to_string());
//! # Ok(())
//! # }
//! ```
//!
//! Scope is explicit; unrestricted authority is never inferred from absence:
//!
//! ```
//! use ecra_core::{Scope, ScopeConstraint, WorkspaceId};
//!
//! # fn main() -> Result<(), ecra_core::DomainError> {
//! let workspace = WorkspaceId::parse_str("00000000-0000-0000-0000-000000000020")?;
//! let scope = Scope::not_applicable().with_workspace(ScopeConstraint::exact(workspace));
//! assert!(matches!(scope.workspace(), ScopeConstraint::Exact(_)));
//! # Ok(())
//! # }
//! ```
//!
//! Capability requests and grants are constructed as different types and IDs:
//!
//! ```
//! use ecra_core::{
//!     ActorId, CapabilityGrant, CapabilityGrantId, CapabilityRequest, CapabilityRequestId,
//!     OperationRef, PrincipalId, PrincipalRef, ResourceId, ResourceKind, ResourceRef, Scope,
//! };
//!
//! # fn main() -> Result<(), ecra_core::DomainError> {
//! let actor = ActorId::parse_str("00000000-0000-0000-0000-000000000001")?;
//! let principal = PrincipalRef::new(PrincipalId::parse_str(
//!     "00000000-0000-0000-0000-000000000002",
//! )?);
//! let operation = OperationRef::new("browser", "read")?;
//! let target = ResourceRef::new(
//!     ResourceId::parse_str("00000000-0000-0000-0000-000000000030")?,
//!     ResourceKind::Abstract,
//!     None,
//!     None,
//! )?;
//! let scope = Scope::not_applicable();
//! let request = CapabilityRequest::new(
//!     CapabilityRequestId::parse_str("00000000-0000-0000-0000-000000000040")?,
//!     principal,
//!     operation.clone(),
//!     target.clone(),
//!     scope.clone(),
//!     actor,
//! );
//! let grant = CapabilityGrant::new(
//!     CapabilityGrantId::parse_str("00000000-0000-0000-0000-000000000041")?,
//!     principal,
//!     operation,
//!     target,
//!     scope,
//!     actor,
//!     None,
//! );
//! assert_ne!(request.id().to_string(), grant.id().to_string());
//! # Ok(())
//! # }
//! ```
//!
//! Information classification is metadata, not permission:
//!
//! ```
//! use ecra_core::{InformationClass, InformationClassification};
//!
//! let classification = InformationClassification::new(InformationClass::Sensitive, Vec::new());
//! assert_eq!(classification.class(), InformationClass::Sensitive);
//! ```
//!
//! Action intent, immutable reference, attempt identity and executor receipt remain separate:
//!
//! ```
//! use ecra_core::{
//!     ActionAttemptId, ActionAttemptRef, ActionId, ActionIntent, ActionOutcome,
//!     ActionParametersRef, ActionReceipt, ActionSemantics, ActorId, EffectProfile,
//!     IdempotencyClass, IdempotencySpec, MutationDomain, OperationRef, ReceiptId, ResourceId,
//!     ResourceKind, ResourceRef, RetryClass, Reversibility, Scope,
//! };
//!
//! # fn main() -> Result<(), ecra_core::DomainError> {
//! let actor = ActorId::parse_str("00000000-0000-0000-0000-000000000001")?;
//! let operation = OperationRef::new("ecra", "inspect")?;
//! let target = ResourceRef::new(
//!     ResourceId::parse_str("00000000-0000-0000-0000-000000000030")?,
//!     ResourceKind::Abstract,
//!     None,
//!     None,
//! )?;
//! let semantics = ActionSemantics::new(
//!     EffectProfile::new(MutationDomain::None, Reversibility::NotApplicable)?,
//!     IdempotencySpec::new(IdempotencyClass::NaturallyIdempotent, None)?,
//!     RetryClass::Safe,
//! )?;
//! let intent = ActionIntent::new(
//!     ActionId::parse_str("00000000-0000-0000-0000-000000000101")?,
//!     actor,
//!     operation,
//!     target,
//!     Scope::not_applicable(),
//!     ActionParametersRef::None,
//!     semantics,
//! );
//! let attempt = ActionAttemptRef::new(
//!     ActionAttemptId::parse_str("00000000-0000-0000-0000-000000000102")?,
//!     intent.action_ref()?,
//! );
//! let receipt = ActionReceipt::new(
//!     ReceiptId::parse_str("00000000-0000-0000-0000-000000000103")?,
//!     attempt,
//!     actor,
//!     ActionOutcome::Unknown,
//! );
//! receipt.validate_for(&intent)?;
//! assert_eq!(receipt.outcome(), ActionOutcome::Unknown);
//! # Ok(())
//! # }
//! ```
//!
//! Independent verification is a separate record and is never inferred from an executor receipt:
//!
//! ```
//! use ecra_core::{
//!     ActorId, ReceiptId, VerificationId, VerificationMethod, VerificationOutcome,
//!     VerificationReceipt, VerificationTarget,
//! };
//!
//! # fn main() -> Result<(), ecra_core::DomainError> {
//! let verification = VerificationReceipt::new(
//!     VerificationId::parse_str("00000000-0000-0000-0000-000000000110")?,
//!     ActorId::parse_str("00000000-0000-0000-0000-000000000001")?,
//!     VerificationTarget::Receipt(ReceiptId::parse_str(
//!         "00000000-0000-0000-0000-000000000103",
//!     )?),
//!     VerificationMethod::StructuredExternalState,
//!     VerificationOutcome::NotEvaluated,
//!     Vec::new(),
//! )?;
//! assert_eq!(verification.outcome(), VerificationOutcome::NotEvaluated);
//! # Ok(())
//! # }
//! ```

pub mod action;
pub mod actor;
pub mod artifact;
pub mod canonical;
pub mod capability;
pub mod digest;
pub mod error;
pub mod evidence;
pub mod id;
pub mod identity;
pub mod information;
pub mod origin;
pub mod receipt;
pub mod resource;
pub mod scope;
pub mod time;
pub mod verification;
pub mod version;

pub use action::{
    ActionAttemptRef, ActionIntent, ActionParameterRef, ActionParametersRef, ActionRef,
    ActionSemantics, EffectProfile, IdempotencyClass, IdempotencySpec, MutationDomain, RetryClass,
    Reversibility,
};
pub use actor::{Actor, ActorKind};
pub use artifact::{ArtifactKind, ArtifactRef, LineageRef};
pub use canonical::to_jcs_vec;
pub use capability::{CapabilityGrant, CapabilityRequest, DelegationRef, OperationRef};
pub use digest::{ActionDigest, ContentDigest, SecurityDigest, SecurityDigestAlgorithm};
pub use error::{DomainError, ErrorCategory, ErrorCode};
pub use evidence::{
    DisputeState, EvidenceKind, EvidenceRef, Fact, FactAssessment, FactValue, FreshnessAssessment,
    FreshnessBasisKind, FreshnessState, Observation, ObservationPayloadRef, Provenance,
};
pub use id::*;
pub use identity::{IdentityAssertionRef, PrincipalRef};
pub use information::{
    InformationClass, InformationClassification, InformationPolicyTag, InformationRef,
    InformationUse, InformationUseKind,
};
pub use origin::{Origin, WebOrigin};
pub use receipt::{ActionOutcome, ActionReceipt, ErrorSummary};
pub use resource::{ResourceKind, ResourceRef};
pub use scope::{PurposeRef, Scope, ScopeConstraint};
pub use time::{
    EpochMillis, EvaluationContext, I_JSON_MAX_SAFE_INTEGER, I_JSON_MIN_SAFE_INTEGER,
    TemporalValidity,
};
pub use verification::{
    ClaimRef, VerificationMethod, VerificationOutcome, VerificationReceipt, VerificationTarget,
};
pub use version::{DOMAIN_SCHEMA_MAJOR, DOMAIN_SCHEMA_MINOR, SchemaVersion, Versioned};
