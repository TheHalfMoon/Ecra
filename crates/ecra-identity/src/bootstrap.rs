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
