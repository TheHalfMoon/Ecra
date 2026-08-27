use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{
    ActionAttemptRef, ActionRef, ActorId, ArtifactId, DomainError, EpochMillis, EvidenceRef, FactId,
    PrincipalRef, ReceiptId, VerificationId,
};

/// Opaque structured claim target. Its strings are descriptive identity only;
/// they are not policy syntax or evidence of truth.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimRef {
    namespace: String,
    reference: String,
}

impl ClaimRef {
    pub fn new(
        namespace: impl Into<String>,
        reference: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let namespace = namespace.into();
        let reference = reference.into();
        if namespace.is_empty() || reference.is_empty() {
            return Err(DomainError::InvalidVerification(
                "claim namespace and reference must be non-empty".to_owned(),
            ));
        }
        Ok(Self {
            namespace,
            reference,
        })
    }

    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ClaimRefWire {
    namespace: String,
    reference: String,
}

impl<'de> Deserialize<'de> for ClaimRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ClaimRefWire::deserialize(deserializer)?;
        Self::new(wire.namespace, wire.reference).map_err(de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum VerificationTarget {
    Action(ActionRef),
    ActionAttempt(ActionAttemptRef),
    Receipt(ReceiptId),
    Fact(FactId),
    Artifact(ArtifactId),
    Claim(ClaimRef),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationMethod {
    StructuredExternalState,
    ApiOrToolResult,
    NetworkReceipt,
    ArtifactValidation,
    DomOrAccessibilityState,
    DeterministicComputation,
    IndependentModelJudgment,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationOutcome {
    Verified,
    Rejected,
    Inconclusive,
    NotEvaluated,
}

/// Independent verification record. It never mutates the provenance or truth
/// state of its target object; consumers read verification from this record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationReceipt {
    id: VerificationId,
    verifier: ActorId,
    verifier_principal: Option<PrincipalRef>,
    target: VerificationTarget,
    method: VerificationMethod,
    evidence: Vec<EvidenceRef>,
    outcome: VerificationOutcome,
    evaluated_at: Option<EpochMillis>,
    notes: Option<String>,
}

impl VerificationReceipt {
    pub fn new(
        id: VerificationId,
        verifier: ActorId,
        target: VerificationTarget,
        method: VerificationMethod,
        outcome: VerificationOutcome,
        evidence: Vec<EvidenceRef>,
    ) -> Result<Self, DomainError> {
        if outcome != VerificationOutcome::NotEvaluated && evidence.is_empty() {
            return Err(DomainError::InvalidVerification(
                "verified, rejected and inconclusive outcomes require evidence".to_owned(),
            ));
        }
        Ok(Self {
            id,
            verifier,
            verifier_principal: None,
            target,
            method,
            evidence,
            outcome,
            evaluated_at: None,
            notes: None,
        })
    }

    #[must_use]
    pub fn with_verifier_principal(mut self, verifier_principal: PrincipalRef) -> Self {
        self.verifier_principal = Some(verifier_principal);
        self
    }

    #[must_use]
    pub fn with_evaluated_at(mut self, evaluated_at: EpochMillis) -> Self {
        self.evaluated_at = Some(evaluated_at);
        self
    }

    pub fn with_notes(mut self, notes: impl Into<String>) -> Result<Self, DomainError> {
        let notes = notes.into();
        if notes.is_empty() {
            return Err(DomainError::InvalidVerification(
                "verification notes must be non-empty when present".to_owned(),
            ));
        }
        self.notes = Some(notes);
        Ok(self)
    }

    #[must_use]
    pub const fn id(&self) -> VerificationId {
        self.id
    }

    #[must_use]
    pub const fn target(&self) -> &VerificationTarget {
        &self.target
    }

    #[must_use]
    pub const fn outcome(&self) -> VerificationOutcome {
        self.outcome
    }

    #[must_use]
    pub fn evidence(&self) -> &[EvidenceRef] {
        &self.evidence
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VerificationReceiptWire {
    id: VerificationId,
    verifier: ActorId,
    verifier_principal: Option<PrincipalRef>,
    target: VerificationTarget,
    method: VerificationMethod,
    evidence: Vec<EvidenceRef>,
    outcome: VerificationOutcome,
    evaluated_at: Option<EpochMillis>,
    notes: Option<String>,
}

impl<'de> Deserialize<'de> for VerificationReceipt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = VerificationReceiptWire::deserialize(deserializer)?;
        let mut value = Self::new(
            wire.id,
            wire.verifier,
            wire.target,
            wire.method,
            wire.outcome,
            wire.evidence,
        )
        .map_err(de::Error::custom)?;
        if let Some(verifier_principal) = wire.verifier_principal {
            value = value.with_verifier_principal(verifier_principal);
        }
        if let Some(evaluated_at) = wire.evaluated_at {
            value = value.with_evaluated_at(evaluated_at);
        }
        if let Some(notes) = wire.notes {
            value = value.with_notes(notes).map_err(de::Error::custom)?;
        }
        Ok(value)
    }
}
