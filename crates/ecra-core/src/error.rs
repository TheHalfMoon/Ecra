use thiserror::Error;

/// Stable machine-readable error categories for the trusted domain kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    Compatibility,
    Identifier,
    Identity,
    Origin,
    Resource,
    Scope,
    Capability,
    Temporal,
    Information,
    Canonicalization,
    Digest,
    Action,
    Attempt,
    Receipt,
    Verification,
    Serialization,
}

/// Stable machine-readable error codes. Display text is intentionally not an API contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    UnsupportedMajorVersion,
    UnsupportedMinorVersion,
    InvalidIdentifier,
    InvalidEpochMillis,
    InvalidTemporalRange,
    InvalidOrigin,
    InvalidResource,
    InvalidScope,
    InvalidCapability,
    InvalidIdentity,
    InvalidInformation,
    CanonicalizationFailed,
    InvalidContentDigest,
    InvalidSecurityDigest,
    InvalidAction,
    InvalidAttempt,
    InvalidReceipt,
    InvalidVerification,
    SerializationFailed,
}

impl ErrorCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedMajorVersion => "unsupported_major_version",
            Self::UnsupportedMinorVersion => "unsupported_minor_version",
            Self::InvalidIdentifier => "invalid_identifier",
            Self::InvalidEpochMillis => "invalid_epoch_millis",
            Self::InvalidTemporalRange => "invalid_temporal_range",
            Self::InvalidOrigin => "invalid_origin",
            Self::InvalidResource => "invalid_resource",
            Self::InvalidScope => "invalid_scope",
            Self::InvalidCapability => "invalid_capability",
            Self::InvalidIdentity => "invalid_identity",
            Self::InvalidInformation => "invalid_information",
            Self::CanonicalizationFailed => "canonicalization_failed",
            Self::InvalidContentDigest => "invalid_content_digest",
            Self::InvalidSecurityDigest => "invalid_security_digest",
            Self::InvalidAction => "invalid_action",
            Self::InvalidAttempt => "invalid_attempt",
            Self::InvalidReceipt => "invalid_receipt",
            Self::InvalidVerification => "invalid_verification",
            Self::SerializationFailed => "serialization_failed",
        }
    }

    #[must_use]
    pub const fn category(self) -> ErrorCategory {
        match self {
            Self::UnsupportedMajorVersion | Self::UnsupportedMinorVersion => {
                ErrorCategory::Compatibility
            }
            Self::InvalidIdentifier => ErrorCategory::Identifier,
            Self::InvalidEpochMillis | Self::InvalidTemporalRange => ErrorCategory::Temporal,
            Self::InvalidOrigin => ErrorCategory::Origin,
            Self::InvalidResource => ErrorCategory::Resource,
            Self::InvalidScope => ErrorCategory::Scope,
            Self::InvalidCapability => ErrorCategory::Capability,
            Self::InvalidIdentity => ErrorCategory::Identity,
            Self::InvalidInformation => ErrorCategory::Information,
            Self::CanonicalizationFailed => ErrorCategory::Canonicalization,
            Self::InvalidContentDigest | Self::InvalidSecurityDigest => ErrorCategory::Digest,
            Self::InvalidAction => ErrorCategory::Action,
            Self::InvalidAttempt => ErrorCategory::Attempt,
            Self::InvalidReceipt => ErrorCategory::Receipt,
            Self::InvalidVerification => ErrorCategory::Verification,
            Self::SerializationFailed => ErrorCategory::Serialization,
        }
    }
}

/// Structured kernel errors. Callers should branch on [`DomainError::code`], never display text.
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("unsupported schema major version {actual}; expected {supported}")]
    UnsupportedMajorVersion { supported: u16, actual: u16 },

    #[error("unsupported schema minor version {actual}; maximum supported is {supported}")]
    UnsupportedMinorVersion { supported: u16, actual: u16 },

    #[error("invalid {kind} identifier: {value}")]
    InvalidIdentifier { kind: &'static str, value: String },

    #[error("epoch milliseconds outside I-JSON exact integer range: {value}")]
    InvalidEpochMillis { value: i64 },

    #[error("temporal validity has not_before after expires_at")]
    InvalidTemporalRange,

    #[error("invalid origin: {0}")]
    InvalidOrigin(String),

    #[error("invalid resource: {0}")]
    InvalidResource(String),

    #[error("invalid scope: {0}")]
    InvalidScope(String),

    #[error("invalid capability: {0}")]
    InvalidCapability(String),

    #[error("invalid identity reference: {0}")]
    InvalidIdentity(String),

    #[error("invalid information value: {0}")]
    InvalidInformation(String),

    #[error("canonicalization failed: {0}")]
    Canonicalization(String),

    #[error("invalid content digest: {0}")]
    InvalidContentDigest(String),

    #[error("invalid security digest: {0}")]
    InvalidSecurityDigest(String),

    #[error("invalid action: {0}")]
    InvalidAction(String),

    #[error("invalid action attempt: {0}")]
    InvalidAttempt(String),

    #[error("invalid action receipt: {0}")]
    InvalidReceipt(String),

    #[error("invalid verification receipt: {0}")]
    InvalidVerification(String),

    #[error("serialization failed: {0}")]
    Serialization(String),
}

impl DomainError {
    #[must_use]
    pub const fn code(&self) -> ErrorCode {
        match self {
            Self::UnsupportedMajorVersion { .. } => ErrorCode::UnsupportedMajorVersion,
            Self::UnsupportedMinorVersion { .. } => ErrorCode::UnsupportedMinorVersion,
            Self::InvalidIdentifier { .. } => ErrorCode::InvalidIdentifier,
            Self::InvalidEpochMillis { .. } => ErrorCode::InvalidEpochMillis,
            Self::InvalidTemporalRange => ErrorCode::InvalidTemporalRange,
            Self::InvalidOrigin(_) => ErrorCode::InvalidOrigin,
            Self::InvalidResource(_) => ErrorCode::InvalidResource,
            Self::InvalidScope(_) => ErrorCode::InvalidScope,
            Self::InvalidCapability(_) => ErrorCode::InvalidCapability,
            Self::InvalidIdentity(_) => ErrorCode::InvalidIdentity,
            Self::InvalidInformation(_) => ErrorCode::InvalidInformation,
            Self::Canonicalization(_) => ErrorCode::CanonicalizationFailed,
            Self::InvalidContentDigest(_) => ErrorCode::InvalidContentDigest,
            Self::InvalidSecurityDigest(_) => ErrorCode::InvalidSecurityDigest,
            Self::InvalidAction(_) => ErrorCode::InvalidAction,
            Self::InvalidAttempt(_) => ErrorCode::InvalidAttempt,
            Self::InvalidReceipt(_) => ErrorCode::InvalidReceipt,
            Self::InvalidVerification(_) => ErrorCode::InvalidVerification,
            Self::Serialization(_) => ErrorCode::SerializationFailed,
        }
    }

    #[must_use]
    pub const fn category(&self) -> ErrorCategory {
        self.code().category()
    }
}
