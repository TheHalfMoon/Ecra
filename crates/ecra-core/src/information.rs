use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{ArtifactId, DomainError, FactId, ObservationId};

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
///
/// Phase 5 introduces the variants needed by Fact lineage. The action-parameter
/// variant is added before ECR-001 v1 closure when ActionParametersRef exists.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
pub enum InformationRef {
    Observation(ObservationId),
    Fact(FactId),
    Artifact(ArtifactId),
}
