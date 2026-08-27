use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::DomainError;

macro_rules! define_id {
    ($name:ident, $kind:literal) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            #[must_use]
            pub const fn from_uuid(value: Uuid) -> Self {
                Self(value)
            }

            #[must_use]
            pub const fn as_uuid(&self) -> &Uuid {
                &self.0
            }

            pub fn parse_str(value: &str) -> Result<Self, DomainError> {
                Uuid::parse_str(value).map(Self).map_err(|_| DomainError::InvalidIdentifier {
                    kind: $kind,
                    value: value.to_owned(),
                })
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl FromStr for $name {
            type Err = DomainError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::parse_str(value)
            }
        }
    };
}

define_id!(ActorId, "actor");
define_id!(PrincipalId, "principal");
define_id!(IdentityAssertionId, "identity_assertion");
define_id!(RunId, "run");
define_id!(ResourceId, "resource");
define_id!(WorkspaceId, "workspace");
define_id!(BrowserSpaceId, "browser_space");
define_id!(ContainerId, "container");
define_id!(TabId, "tab");
define_id!(SessionId, "session");
define_id!(TaskId, "task");
define_id!(CapabilityRequestId, "capability_request");
define_id!(CapabilityGrantId, "capability_grant");
define_id!(ObservationId, "observation");
define_id!(FactId, "fact");
define_id!(EvidenceId, "evidence");
define_id!(ArtifactId, "artifact");
define_id!(ActionId, "action");
define_id!(ActionAttemptId, "action_attempt");
define_id!(ReceiptId, "receipt");
define_id!(VerificationId, "verification");
