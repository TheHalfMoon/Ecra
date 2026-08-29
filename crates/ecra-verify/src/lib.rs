#![forbid(unsafe_code)]

//! Independent verification and reconciliation foundations for Ecra.
//!
//! ECR-004 consumes canonical ECR-001 verification/evidence values and
//! read-only ECR-002 run/attempt truth. It does not authorize execution,
//! fabricate provider receipts, mutate ECR-002 run state, or acquire remote
//! evidence.
//!
//! Semantic verification modules remain pure. Local journal I/O is isolated
//! behind the later `store` module and must not leak provider/runtime behavior
//! into verification logic.

pub mod aggregate;
pub mod error;
pub mod evidence;
pub mod ids;
pub mod request;

pub use aggregate::{
    MAX_RECEIPTS_PER_TARGET, VerificationAggregateStateV1, VerificationAggregateViewV1,
};
pub use error::{VerifyError, VerifyErrorCategory, VerifyErrorCode};
pub use evidence::{
    DecisionGradeAssessmentV1, DecisionGradeReasonV1, DecisionGradeRuleV1,
    DecisionGradeStatusV1, FreshnessRuleV1, assess_request, verify_request,
};
pub use ids::{CheckpointId, ReconciliationId};
pub use request::{
    MAX_EVIDENCE_REFS_PER_REQUEST, MAX_NOTES_BYTES, MAX_RULE_ID_BYTES, VerificationRequestFieldsV1,
    VerificationRequestV1,
};

/// Phase 1 remains inspectable as implementation history; returning `true`
/// carries no semantic verification or authorization meaning.
#[must_use]
pub const fn phase_one_scaffold() -> bool {
    true
}
