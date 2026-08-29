use std::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, de};
use uuid::Uuid;

use crate::{VerifyError, VerifyErrorCategory, VerifyErrorCode};

macro_rules! define_non_nil_id {
    ($name:ident, $kind:literal) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            pub fn from_uuid(value: Uuid) -> Result<Self, VerifyError> {
                if value.is_nil() {
                    return Err(VerifyError::new(
                        VerifyErrorCategory::Validation,
                        VerifyErrorCode::InvalidIdentifier,
                        concat!($kind, " id must be a non-nil UUID"),
                    ));
                }
                Ok(Self(value))
            }

            pub fn parse_str(value: &str) -> Result<Self, VerifyError> {
                let parsed = Uuid::parse_str(value).map_err(|_| {
                    VerifyError::new(
                        VerifyErrorCategory::Validation,
                        VerifyErrorCode::InvalidIdentifier,
                        concat!($kind, " id must be a valid UUID"),
                    )
                })?;
                Self::from_uuid(parsed)
            }

            #[must_use]
            pub const fn as_uuid(&self) -> &Uuid {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl FromStr for $name {
            type Err = VerifyError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::parse_str(value)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = Uuid::deserialize(deserializer)?;
                Self::from_uuid(value).map_err(de::Error::custom)
            }
        }
    };
}

define_non_nil_id!(CheckpointId, "checkpoint");
define_non_nil_id!(ReconciliationId, "reconciliation");
