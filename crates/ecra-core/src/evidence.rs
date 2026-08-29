use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use crate::{
    ActorId, ArtifactId, ContentDigest, DomainError, EpochMillis, EvidenceId, FactId,
    I_JSON_MAX_SAFE_INTEGER, I_JSON_MIN_SAFE_INTEGER, InformationClassification, InformationRef,
    ObservationId, Origin, ReceiptId, ResourceId, ResourceRef,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    Observation,
    Artifact,
    StructuredToolResult,
    NetworkReceipt,
    ExternalState,
    Computation,
    ModelJudgment,
    Other,
}

/// Stable evidence identity plus optional typed links. This type never embeds a
/// large evidence blob and its kind proves nothing by itself.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceRef {
    id: EvidenceId,
    kind: EvidenceKind,
    artifact: Option<ArtifactId>,
    observation: Option<ObservationId>,
    receipt: Option<ReceiptId>,
    external_ref: Option<String>,
    content_digest: Option<ContentDigest>,
    as_of: Option<EpochMillis>,
}

impl EvidenceRef {
    #[must_use]
    pub fn new(id: EvidenceId, kind: EvidenceKind) -> Self {
        Self {
            id,
            kind,
            artifact: None,
            observation: None,
            receipt: None,
            external_ref: None,
            content_digest: None,
            as_of: None,
        }
    }

    #[must_use]
    pub fn with_artifact(mut self, artifact: ArtifactId) -> Self {
        self.artifact = Some(artifact);
        self
    }

    #[must_use]
    pub fn with_observation(mut self, observation: ObservationId) -> Self {
        self.observation = Some(observation);
        self
    }

    #[must_use]
    pub fn with_receipt(mut self, receipt: ReceiptId) -> Self {
        self.receipt = Some(receipt);
        self
    }

    pub fn with_external_ref(mut self, value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if value.is_empty() {
            return Err(DomainError::InvalidInformation(
                "evidence external_ref must be non-empty".to_owned(),
            ));
        }
        self.external_ref = Some(value);
        Ok(self)
    }

    #[must_use]
    pub fn with_content_digest(mut self, digest: ContentDigest) -> Self {
        self.content_digest = Some(digest);
        self
    }

    #[must_use]
    pub fn with_as_of(mut self, as_of: EpochMillis) -> Self {
        self.as_of = Some(as_of);
        self
    }

    #[must_use]
    pub const fn id(&self) -> EvidenceId {
        self.id
    }

    #[must_use]
    pub const fn kind(&self) -> EvidenceKind {
        self.kind
    }

    #[must_use]
    pub const fn artifact(&self) -> Option<ArtifactId> {
        self.artifact
    }

    #[must_use]
    pub const fn observation(&self) -> Option<ObservationId> {
        self.observation
    }

    #[must_use]
    pub const fn receipt(&self) -> Option<ReceiptId> {
        self.receipt
    }

    #[must_use]
    pub fn external_ref(&self) -> Option<&str> {
        self.external_ref.as_deref()
    }

    #[must_use]
    pub const fn content_digest(&self) -> Option<&ContentDigest> {
        self.content_digest.as_ref()
    }

    #[must_use]
    pub const fn as_of(&self) -> Option<EpochMillis> {
        self.as_of
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EvidenceRefWire {
    id: EvidenceId,
    kind: EvidenceKind,
    artifact: Option<ArtifactId>,
    observation: Option<ObservationId>,
    receipt: Option<ReceiptId>,
    external_ref: Option<String>,
    content_digest: Option<ContentDigest>,
    as_of: Option<EpochMillis>,
}

impl<'de> Deserialize<'de> for EvidenceRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = EvidenceRefWire::deserialize(deserializer)?;
        let mut value = Self::new(wire.id, wire.kind);
        if let Some(artifact) = wire.artifact {
            value = value.with_artifact(artifact);
        }
        if let Some(observation) = wire.observation {
            value = value.with_observation(observation);
        }
        if let Some(receipt) = wire.receipt {
            value = value.with_receipt(receipt);
        }
        if let Some(external_ref) = wire.external_ref {
            value = value
                .with_external_ref(external_ref)
                .map_err(de::Error::custom)?;
        }
        if let Some(digest) = wire.content_digest {
            value = value.with_content_digest(digest);
        }
        if let Some(as_of) = wire.as_of {
            value = value.with_as_of(as_of);
        }
        Ok(value)
    }
}

/// Reference to observation payload data; arbitrary payload blobs stay outside
/// the trusted domain object.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum ObservationPayloadRef {
    Artifact(ArtifactId),
    Evidence(EvidenceId),
    Resource(ResourceId),
    ExternalRef(String),
}

#[derive(Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum ObservationPayloadRefWire {
    Artifact(ArtifactId),
    Evidence(EvidenceId),
    Resource(ResourceId),
    ExternalRef(String),
}

