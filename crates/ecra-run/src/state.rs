use std::collections::{BTreeMap, BTreeSet};

use ecra_core::{ActionAttemptId, ActionAttemptRef, ActionReceipt, ActorId, RunId, to_jcs_vec};
use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{
    BudgetAmount, BudgetDimension, BudgetUsage, EventSequence, LedgerDigest, RunBudget, RunError,
    RunErrorCategory, RunErrorCode, RunEvent, RunEventEnvelope,
};

pub const MAX_SUSPENSION_OTHER_CODE_BYTES: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunPhase {
    Created,
    Running,
    Suspended,
    CancellationRequested,
    Cancelled,
    Failed,
    ExecutionCompleted,
}

impl RunPhase {
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Cancelled | Self::Failed | Self::ExecutionCompleted
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SuspensionReason {
    UserPause,
    BudgetExhausted { dimension: BudgetDimension },
    ReconciliationRequired { attempt: ActionAttemptRef },
    CancellationInProgress,
    RuntimeInterruption,
    Other { code: String },
}

impl SuspensionReason {
    pub fn other(code: impl Into<String>) -> Result<Self, RunError> {
        let code = code.into();
        if code.is_empty() || code.len() > MAX_SUSPENSION_OTHER_CODE_BYTES {
            return Err(RunError::new(
                RunErrorCategory::State,
                RunErrorCode::InvalidStateTransition,
                "suspension other code must be 1..=256 UTF-8 bytes",
            ));
        }
        Ok(Self::Other { code })
    }

    #[must_use]
    pub const fn is_directly_resumable(&self) -> bool {
        matches!(self, Self::UserPause | Self::RuntimeInterruption)
    }
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum SuspensionReasonWire {
    UserPause,
    BudgetExhausted { dimension: BudgetDimension },
    ReconciliationRequired { attempt: ActionAttemptRef },
    CancellationInProgress,
    RuntimeInterruption,
    Other { code: String },
}

impl<'de> Deserialize<'de> for SuspensionReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match SuspensionReasonWire::deserialize(deserializer)? {
            SuspensionReasonWire::UserPause => Ok(Self::UserPause),
            SuspensionReasonWire::BudgetExhausted { dimension } => {
                Ok(Self::BudgetExhausted { dimension })
            }
            SuspensionReasonWire::ReconciliationRequired { attempt } => {
                Ok(Self::ReconciliationRequired { attempt })
            }
            SuspensionReasonWire::CancellationInProgress => Ok(Self::CancellationInProgress),
            SuspensionReasonWire::RuntimeInterruption => Ok(Self::RuntimeInterruption),
            SuspensionReasonWire::Other { code } => Self::other(code).map_err(de::Error::custom),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PreparedAttemptState {
    attempt: ActionAttemptRef,
    prepared_at_sequence: EventSequence,
    receipt: Option<ActionReceipt>,
    unresolved: bool,
}

impl PreparedAttemptState {
    #[must_use]
    pub fn new(attempt: ActionAttemptRef, prepared_at_sequence: EventSequence) -> Self {
        Self {
            attempt,
            prepared_at_sequence,
            receipt: None,
            unresolved: false,
        }
    }

    #[must_use]
    pub fn attempt(&self) -> &ActionAttemptRef {
        &self.attempt
    }

    #[must_use]
    pub const fn prepared_at_sequence(&self) -> EventSequence {
        self.prepared_at_sequence
    }

    #[must_use]
    pub fn receipt(&self) -> Option<&ActionReceipt> {
        self.receipt.as_ref()
    }

    #[must_use]
    pub const fn unresolved(&self) -> bool {
        self.unresolved
    }

    pub(crate) fn mark_unresolved(&mut self) {
        self.unresolved = true;
    }

