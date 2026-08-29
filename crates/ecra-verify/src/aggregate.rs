use std::collections::BTreeSet;

use ecra_core::{VerificationId, VerificationOutcome, VerificationReceipt, VerificationTarget};
use serde::{Deserialize, Serialize};

use crate::{VerifyError, VerifyErrorCategory, VerifyErrorCode};

pub const MAX_RECEIPTS_PER_TARGET: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationAggregateStateV1 {
    Absent,
    Verified,
    Rejected,
    Inconclusive,
    Conflicted,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationAggregateViewV1 {
    target: VerificationTarget,
    state: VerificationAggregateStateV1,
    receipt_ids: Vec<VerificationId>,
    verified_ids: Vec<VerificationId>,
    rejected_ids: Vec<VerificationId>,
    inconclusive_ids: Vec<VerificationId>,
    not_evaluated_ids: Vec<VerificationId>,
}

impl VerificationAggregateViewV1 {
    pub fn from_receipts(
        target: VerificationTarget,
        receipts: &[VerificationReceipt],
    ) -> Result<Self, VerifyError> {
        if receipts.len() > MAX_RECEIPTS_PER_TARGET {
            return Err(VerifyError::new(
                VerifyErrorCategory::ResourceLimit,
                VerifyErrorCode::ResourceLimitExceeded,
                "verification aggregate receipt count exceeds the v1 limit",
            ));
        }

        let mut receipt_ids = BTreeSet::new();
        let mut verified_ids = BTreeSet::new();
        let mut rejected_ids = BTreeSet::new();
        let mut inconclusive_ids = BTreeSet::new();
        let mut not_evaluated_ids = BTreeSet::new();

        for receipt in receipts {
            if receipt.target() != &target {
                return Err(VerifyError::new(
                    VerifyErrorCategory::Aggregation,
                    VerifyErrorCode::InvalidTarget,
                    "verification aggregate contains a receipt for a different target",
                ));
            }
            if !receipt_ids.insert(receipt.id()) {
                return Err(VerifyError::new(
                    VerifyErrorCategory::Aggregation,
                    VerifyErrorCode::DuplicateId,
                    "verification aggregate contains a duplicate receipt id",
                ));
            }
            match receipt.outcome() {
                VerificationOutcome::Verified => {
                    verified_ids.insert(receipt.id());
                }
                VerificationOutcome::Rejected => {
                    rejected_ids.insert(receipt.id());
                }
                VerificationOutcome::Inconclusive => {
                    inconclusive_ids.insert(receipt.id());
                }
                VerificationOutcome::NotEvaluated => {
                    not_evaluated_ids.insert(receipt.id());
                }
            }
        }

        let state = match (
            !verified_ids.is_empty(),
            !rejected_ids.is_empty(),
            !inconclusive_ids.is_empty(),
        ) {
            (true, true, _) => VerificationAggregateStateV1::Conflicted,
            (true, false, _) => VerificationAggregateStateV1::Verified,
            (false, true, _) => VerificationAggregateStateV1::Rejected,
            (false, false, true) => VerificationAggregateStateV1::Inconclusive,
            (false, false, false) => VerificationAggregateStateV1::Absent,
        };

        Ok(Self {
            target,
            state,
            receipt_ids: receipt_ids.into_iter().collect(),
            verified_ids: verified_ids.into_iter().collect(),
            rejected_ids: rejected_ids.into_iter().collect(),
            inconclusive_ids: inconclusive_ids.into_iter().collect(),
            not_evaluated_ids: not_evaluated_ids.into_iter().collect(),
        })
    }

    #[must_use]
    pub const fn target(&self) -> &VerificationTarget {
        &self.target
    }

    #[must_use]
    pub const fn state(&self) -> VerificationAggregateStateV1 {
        self.state
    }

    #[must_use]
    pub fn receipt_ids(&self) -> &[VerificationId] {
        &self.receipt_ids
    }

    #[must_use]
    pub fn verified_ids(&self) -> &[VerificationId] {
        &self.verified_ids
    }

    #[must_use]
    pub fn rejected_ids(&self) -> &[VerificationId] {
        &self.rejected_ids
    }

    #[must_use]
    pub fn inconclusive_ids(&self) -> &[VerificationId] {
        &self.inconclusive_ids
    }

    #[must_use]
    pub fn not_evaluated_ids(&self) -> &[VerificationId] {
        &self.not_evaluated_ids
    }
}
