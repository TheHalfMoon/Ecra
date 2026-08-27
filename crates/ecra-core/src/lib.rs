#![forbid(unsafe_code)]

//! Ecra's zero-I/O trusted domain kernel.
//!
//! ECR-001 intentionally contains only provider-neutral value objects,
//! validation, serialization and canonical security-binding helpers.
//! Runtime execution, authentication, authorization, persistence,
//! browser/model integration, protocols, secrets and telemetry belong to
//! downstream slices.

pub mod actor;
pub mod canonical;
pub mod capability;
pub mod digest;
pub mod error;
pub mod id;
pub mod identity;
pub mod origin;
pub mod resource;
pub mod scope;
pub mod time;
pub mod version;

pub use actor::{Actor, ActorKind};
pub use canonical::to_jcs_vec;
pub use capability::{CapabilityGrant, CapabilityRequest, DelegationRef, OperationRef};
pub use digest::{ContentDigest, SecurityDigest, SecurityDigestAlgorithm};
pub use error::{DomainError, ErrorCategory, ErrorCode};
pub use id::*;
pub use identity::{IdentityAssertionRef, PrincipalRef};
pub use origin::{Origin, WebOrigin};
pub use resource::{ResourceKind, ResourceRef};
pub use scope::{PurposeRef, Scope, ScopeConstraint};
pub use time::{
    EpochMillis, EvaluationContext, I_JSON_MAX_SAFE_INTEGER, I_JSON_MIN_SAFE_INTEGER,
    TemporalValidity,
};
pub use version::{DOMAIN_SCHEMA_MAJOR, DOMAIN_SCHEMA_MINOR, SchemaVersion, Versioned};
