use std::{error::Error, fmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IdentityErrorCategory {
    InvalidInput,
    Compatibility,
    IdentityValidation,
    TrustBackend,
    KeyState,
    CryptographicAuthentication,
    ProtectedStorage,
    Corruption,
    PlatformUnavailable,
    Bootstrap,
    Issuance,
}

impl IdentityErrorCategory {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidInput => "invalid_input",
            Self::Compatibility => "compatibility",
            Self::IdentityValidation => "identity_validation",
            Self::TrustBackend => "trust_backend",
            Self::KeyState => "key_state",
            Self::CryptographicAuthentication => "cryptographic_authentication",
            Self::ProtectedStorage => "protected_storage",
            Self::Corruption => "corruption",
            Self::PlatformUnavailable => "platform_unavailable",
            Self::Bootstrap => "bootstrap",
            Self::Issuance => "issuance",
        }
    }
}

impl fmt::Display for IdentityErrorCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IdentityErrorCode {
    InvalidIdentifier,
    InvalidJson,
    WireLimitExceeded,
    JsonDepthExceeded,
    CollectionLimitExceeded,
    UnsupportedAlgorithm,
    UnsupportedVersion,
    CanonicalizationFailed,
    AssertionSignatureInvalid,
    AssertionTemporalInvalid,
    AssertionExpired,
    AssertionNotYetValid,
    AssertionAudienceMismatch,
    AssertionActorMismatch,
    AssertionPrincipalMismatch,
    AssertionDelegationInvalid,
    AssertionReplayRejected,
    TrustRootUnavailable,
    TrustRootLocked,
    TrustSnapshotAuthenticationFailed,
    TrustSnapshotStaleOrMismatched,
    TrustSnapshotLifecycleInvalid,
    BootstrapIncomplete,
    EnrollmentUnavailable,
    IssuerSessionUnavailable,
    SubjectPrincipalOverrideRejected,
    KeyNotFound,
    KeyNotActive,
    KeyRevoked,
    ProtectedEnvelopeInvalid,
    AuthenticationFailed,
    BackendUnsupported,
    BackendInvariantViolation,
}

impl IdentityErrorCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidIdentifier => "invalid_identifier",
            Self::InvalidJson => "invalid_json",
            Self::WireLimitExceeded => "wire_limit_exceeded",
            Self::JsonDepthExceeded => "json_depth_exceeded",
            Self::CollectionLimitExceeded => "collection_limit_exceeded",
            Self::UnsupportedAlgorithm => "unsupported_algorithm",
            Self::UnsupportedVersion => "unsupported_version",
            Self::CanonicalizationFailed => "canonicalization_failed",
            Self::AssertionSignatureInvalid => "assertion_signature_invalid",
            Self::AssertionTemporalInvalid => "assertion_temporal_invalid",
            Self::AssertionExpired => "assertion_expired",
            Self::AssertionNotYetValid => "assertion_not_yet_valid",
            Self::AssertionAudienceMismatch => "assertion_audience_mismatch",
            Self::AssertionActorMismatch => "assertion_actor_mismatch",
            Self::AssertionPrincipalMismatch => "assertion_principal_mismatch",
            Self::AssertionDelegationInvalid => "assertion_delegation_invalid",
            Self::AssertionReplayRejected => "assertion_replay_rejected",
            Self::TrustRootUnavailable => "trust_root_unavailable",
            Self::TrustRootLocked => "trust_root_locked",
            Self::TrustSnapshotAuthenticationFailed => "trust_snapshot_authentication_failed",
            Self::TrustSnapshotStaleOrMismatched => "trust_snapshot_stale_or_mismatched",
            Self::TrustSnapshotLifecycleInvalid => "trust_snapshot_lifecycle_invalid",
            Self::BootstrapIncomplete => "bootstrap_incomplete",
            Self::EnrollmentUnavailable => "enrollment_unavailable",
            Self::IssuerSessionUnavailable => "issuer_session_unavailable",
            Self::SubjectPrincipalOverrideRejected => "subject_principal_override_rejected",
            Self::KeyNotFound => "key_not_found",
            Self::KeyNotActive => "key_not_active",
            Self::KeyRevoked => "key_revoked",
            Self::ProtectedEnvelopeInvalid => "protected_envelope_invalid",
            Self::AuthenticationFailed => "authentication_failed",
            Self::BackendUnsupported => "backend_unsupported",
            Self::BackendInvariantViolation => "backend_invariant_violation",
        }
    }
}

impl fmt::Display for IdentityErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Public ECR-031 errors contain only closed category/code values and optional
/// static context labels. Raw input, secret bytes and backend payloads are never
/// retained in the public error value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IdentityError {
    category: IdentityErrorCategory,
    code: IdentityErrorCode,
    safe_context: Option<&'static str>,
}

impl IdentityError {
    #[must_use]
    pub const fn new(
        category: IdentityErrorCategory,
        code: IdentityErrorCode,
        safe_context: Option<&'static str>,
    ) -> Self {
        Self {
            category,
            code,
            safe_context,
        }
    }

    #[must_use]
    pub const fn invalid_identifier(kind: &'static str) -> Self {
        Self::new(
            IdentityErrorCategory::InvalidInput,
            IdentityErrorCode::InvalidIdentifier,
            Some(kind),
        )
    }

    #[must_use]
    pub const fn category(self) -> IdentityErrorCategory {
        self.category
    }

    #[must_use]
    pub const fn code(self) -> IdentityErrorCode {
        self.code
    }

    #[must_use]
    pub const fn safe_context(self) -> Option<&'static str> {
        self.safe_context
    }
}

impl fmt::Display for IdentityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "identity error [{}/{}]",
            self.category, self.code
        )?;
        if let Some(context) = self.safe_context {
            write!(formatter, " ({context})")?;
        }
        Ok(())
    }
}

impl Error for IdentityError {}
