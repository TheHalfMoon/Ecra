#![forbid(unsafe_code)]

//! Local identity, trust-root and protected-storage foundations for Ecra.
//!
//! ECR-031 owns authenticated local principal context, local enrollment and
//! trust-root lifecycle, bounded assertion issuance/validation, protected
//! trust-state, native secret custody abstractions, protected envelopes and
//! protected anchors.
//!
//! This crate builds on ECR-001 identifiers and domain primitives rather than
//! redefining them. Identity evidence answers **who / on whose behalf** under
//! a bounded trusted local context. It never grants capability authority,
//! approval, declassification, disclosure permission or an execution lease.
//!
//! # Misuse resistance
//!
//! - Actor attribution is not authenticated principal identity.
//! - An assertion reference is not a validated assertion.
//! - Ordinary file/database metadata is not authoritative key lifecycle state.
//! - A protected anchor is not an independent verification receipt.
//! - Native backend absence, lock or unsupported state must fail closed.
//! - ECR-031 has no browser, model, network, provider or protocol execution
//!   surface.
//! - Production secret material must never fall back to plaintext files,
//!   environment variables or an unprotected in-memory substitute.
//!
//! Strong identifiers remain distinct at compile time:
//!
//! ```compile_fail
//! use ecra_identity::{KeyId, TrustRootId};
//!
//! fn requires_trust_root(_: TrustRootId) {}
//! let key = KeyId::parse_str("00000000-0000-0000-0000-000000000003").unwrap();
//! requires_trust_root(key);
//! ```
//!
//! ECR-001 principal identity is not interchangeable with ECR-031 trust-root
//! identity:
//!
//! ```compile_fail
//! use ecra_core::PrincipalId;
//! use ecra_identity::TrustRootId;
//!
//! fn requires_trust_root(_: TrustRootId) {}
//! let principal = PrincipalId::parse_str("00000000-0000-0000-0000-000000000004").unwrap();
//! requires_trust_root(principal);
//! ```
//!
//! Issuer sessions are process-local capabilities rather than reusable wire
//! tokens:
//!
//! ```compile_fail
//! fn requires_serialize<T: serde::Serialize>() {}
//! requires_serialize::<ecra_identity::IssuerSession>();
//! ```
//!
//! Verified trust snapshots cannot be fabricated by deserializing ordinary
//! metadata:
//!
//! ```compile_fail
//! fn requires_deserialize<T: serde::de::DeserializeOwned>() {}
//! requires_deserialize::<ecra_identity::VerifiedTrustSnapshot>();
//! ```

pub mod algorithm;
pub mod anchor;
pub mod assertion;
pub mod backend;
pub mod bootstrap;
pub mod envelope;
pub mod error;
pub mod ids;
pub mod issuance;
pub mod key;
pub mod validation;

#[cfg(test)]
mod phase3_tests;

use ecra_core::SchemaVersion;

pub use algorithm::{AeadAlgorithm, SignatureAlgorithm};
pub use anchor::{
    PROTECTED_ANCHOR_DOMAIN, canonical_protected_anchor_input, protected_anchor_input_digest_bytes,
};
pub use assertion::{
    ActorBinding, AssertionAttributes, AssertionAudience, AssertionAudienceService,
    AssertionIssuer, AssertionSignature, AudienceInstanceId, IDENTITY_ASSERTION_DIGEST_DOMAIN,
    IDENTITY_ASSERTION_SIGNING_DOMAIN, IdentityAssertionDigest, IdentityAssertionPayloadV1,
    IdentityAssertionV1, MAX_ASSERTION_ATTRIBUTE_KEY_BYTES, MAX_ASSERTION_ATTRIBUTE_VALUE_BYTES,
    MAX_ASSERTION_ATTRIBUTES, MAX_ASSERTION_VALIDITY_MILLIS, MAX_AUDIENCE_INSTANCE_BYTES,
    MAX_IDENTITY_ASSERTION_WIRE_BYTES, MAX_JSON_DEPTH, OnBehalfOfBinding,
    canonical_assertion_signing_input, identity_assertion_digest_bytes, validate_collection_count,
    validate_json_limits,
};
pub use backend::TrustBackendKind;
pub use bootstrap::{EnrolledPrincipalHandle, EnrollmentKind, EnrollmentRecord};
pub use envelope::{
    EnvelopeKeyRef, MAX_PROTECTED_ENVELOPE_WIRE_BYTES, PROTECTED_ENVELOPE_AAD_DOMAIN,
    PROTECTED_ENVELOPE_NONCE_BYTES, PROTECTED_ENVELOPE_TAG_BYTES, ProtectedEnvelopeV1,
    ProtectedInformationClass, ProtectedPurpose, SensitiveBytes,
};
pub use error::{IdentityError, IdentityErrorCategory, IdentityErrorCode};
pub use ids::{
    AssertionNonceId, DelegationId, EnrollmentId, KeyId, ProtectedObjectId, TrustRootId,
};
pub use issuance::{AssertionIssuanceRequest, IssuerSession};
pub use key::{
    KeyPurpose, KeyStatus, MAX_I_JSON_U64, MAX_PROTECTED_TRUST_STATE_KEYS, MAX_REVOKED_KEY_IDS,
    TrustStateDigest, VerifiedAssertionKey, VerifiedTrustSnapshot,
};
pub use validation::{
    IdentityValidationContext, ReplayMode, ReplayValidationInput,
    VALIDATED_IDENTITY_CONTEXT_DOMAIN, ValidatedIdentityContext, ValidatedOnBehalfOf,
    validate_identity_assertion,
};

pub const ECR_031_CONTRACT_MAJOR: u16 = 1;
pub const ECR_031_CONTRACT_MINOR: u16 = 0;
pub const ECR_031_CONTRACT_VERSION: SchemaVersion =
    SchemaVersion::new(ECR_031_CONTRACT_MAJOR, ECR_031_CONTRACT_MINOR);

pub fn validate_ecr031_version(version: SchemaVersion) -> Result<(), IdentityError> {
    if version.major() != ECR_031_CONTRACT_MAJOR || version.minor() > ECR_031_CONTRACT_MINOR {
        return Err(IdentityError::new(
            IdentityErrorCategory::Compatibility,
            IdentityErrorCode::UnsupportedVersion,
            Some("ecr_031_version"),
        ));
    }
    Ok(())
}
