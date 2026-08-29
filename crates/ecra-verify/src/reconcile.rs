use std::collections::{BTreeMap, BTreeSet};

use ecra_core::{
    ActionAttemptRef, ActionIntent, ActionRef, EpochMillis, IdempotencyClass, RetryClass, RunId,
    SchemaVersion, VerificationId, VerificationOutcome, VerificationReceipt, VerificationTarget,
};
use ecra_run::RunState;
use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{ReconciliationId, VerifyError, VerifyErrorCategory, VerifyErrorCode};

pub const MAX_RECONCILIATION_SUPPORT_IDS: usize = 64;
pub const MAX_RECONCILIATION_NOTES_BYTES: usize = 4_096;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReconciliationOutcomeV1 {
    EffectConfirmed,
    NoEffectConfirmed,
    StillUnknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReconciliationRecordFieldsV1 {
    pub id: ReconciliationId,
    pub run_id: RunId,
    pub attempt: ActionAttemptRef,
    pub action: ActionRef,
    pub outcome: ReconciliationOutcomeV1,
    pub verification_receipts: Vec<VerificationId>,
    pub reconciled_at: Option<EpochMillis>,
    pub notes: Option<String>,
}

/// Non-persisted construction input. The reconciliation outcome is derived
/// from resolved canonical verification receipts and is never caller-selected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReconciliationInputV1 {
    pub id: ReconciliationId,
    pub run_id: RunId,
    pub attempt: ActionAttemptRef,
    pub action: ActionRef,
    pub verification_receipts: Vec<VerificationId>,
    pub reconciled_at: Option<EpochMillis>,
    pub notes: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReconciliationRecordV1 {
    version: SchemaVersion,
    id: ReconciliationId,
    run_id: RunId,
    attempt: ActionAttemptRef,
    action: ActionRef,
    outcome: ReconciliationOutcomeV1,
    verification_receipts: Vec<VerificationId>,
    reconciled_at: Option<EpochMillis>,
    notes: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ReconciliationRecordWire {
    version: SchemaVersion,
    id: ReconciliationId,
    run_id: RunId,
    attempt: ActionAttemptRef,
    action: ActionRef,
    outcome: ReconciliationOutcomeV1,
    verification_receipts: Vec<VerificationId>,
    reconciled_at: Option<EpochMillis>,
    notes: Option<String>,
}

impl ReconciliationRecordV1 {
    pub fn from_fields(fields: ReconciliationRecordFieldsV1) -> Result<Self, VerifyError> {
        Self::validate_static(SchemaVersion::V1_0, fields)
    }

    fn validate_static(
        version: SchemaVersion,
        mut fields: ReconciliationRecordFieldsV1,
    ) -> Result<Self, VerifyError> {
        if version.validate_supported().is_err() || version != SchemaVersion::V1_0 {
            return Err(VerifyError::new(
                VerifyErrorCategory::Compatibility,
                VerifyErrorCode::UnsupportedVersion,
                "reconciliation record version is not supported",
            ));
        }
        if fields.attempt.action() != &fields.action {
            return Err(binding_error(
                "reconciliation attempt does not bind the supplied action",
            ));
        }
        if fields.verification_receipts.len() > MAX_RECONCILIATION_SUPPORT_IDS {
            return Err(VerifyError::new(
                VerifyErrorCategory::ResourceLimit,
                VerifyErrorCode::ResourceLimitExceeded,
                "reconciliation support receipt count exceeds the v1 limit",
            ));
        }
        let mut support_ids = BTreeSet::new();
        for id in fields.verification_receipts.drain(..) {
            if !support_ids.insert(id) {
                return Err(VerifyError::new(
                    VerifyErrorCategory::Reconciliation,
                    VerifyErrorCode::DuplicateId,
                    "reconciliation support receipt ids must be unique",
                ));
            }
        }
        fields.verification_receipts = support_ids.into_iter().collect();
        if !matches!(fields.outcome, ReconciliationOutcomeV1::StillUnknown)
            && fields.verification_receipts.is_empty()
        {
            return Err(VerifyError::new(
                VerifyErrorCategory::Evidence,
                VerifyErrorCode::EvidenceInsufficient,
                "conclusive reconciliation requires supporting verification receipts",
            ));
        }
        if let Some(notes) = fields.notes.as_ref() {
            if notes.is_empty() {
                return Err(VerifyError::new(
                    VerifyErrorCategory::Validation,
                    VerifyErrorCode::InvalidEvidence,
                    "reconciliation notes must be non-empty when present",
                ));
            }
            if notes.len() > MAX_RECONCILIATION_NOTES_BYTES {
                return Err(VerifyError::new(
                    VerifyErrorCategory::ResourceLimit,
                    VerifyErrorCode::ResourceLimitExceeded,
                    "reconciliation notes exceed the v1 byte limit",
                ));
            }
        }

        Ok(Self {
            version,
            id: fields.id,
            run_id: fields.run_id,
            attempt: fields.attempt,
            action: fields.action,
            outcome: fields.outcome,
            verification_receipts: fields.verification_receipts,
            reconciled_at: fields.reconciled_at,
            notes: fields.notes,
        })
    }

    pub fn from_json_slice(input: &[u8]) -> Result<Self, VerifyError> {
        let wire: ReconciliationRecordWire = serde_json::from_slice(input).map_err(|_| {
            VerifyError::new(
                VerifyErrorCategory::Validation,
                VerifyErrorCode::InvalidEvidence,
                "reconciliation record JSON is malformed or contains unsupported fields",
            )
        })?;
        Self::validate_static(
            wire.version,
            ReconciliationRecordFieldsV1 {
                id: wire.id,
                run_id: wire.run_id,
                attempt: wire.attempt,
                action: wire.action,
                outcome: wire.outcome,
                verification_receipts: wire.verification_receipts,
                reconciled_at: wire.reconciled_at,
                notes: wire.notes,
            },
        )
    }

    pub fn validate_against(
        &self,
        state: &RunState,
        available_receipts: &[VerificationReceipt],
    ) -> Result<(), VerifyError> {
        validate_state_binding(self.run_id, &self.attempt, &self.action, state)?;
        let resolved = resolve_support(
            &self.attempt,
            &self.verification_receipts,
            available_receipts,
        )?;
        let expected = derive_outcome(&resolved);
        if expected != self.outcome {
            return Err(VerifyError::new(
                VerifyErrorCategory::Reconciliation,
                VerifyErrorCode::VerificationConflict,
                "reconciliation outcome does not match resolved verification evidence",
            ));
        }
        Ok(())
    }

    #[must_use]
    pub const fn version(&self) -> SchemaVersion {
        self.version
    }

    #[must_use]
    pub const fn id(&self) -> ReconciliationId {
        self.id
    }

    #[must_use]
    pub const fn run_id(&self) -> RunId {
        self.run_id
    }

    #[must_use]
    pub const fn attempt(&self) -> &ActionAttemptRef {
        &self.attempt
    }

    #[must_use]
    pub const fn action(&self) -> &ActionRef {
        &self.action
    }

    #[must_use]
    pub const fn outcome(&self) -> ReconciliationOutcomeV1 {
        self.outcome
    }

    #[must_use]
    pub fn verification_receipts(&self) -> &[VerificationId] {
        &self.verification_receipts
    }

    #[must_use]
    pub const fn reconciled_at(&self) -> Option<EpochMillis> {
        self.reconciled_at
    }

    #[must_use]
    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }
}

impl<'de> Deserialize<'de> for ReconciliationRecordV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ReconciliationRecordWire::deserialize(deserializer)?;
        Self::validate_static(
            wire.version,
            ReconciliationRecordFieldsV1 {
                id: wire.id,
                run_id: wire.run_id,
                attempt: wire.attempt,
                action: wire.action,
                outcome: wire.outcome,
                verification_receipts: wire.verification_receipts,
                reconciled_at: wire.reconciled_at,
                notes: wire.notes,
            },
        )
        .map_err(de::Error::custom)
    }
}

