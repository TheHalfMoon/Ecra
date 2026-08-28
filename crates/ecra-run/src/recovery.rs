use std::collections::{BTreeMap, BTreeSet};

use ecra_core::{
    ActionAttemptId, ActionAttemptRef, ActionIntent, IdempotencyClass, RetryClass, RunId,
};

use crate::state::PreparedAttemptState;
use crate::{
    EventSequence, LedgerDigest, RunError, RunErrorCategory, RunErrorCode, RunPhase, RunState,
};

/// Proof returned by the store only after `attempt_prepared` commits durably.
///
/// This value does not authorize provider execution. A later owning executor may
/// use it only as evidence that the exact attempt is present in durable run truth.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreparedAttemptGuard {
    run_id: RunId,
    attempt: ActionAttemptRef,
    committed_sequence: EventSequence,
    committed_digest: LedgerDigest,
}

impl PreparedAttemptGuard {
    pub(crate) fn new(
        run_id: RunId,
        attempt: ActionAttemptRef,
        committed_sequence: EventSequence,
        committed_digest: LedgerDigest,
    ) -> Self {
        Self {
            run_id,
            attempt,
            committed_sequence,
            committed_digest,
        }
    }

    #[must_use]
    pub const fn run_id(&self) -> RunId {
        self.run_id
    }

    #[must_use]
    pub fn attempt(&self) -> &ActionAttemptRef {
        &self.attempt
    }

    #[must_use]
    pub const fn committed_sequence(&self) -> EventSequence {
        self.committed_sequence
    }

    #[must_use]
    pub fn committed_digest(&self) -> &LedgerDigest {
        &self.committed_digest
    }
}

/// Recovery result after an explicit recovery boundary is durably appended.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecoveryResult {
    unreceipted_attempts: Vec<ActionAttemptRef>,
    state: RunState,
}

impl RecoveryResult {
    pub(crate) fn new(unreceipted_attempts: Vec<ActionAttemptRef>, state: RunState) -> Self {
        Self {
            unreceipted_attempts,
            state,
        }
    }

    #[must_use]
    pub fn unreceipted_attempts(&self) -> &[ActionAttemptRef] {
        &self.unreceipted_attempts
    }

    #[must_use]
    pub const fn state(&self) -> &RunState {
        &self.state
    }
}

pub(crate) fn scan_unreceipted_attempts(state: &RunState) -> Vec<ActionAttemptRef> {
    state
        .prepared_attempts()
        .values()
        .filter(|prepared| prepared.receipt().is_none())
        .map(|prepared| prepared.attempt().clone())
        .collect()
}

/// Refuse blind retries unless the exact ECR-001 action semantics permit them.
///
/// This is a runtime safety guard, not an authorization or reconciliation
/// decision. Exact action binding ensures `requires_same_idempotency_key` is
/// evaluated against the same `ActionIntent`, including its key reference.
pub fn ensure_retry_allowed(
    state: &RunState,
    intent: &ActionIntent,
    prior_attempt: &ActionAttemptRef,
) -> Result<(), RunError> {
    prior_attempt.validate_for(intent).map_err(|_| {
        attempt_error(
            RunErrorCode::AttemptBindingMismatch,
            "retry candidate does not bind the exact ActionIntent",
        )
    })?;

    let prepared = state
        .prepared_attempts()
        .get(&prior_attempt.id())
        .ok_or_else(|| {
            attempt_error(
                RunErrorCode::AttemptBindingMismatch,
                "retry candidate has no matching prepared attempt",
            )
        })?;
    if prepared.attempt() != prior_attempt {
        return Err(attempt_error(
            RunErrorCode::AttemptBindingMismatch,
            "retry candidate conflicts with the durable attempt binding",
        ));
    }

    if state.phase() != RunPhase::Running {
        return Err(blind_retry_error(
            "blind retry requires a running run phase",
        ));
    }
    if state.has_hard_budget_blocker() {
        return Err(RunError::new(
            RunErrorCategory::Budget,
            RunErrorCode::BudgetExhausted,
            "blind retry is blocked by an exhausted hard budget",
        ));
    }
    if prepared.receipt().is_none()
        || prepared.unresolved()
        || state.unresolved_attempts().contains(&prior_attempt.id())
    {
        return Err(blind_retry_error(
            "blind retry is forbidden while the prior attempt is unreceipted or unresolved",
        ));
    }

    if matches!(
        intent.idempotency().class(),
        IdempotencyClass::NonIdempotent | IdempotencyClass::Unknown
    ) {
        return Err(blind_retry_error(
            "blind retry is forbidden for non-idempotent or unknown idempotency semantics",
        ));
    }

    match intent.retry() {
        RetryClass::Safe | RetryClass::RequiresSameIdempotencyKey => Ok(()),
        RetryClass::RequiresExternalReconciliation | RetryClass::NeverBlindRetry => Err(
            blind_retry_error("ECR-001 retry class requires reconciliation or forbids blind retry"),
        ),
    }
}

pub(crate) fn mark_unreceipted_attempts_unresolved(
    prepared_attempts: &mut BTreeMap<ActionAttemptId, PreparedAttemptState>,
    unresolved_attempts: &mut BTreeSet<ActionAttemptId>,
) -> Option<ActionAttemptRef> {
    let mut first_unresolved = None;
    for (attempt_id, prepared) in prepared_attempts {
        if prepared.receipt().is_none() {
            prepared.mark_unresolved();
            unresolved_attempts.insert(*attempt_id);
            if first_unresolved.is_none() {
                first_unresolved = Some(prepared.attempt().clone());
            }
        }
    }
    first_unresolved
}

fn blind_retry_error(message: impl Into<String>) -> RunError {
    RunError::new(
        RunErrorCategory::Recovery,
        RunErrorCode::BlindRetryForbidden,
        message,
    )
}

fn attempt_error(code: RunErrorCode, message: impl Into<String>) -> RunError {
    RunError::new(RunErrorCategory::Attempt, code, message)
}
