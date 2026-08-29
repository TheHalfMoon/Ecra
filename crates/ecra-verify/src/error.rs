use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerifyErrorCategory {
    Validation,
    Verification,
    Evidence,
    Aggregation,
    Reconciliation,
    Persistence,
    Compatibility,
    ResourceLimit,
}

impl VerifyErrorCategory {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Validation => "validation",
            Self::Verification => "verification",
            Self::Evidence => "evidence",
            Self::Aggregation => "aggregation",
            Self::Reconciliation => "reconciliation",
            Self::Persistence => "persistence",
            Self::Compatibility => "compatibility",
            Self::ResourceLimit => "resource_limit",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerifyErrorCode {
    UnsupportedVersion,
    InvalidIdentifier,
    InvalidTarget,
    InvalidEvidence,
    EvidenceInsufficient,
    SelfAttestingReceipt,
    VerificationConflict,
    DuplicateId,
    AttemptBindingMismatch,
    ReconciliationUnresolved,
    RetryBlocked,
    JournalSequenceMismatch,
    JournalDigestMismatch,
    StoreCorrupt,
    ResourceLimitExceeded,
}

impl VerifyErrorCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedVersion => "unsupported_version",
            Self::InvalidIdentifier => "invalid_identifier",
            Self::InvalidTarget => "invalid_target",
            Self::InvalidEvidence => "invalid_evidence",
            Self::EvidenceInsufficient => "evidence_insufficient",
            Self::SelfAttestingReceipt => "self_attesting_receipt",
            Self::VerificationConflict => "verification_conflict",
            Self::DuplicateId => "duplicate_id",
            Self::AttemptBindingMismatch => "attempt_binding_mismatch",
            Self::ReconciliationUnresolved => "reconciliation_unresolved",
            Self::RetryBlocked => "retry_blocked",
            Self::JournalSequenceMismatch => "journal_sequence_mismatch",
            Self::JournalDigestMismatch => "journal_digest_mismatch",
            Self::StoreCorrupt => "store_corrupt",
            Self::ResourceLimitExceeded => "resource_limit_exceeded",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifyError {
    category: VerifyErrorCategory,
    code: VerifyErrorCode,
    diagnostic: &'static str,
}

impl VerifyError {
    #[must_use]
    pub const fn new(
        category: VerifyErrorCategory,
        code: VerifyErrorCode,
        diagnostic: &'static str,
    ) -> Self {
        Self {
            category,
            code,
            diagnostic,
        }
    }

    #[must_use]
    pub const fn category(&self) -> VerifyErrorCategory {
        self.category
    }

    #[must_use]
    pub const fn code(&self) -> VerifyErrorCode {
        self.code
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &'static str {
        self.diagnostic
    }
}

impl fmt::Display for VerifyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}: {}",
            self.category.as_str(),
            self.code.as_str(),
            self.diagnostic
        )
    }
}

impl Error for VerifyError {}