pub fn reconcile(
    input: ReconciliationInputV1,
    state: &RunState,
    available_receipts: &[VerificationReceipt],
) -> Result<ReconciliationRecordV1, VerifyError> {
    validate_state_binding(input.run_id, &input.attempt, &input.action, state)?;
    let support_ids = canonical_support_ids(input.verification_receipts)?;
    let resolved = resolve_support(&input.attempt, &support_ids, available_receipts)?;
    let outcome = derive_outcome(&resolved);
    ReconciliationRecordV1::from_fields(ReconciliationRecordFieldsV1 {
        id: input.id,
        run_id: input.run_id,
        attempt: input.attempt,
        action: input.action,
        outcome,
        verification_receipts: support_ids,
        reconciled_at: input.reconciled_at,
        notes: input.notes,
    })
}

fn canonical_support_ids(ids: Vec<VerificationId>) -> Result<Vec<VerificationId>, VerifyError> {
    if ids.len() > MAX_RECONCILIATION_SUPPORT_IDS {
        return Err(VerifyError::new(
            VerifyErrorCategory::ResourceLimit,
            VerifyErrorCode::ResourceLimitExceeded,
            "reconciliation support receipt count exceeds the v1 limit",
        ));
    }
    let mut unique = BTreeSet::new();
    for id in ids {
        if !unique.insert(id) {
            return Err(VerifyError::new(
                VerifyErrorCategory::Reconciliation,
                VerifyErrorCode::DuplicateId,
                "reconciliation support receipt ids must be unique",
            ));
        }
    }
    Ok(unique.into_iter().collect())
}

