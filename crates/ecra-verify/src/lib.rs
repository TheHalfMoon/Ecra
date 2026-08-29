#![forbid(unsafe_code)]

//! Independent verification and reconciliation foundations for Ecra.
//!
//! ECR-004 consumes canonical ECR-001 verification/evidence values and
//! read-only ECR-002 run/attempt truth. It does not authorize execution,
//! fabricate provider receipts, mutate ECR-002 run state, or acquire remote
//! evidence.
//!
//! Semantic verification modules remain pure. Local journal I/O is isolated
//! behind the `store` module and must not leak provider/runtime behavior into
//! verification logic.

pub mod aggregate;
pub mod checkpoint;
pub mod error;
pub mod evidence;
pub mod ids;
pub mod journal;
pub mod reconcile;
pub mod request;
pub mod store;

pub use aggregate::{
    MAX_RECEIPTS_PER_TARGET, VerificationAggregateStateV1, VerificationAggregateViewV1,
};
pub use checkpoint::{
    CheckpointEvaluationV1, MAX_ACCEPTED_STATES_PER_REQUIREMENT, MAX_CHECKPOINT_LABEL_BYTES,
    MAX_CHECKPOINT_REQUIREMENTS, MAX_VERIFICATION_CHECKPOINT_BYTES,
    VerificationCheckpointFieldsV1, VerificationCheckpointV1, VerificationRequirementV1,
};
pub use error::{VerifyError, VerifyErrorCategory, VerifyErrorCode};
pub use evidence::{
    DecisionGradeAssessmentV1, DecisionGradeReasonV1, DecisionGradeRuleV1, DecisionGradeStatusV1,
    FreshnessRuleV1, assess_request, verify_request,
};
pub use ids::{CheckpointId, ReconciliationId};
pub use journal::{
    MAX_VERIFICATION_JOURNAL_ENTRY_BYTES, MAX_VERIFICATION_JOURNAL_SEQUENCE,
    VerificationJournalBodyV1, VerificationJournalDigest, VerificationJournalDigestAlgorithm,
    VerificationJournalEntryV1, VerificationJournalSequence,
};
pub use reconcile::{
    MAX_RECONCILIATION_AVAILABLE_RECEIPTS, MAX_RECONCILIATION_NOTES_BYTES,
    MAX_RECONCILIATION_SUPPORT_IDS, ReconciliationInputV1, ReconciliationOutcomeV1,
    ReconciliationRecordFieldsV1, ReconciliationRecordV1, RetryDispositionV1, reconcile,
    retry_disposition,
};
pub use request::{
    MAX_EVIDENCE_REFS_PER_REQUEST, MAX_NOTES_BYTES, MAX_RULE_ID_BYTES,
    MAX_VERIFICATION_REQUEST_BYTES, VerificationRequestFieldsV1, VerificationRequestV1,
};
pub use store::{
    ECR_VERIFY_SCHEMA_VERSION, ExpectedVerificationHead, MAX_MATERIALIZED_JOURNAL_ENTRIES,
    VerificationSnapshotV1, VerificationStore,
};

/// Phase 1 remains inspectable as implementation history; returning `true`
/// carries no semantic verification or authorization meaning.
#[must_use]
pub const fn phase_one_scaffold() -> bool {
    true
}
