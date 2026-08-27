use serde::{Deserialize, Serialize};

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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Versioned<T> {
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

impl<T> Versioned<T>
where
    T: serde::de::DeserializeOwned,
{
    pub fn from_json_slice(input: &[u8]) -> Result<Self, DomainError> {
        let value: Self = serde_json::from_slice(input)
            .map_err(|error| DomainError::Serialization(error.to_string()))?;
        value.validate_schema()?;
        Ok(value)
    }
}