impl<'de> Deserialize<'de> for ObservationPayloadRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match ObservationPayloadRefWire::deserialize(deserializer)? {
            ObservationPayloadRefWire::Artifact(id) => Ok(Self::Artifact(id)),
            ObservationPayloadRefWire::Evidence(id) => Ok(Self::Evidence(id)),
            ObservationPayloadRefWire::Resource(id) => Ok(Self::Resource(id)),
            ObservationPayloadRefWire::ExternalRef(value) if !value.is_empty() => {
                Ok(Self::ExternalRef(value))
            }
            ObservationPayloadRefWire::ExternalRef(_) => Err(de::Error::custom(
                "observation payload external_ref must be non-empty",
            )),
        }
    }
}

/// What an actor observed at a source. An observation is not universal truth.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    id: ObservationId,
    actor: ActorId,
    origin: Origin,
    observed_at: Option<EpochMillis>,
    subject: ResourceRef,
    payload: ObservationPayloadRef,
    classification: InformationClassification,
    evidence: Vec<EvidenceRef>,
}

impl Observation {
    #[must_use]
    pub const fn id(&self) -> ObservationId {
        self.id
    }

    #[must_use]
    pub const fn classification(&self) -> &InformationClassification {
        &self.classification
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessState {
    Current,
    Stale,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessBasisKind {
    ObservedAt,
    RetrievedAt,
    PublishedAt,
    EffectiveAt,
    SourceReported,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FreshnessAssessment {
    state: FreshnessState,
    assessed_at: Option<EpochMillis>,
    basis_kind: Option<FreshnessBasisKind>,
    basis_time: Option<EpochMillis>,
    basis_evidence: Option<EvidenceId>,
}

impl FreshnessAssessment {
    pub fn new(
        state: FreshnessState,
        assessed_at: Option<EpochMillis>,
        basis_kind: Option<FreshnessBasisKind>,
        basis_time: Option<EpochMillis>,
        basis_evidence: Option<EvidenceId>,
    ) -> Result<Self, DomainError> {
        if basis_kind.is_some() != basis_time.is_some() {
            return Err(DomainError::InvalidInformation(
                "freshness basis_kind and basis_time must appear together".to_owned(),
            ));
        }
        Ok(Self {
            state,
            assessed_at,
            basis_kind,
            basis_time,
            basis_evidence,
        })
    }

    #[must_use]
    pub const fn state(self) -> FreshnessState {
        self.state
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FreshnessAssessmentWire {
    state: FreshnessState,
    assessed_at: Option<EpochMillis>,
    basis_kind: Option<FreshnessBasisKind>,
    basis_time: Option<EpochMillis>,
    basis_evidence: Option<EvidenceId>,
}

impl<'de> Deserialize<'de> for FreshnessAssessment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = FreshnessAssessmentWire::deserialize(deserializer)?;
        Self::new(
            wire.state,
            wire.assessed_at,
            wire.basis_kind,
            wire.basis_time,
            wire.basis_evidence,
        )
        .map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Provenance {
    UserProvided,
    ObservedWeb,
    ObservedLocal,
    Retrieved,
    ToolProvided,
    ModelInferred,
    SystemDerived,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisputeState {
    Undisputed,
    Contradicted,
    Disputed,
    Inconclusive,
    Unknown,
}

fn validate_decimal(value: &str) -> bool {
    if value.is_empty() || value.starts_with('+') {
        return false;
    }
    let (negative, body) = match value.strip_prefix('-') {
        Some(body) => (true, body),
        None => (false, value),
    };
    if body.is_empty() {
        return false;
    }
    let (integer, fraction) = match body.split_once('.') {
        Some((integer, fraction)) => (integer, Some(fraction)),
        None => (body, None),
    };
    if integer.is_empty()
        || !integer.bytes().all(|byte| byte.is_ascii_digit())
        || (integer.len() > 1 && integer.starts_with('0'))
        || fraction.is_some_and(|fraction| {
            fraction.is_empty() || !fraction.bytes().all(|byte| byte.is_ascii_digit())
        })
    {
        return false;
    }
    if negative
        && integer.bytes().all(|byte| byte == b'0')
        && fraction.is_none_or(|fraction| fraction.bytes().all(|byte| byte == b'0'))
    {
        return false;
    }
    true
}

/// I-JSON-safe integer payload for [`FactValue`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FactInteger(i64);

impl FactInteger {
    pub fn new(value: i64) -> Result<Self, DomainError> {
        if !(I_JSON_MIN_SAFE_INTEGER..=I_JSON_MAX_SAFE_INTEGER).contains(&value) {
            return Err(DomainError::InvalidInformation(
                "fact integer must remain in the I-JSON exact integer range".to_owned(),
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}

impl Serialize for FactInteger {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.0)
    }
}

impl<'de> Deserialize<'de> for FactInteger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i64::deserialize(deserializer)?;
        Self::new(value).map_err(de::Error::custom)
    }
}

/// Canonical decimal-string payload for [`FactValue`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactDecimal(String);

impl FactDecimal {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if !validate_decimal(&value) {
            return Err(DomainError::InvalidInformation(
                "fact decimal must use canonical decimal-string form".to_owned(),
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Serialize for FactDecimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for FactDecimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum FactValue {
    Text(String),
    Boolean(bool),
    Integer(FactInteger),
    Decimal(FactDecimal),
    Resource(ResourceId),
    Artifact(ArtifactId),
}

impl FactValue {
    pub fn integer(value: i64) -> Result<Self, DomainError> {
        FactInteger::new(value).map(Self::Integer)
    }

    pub fn decimal(value: impl Into<String>) -> Result<Self, DomainError> {
        FactDecimal::new(value).map(Self::Decimal)
    }
}

#[derive(Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum FactValueWire {
    Text(String),
    Boolean(bool),
    Integer(FactInteger),
    Decimal(FactDecimal),
    Resource(ResourceId),
    Artifact(ArtifactId),
}

impl<'de> Deserialize<'de> for FactValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match FactValueWire::deserialize(deserializer)? {
            FactValueWire::Text(value) => Ok(Self::Text(value)),
            FactValueWire::Boolean(value) => Ok(Self::Boolean(value)),
            FactValueWire::Integer(value) => Ok(Self::Integer(value)),
            FactValueWire::Decimal(value) => Ok(Self::Decimal(value)),
            FactValueWire::Resource(id) => Ok(Self::Resource(id)),
            FactValueWire::Artifact(id) => Ok(Self::Artifact(id)),
        }
    }
}

/// The three independent assessment axes that accompany a Fact. This helper is
/// an API construction value only; the canonical Fact JSON remains flat.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FactAssessment {
    provenance: Provenance,
    freshness: FreshnessAssessment,
    dispute: DisputeState,
}

impl FactAssessment {
    #[must_use]
    pub const fn new(
        provenance: Provenance,
        freshness: FreshnessAssessment,
        dispute: DisputeState,
    ) -> Self {
        Self {
            provenance,
            freshness,
            dispute,
        }
    }
}

/// A claim derived from evidence. There is deliberately no `verified` field;
/// independent verification is represented only by VerificationReceipt later.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Fact {
    id: FactId,
    subject: ResourceRef,
    predicate: String,
    value: FactValue,
    provenance: Provenance,
    classification: InformationClassification,
    freshness: FreshnessAssessment,
    dispute: DisputeState,
    evidence: Vec<EvidenceRef>,
    derived_from: Vec<InformationRef>,
}

impl Fact {
    pub fn new(
        id: FactId,
        subject: ResourceRef,
        predicate: impl Into<String>,
        value: FactValue,
        classification: InformationClassification,
        assessment: FactAssessment,
    ) -> Result<Self, DomainError> {
        let predicate = predicate.into();
        if predicate.is_empty() {
            return Err(DomainError::InvalidInformation(
                "fact predicate must be non-empty".to_owned(),
            ));
        }
        Ok(Self {
            id,
            subject,
            predicate,
            value,
            provenance: assessment.provenance,
            classification,
            freshness: assessment.freshness,
            dispute: assessment.dispute,
            evidence: Vec::new(),
            derived_from: Vec::new(),
        })
    }

    #[must_use]
    pub fn with_evidence(mut self, evidence: Vec<EvidenceRef>) -> Self {
        self.evidence = evidence;
        self
    }

    #[must_use]
    pub fn with_derived_from(mut self, derived_from: Vec<InformationRef>) -> Self {
        self.derived_from = derived_from;
        self
    }

    #[must_use]
    pub const fn provenance(&self) -> Provenance {
        self.provenance
    }

    #[must_use]
    pub const fn classification(&self) -> &InformationClassification {
        &self.classification
    }

    #[must_use]
    pub const fn freshness(&self) -> FreshnessAssessment {
        self.freshness
    }

    #[must_use]
    pub const fn dispute(&self) -> DisputeState {
        self.dispute
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FactWire {
    id: FactId,
    subject: ResourceRef,
    predicate: String,
    value: FactValue,
    provenance: Provenance,
    classification: InformationClassification,
    freshness: FreshnessAssessment,
    dispute: DisputeState,
    evidence: Vec<EvidenceRef>,
    derived_from: Vec<InformationRef>,
}

impl<'de> Deserialize<'de> for Fact {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = FactWire::deserialize(deserializer)?;
        Self::new(
            wire.id,
            wire.subject,
            wire.predicate,
            wire.value,
            wire.classification,
            FactAssessment::new(wire.provenance, wire.freshness, wire.dispute),
        )
        .map(|value| {
            value
                .with_evidence(wire.evidence)
                .with_derived_from(wire.derived_from)
        })
        .map_err(de::Error::custom)
    }
}
