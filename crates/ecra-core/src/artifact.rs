use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{
    ArtifactId, ContentDigest, DomainError, FactId, InformationClassification, ObservationId,
};

/// Stable lineage relation for an artifact. Locators and labels are not lineage identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
pub enum LineageRef {
    Observation(ObservationId),
    Fact(FactId),
    Artifact(ArtifactId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactKind {
    File,
    Document,
    Image,
    StructuredData,
    ModelOutput,
    BrowserSnapshot,
    NetworkCapture,
    Other,
}

/// Stable artifact identity plus deterministic metadata. `storage_locator` is
/// opaque/non-authoritative and never grants access to the referenced storage.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactRef {
    id: ArtifactId,
    kind: ArtifactKind,
    media_type: Option<String>,
    logical_name: Option<String>,
    classification: InformationClassification,
    content_digest: Option<ContentDigest>,
    byte_size_decimal: Option<String>,
    storage_locator: Option<String>,
    lineage: Vec<LineageRef>,
}

fn validate_optional_text(
    field: &'static str,
    value: Option<String>,
) -> Result<Option<String>, DomainError> {
    if value.as_ref().is_some_and(String::is_empty) {
        return Err(DomainError::InvalidInformation(format!(
            "artifact {field} must be non-empty when present"
        )));
    }
    Ok(value)
}

fn is_canonical_byte_size(value: &str) -> bool {
    value == "0"
        || (!value.is_empty()
            && !value.starts_with('0')
            && value.bytes().all(|byte| byte.is_ascii_digit()))
}

impl ArtifactRef {
    #[must_use]
    pub fn new(
        id: ArtifactId,
        kind: ArtifactKind,
        classification: InformationClassification,
    ) -> Self {
        Self {
            id,
            kind,
            media_type: None,
            logical_name: None,
            classification,
            content_digest: None,
            byte_size_decimal: None,
            storage_locator: None,
            lineage: Vec::new(),
        }
    }

    pub fn with_media_type(mut self, value: impl Into<String>) -> Result<Self, DomainError> {
        self.media_type = validate_optional_text("media_type", Some(value.into()))?;
        Ok(self)
    }

    pub fn with_logical_name(mut self, value: impl Into<String>) -> Result<Self, DomainError> {
        self.logical_name = validate_optional_text("logical_name", Some(value.into()))?;
        Ok(self)
    }

    #[must_use]
    pub fn with_content_digest(mut self, value: ContentDigest) -> Self {
        self.content_digest = Some(value);
        self
    }

    pub fn with_byte_size_decimal(mut self, value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if !is_canonical_byte_size(&value) {
            return Err(DomainError::InvalidInformation(
                "artifact byte_size_decimal must be canonical non-negative base-10 integer text"
                    .to_owned(),
            ));
        }
        self.byte_size_decimal = Some(value);
        Ok(self)
    }

    pub fn with_storage_locator(mut self, value: impl Into<String>) -> Result<Self, DomainError> {
        self.storage_locator = validate_optional_text("storage_locator", Some(value.into()))?;
        Ok(self)
    }

    #[must_use]
    pub fn with_lineage(mut self, lineage: Vec<LineageRef>) -> Self {
        self.lineage = lineage;
        self
    }

    #[must_use]
    pub const fn id(&self) -> ArtifactId {
        self.id
    }

    #[must_use]
    pub const fn classification(&self) -> &InformationClassification {
        &self.classification
    }

    #[must_use]
    pub fn byte_size_decimal(&self) -> Option<&str> {
        self.byte_size_decimal.as_deref()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ArtifactRefWire {
    id: ArtifactId,
    kind: ArtifactKind,
    media_type: Option<String>,
    logical_name: Option<String>,
    classification: InformationClassification,
    content_digest: Option<ContentDigest>,
    byte_size_decimal: Option<String>,
    storage_locator: Option<String>,
    lineage: Vec<LineageRef>,
}

impl<'de> Deserialize<'de> for ArtifactRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ArtifactRefWire::deserialize(deserializer)?;
        let mut value = Self::new(wire.id, wire.kind, wire.classification);
        if let Some(media_type) = wire.media_type {
            value = value.with_media_type(media_type).map_err(de::Error::custom)?;
        }
        if let Some(logical_name) = wire.logical_name {
            value = value.with_logical_name(logical_name).map_err(de::Error::custom)?;
        }
        if let Some(digest) = wire.content_digest {
            value = value.with_content_digest(digest);
        }
        if let Some(byte_size) = wire.byte_size_decimal {
            value = value
                .with_byte_size_decimal(byte_size)
                .map_err(de::Error::custom)?;
        }
        if let Some(locator) = wire.storage_locator {
            value = value
                .with_storage_locator(locator)
                .map_err(de::Error::custom)?;
        }
        Ok(value.with_lineage(wire.lineage))
    }
}
