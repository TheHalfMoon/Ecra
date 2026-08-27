use serde::{Deserialize, Deserializer, Serialize, de};
use url::Url;

use crate::{DomainError, OpaqueOriginId};

/// Canonical web security origin representation.
///
/// Tuple origins normalize scheme/host/default-port through the `url` crate.
/// Opaque origins carry a strong provider-supplied identity so unrelated opaque
/// origins cannot collapse into one global value.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum WebOrigin {
    Tuple {
        scheme: String,
        host: String,
        port: Option<u16>,
    },
    Opaque {
        id: OpaqueOriginId,
    },
}

impl WebOrigin {
    pub fn tuple(
        scheme: impl Into<String>,
        host: impl Into<String>,
        port: Option<u16>,
    ) -> Result<Self, DomainError> {
        let scheme = scheme.into();
        let host = host.into();
        if scheme.is_empty() || host.is_empty() {
            return Err(DomainError::InvalidOrigin(
                "tuple web origin requires non-empty scheme and host".to_owned(),
            ));
        }

        let authority_host = if host.contains(':') && !host.starts_with('[') {
            format!("[{host}]")
        } else {
            host
        };
        let candidate = match port {
            Some(port) => format!("{scheme}://{authority_host}:{port}/"),
            None => format!("{scheme}://{authority_host}/"),
        };
        let parsed = Url::parse(&candidate)
            .map_err(|error| DomainError::InvalidOrigin(error.to_string()))?;
        let canonical_host = parsed
            .host_str()
            .ok_or_else(|| DomainError::InvalidOrigin("web origin has no host".to_owned()))?;

        Ok(Self::Tuple {
            scheme: parsed.scheme().to_owned(),
            host: canonical_host.to_owned(),
            port: parsed.port(),
        })
    }

    pub fn from_url_str(value: &str) -> Result<Self, DomainError> {
        let parsed =
            Url::parse(value).map_err(|error| DomainError::InvalidOrigin(error.to_string()))?;
        let host = parsed
            .host_str()
            .ok_or_else(|| DomainError::InvalidOrigin("URL has no tuple origin host".to_owned()))?;
        Self::tuple(parsed.scheme(), host, parsed.port())
    }

    #[must_use]
    pub const fn opaque(id: OpaqueOriginId) -> Self {
        Self::Opaque { id }
    }

    #[must_use]
    pub fn scheme(&self) -> Option<&str> {
        match self {
            Self::Tuple { scheme, .. } => Some(scheme),
            Self::Opaque { .. } => None,
        }
    }

    #[must_use]
    pub fn host(&self) -> Option<&str> {
        match self {
            Self::Tuple { host, .. } => Some(host),
            Self::Opaque { .. } => None,
        }
    }

    #[must_use]
    pub const fn port(&self) -> Option<u16> {
        match self {
            Self::Tuple { port, .. } => *port,
            Self::Opaque { .. } => None,
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum WebOriginWire {
    Tuple {
        scheme: String,
        host: String,
        port: Option<u16>,
    },
    Opaque {
        id: OpaqueOriginId,
    },
}

impl<'de> Deserialize<'de> for WebOrigin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match WebOriginWire::deserialize(deserializer)? {
            WebOriginWire::Tuple { scheme, host, port } => Self::tuple(scheme, host, port),
            WebOriginWire::Opaque { id } => Ok(Self::opaque(id)),
        }
        .map_err(de::Error::custom)
    }
}

/// Provenance/security context for an observation or reference.
///
/// Origin is never instruction authority or a capability grant.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "detail", rename_all = "snake_case")]
pub enum Origin {
    UserInput,
    Web(WebOrigin),
    Local,
    Retrieval,
    Tool,
    Model,
    Memory,
    SystemPolicy,
}