fn validate_state_binding(
    run_id: RunId,
    attempt: &ActionAttemptRef,
    action: &ActionRef,
    state: &RunState,
) -> Result<(), VerifyError> {
    if run_id != state.run_id() || attempt.action() != action {
        return Err(binding_error(
            "reconciliation run, attempt and action binding must match exactly",
        ));
    }
    let Some(prepared) = state.prepared_attempts().get(&attempt.id()) else {
        return Err(binding_error(
            "reconciliation attempt does not exist in the supplied run state",
        ));
    };
    if prepared.attempt() != attempt || prepared.attempt().action() != action {
        return Err(binding_error(
            "reconciliation attempt binding differs from durable run state",
        ));
    }
    if !prepared.unresolved() || !state.unresolved_attempts().contains(&attempt.id()) {
        return Err(VerifyError::new(
            VerifyErrorCategory::Reconciliation,
            VerifyErrorCode::ReconciliationUnresolved,
            "reconciliation requires the exact durable unresolved attempt",
        ));
    }
    Ok(())
}

fn resolve_support<'a>(
    attempt: &ActionAttemptRef,
    support_ids: &[VerificationId],
    available_receipts: &'a [VerificationReceipt],
) -> Result<Vec<&'a VerificationReceipt>, VerifyError> {
    let mut by_id: BTreeMap<VerificationId, &VerificationReceipt> = BTreeMap::new();
    for receipt in available_receipts {
        if by_id.insert(receipt.id(), receipt).is_some() {
            return Err(VerifyError::new(
                VerifyErrorCategory::Evidence,
                VerifyErrorCode::DuplicateId,
                "available verification receipts contain a duplicate id",
            ));
        }
    }

    let exact_target = VerificationTarget::ActionAttempt(attempt.clone());
    let mut resolved = Vec::with_capacity(support_ids.len());
    for id in support_ids {
        let receipt = by_id.get(id).copied().ok_or_else(|| {
            VerifyError::new(
                VerifyErrorCategory::Evidence,
                VerifyErrorCode::InvalidEvidence,
                "reconciliation support receipt id could not be resolved",
            )
        })?;
        if receipt.target() != &exact_target {
            return Err(VerifyError::new(
                VerifyErrorCategory::Reconciliation,
                VerifyErrorCode::InvalidTarget,
                "reconciliation support receipt targets a different action attempt",
            ));
        }
        resolved.push(receipt);
    }
    Ok(resolved)
}

