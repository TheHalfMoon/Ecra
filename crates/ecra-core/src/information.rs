use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{
    ActionParameterRef, ArtifactId, DomainError, FactId, ObservationId, ResourceRef, WebOrigin,
};

/// Conservative information classification carried independently from authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InformationClass {
    Public,
    Private,
    Sensitive,
    Secret,
    Unknown,
}

/// Opaque structured tag for later policy. Tag text is data, not executable policy.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InformationPolicyTag {
    namespace: String,
    name: String,
}

impl InformationPolicyTag {
    pub fn new(namespace: impl Into<String>, name: impl Into<String>) -> Result<Self, DomainError> {
        let namespace = namespace.into();
        let name = name.into();
        if namespace.is_empty() || name.is_empty() {
            return Err(DomainError::InvalidInformation(
                "information policy tag namespace and name must be non-empty".to_owned(),
            ));
        }
        Ok(Self { namespace, name })
    }

    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InformationPolicyTagWire {
    namespace: String,
    name: String,
}

impl<'de> Deserialize<'de> for InformationPolicyTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = InformationPolicyTagWire::deserialize(deserializer)?;
        Self::new(wire.namespace, wire.name).map_err(de::Error::custom)
    }
}

/// Classification metadata. Classification never grants permission or authority.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InformationClassification {
    class: InformationClass,
    policy_tags: Vec<InformationPolicyTag>,
}

impl InformationClassification {
    #[must_use]
    pub fn new(class: InformationClass, policy_tags: Vec<InformationPolicyTag>) -> Self {
        Self { class, policy_tags }
    }

    #[must_use]
    pub const fn class(&self) -> InformationClass {
        self.class
    }

    #[must_use]
    pub fn policy_tags(&self) -> &[InformationPolicyTag] {
        &self.policy_tags
    }
}

/// Stable reference to information used for lineage and later source-to-sink policy.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "id",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum InformationRef {
    Observation(ObservationId),
    Fact(FactId),
    Artifact(ArtifactId),
    ActionParameter(ActionParameterRef),
}

/// Declared purpose for using information. This enum has no authorization semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InformationUseKind {
    LocalCompute,
    ModelContext,
    Persist,
    LogOrDiagnostic,
    ExternalDisclosure,
    RemoteProvider,
    Other,
}

/// A source-to-sink use declaration. It is intentionally not an authorization object.
///
/// A downstream policy engine may evaluate this declaration in ECR-003. ECR-001
/// only preserves the requested flow and fails closed on structurally invalid data.
///
/// ```compile_fail
/// use ecra_core::{CapabilityGrant, InformationUse};
///
/// fn requires_grant(_: CapabilityGrant) {}
/// let declared_use: InformationUse = todo!();
/// requires_grant(declared_use);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InformationUse {
    sources: Vec<InformationRef>,
    kind: InformationUseKind,
    destination: Option<ResourceRef>,
    destination_origin: Option<WebOrigin>,
    declared_output_classification: Option<InformationClassification>,
}

impl InformationUse {
    pub fn new(
        sources: Vec<InformationRef>,
        kind: InformationUseKind,
    ) -> Result<Self, DomainError> {
        if sources.is_empty() {
            return Err(DomainError::InvalidInformation(
                "information use requires at least one source".to_owned(),
            ));
        }
        Ok(Self {
            sources,
            kind,
            destination: None,
            destination_origin: None,
            declared_output_classification: None,
        })
    }

    #[must_use]
    pub fn with_destination(mut self, destination: ResourceRef) -> Self {
        self.destination = Some(destination);
        self
    }

    #[must_use]
    pub fn with_destination_origin(mut self, origin: WebOrigin) -> Self {
        self.destination_origin = Some(origin);
        self
    }

    #[must_use]
    pub fn with_declared_output_classification(
        mut self,
        classification: InformationClassification,
    ) -> Self {
        self.declared_output_classification = Some(classification);
        self
    }

    #[must_use]
    pub fn sources(&self) -> &[InformationRef] {
        &self.sources
    }

    #[must_use]
    pub const fn kind(&self) -> InformationUseKind {
        self.kind
    }

    #[must_use]
    pub const fn destination(&self) -> Option<&ResourceRef> {
        self.destination.as_ref()
    }

    #[must_use]
    pub const fn destination_origin(&self) -> Option<&WebOrigin> {
        self.destination_origin.as_ref()
    }

    #[must_use]
    pub const fn declared_output_classification(&self) -> Option<&InformationClassification> {
        self.declared_output_classification.as_ref()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InformationUseWire {
    sources: Vec<InformationRef>,
    kind: InformationUseKind,
    destination: Option<ResourceRef>,
    destination_origin: Option<WebOrigin>,
    declared_output_classification: Option<InformationClassification>,
}

impl<'de> Deserialize<'de> for InformationUse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = InformationUseWire::deserialize(deserializer)?;
        let mut value = Self::new(wire.sources, wire.kind).map_err(de::Error::custom)?;
        if let Some(destination) = wire.destination {
            value = value.with_destination(destination);
        }
        if let Some(origin) = wire.destination_origin {
            value = value.with_destination_origin(origin);
        }
        if let Some(classification) = wire.declared_output_classification {
            value = value.with_declared_output_classification(classification);
        }
        Ok(value)
    }
}
