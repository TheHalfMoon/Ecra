use serde::{Deserialize, Deserializer, Serialize, de};

use crate::error::DomainError;

pub const DOMAIN_SCHEMA_MAJOR: u16 = 1;
pub const DOMAIN_SCHEMA_MINOR: u16 = 0;

/// Version of the normative Ecra trusted-domain wire contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SchemaVersion {
    major: u16,
    minor: u16,
}

impl SchemaVersion {
    pub const V1_0: Self = Self {
        major: DOMAIN_SCHEMA_MAJOR,
        minor: DOMAIN_SCHEMA_MINOR,
    };

    #[must_use]
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    #[must_use]
    pub const fn major(self) -> u16 {
        self.major
    }

    #[must_use]
    pub const fn minor(self) -> u16 {
        self.minor
    }

    pub const fn validate_supported(self) -> Result<(), DomainError> {
        if self.major != DOMAIN_SCHEMA_MAJOR {
            return Err(DomainError::UnsupportedMajorVersion {
                supported: DOMAIN_SCHEMA_MAJOR,
                actual: self.major,
            });
        }
        if self.minor > DOMAIN_SCHEMA_MINOR {
            return Err(DomainError::UnsupportedMinorVersion {
                supported: DOMAIN_SCHEMA_MINOR,
                actual: self.minor,
            });
        }
        Ok(())
    }
}

/// Explicit version envelope used by normative ECR-001 values.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Versioned<T> {
    schema_version: SchemaVersion,
    value: T,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VersionedWire<T> {
    schema_version: SchemaVersion,
    value: T,
}

impl<T> Versioned<T> {
    #[must_use]
    pub const fn new(schema_version: SchemaVersion, value: T) -> Self {
        Self {
            schema_version,
            value,
        }
    }

    #[must_use]
    pub const fn v1(value: T) -> Self {
        Self::new(SchemaVersion::V1_0, value)
    }

    #[must_use]
    pub const fn schema_version(&self) -> SchemaVersion {
        self.schema_version
    }

    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }

    #[must_use]
    pub fn into_value(self) -> T {
        self.value
    }

    pub const fn validate_schema(&self) -> Result<(), DomainError> {
        self.schema_version.validate_supported()
    }
}

impl<'de, T> Deserialize<'de> for Versioned<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = VersionedWire::<T>::deserialize(deserializer)?;
        wire.schema_version
            .validate_supported()
            .map_err(de::Error::custom)?;
        Ok(Self::new(wire.schema_version, wire.value))
    }
}

impl<T> Versioned<T>
where
    T: serde::de::DeserializeOwned,
{
    pub fn from_json_slice(input: &[u8]) -> Result<Self, DomainError> {
        let wire: VersionedWire<T> = serde_json::from_slice(input)
            .map_err(|error| DomainError::Serialization(error.to_string()))?;
        wire.schema_version.validate_supported()?;
        Ok(Self::new(wire.schema_version, wire.value))
    }
}