fn derive_outcome(receipts: &[&VerificationReceipt]) -> ReconciliationOutcomeV1 {
    let mut effect_confirmed = false;
    let mut no_effect_confirmed = false;
    for receipt in receipts {
        if !has_immutable_evidence_binding(receipt) {
            continue;
        }
        match receipt.outcome() {
            VerificationOutcome::Verified => effect_confirmed = true,
            VerificationOutcome::Rejected => no_effect_confirmed = true,
            VerificationOutcome::Inconclusive | VerificationOutcome::NotEvaluated => {}
        }
    }
    match (effect_confirmed, no_effect_confirmed) {
        (true, false) => ReconciliationOutcomeV1::EffectConfirmed,
        (false, true) => ReconciliationOutcomeV1::NoEffectConfirmed,
        (true, true) | (false, false) => ReconciliationOutcomeV1::StillUnknown,
    }
}

fn has_immutable_evidence_binding(receipt: &VerificationReceipt) -> bool {
    receipt
        .evidence()
        .iter()
        .any(|evidence| evidence.artifact().is_some() || evidence.content_digest().is_some())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetryDispositionV1 {
    DuplicateRetryBlocked,
    ReconciliationRequired,
    SemanticallyRetryable,
    SemanticallyRetryableSameKey,
    RequiresExplicitNonblindPath,
}

pub fn retry_disposition(
    intent: &ActionIntent,
    attempt: &ActionAttemptRef,
    state: &RunState,
    reconciliation: Option<&ReconciliationRecordV1>,
    proposed_idempotency_key: Option<&str>,
) -> Result<RetryDispositionV1, VerifyError> {
    let action = intent.action_ref().map_err(|_| {
        VerifyError::new(
            VerifyErrorCategory::Validation,
            VerifyErrorCode::InvalidTarget,
            "action intent could not produce its canonical action reference",
        )
    })?;
    validate_state_binding(state.run_id(), attempt, &action, state)?;

    let outcome = if let Some(record) = reconciliation {
        if record.run_id != state.run_id()
            || record.attempt != *attempt
            || record.action != action
        {
            return Err(binding_error(
                "retry advisory reconciliation does not bind the exact prior attempt",
            ));
        }
        Some(record.outcome)
    } else {
        None
    };

    match outcome {
        Some(ReconciliationOutcomeV1::EffectConfirmed) => {
            Ok(RetryDispositionV1::DuplicateRetryBlocked)
        }
        None | Some(ReconciliationOutcomeV1::StillUnknown) => {
            Ok(RetryDispositionV1::ReconciliationRequired)
        }
        Some(ReconciliationOutcomeV1::NoEffectConfirmed) => match intent.retry() {
            RetryClass::Safe
                if intent.idempotency().class() == IdempotencyClass::NaturallyIdempotent =>
            {
                Ok(RetryDispositionV1::SemanticallyRetryable)
            }
            RetryClass::RequiresSameIdempotencyKey
                if intent.idempotency().class() == IdempotencyClass::IdempotentWithKey
                    && intent.idempotency().key_ref() == proposed_idempotency_key =>
            {
                Ok(RetryDispositionV1::SemanticallyRetryableSameKey)
            }
            RetryClass::Safe
            | RetryClass::RequiresSameIdempotencyKey
            | RetryClass::RequiresExternalReconciliation
            | RetryClass::NeverBlindRetry => Ok(RetryDispositionV1::RequiresExplicitNonblindPath),
        },
    }
}

fn binding_error(diagnostic: &'static str) -> VerifyError {
    VerifyError::new(
        VerifyErrorCategory::Reconciliation,
        VerifyErrorCode::AttemptBindingMismatch,
        diagnostic,
    )
}
