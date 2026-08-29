use std::collections::BTreeSet;

use ecra_core::{EvidenceKind, VerificationMethod, VerificationOutcome, VerificationTarget};
use serde::{Deserialize, Serialize};

use crate::{
    VerificationRequestV1, VerifyError, VerifyErrorCategory, VerifyErrorCode,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionGradeStatusV1 {
    DecisionGrade,
    NonDecisionGrade,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionGradeReasonV1 {
    MissingEvidenceBinding,
    MissingImmutableBinding,
    MissingEvaluationTime,
    MissingFreshness,
    EvidenceFromFuture,
    EvidenceStale,
    SelfAttestingExecutionReceipt,
    ModelJudgmentRequiresIndependentEvidence,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DecisionGradeAssessmentV1 {
    status: DecisionGradeStatusV1,
    reasons: Vec<DecisionGradeReasonV1>,
}

impl DecisionGradeAssessmentV1 {
    fn from_reasons(reasons: BTreeSet<DecisionGradeReasonV1>) -> Self {
        if reasons.is_empty() {
            Self {
                status: DecisionGradeStatusV1::DecisionGrade,
                reasons: Vec::new(),
            }
        } else {
            Self {
                status: DecisionGradeStatusV1::NonDecisionGrade,
                reasons: reasons.into_iter().collect(),
            }
        }
    }

    #[must_use]
    pub const fn status(&self) -> DecisionGradeStatusV1 {
        self.status
    }

    #[must_use]
    pub fn reasons(&self) -> &[DecisionGradeReasonV1] {
        &self.reasons
    }

    #[must_use]
    pub const fn is_decision_grade(&self) -> bool {
        matches!(self.status, DecisionGradeStatusV1::DecisionGrade)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreshnessRuleV1 {
    max_age_millis: u64,
}

impl FreshnessRuleV1 {
    #[must_use]
    pub const fn new(max_age_millis: u64) -> Self {
        Self { max_age_millis }
    }

    #[must_use]
    pub const fn max_age_millis(self) -> u64 {
        self.max_age_millis
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DecisionGradeRuleV1 {
    require_immutable_binding: bool,
    freshness: Option<FreshnessRuleV1>,
}

impl DecisionGradeRuleV1 {
    #[must_use]
    pub const fn new(
        require_immutable_binding: bool,
        freshness: Option<FreshnessRuleV1>,
    ) -> Self {
        Self {
            require_immutable_binding,
            freshness,
        }
    }

    #[must_use]
    pub const fn standard() -> Self {
        Self::new(true, None)
    }

    #[must_use]
    pub const fn freshness(self) -> Option<FreshnessRuleV1> {
        self.freshness
    }
}

pub fn assess_request(
    request: &VerificationRequestV1,
    rule: DecisionGradeRuleV1,
) -> Result<DecisionGradeAssessmentV1, VerifyError> {
    let evidence = request.evidence();
    let mut ids = BTreeSet::new();
    let mut reasons = BTreeSet::new();
    let mut any_immutable = false;
    let mut any_independent_non_model = false;
    let mut only_self_attesting_receipt = !evidence.is_empty();

    for item in evidence {
        if !ids.insert(item.id()) {
            return Err(VerifyError::new(
                VerifyErrorCategory::Evidence,
                VerifyErrorCode::DuplicateId,
                "decision-grade assessment received duplicate evidence ids",
            ));
        }

        let has_binding = item.artifact().is_some()
            || item.observation().is_some()
            || item.receipt().is_some()
            || item.external_ref().is_some()
            || item.content_digest().is_some();
        if !has_binding {
            reasons.insert(DecisionGradeReasonV1::MissingEvidenceBinding);
        }

        let immutable = item.artifact().is_some() || item.content_digest().is_some();
        any_immutable |= immutable;
        any_independent_non_model |= !matches!(item.kind(), EvidenceKind::ModelJudgment)
            && (immutable || item.observation().is_some() || item.external_ref().is_some());

        let is_same_receipt = match request.target() {
            VerificationTarget::Receipt(target) => item.receipt() == Some(*target),
            _ => false,
        };
        if !is_same_receipt || immutable || item.observation().is_some() {
            only_self_attesting_receipt = false;
        }

        if let Some(freshness) = rule.freshness() {
            match (request.evaluated_at(), item.as_of()) {
                (None, _) => {
                    reasons.insert(DecisionGradeReasonV1::MissingEvaluationTime);
                }
                (Some(_), None) => {
                    reasons.insert(DecisionGradeReasonV1::MissingFreshness);
                }
                (Some(evaluated_at), Some(as_of)) => {
                    let evaluated = i128::from(evaluated_at.get());
                    let observed = i128::from(as_of.get());
                    if observed > evaluated {
                        reasons.insert(DecisionGradeReasonV1::EvidenceFromFuture);
                    } else if u128::try_from(evaluated - observed)
                        .is_ok_and(|age| age > u128::from(freshness.max_age_millis()))
                    {
                        reasons.insert(DecisionGradeReasonV1::EvidenceStale);
                    }
                }
            }
        }
    }

    if matches!(
        request.proposed_outcome(),
        VerificationOutcome::Verified | VerificationOutcome::Rejected
    ) {
        if evidence.is_empty() {
            reasons.insert(DecisionGradeReasonV1::MissingEvidenceBinding);
        }
        if rule.require_immutable_binding && !any_immutable {
            reasons.insert(DecisionGradeReasonV1::MissingImmutableBinding);
        }
        if only_self_attesting_receipt {
            return Err(VerifyError::new(
                VerifyErrorCategory::Evidence,
                VerifyErrorCode::SelfAttestingReceipt,
                "an execution receipt cannot alone prove its own conclusive verification claim",
            ));
        }
        if request.method() == VerificationMethod::IndependentModelJudgment
            && !any_independent_non_model
        {
            reasons.insert(DecisionGradeReasonV1::ModelJudgmentRequiresIndependentEvidence);
        }
    }

    Ok(DecisionGradeAssessmentV1::from_reasons(reasons))
}

pub fn verify_request(
    request: &VerificationRequestV1,
    rule: DecisionGradeRuleV1,
) -> Result<ecra_core::VerificationReceipt, VerifyError> {
    let assessment = assess_request(request, rule)?;
    if matches!(
        request.proposed_outcome(),
        VerificationOutcome::Verified | VerificationOutcome::Rejected
    ) && !assessment.is_decision_grade()
    {
        return Err(VerifyError::new(
            VerifyErrorCategory::Evidence,
            VerifyErrorCode::EvidenceInsufficient,
            "conclusive verification requires decision-grade evidence",
        ));
    }
    request.construct_receipt()
}
