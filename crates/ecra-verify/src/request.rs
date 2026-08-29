use std::collections::BTreeSet;

use ecra_core::{
    ActorId, EpochMillis, EvidenceRef, PrincipalRef, SchemaVersion, VerificationId,
    VerificationMethod, VerificationOutcome, VerificationReceipt, VerificationTarget,
};
use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{VerifyError, VerifyErrorCategory, VerifyErrorCode};

pub const MAX_EVIDENCE_REFS_PER_REQUEST: usize = 32;
pub const MAX_RULE_ID_BYTES: usize = 128;
pub const MAX_NOTES_BYTES: usize = 4096;
/// Maximum complete serialized v1 request size. This bounds every nested opaque
/// string/reference in addition to the field-specific ceilings below.
pub const MAX_VERIFICATION_REQUEST_BYTES: usize = 64 * 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerificationRequestFieldsV1 {
    pub receipt_id: VerificationId,
    pub verifier: ActorId,
    pub verifier_principal: Option<PrincipalRef>,
    pub target: VerificationTarget,
    pub method: VerificationMethod,
    pub evidence: Vec<EvidenceRef>,
    pub proposed_outcome: VerificationOutcome,
    pub evaluated_at: Option<EpochMillis>,
    pub rule_id: String,
    pub notes: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationRequestV1 {
    version: SchemaVersion,
    receipt_id: VerificationId,
    verifier: ActorId,
    verifier_principal: Option<PrincipalRef>,
    target: VerificationTarget,
    method: VerificationMethod,
    evidence: Vec<EvidenceRef>,
    proposed_outcome: VerificationOutcome,
    evaluated_at: Option<EpochMillis>,
    rule_id: String,
    notes: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VerificationRequestWire {
    version: SchemaVersion,
    receipt_id: VerificationId,
    verifier: ActorId,
    verifier_principal: Option<PrincipalRef>,
    target: VerificationTarget,
    method: VerificationMethod,
    evidence: Vec<EvidenceRef>,
    proposed_outcome: VerificationOutcome,
    evaluated_at: Option<EpochMillis>,
    rule_id: String,
    notes: Option<String>,
}

impl VerificationRequestV1 {
    pub fn from_fields(fields: VerificationRequestFieldsV1) -> Result<Self, VerifyError> {
        Self::validate(SchemaVersion::V1_0, fields)
    }

    fn from_wire(wire: VerificationRequestWire) -> Result<Self, VerifyError> {
        let fields = VerificationRequestFieldsV1 {
            receipt_id: wire.receipt_id,
            verifier: wire.verifier,
            verifier_principal: wire.verifier_principal,
            target: wire.target,
            method: wire.method,
            evidence: wire.evidence,
            proposed_outcome: wire.proposed_outcome,
            evaluated_at: wire.evaluated_at,
            rule_id: wire.rule_id,
            notes: wire.notes,
        };
        Self::validate(wire.version, fields)
    }

    fn validate(
        version: SchemaVersion,
        fields: VerificationRequestFieldsV1,
    ) -> Result<Self, VerifyError> {
        if version.validate_supported().is_err() || version != SchemaVersion::V1_0 {
            return Err(VerifyError::new(
                VerifyErrorCategory::Compatibility,
                VerifyErrorCode::UnsupportedVersion,
                "verification request version is not supported",
            ));
        }
        if fields.evidence.len() > MAX_EVIDENCE_REFS_PER_REQUEST {
            return Err(VerifyError::new(
                VerifyErrorCategory::ResourceLimit,
                VerifyErrorCode::ResourceLimitExceeded,
                "verification request evidence count exceeds the v1 limit",
            ));
        }
        let mut evidence_ids = BTreeSet::new();
        for evidence in &fields.evidence {
            if !evidence_ids.insert(evidence.id()) {
                return Err(VerifyError::new(
                    VerifyErrorCategory::Validation,
                    VerifyErrorCode::DuplicateId,
                    "verification request contains duplicate evidence ids",
                ));
            }
        }
        if fields.proposed_outcome != VerificationOutcome::NotEvaluated
            && fields.evidence.is_empty()
        {
            return Err(VerifyError::new(
                VerifyErrorCategory::Evidence,
                VerifyErrorCode::EvidenceInsufficient,
                "conclusive or inconclusive verification outcomes require evidence",
            ));
        }
        if fields.rule_id.is_empty() {
            return Err(VerifyError::new(
                VerifyErrorCategory::Validation,
                VerifyErrorCode::InvalidTarget,
                "verification rule_id must be non-empty",
            ));
        }
        if fields.rule_id.len() > MAX_RULE_ID_BYTES {
            return Err(VerifyError::new(
                VerifyErrorCategory::ResourceLimit,
                VerifyErrorCode::ResourceLimitExceeded,
                "verification rule_id exceeds the v1 byte limit",
            ));
        }
        if let Some(notes) = &fields.notes {
            if notes.is_empty() {
                return Err(VerifyError::new(
                    VerifyErrorCategory::Validation,
                    VerifyErrorCode::InvalidTarget,
                    "verification notes must be non-empty when present",
                ));
            }
            if notes.len() > MAX_NOTES_BYTES {
                return Err(VerifyError::new(
                    VerifyErrorCategory::ResourceLimit,
                    VerifyErrorCode::ResourceLimitExceeded,
                    "verification notes exceed the v1 byte limit",
                ));
            }
        }

        let value = Self {
            version,
            receipt_id: fields.receipt_id,
            verifier: fields.verifier,
            verifier_principal: fields.verifier_principal,
            target: fields.target,
            method: fields.method,
            evidence: fields.evidence,
            proposed_outcome: fields.proposed_outcome,
            evaluated_at: fields.evaluated_at,
            rule_id: fields.rule_id,
            notes: fields.notes,
        };
        let serialized = serde_json::to_vec(&value).map_err(|_| {
            VerifyError::new(
                VerifyErrorCategory::Validation,
                VerifyErrorCode::InvalidTarget,
                "verification request could not be size-checked",
            )
        })?;
        if serialized.len() > MAX_VERIFICATION_REQUEST_BYTES {
            return Err(VerifyError::new(
                VerifyErrorCategory::ResourceLimit,
                VerifyErrorCode::ResourceLimitExceeded,
                "verification request exceeds the complete v1 byte limit",
            ));
        }
        Ok(value)
    }

    pub fn from_json_slice(input: &[u8]) -> Result<Self, VerifyError> {
        if input.len() > MAX_VERIFICATION_REQUEST_BYTES {
            return Err(VerifyError::new(
                VerifyErrorCategory::ResourceLimit,
                VerifyErrorCode::ResourceLimitExceeded,
                "verification request JSON exceeds the complete v1 byte limit",
            ));
        }
        let wire: VerificationRequestWire = serde_json::from_slice(input).map_err(|_| {
            VerifyError::new(
                VerifyErrorCategory::Validation,
                VerifyErrorCode::InvalidTarget,
                "verification request JSON is malformed or contains unsupported fields",
            )
        })?;
        Self::from_wire(wire)
    }

    pub(crate) fn construct_receipt(&self) -> Result<VerificationReceipt, VerifyError> {
        let mut receipt = VerificationReceipt::new(
            self.receipt_id,
            self.verifier,
            self.target.clone(),
            self.method,
            self.proposed_outcome,
            self.evidence.clone(),
        )
        .map_err(|_| {
            VerifyError::new(
                VerifyErrorCategory::Verification,
                VerifyErrorCode::InvalidEvidence,
                "canonical verification receipt rejected validated request inputs",
            )
        })?;
        if let Some(principal) = self.verifier_principal {
            receipt = receipt.with_verifier_principal(principal);
        }
        if let Some(evaluated_at) = self.evaluated_at {
            receipt = receipt.with_evaluated_at(evaluated_at);
        }
        if let Some(notes) = &self.notes {
            receipt = receipt.with_notes(notes.clone()).map_err(|_| {
                VerifyError::new(
                    VerifyErrorCategory::Verification,
                    VerifyErrorCode::InvalidEvidence,
                    "canonical verification receipt rejected validated notes",
                )
            })?;
        }
        Ok(receipt)
    }

    #[must_use]
    pub const fn version(&self) -> SchemaVersion {
        self.version
    }

    #[must_use]
    pub const fn receipt_id(&self) -> VerificationId {
        self.receipt_id
    }

    #[must_use]
    pub const fn verifier(&self) -> ActorId {
        self.verifier
    }

    #[must_use]
    pub const fn verifier_principal(&self) -> Option<PrincipalRef> {
        self.verifier_principal
    }

    #[must_use]
    pub const fn target(&self) -> &VerificationTarget {
        &self.target
    }

    #[must_use]
    pub const fn method(&self) -> VerificationMethod {
        self.method
    }

    #[must_use]
    pub fn evidence(&self) -> &[EvidenceRef] {
        &self.evidence
    }

    #[must_use]
    pub const fn proposed_outcome(&self) -> VerificationOutcome {
        self.proposed_outcome
    }

    #[must_use]
    pub const fn evaluated_at(&self) -> Option<EpochMillis> {
        self.evaluated_at
    }

    #[must_use]
    pub fn rule_id(&self) -> &str {
        &self.rule_id
    }

    #[must_use]
    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }
}

impl<'de> Deserialize<'de> for VerificationRequestV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = VerificationRequestWire::deserialize(deserializer)?;
        Self::from_wire(wire).map_err(de::Error::custom)
    }
}
