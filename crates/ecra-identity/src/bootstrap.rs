use ecra_core::{EpochMillis, PrincipalId, PrincipalRef, SchemaVersion};
use serde::{Deserialize, Serialize};

use crate::{
    ECR_031_CONTRACT_VERSION, EnrollmentId, IdentityError, TrustRootId, TrustStateDigest,
    VerifiedTrustSnapshot, validate_ecr031_version,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnrollmentKind {
    #[serde(rename = "ecra_local_installation_principal")]
    EcraLocalInstallationPrincipal,
}

/// Ordinary enrollment metadata. This record is never sufficient to create an
/// authenticated principal handle; only a verified protected trust snapshot can
/// produce that process-local capability.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnrollmentRecord {
    version: SchemaVersion,
    enrollment_id: EnrollmentId,
    principal_id: PrincipalId,
    trust_root_id: TrustRootId,
    created_at: EpochMillis,
    kind: EnrollmentKind,
}

impl EnrollmentRecord {
    #[must_use]
    pub const fn new(
        enrollment_id: EnrollmentId,
        principal_id: PrincipalId,
        trust_root_id: TrustRootId,
        created_at: EpochMillis,
    ) -> Self {
        Self {
            version: ECR_031_CONTRACT_VERSION,
            enrollment_id,
            principal_id,
            trust_root_id,
            created_at,
            kind: EnrollmentKind::EcraLocalInstallationPrincipal,
        }
    }

    pub fn validate(&self) -> Result<(), IdentityError> {
        validate_ecr031_version(self.version)
    }

    #[must_use]
    pub const fn enrollment_id(self) -> EnrollmentId {
        self.enrollment_id
    }

    #[must_use]
    pub const fn principal_id(self) -> PrincipalId {
        self.principal_id
    }

    #[must_use]
    pub const fn trust_root_id(self) -> TrustRootId {
        self.trust_root_id
    }

    #[must_use]
    pub const fn created_at(self) -> EpochMillis {
        self.created_at
    }

    #[must_use]
    pub const fn kind(self) -> EnrollmentKind {
        self.kind
    }

    #[must_use]
    pub const fn protected_identity(self) -> ProtectedEnrollmentV1 {
        ProtectedEnrollmentV1::new(self.enrollment_id, self.principal_id)
    }
}

/// Identity fields embedded in the authenticated protected trust-state payload.
///
/// The type accepts only already-typed opaque IDs. OS usernames, email
/// addresses, display labels, filesystem paths and protocol subject strings
/// have no conversion path into this schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtectedEnrollmentV1 {
    enrollment_id: EnrollmentId,
    principal_id: PrincipalId,
    kind: EnrollmentKind,
}

impl ProtectedEnrollmentV1 {
    #[must_use]
    pub const fn new(enrollment_id: EnrollmentId, principal_id: PrincipalId) -> Self {
        Self {
            enrollment_id,
            principal_id,
            kind: EnrollmentKind::EcraLocalInstallationPrincipal,
        }
    }

    #[must_use]
    pub const fn enrollment_id(self) -> EnrollmentId {
        self.enrollment_id
    }

    #[must_use]
    pub const fn principal_id(self) -> PrincipalId {
        self.principal_id
    }

    #[must_use]
    pub const fn kind(self) -> EnrollmentKind {
        self.kind
    }
}

/// Opaque, non-serializable proof that one local principal was reopened from
/// authenticated protected trust state. IDs or ordinary metadata cannot create
/// this value.
pub struct EnrolledPrincipalHandle {
    principal: PrincipalRef,
    enrollment_id: EnrollmentId,
    trust_root_id: TrustRootId,
    trust_state_generation: u64,
    trust_state_digest: TrustStateDigest,
}

impl EnrolledPrincipalHandle {
    // Phase 4 protected-state opening is the production caller; keep this non-public.
    #[allow(dead_code)]
    pub(crate) fn from_verified_snapshot(snapshot: &VerifiedTrustSnapshot) -> Self {
        Self {
            principal: snapshot.principal(),
            enrollment_id: snapshot.enrollment_id(),
            trust_root_id: snapshot.trust_root_id(),
            trust_state_generation: snapshot.generation(),
            trust_state_digest: snapshot.trust_state_digest(),
        }
    }

    #[must_use]
    pub const fn principal(&self) -> PrincipalRef {
        self.principal
    }

    #[must_use]
    pub const fn enrollment_id(&self) -> EnrollmentId {
        self.enrollment_id
    }

    #[must_use]
    pub const fn trust_root_id(&self) -> TrustRootId {
        self.trust_root_id
    }

    #[must_use]
    pub const fn trust_state_generation(&self) -> u64 {
        self.trust_state_generation
    }

    #[must_use]
    pub const fn trust_state_digest(&self) -> TrustStateDigest {
        self.trust_state_digest
    }
}

#[cfg(test)]
mod tests {
    use ecra_core::{EpochMillis, PrincipalId};

    use super::{EnrollmentRecord, ProtectedEnrollmentV1};
    use crate::{EnrollmentId, TrustRootId};

    #[test]
    fn protected_enrollment_contains_only_typed_local_identity_fields() {
        let enrollment = EnrollmentRecord::new(
            EnrollmentId::parse_str("00000000-0000-0000-0000-000000000030").unwrap(),
            PrincipalId::parse_str("00000000-0000-0000-0000-000000000004").unwrap(),
            TrustRootId::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
            EpochMillis::new(1_000).unwrap(),
        );

        let protected = enrollment.protected_identity();
        assert_eq!(
            protected,
            ProtectedEnrollmentV1::new(enrollment.enrollment_id(), enrollment.principal_id())
        );

        let json = serde_json::to_value(protected).unwrap();
        let object = json.as_object().unwrap();
        assert_eq!(object.len(), 3);
        assert!(object.contains_key("enrollment_id"));
        assert!(object.contains_key("principal_id"));
        assert!(object.contains_key("kind"));
        assert!(!object.contains_key("username"));
        assert!(!object.contains_key("email"));
        assert!(!object.contains_key("label"));
        assert!(!object.contains_key("path"));
    }

    #[test]
    fn protected_enrollment_rejects_unknown_fields() {
        let invalid = br#"{
            "enrollment_id":"00000000-0000-0000-0000-000000000030",
            "principal_id":"00000000-0000-0000-0000-000000000004",
            "kind":"ecra_local_installation_principal",
            "username":"must-not-be-identity"
        }"#;
        assert!(serde_json::from_slice::<ProtectedEnrollmentV1>(invalid).is_err());
    }
}
