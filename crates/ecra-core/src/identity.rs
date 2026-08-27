use serde::{Deserialize, Serialize};

use crate::{IdentityAssertionId, PrincipalId};

/// Opaque reference to an authorization subject.
///
/// The reference does not prove authentication. ECR-031 owns validation of
/// identity assertions, trust roots, revocation and on-behalf-of relationships.
///
/// Actor attribution and authenticated principal identity are intentionally
/// different Rust types:
///
/// ```compile_fail
/// use ecra_core::{ActorId, PrincipalId};
///
/// let actor = ActorId::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
/// let _principal: PrincipalId = actor;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrincipalRef {
    id: PrincipalId,
}

impl PrincipalRef {
    #[must_use]
    pub const fn new(id: PrincipalId) -> Self {
        Self { id }
    }

    #[must_use]
    pub const fn id(self) -> PrincipalId {
        self.id
    }
}

/// Opaque reference to identity evidence that may bind a principal later.
///
/// Existence of this value is not evidence that the assertion is valid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityAssertionRef {
    id: IdentityAssertionId,
    principal: PrincipalId,
}

impl IdentityAssertionRef {
    #[must_use]
    pub const fn new(id: IdentityAssertionId, principal: PrincipalId) -> Self {
        Self { id, principal }
    }

    #[must_use]
    pub const fn id(self) -> IdentityAssertionId {
        self.id
    }

    #[must_use]
    pub const fn principal(self) -> PrincipalId {
        self.principal
    }
}
