use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{DomainError, ResourceId, WebOrigin};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceKind {
    WebResource,
    LocalResource,
    WorkspaceResource,
    ToolResource,
    Artifact,
    Abstract,
}

/// Stable resource identity plus optional provider/display metadata.
///
/// `locator` is deliberately non-authoritative: filesystem paths, URLs, tool
/// names and display strings can alias or change. Downstream policy must use
/// provider-resolved identity/constraints rather than infer authority from text.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceRef {
    id: ResourceId,
    kind: ResourceKind,
    locator: Option<String>,
    origin: Option<WebOrigin>,
}

impl ResourceRef {
    pub fn new(
        id: ResourceId,
        kind: ResourceKind,
        locator: Option<String>,
        origin: Option<WebOrigin>,
    ) -> Result<Self, DomainError> {
        if locator.as_ref().is_some_and(|value| value.is_empty()) {
            return Err(DomainError::InvalidResource(
                "resource locator cannot be an empty string".to_owned(),
            ));
        }
        Ok(Self {
            id,
            kind,
            locator,
            origin,
        })
    }

    #[must_use]
    pub const fn id(&self) -> ResourceId {
        self.id
    }

    #[must_use]
    pub const fn kind(&self) -> ResourceKind {
        self.kind
    }

    #[must_use]
    pub fn locator(&self) -> Option<&str> {
        self.locator.as_deref()
    }

    #[must_use]
    pub const fn origin(&self) -> Option<&WebOrigin> {
        self.origin.as_ref()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ResourceRefWire {
    id: ResourceId,
    kind: ResourceKind,
    locator: Option<String>,
    origin: Option<WebOrigin>,
}

impl<'de> Deserialize<'de> for ResourceRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ResourceRefWire::deserialize(deserializer)?;
        Self::new(wire.id, wire.kind, wire.locator, wire.origin).map_err(de::Error::custom)
    }
}
