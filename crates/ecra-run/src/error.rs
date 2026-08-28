use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunErrorCategory {
    Compatibility,
    Event,
    State,
    Attempt,
    Ledger,
    Storage,
    Migration,
    Budget,
    Archive,
    Integrity,
    Recovery,
    Serialization,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunErrorCode {
    UnsupportedMajorVersion,
    UnsupportedMinorVersion,
    InvalidEventSequence,
    InvalidEvent,
    InvalidStateTransition,
    DuplicateAttempt,
    AttemptBindingMismatch,
    ReceiptBindingMismatch,
    UnresolvedAttempt,
    BlindRetryForbidden,
    LedgerHeadMismatch,
    LedgerChainInvalid,
    LedgerDigestMismatch,
    StoreConfigurationInvalid,
    StoreBusy,
    StorageError,
    UnsupportedStoreVersion,
    MigrationFailed,
    InvalidBudget,
    BudgetOverflow,
    BudgetPreflightExceeded,
    BudgetExhausted,
    ArchivePathInvalid,
    ArchiveDuplicateEntry,
    ArchiveFeatureUnsupported,
    ArchiveLimitExceeded,
    ArchiveManifestInvalid,
    ArchiveDigestMismatch,
    RecoveryRequired,
    SerializationFailed,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
#[error("{category:?}/{code:?}: {message}")]
pub struct RunError {
    category: RunErrorCategory,
    code: RunErrorCode,
    message: String,
}

impl RunError {
    #[must_use]
    pub fn new(category: RunErrorCategory, code: RunErrorCode, message: impl Into<String>) -> Self {
        Self {
            category,
            code,
            message: message.into(),
        }
    }

    #[must_use]
    pub const fn category(&self) -> RunErrorCategory {
        self.category
    }

    #[must_use]
    pub const fn code(&self) -> RunErrorCode {
        self.code
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub fn serialization(message: impl Into<String>) -> Self {
        Self::new(
            RunErrorCategory::Serialization,
            RunErrorCode::SerializationFailed,
            message,
        )
    }

    #[must_use]
    pub fn invalid_event_sequence(message: impl Into<String>) -> Self {
        Self::new(
            RunErrorCategory::Event,
            RunErrorCode::InvalidEventSequence,
            message,
        )
    }

    #[must_use]
    pub fn invalid_event(message: impl Into<String>) -> Self {
        Self::new(RunErrorCategory::Event, RunErrorCode::InvalidEvent, message)
    }

    #[must_use]
    pub fn invalid_budget(message: impl Into<String>) -> Self {
        Self::new(
            RunErrorCategory::Budget,
            RunErrorCode::InvalidBudget,
            message,
        )
    }

    #[must_use]
    pub fn ledger_chain_invalid(message: impl Into<String>) -> Self {
        Self::new(
            RunErrorCategory::Ledger,
            RunErrorCode::LedgerChainInvalid,
            message,
        )
    }

    #[must_use]
    pub fn ledger_digest_mismatch(message: impl Into<String>) -> Self {
        Self::new(
            RunErrorCategory::Ledger,
            RunErrorCode::LedgerDigestMismatch,
            message,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunErrorSummary {
    category: RunErrorCategory,
    code: RunErrorCode,
    message: Option<String>,
}

impl RunErrorSummary {
    #[must_use]
    pub const fn new(
        category: RunErrorCategory,
        code: RunErrorCode,
        message: Option<String>,
    ) -> Self {
        Self {
            category,
            code,
            message,
        }
    }

    #[must_use]
    pub const fn category(&self) -> RunErrorCategory {
        self.category
    }

    #[must_use]
    pub const fn code(&self) -> RunErrorCode {
        self.code
    }

    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
