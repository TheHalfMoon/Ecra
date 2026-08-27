use ecra_core::{
    ActionAttemptRef, ActionReceipt, ActorId, EpochMillis, RunId, SchemaVersion,
};
use serde::{Deserialize, Deserializer, Serialize, de};

use crate::digest::canonical_event_material;
use crate::{
    BudgetAmount, BudgetDimension, LedgerDigest, RunBudget, RunError, RunErrorCategory,
    RunErrorCode, RunErrorSummary, SuspensionReason,
};

pub const MAX_EVENT_SEQUENCE: u64 = 9_007_199_254_740_991;
pub const MAX_INTERVENTION_NOTE_BYTES: usize = 4_096;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct EventSequence(u64);

impl EventSequence {
    pub fn new(value: u64) -> Result<Self, RunError> {
        if value == 0 || value > MAX_EVENT_SEQUENCE {
            return Err(RunError::new(
                RunErrorCategory::Event,
                RunErrorCode::InvalidEventSequence,
                "event sequence must be in 1..=9_007_199_254_740_991",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    pub fn checked_next(self) -> Result<Self, RunError> {
        let next = self.0.checked_add(1).ok_or_else(|| {
            RunError::invalid_event_sequence("event sequence increment overflowed")
        })?;
        Self::new(next)
    }
}

impl<'de> Deserialize<'de> for EventSequence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Self::new(value).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryReason {
    ProcessRestart,
    ExplicitRecovery,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptUnknownCause {
    InterruptedBeforeReceipt,
    ProviderAmbiguous,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterventionKind {
    Takeover,
    HandBack,
    PauseRequest,
    Edit,
    Denial,
    Note,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum RunEvent {
    RunCreated {
        actor: ActorId,
        budget: RunBudget,
    },
    RunStarted {},
    RunSuspended {
        reason: SuspensionReason,
    },
    RunResumed {},
    CancellationRequested {
        actor: ActorId,
    },
    RunCancelled {},
    RunFailed {
        error: RunErrorSummary,
    },
    ExecutionCompleted {},
    AttemptPrepared {
        attempt: ActionAttemptRef,
    },
    ReceiptRecorded {
        receipt: ActionReceipt,
    },
    RecoveryBoundary {
        reason: RecoveryReason,
    },
    AttemptMarkedUnknown {
        attempt: ActionAttemptRef,
        cause: AttemptUnknownCause,
    },
    ReconciliationRequested {
        attempt: ActionAttemptRef,
    },
    ResourceUsageRecorded {
        dimension: BudgetDimension,
        amount: BudgetAmount,
    },
    BudgetSoftLimitReached {
        dimension: BudgetDimension,
        soft_limit: BudgetAmount,
        cumulative_usage: BudgetAmount,
    },
    BudgetExhausted {
        dimension: BudgetDimension,
        hard_limit: BudgetAmount,
        cumulative_usage: BudgetAmount,
    },
    InterventionRecorded {
        actor: ActorId,
        kind: InterventionKind,
        note: Option<String>,
    },
}

impl RunEvent {
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::RunCreated { .. } => "run_created",
            Self::RunStarted {} => "run_started",
            Self::RunSuspended { .. } => "run_suspended",
            Self::RunResumed {} => "run_resumed",
            Self::CancellationRequested { .. } => "cancellation_requested",
            Self::RunCancelled {} => "run_cancelled",
            Self::RunFailed { .. } => "run_failed",
            Self::ExecutionCompleted {} => "execution_completed",
            Self::AttemptPrepared { .. } => "attempt_prepared",
            Self::ReceiptRecorded { .. } => "receipt_recorded",
            Self::RecoveryBoundary { .. } => "recovery_boundary",
            Self::AttemptMarkedUnknown { .. } => "attempt_marked_unknown",
            Self::ReconciliationRequested { .. } => "reconciliation_requested",
            Self::ResourceUsageRecorded { .. } => "resource_usage_recorded",
            Self::BudgetSoftLimitReached { .. } => "budget_soft_limit_reached",
            Self::BudgetExhausted { .. } => "budget_exhausted",
            Self::InterventionRecorded { .. } => "intervention_recorded",
        }
    }

    #[must_use]
    pub const fn is_run_created(&self) -> bool {
        matches!(self, Self::RunCreated { .. })
    }

    fn validate_basic(&self) -> Result<(), RunError> {
        if let Self::InterventionRecorded {
            note: Some(note), ..
        } = self
            && note.len() > MAX_INTERVENTION_NOTE_BYTES
        {
            return Err(RunError::invalid_event(
                "intervention note exceeds 4096 UTF-8 bytes",
            ));
        }
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case", deny_unknown_fields)]
enum RunEventWire {
    RunCreated {
        actor: ActorId,
        budget: RunBudget,
    },
    RunStarted {},
    RunSuspended {
        reason: SuspensionReason,
    },
    RunResumed {},
    CancellationRequested {
        actor: ActorId,
    },
    RunCancelled {},
    RunFailed {
        error: RunErrorSummary,
    },
    ExecutionCompleted {},
    AttemptPrepared {
        attempt: ActionAttemptRef,
    },
    ReceiptRecorded {
        receipt: ActionReceipt,
    },
    RecoveryBoundary {
        reason: RecoveryReason,
    },
    AttemptMarkedUnknown {
        attempt: ActionAttemptRef,
        cause: AttemptUnknownCause,
    },
    ReconciliationRequested {
        attempt: ActionAttemptRef,
    },
    ResourceUsageRecorded {
        dimension: BudgetDimension,
        amount: BudgetAmount,
    },
    BudgetSoftLimitReached {
        dimension: BudgetDimension,
        soft_limit: BudgetAmount,
        cumulative_usage: BudgetAmount,
    },
    BudgetExhausted {
        dimension: BudgetDimension,
        hard_limit: BudgetAmount,
        cumulative_usage: BudgetAmount,
    },
    InterventionRecorded {
        actor: ActorId,
        kind: InterventionKind,
        note: Option<String>,
    },
}

impl From<RunEventWire> for RunEvent {
    fn from(value: RunEventWire) -> Self {
        match value {
            RunEventWire::RunCreated { actor, budget } => Self::RunCreated { actor, budget },
            RunEventWire::RunStarted {} => Self::RunStarted {},
            RunEventWire::RunSuspended { reason } => Self::RunSuspended { reason },
            RunEventWire::RunResumed {} => Self::RunResumed {},
            RunEventWire::CancellationRequested { actor } => {
                Self::CancellationRequested { actor }
            }
            RunEventWire::RunCancelled {} => Self::RunCancelled {},
            RunEventWire::RunFailed { error } => Self::RunFailed { error },
            RunEventWire::ExecutionCompleted {} => Self::ExecutionCompleted {},
            RunEventWire::AttemptPrepared { attempt } => Self::AttemptPrepared { attempt },
            RunEventWire::ReceiptRecorded { receipt } => Self::ReceiptRecorded { receipt },
            RunEventWire::RecoveryBoundary { reason } => Self::RecoveryBoundary { reason },
            RunEventWire::AttemptMarkedUnknown { attempt, cause } => {
                Self::AttemptMarkedUnknown { attempt, cause }
            }
            RunEventWire::ReconciliationRequested { attempt } => {
                Self::ReconciliationRequested { attempt }
            }
            RunEventWire::ResourceUsageRecorded { dimension, amount } => {
                Self::ResourceUsageRecorded { dimension, amount }
            }
            RunEventWire::BudgetSoftLimitReached {
                dimension,
                soft_limit,
                cumulative_usage,
            } => Self::BudgetSoftLimitReached {
                dimension,
                soft_limit,
                cumulative_usage,
            },
            RunEventWire::BudgetExhausted {
                dimension,
                hard_limit,
                cumulative_usage,
            } => Self::BudgetExhausted {
                dimension,
                hard_limit,
                cumulative_usage,
            },
            RunEventWire::InterventionRecorded { actor, kind, note } => {
                Self::InterventionRecorded { actor, kind, note }
            }
        }
    }
}

impl<'de> Deserialize<'de> for RunEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let event = Self::from(RunEventWire::deserialize(deserializer)?);
        event.validate_basic().map_err(de::Error::custom)?;
        Ok(event)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RunEventEnvelope {
    schema_version: SchemaVersion,
    run_id: RunId,
    sequence: EventSequence,
    recorded_at: EpochMillis,
    previous_digest: Option<LedgerDigest>,
    event: RunEvent,
    event_digest: LedgerDigest,
}

impl RunEventEnvelope {
    pub fn new(
        run_id: RunId,
        sequence: EventSequence,
        recorded_at: EpochMillis,
        previous_digest: Option<LedgerDigest>,
        event: RunEvent,
    ) -> Result<Self, RunError> {
        event.validate_basic()?;
        Self::validate_sequence_shape(sequence, previous_digest.as_ref(), &event)?;
        let schema_version = SchemaVersion::V1_0;
        let event_digest = LedgerDigest::for_event(
            schema_version,
            run_id,
            sequence,
            recorded_at,
            previous_digest.as_ref(),
            &event,
        )?;
        Ok(Self {
            schema_version,
            run_id,
            sequence,
            recorded_at,
            previous_digest,
            event,
            event_digest,
        })
    }

    fn from_wire(wire: RunEventEnvelopeWire) -> Result<Self, RunError> {
        validate_schema_version(wire.schema_version)?;
        wire.event.validate_basic()?;
        Self::validate_sequence_shape(wire.sequence, wire.previous_digest.as_ref(), &wire.event)?;
        let expected = LedgerDigest::for_event(
            wire.schema_version,
            wire.run_id,
            wire.sequence,
            wire.recorded_at,
            wire.previous_digest.as_ref(),
            &wire.event,
        )?;
        if expected != wire.event_digest {
            return Err(RunError::ledger_digest_mismatch(
                "event_digest does not match the canonical run-event preimage",
            ));
        }
        Ok(Self {
            schema_version: wire.schema_version,
            run_id: wire.run_id,
            sequence: wire.sequence,
            recorded_at: wire.recorded_at,
            previous_digest: wire.previous_digest,
            event: wire.event,
            event_digest: wire.event_digest,
        })
    }

    fn validate_sequence_shape(
        sequence: EventSequence,
        previous_digest: Option<&LedgerDigest>,
        event: &RunEvent,
    ) -> Result<(), RunError> {
        if sequence.get() == 1 {
            if previous_digest.is_some() {
                return Err(RunError::ledger_chain_invalid(
                    "genesis event requires previous_digest = null",
                ));
            }
            if !event.is_run_created() {
                return Err(RunError::invalid_event(
                    "sequence 1 must contain run_created",
                ));
            }
        } else {
            if previous_digest.is_none() {
                return Err(RunError::ledger_chain_invalid(
                    "non-genesis event requires previous_digest",
                ));
            }
            if event.is_run_created() {
                return Err(RunError::invalid_event(
                    "run_created is valid only at sequence 1",
                ));
            }
        }
        Ok(())
    }

    pub fn validate_successor(&self, previous: &Self) -> Result<(), RunError> {
        if self.run_id != previous.run_id {
            return Err(RunError::ledger_chain_invalid(
                "successor run_id does not match previous envelope",
            ));
        }
        if self.sequence != previous.sequence.checked_next()? {
            return Err(RunError::invalid_event_sequence(
                "successor sequence is not exactly previous + 1",
            ));
        }
        if self.previous_digest.as_ref() != Some(&previous.event_digest) {
            return Err(RunError::ledger_chain_invalid(
                "successor previous_digest does not bind the previous event_digest",
            ));
        }
        Ok(())
    }

    pub fn canonical_digest_material(&self) -> Result<Vec<u8>, RunError> {
        canonical_event_material(
            self.schema_version,
            self.run_id,
            self.sequence,
            self.recorded_at,
            self.previous_digest.as_ref(),
            &self.event,
        )
    }

    pub fn from_json_slice(input: &[u8]) -> Result<Self, RunError> {
        let raw: serde_json::Value = serde_json::from_slice(input)
            .map_err(|error| RunError::serialization(error.to_string()))?;
        preflight_version_and_sequence(&raw)?;
        let wire: RunEventEnvelopeWire = serde_json::from_value(raw)
            .map_err(|error| RunError::serialization(error.to_string()))?;
        Self::from_wire(wire)
    }

    #[must_use]
    pub const fn schema_version(&self) -> SchemaVersion {
        self.schema_version
    }

    #[must_use]
    pub const fn run_id(&self) -> RunId {
        self.run_id
    }

    #[must_use]
    pub const fn sequence(&self) -> EventSequence {
        self.sequence
    }

    #[must_use]
    pub const fn recorded_at(&self) -> EpochMillis {
        self.recorded_at
    }

    #[must_use]
    pub const fn previous_digest(&self) -> Option<&LedgerDigest> {
        self.previous_digest.as_ref()
    }

    #[must_use]
    pub const fn event(&self) -> &RunEvent {
        &self.event
    }

    #[must_use]
    pub const fn event_digest(&self) -> &LedgerDigest {
        &self.event_digest
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RunEventEnvelopeWire {
    schema_version: SchemaVersion,
    run_id: RunId,
    sequence: EventSequence,
    recorded_at: EpochMillis,
    previous_digest: Option<LedgerDigest>,
    event: RunEvent,
    event_digest: LedgerDigest,
}

impl<'de> Deserialize<'de> for RunEventEnvelope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::from_wire(RunEventEnvelopeWire::deserialize(deserializer)?).map_err(de::Error::custom)
    }
}

fn validate_schema_version(version: SchemaVersion) -> Result<(), RunError> {
    if version.major() != 1 {
        return Err(RunError::new(
            RunErrorCategory::Compatibility,
            RunErrorCode::UnsupportedMajorVersion,
            format!("unsupported run-event major version {}", version.major()),
        ));
    }
    if version.minor() > 0 {
        return Err(RunError::new(
            RunErrorCategory::Compatibility,
            RunErrorCode::UnsupportedMinorVersion,
            format!("unsupported run-event minor version {}", version.minor()),
        ));
    }
    Ok(())
}

fn preflight_version_and_sequence(raw: &serde_json::Value) -> Result<(), RunError> {
    let object = raw
        .as_object()
        .ok_or_else(|| RunError::serialization("run event envelope must be a JSON object"))?;
    let version = object
        .get("schema_version")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| RunError::serialization("missing or malformed schema_version"))?;
    let major = version
        .get("major")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| RunError::serialization("missing or malformed schema_version.major"))?;
    let minor = version
        .get("minor")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| RunError::serialization("missing or malformed schema_version.minor"))?;
    if major != 1 {
        return Err(RunError::new(
            RunErrorCategory::Compatibility,
            RunErrorCode::UnsupportedMajorVersion,
            format!("unsupported run-event major version {major}"),
        ));
    }
    if minor > 0 {
        return Err(RunError::new(
            RunErrorCategory::Compatibility,
            RunErrorCode::UnsupportedMinorVersion,
            format!("unsupported run-event minor version {minor}"),
        ));
    }
    let sequence = object
        .get("sequence")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| RunError::serialization("missing or malformed sequence"))?;
    EventSequence::new(sequence)?;
    Ok(())
}
