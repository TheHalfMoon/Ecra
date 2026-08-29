use std::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, de};
use uuid::Uuid;

use crate::IdentityError;

macro_rules! define_identity_id {
    ($name:ident, $kind:literal) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            pub fn from_uuid(value: Uuid) -> Result<Self, IdentityError> {
                if value.is_nil() {
                    return Err(IdentityError::invalid_identifier($kind));
                }
                Ok(Self(value))
            }

            #[must_use]
            pub const fn as_uuid(&self) -> &Uuid {
                &self.0
            }

            pub fn parse_str(value: &str) -> Result<Self, IdentityError> {
                let parsed =
                    Uuid::parse_str(value).map_err(|_| IdentityError::invalid_identifier($kind))?;
                if parsed.is_nil() || value != parsed.hyphenated().to_string() {
                    return Err(IdentityError::invalid_identifier($kind));
                }
                Ok(Self(parsed))
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl FromStr for $name {
            type Err = IdentityError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::parse_str(value)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::parse_str(&value).map_err(de::Error::custom)
            }
        }
    };
}

define_identity_id!(TrustRootId, "trust_root_id");
define_identity_id!(KeyId, "key_id");
define_identity_id!(ProtectedObjectId, "protected_object_id");
define_identity_id!(AssertionNonceId, "assertion_nonce_id");
define_identity_id!(EnrollmentId, "enrollment_id");
define_identity_id!(DelegationId, "delegation_id");
