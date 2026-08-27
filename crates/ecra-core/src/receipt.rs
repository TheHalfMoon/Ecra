use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{
    ActionAttemptRef, ActionIntent, ActorId, DomainError, EpochMillis, EvidenceRef, ReceiptId,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionOutcome {
    ExecutorObservedSuccess,
    ExecutorObservedFailure,
    Unknown,
}

/// Bounded executor diagnostic metadata. This is neither a DomainError nor a
/// verification result, and provider text never becomes authority.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ErrorSummary {
    code: String,
    message: Option<String>,
}

impl ErrorSummary {
    pub fn new(code: impl Into<String>, message: Option<String>) -> Result<Self, DomainError> {
        let code = code.into();
        if code.is_empty() {
            return Err(DomainError::InvalidReceipt(
                "receipt error code must be non-empty".to_owned(),
            ));
        }
        if message.as_ref().is_some_and(String::is_empty) {
            return Err(DomainError::InvalidReceipt(
                "receipt error message must be non-empty when present".to_owned(),
            ));
        }
        Ok(Self { code, message })
    }

    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ErrorSummaryWire {
    code: String,
    message: Option<String>,
}

impl<'de> Deserialize<'de> for ErrorSummary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ErrorSummaryWire::deserialize(deserializer)?;
        Self::new(wire.code, wire.message).map_err(de::Error::custom)
    }
}

/// Executor-known record for one exact action attempt.
///
/// `executor_observed_success` is explicitly not independent verification.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActionReceipt {
    id: ReceiptId,
    attempt: ActionAttemptRef,
    executor_actor: ActorId,
    started_at: Option<EpochMillis>,
    completed_at: Option<EpochMillis>,
    outcome: ActionOutcome,
    evidence: Vec<EvidenceRef>,
    external_reference: Option<String>,
    error: Option<ErrorSummary>,
}

impl ActionReceipt {
    #[must_use]
    pub fn new(
        id: ReceiptId,
        attempt: ActionAttemptRef,
        executor_actor: ActorId,
        outcome: ActionOutcome,
    ) -> Self {
        Self {
            id,
            attempt,
            executor_actor,
            started_at: None,
            completed_at: None,
            outcome,
            evidence: Vec::new(),
            external_reference: None,
            error: None,
        }
    }

    pub fn with_timing(
        mut self,
        started_at: Option<EpochMillis>,
        completed_at: Option<EpochMillis>,
    ) -> Result<Self, DomainError> {
        if let (Some(start), Some(end)) = (started_at, completed_at)
            && end < start
        {
            return Err(DomainError::InvalidReceipt(
                "receipt completed_at must not precede started_at".to_owned(),
            ));
        }
        self.started_at = started_at;
        self.completed_at = completed_at;
        Ok(self)
    }

    #[must_use]
    pub fn with_evidence(mut self, evidence: Vec<EvidenceRef>) -> Self {
        self.evidence = evidence;
        self
    }

    pub fn with_external_reference(
        mut self,
        external_reference: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let external_reference = external_reference.into();
        if external_reference.is_empty() {
            return Err(DomainError::InvalidReceipt(
                "receipt external_reference must be non-empty".to_owned(),
            ));
        }
        self.external_reference = Some(external_reference);
        Ok(self)
    }

    #[must_use]
    pub fn with_error(mut self, error: ErrorSummary) -> Self {
        self.error = Some(error);
        self
    }

    #[must_use]
    pub const fn id(&self) -> ReceiptId {
        self.id
    }

    #[must_use]
    pub const fn attempt(&self) -> &ActionAttemptRef {
        &self.attempt
    }

    #[must_use]
    pub const fn outcome(&self) -> ActionOutcome {
        self.outcome
    }

    #[must_use]
    pub fn evidence(&self) -> &[EvidenceRef] {
        &self.evidence
    }

    pub fn validate_for(&self, intent: &ActionIntent) -> Result<(), DomainError> {
        self.attempt.validate_for(intent).map_err(|_| {
            DomainError::InvalidReceipt(
                "receipt attempt does not bind the exact action intent".to_owned(),
            )
        })
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionReceiptWire {
    id: ReceiptId,
    attempt: ActionAttemptRef,
    executor_actor: ActorId,
    started_at: Option<EpochMillis>,
    completed_at: Option<EpochMillis>,
    outcome: ActionOutcome,
    evidence: Vec<EvidenceRef>,
    external_reference: Option<String>,
    error: Option<ErrorSummary>,
}

impl<'de> Deserialize<'de> for ActionReceipt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ActionReceiptWire::deserialize(deserializer)?;
        let mut value = Self::new(wire.id, wire.attempt, wire.executor_actor, wire.outcome)
            .with_timing(wire.started_at, wire.completed_at)
            .map_err(de::Error::custom)?
            .with_evidence(wire.evidence);
        if let Some(external_reference) = wire.external_reference {
            value = value
                .with_external_reference(external_reference)
                .map_err(de::Error::custom)?;
        }
        if let Some(error) = wire.error {
            value = value.with_error(error);
        }
        Ok(value)
    }
}