    pub(crate) fn record_receipt(&mut self, receipt: ActionReceipt) {
        self.receipt = Some(receipt);
        self.unresolved = false;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RunState {
    run_id: RunId,
    phase: RunPhase,
    actor: ActorId,
    budget: RunBudget,
    usage: BudgetUsage,
    #[serde(skip)]
    soft_limits_reached: BTreeSet<BudgetDimension>,
    #[serde(skip)]
    pending_soft_crossings: BTreeMap<BudgetDimension, (BudgetAmount, BudgetAmount)>,
    prepared_attempts: BTreeMap<ActionAttemptId, PreparedAttemptState>,
    unresolved_attempts: BTreeSet<ActionAttemptId>,
    last_sequence: EventSequence,
    last_digest: LedgerDigest,
    suspension: Option<SuspensionReason>,
}

impl RunState {
    #[must_use]
    pub const fn run_id(&self) -> RunId {
        self.run_id
    }

    #[must_use]
    pub const fn phase(&self) -> RunPhase {
        self.phase
    }

    #[must_use]
    pub const fn actor(&self) -> ActorId {
        self.actor
    }

    #[must_use]
    pub fn budget(&self) -> &RunBudget {
        &self.budget
    }

    #[must_use]
    pub fn usage(&self) -> &BTreeMap<BudgetDimension, BudgetAmount> {
        self.usage.amounts()
    }

    #[must_use]
    pub fn prepared_attempts(&self) -> &BTreeMap<ActionAttemptId, PreparedAttemptState> {
        &self.prepared_attempts
    }

    #[must_use]
    pub fn unresolved_attempts(&self) -> &BTreeSet<ActionAttemptId> {
        &self.unresolved_attempts
    }

    #[must_use]
    pub const fn last_sequence(&self) -> EventSequence {
        self.last_sequence
    }

    #[must_use]
    pub fn last_digest(&self) -> &LedgerDigest {
        &self.last_digest
    }

    #[must_use]
    pub fn suspension(&self) -> Option<&SuspensionReason> {
        self.suspension.as_ref()
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, RunError> {
        to_jcs_vec(self).map_err(|error| RunError::serialization(error.to_string()))
    }

    #[must_use]
    pub fn usage_for(&self, dimension: BudgetDimension) -> Option<BudgetAmount> {
        self.usage.recorded(dimension)
    }

    #[must_use]
    pub fn has_unreceipted_attempts(&self) -> bool {
        self.prepared_attempts
            .values()
            .any(|attempt| attempt.receipt().is_none())
    }

    #[must_use]
    pub fn has_hard_budget_blocker(&self) -> bool {
        self.budget
            .limits()
            .iter()
            .any(|limit| self.usage.get(limit.dimension()) >= limit.hard())
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RunReducer;

impl RunReducer {
    pub fn reduce(history: &[RunEventEnvelope]) -> Result<RunState, RunError> {
        let (first, rest) = history
            .split_first()
            .ok_or_else(|| state_error("run reduction requires a run_created genesis envelope"))?;
        let mut state = Self::initialize(first)?;
        for envelope in rest {
            state = Self::apply(&state, envelope)?;
        }
        Ok(state)
    }

    pub fn apply(state: &RunState, envelope: &RunEventEnvelope) -> Result<RunState, RunError> {
        if state.phase.is_terminal() {
            return Err(state_error(
                "terminal run phase rejects all later v1 events",
            ));
        }
        if envelope.run_id() != state.run_id {
            return Err(RunError::ledger_chain_invalid(
                "run reducer successor belongs to a different run",
            ));
        }
        let expected_sequence = state.last_sequence.checked_next()?;
        if envelope.sequence() != expected_sequence {
            return Err(RunError::invalid_event_sequence(
                "run reducer successor sequence is not exactly previous + 1",
            ));
        }
        if envelope.previous_digest() != Some(&state.last_digest) {
            return Err(RunError::ledger_chain_invalid(
                "run reducer successor previous_digest does not match the current state head",
            ));
        }

        let mut next = state.clone();
        Self::apply_event(&mut next, envelope.sequence(), envelope.event())?;
        next.last_sequence = envelope.sequence();
        next.last_digest = envelope.event_digest().clone();
        Ok(next)
    }

    fn initialize(envelope: &RunEventEnvelope) -> Result<RunState, RunError> {
        match envelope.event() {
            RunEvent::RunCreated { actor, budget } if envelope.sequence().get() == 1 => {
                Ok(RunState {
                    run_id: envelope.run_id(),
                    phase: RunPhase::Created,
                    actor: *actor,
                    budget: budget.clone(),
                    usage: BudgetUsage::default(),
                    soft_limits_reached: BTreeSet::new(),
                    pending_soft_crossings: BTreeMap::new(),
                    prepared_attempts: BTreeMap::new(),
                    unresolved_attempts: BTreeSet::new(),
                    last_sequence: envelope.sequence(),
                    last_digest: envelope.event_digest().clone(),
                    suspension: None,
                })
            }
            RunEvent::RunCreated { .. } => Err(RunError::invalid_event_sequence(
                "run_created must be sequence 1",
            )),
            _ => Err(state_error("run reduction must begin with run_created")),
        }
    }

    fn apply_event(
        state: &mut RunState,
        sequence: EventSequence,
        event: &RunEvent,
    ) -> Result<(), RunError> {
        match event {
            RunEvent::RunCreated { .. } => {
                return Err(state_error("run_created is only valid as genesis"));
            }
            RunEvent::RunStarted {} => {
                ensure_phase(state, &[RunPhase::Created], "run_started")?;
                if !state.unresolved_attempts.is_empty() {
                    return Err(unresolved_error(
                        "run_started is blocked by unresolved attempts",
                    ));
                }
                state.phase = RunPhase::Running;
                state.suspension = None;
            }
            RunEvent::RunSuspended { reason } => {
                ensure_phase(state, &[RunPhase::Running], "run_suspended")?;
                state.phase = RunPhase::Suspended;
                state.suspension = Some(reason.clone());
            }
            RunEvent::RunResumed {} => {
                ensure_phase(state, &[RunPhase::Suspended], "run_resumed")?;
                let suspension = state
                    .suspension
                    .as_ref()
                    .ok_or_else(|| state_error("suspended run is missing a SuspensionReason"))?;
                if !suspension.is_directly_resumable() {
                    return Err(state_error(
                        "run suspension reason is not directly resumable in v1",
                    ));
                }
                if !state.unresolved_attempts.is_empty() || state.has_unreceipted_attempts() {
                    return Err(unresolved_error(
                        "run_resumed is blocked by unresolved or unreceipted attempts",
                    ));
                }
                if state.has_hard_budget_blocker() {
                    return Err(RunError::new(
                        RunErrorCategory::Budget,
                        RunErrorCode::BudgetExhausted,
                        "run_resumed is blocked by a hard budget",
                    ));
                }
                state.phase = RunPhase::Running;
                state.suspension = None;
            }
            RunEvent::CancellationRequested { .. } => {
                ensure_phase(
                    state,
                    &[RunPhase::Created, RunPhase::Running, RunPhase::Suspended],
                    "cancellation_requested",
                )?;
                state.phase = RunPhase::CancellationRequested;
                state.suspension = None;
            }
            RunEvent::RunCancelled {} => {
                ensure_phase(state, &[RunPhase::CancellationRequested], "run_cancelled")?;
                ensure_no_unreceipted_attempts(state, "run_cancelled")?;
                state.phase = RunPhase::Cancelled;
                state.suspension = None;
            }
            RunEvent::RunFailed { .. } => {
                ensure_phase(
                    state,
                    &[
                        RunPhase::Created,
                        RunPhase::Running,
                        RunPhase::Suspended,
                        RunPhase::CancellationRequested,
                    ],
                    "run_failed",
                )?;
                ensure_no_unreceipted_attempts(state, "run_failed")?;
                state.phase = RunPhase::Failed;
                state.suspension = None;
            }
            RunEvent::ExecutionCompleted {} => {
                ensure_phase(state, &[RunPhase::Running], "execution_completed")?;
                ensure_no_unreceipted_attempts(state, "execution_completed")?;
                if !state.unresolved_attempts.is_empty() {
                    return Err(unresolved_error(
                        "execution_completed is blocked by unresolved attempts",
                    ));
                }
                if state.has_hard_budget_blocker() {
                    return Err(RunError::new(
                        RunErrorCategory::Budget,
                        RunErrorCode::BudgetExhausted,
                        "execution_completed is blocked by a hard budget",
                    ));
                }
                state.phase = RunPhase::ExecutionCompleted;
                state.suspension = None;
            }
            RunEvent::AttemptPrepared { attempt } => {
                ensure_phase(state, &[RunPhase::Running], "attempt_prepared")?;
                if state.has_hard_budget_blocker() {
                    return Err(RunError::new(
                        RunErrorCategory::Budget,
                        RunErrorCode::BudgetExhausted,
                        "attempt_prepared is blocked by an exhausted hard budget",
                    ));
                }
                let attempt_id = attempt.id();
                if let Some(existing) = state.prepared_attempts.get(&attempt_id) {
                    let code = if existing.attempt() == attempt {
                        RunErrorCode::DuplicateAttempt
                    } else {
                        RunErrorCode::AttemptBindingMismatch
                    };
                    return Err(attempt_error(
                        code,
                        "attempt identity is already prepared in this run",
                    ));
                }
                state.prepared_attempts.insert(
                    attempt_id,
                    PreparedAttemptState::new(attempt.clone(), sequence),
                );
            }
            RunEvent::ReceiptRecorded { receipt } => {
                let attempt_id = receipt.attempt().id();
                let prepared = state
                    .prepared_attempts
                    .get_mut(&attempt_id)
                    .ok_or_else(|| {
                        attempt_error(
                            RunErrorCode::ReceiptBindingMismatch,
                            "receipt has no matching prepared attempt",
                        )
                    })?;
                if prepared.attempt() != receipt.attempt() {
                    return Err(attempt_error(
                        RunErrorCode::ReceiptBindingMismatch,
                        "receipt does not bind the exact prepared attempt/action",
                    ));
                }
                if prepared.receipt().is_some() {
                    return Err(attempt_error(
                        RunErrorCode::ReceiptBindingMismatch,
                        "attempt already has a durable receipt",
                    ));
                }
                prepared.record_receipt(receipt.clone());
                state.unresolved_attempts.remove(&attempt_id);
            }
            RunEvent::RecoveryBoundary { .. } => {
                ensure_phase(
                    state,
                    &[RunPhase::Running, RunPhase::CancellationRequested],
                    "recovery_boundary",
                )?;
                let unresolved = crate::recovery::mark_unreceipted_attempts_unresolved(
                    &mut state.prepared_attempts,
                    &mut state.unresolved_attempts,
                );
                state.phase = RunPhase::Suspended;
                state.suspension = Some(match unresolved {
                    Some(attempt) => SuspensionReason::ReconciliationRequired { attempt },
                    None => SuspensionReason::RuntimeInterruption,
                });
            }
            RunEvent::AttemptMarkedUnknown { attempt, .. } => {
                let attempt_id = attempt.id();
                let prepared = state
                    .prepared_attempts
                    .get_mut(&attempt_id)
                    .ok_or_else(|| {
                        attempt_error(
                            RunErrorCode::AttemptBindingMismatch,
                            "unknown marker has no matching prepared attempt",
                        )
                    })?;
                if prepared.attempt() != attempt {
                    return Err(attempt_error(
                        RunErrorCode::AttemptBindingMismatch,
                        "unknown marker does not bind the exact prepared attempt/action",
                    ));
                }
                if prepared.receipt().is_some() {
                    return Err(attempt_error(
                        RunErrorCode::AttemptBindingMismatch,
                        "received attempt cannot later be marked unknown",
                    ));
                }
                prepared.mark_unresolved();
                state.unresolved_attempts.insert(attempt_id);
            }
            RunEvent::ReconciliationRequested { attempt } => {
                let attempt_id = attempt.id();
                let prepared = state.prepared_attempts.get(&attempt_id).ok_or_else(|| {
                    attempt_error(
                        RunErrorCode::AttemptBindingMismatch,
                        "reconciliation request has no matching prepared attempt",
                    )
                })?;
                if prepared.attempt() != attempt {
                    return Err(attempt_error(
                        RunErrorCode::AttemptBindingMismatch,
                        "reconciliation request does not bind the exact prepared attempt/action",
                    ));
                }
                if !prepared.unresolved() || !state.unresolved_attempts.contains(&attempt_id) {
                    return Err(unresolved_error(
                        "reconciliation request requires an unresolved prepared attempt",
                    ));
                }
            }
            RunEvent::ResourceUsageRecorded { dimension, amount } => {
                ensure_phase(state, &[RunPhase::Running], "resource_usage_recorded")?;
                if state.has_hard_budget_blocker() {
                    return Err(RunError::new(
                        RunErrorCategory::Budget,
                        RunErrorCode::BudgetExhausted,
                        "resource usage is blocked after hard budget exhaustion",
                    ));
                }
                let (previous, cumulative) = state.usage.charge(*dimension, *amount)?;
                if !state.soft_limits_reached.contains(dimension)
                    && let Some(crossing) =
                        state.budget.soft_crossing(*dimension, previous, cumulative)
                {
                    state
                        .pending_soft_crossings
                        .entry(*dimension)
                        .or_insert(crossing);
                }
            }
            RunEvent::BudgetSoftLimitReached {
                dimension,
                soft_limit,
                cumulative_usage,
            } => {
                ensure_phase(state, &[RunPhase::Running], "budget_soft_limit_reached")?;
                if state.soft_limits_reached.contains(dimension) {
                    return Err(budget_error(
                        "budget soft-limit evidence is valid only for the first crossing",
                    ));
                }
                let Some(expected) = state.pending_soft_crossings.get(dimension).copied() else {
                    return Err(budget_error(
                        "budget soft-limit evidence has no matching first threshold crossing",
                    ));
                };
                if expected != (*soft_limit, *cumulative_usage) {
                    return Err(budget_error(
                        "budget soft-limit evidence does not match the configured first crossing",
                    ));
                }
                state.pending_soft_crossings.remove(dimension);
                state.soft_limits_reached.insert(*dimension);
            }
            RunEvent::BudgetExhausted {
                dimension,
                hard_limit,
                cumulative_usage,
            } => {
                ensure_phase(state, &[RunPhase::Running], "budget_exhausted")?;
                let Some(limit) = state.budget.limit(*dimension) else {
                    return Err(budget_error(
                        "budget exhaustion evidence references an unconfigured dimension",
                    ));
                };
                let current = state.usage.get(*dimension);
                if limit.hard() != *hard_limit
                    || current != *cumulative_usage
                    || current < *hard_limit
                {
                    return Err(budget_error(
                        "budget exhaustion evidence does not match configured hard limit and cumulative usage",
                    ));
                }
                state.phase = RunPhase::Suspended;
                state.suspension = Some(SuspensionReason::BudgetExhausted {
                    dimension: *dimension,
                });
            }
            RunEvent::InterventionRecorded { .. } => {}
        }
        Ok(())
    }
}

fn ensure_phase(state: &RunState, allowed: &[RunPhase], event: &str) -> Result<(), RunError> {
    if allowed.contains(&state.phase) {
        Ok(())
    } else {
        Err(state_error(format!(
            "{event} is invalid from run phase {:?}",
            state.phase
        )))
    }
}

fn ensure_no_unreceipted_attempts(state: &RunState, event: &str) -> Result<(), RunError> {
    if state.has_unreceipted_attempts() {
        Err(unresolved_error(format!(
            "{event} is blocked by a prepared attempt without receipt"
        )))
    } else {
        Ok(())
    }
}

fn state_error(message: impl Into<String>) -> RunError {
    RunError::new(
        RunErrorCategory::State,
        RunErrorCode::InvalidStateTransition,
        message,
    )
}

fn attempt_error(code: RunErrorCode, message: impl Into<String>) -> RunError {
    RunError::new(RunErrorCategory::Attempt, code, message)
}

fn unresolved_error(message: impl Into<String>) -> RunError {
    RunError::new(
        RunErrorCategory::Attempt,
        RunErrorCode::UnresolvedAttempt,
        message,
    )
}

fn budget_error(message: impl Into<String>) -> RunError {
    RunError::new(
        RunErrorCategory::Budget,
        RunErrorCode::InvalidBudget,
        message,
    )
}
