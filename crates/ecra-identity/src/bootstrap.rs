use std::collections::BTreeSet;

use ecra_core::{EpochMillis, PrincipalId, PrincipalRef, SchemaVersion};
use ed25519_dalek::SigningKey;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::Zeroizing;

use crate::backend::{SecureRandom, TrustBackend, TrustBackendSecretRef, TrustBackendStatus};
use crate::key::{KeyRecord, ProtectedTrustStateV1};
use crate::store::{AuthenticatedTrustState, BootstrapStoreLocation, ProtectedTrustStateStore};
use crate::{
    ECR_031_CONTRACT_VERSION, EnrollmentId, IdentityError, IdentityErrorCategory,
    IdentityErrorCode, KeyId, KeyPurpose, KeyStatus, ProtectedObjectId, SensitiveBytes,
    TrustRootId, TrustStateDigest, VerifiedAssertionKey, VerifiedTrustSnapshot,
    validate_ecr031_version,
};

const INITIAL_KEY_GENERATION: u64 = 1;
const INITIAL_TRUST_STATE_GENERATION: u64 = 1;
const SOFTWARE_SECRET_BYTES: usize = 32;

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
/// The type accepts only already-typed opaque IDs. Free-form account/session
/// metadata has no conversion path into this schema.
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
    // Protected-state authentication is the only production construction path.
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

/// Complete local bootstrap or authoritative reopen transaction.
///
/// Fresh identity/key identifiers and software key material come only from the
/// injected production CSPRNG boundary. A durable non-authoritative in-progress
/// marker is written before any backend material is created. If protected state
/// is absent while that marker remains, the transaction fails as
/// `incomplete_bootstrap` and never silently mints a replacement identity.
pub(crate) fn bootstrap_or_reopen_local_principal(
    location: &BootstrapStoreLocation,
    backend: &impl TrustBackend,
    random: &mut impl SecureRandom,
    created_at: EpochMillis,
) -> Result<EnrolledPrincipalHandle, IdentityError> {
    let store_path = location.path();
    if ProtectedTrustStateStore::store_exists(store_path)? {
        let (_, authenticated) =
            ProtectedTrustStateStore::open_existing(store_path.to_path_buf(), backend)?;
        let handle = enrolled_handle_from_authenticated(&authenticated)?;
        ProtectedTrustStateStore::clear_bootstrap_marker(store_path)?;
        return Ok(handle);
    }

    if ProtectedTrustStateStore::bootstrap_marker_exists(store_path)? {
        return Err(incomplete_bootstrap_error());
    }
    ensure_bootstrap_backend_available(backend)?;
    ProtectedTrustStateStore::write_bootstrap_marker(store_path)?;

    let principal_id = PrincipalId::from_uuid(random_uuid_v4(random)?);
    let trust_root_id = TrustRootId::from_uuid(random_uuid_v4(random)?)?;
    let enrollment_id = EnrollmentId::from_uuid(random_uuid_v4(random)?)?;
    let envelope_root_key_id = KeyId::from_uuid(random_uuid_v4(random)?)?;
    let assertion_signing_key_id = KeyId::from_uuid(random_uuid_v4(random)?)?;
    let protected_object_id = ProtectedObjectId::from_uuid(random_uuid_v4(random)?)?;

    let mut envelope_root_secret = Zeroizing::new([0_u8; SOFTWARE_SECRET_BYTES]);
    random.fill(&mut *envelope_root_secret)?;
    let envelope_root_ref = TrustBackendSecretRef::new(
        trust_root_id,
        envelope_root_key_id,
        INITIAL_KEY_GENERATION,
        KeyPurpose::ProtectedEnvelopeRoot,
    )?;
    backend.protect_secret(
        envelope_root_ref,
        &SensitiveBytes::new(envelope_root_secret.to_vec()),
    )?;

    let mut assertion_seed = Zeroizing::new([0_u8; SOFTWARE_SECRET_BYTES]);
    random.fill(&mut *assertion_seed)?;
    let signing_key = SigningKey::from_bytes(&*assertion_seed);
    let assertion_signing_ref = TrustBackendSecretRef::new(
        trust_root_id,
        assertion_signing_key_id,
        INITIAL_KEY_GENERATION,
        KeyPurpose::IdentityAssertionSigning,
    )?;
    backend.protect_secret(
        assertion_signing_ref,
        &SensitiveBytes::new(assertion_seed.to_vec()),
    )?;

    let envelope_root_record = KeyRecord::new_protected_envelope_root(
        envelope_root_key_id,
        trust_root_id,
        INITIAL_KEY_GENERATION,
        KeyStatus::Active,
        created_at,
        created_at,
        None,
        None,
    )?;
    let assertion_signing_record = KeyRecord::new_ed25519(
        assertion_signing_key_id,
        trust_root_id,
        KeyPurpose::IdentityAssertionSigning,
        INITIAL_KEY_GENERATION,
        KeyStatus::Active,
        signing_key.verifying_key().to_bytes(),
        created_at,
        created_at,
        None,
        None,
    )?;
    drop(signing_key);

    let protected_state = ProtectedTrustStateV1::new(
        trust_root_id,
        ProtectedEnrollmentV1::new(enrollment_id, principal_id),
        INITIAL_TRUST_STATE_GENERATION,
        vec![envelope_root_record, assertion_signing_record],
        BTreeSet::new(),
        created_at,
    )?;
    let store = ProtectedTrustStateStore::new(store_path.to_path_buf(), protected_object_id)?;
    store.publish(backend, random, &protected_state)?;
    let authenticated = store.open_authenticated(backend)?;
    let handle = enrolled_handle_from_authenticated(&authenticated)?;
    ProtectedTrustStateStore::clear_bootstrap_marker(store_path)?;
    Ok(handle)
}

fn enrolled_handle_from_authenticated(
    authenticated: &AuthenticatedTrustState,
) -> Result<EnrolledPrincipalHandle, IdentityError> {
    let state = authenticated.state();
    state.active_key(KeyPurpose::IdentityAssertionSigning)?;

    let mut assertion_keys = Vec::new();
    let mut revoked_assertion_key_ids = BTreeSet::new();
    for record in state
        .keys()
        .iter()
        .filter(|record| record.purpose() == KeyPurpose::IdentityAssertionSigning)
    {
        let public_key = record.ed25519_public_key()?.ok_or_else(|| {
            IdentityError::new(
                IdentityErrorCategory::IdentityValidation,
                IdentityErrorCode::TrustSnapshotLifecycleInvalid,
                Some("assertion_public_key_missing"),
            )
        })?;
        assertion_keys.push(VerifiedAssertionKey::new(
            record.key_id(),
            record.generation(),
            record.status(),
            public_key,
        )?);
        if matches!(record.status(), KeyStatus::Revoked) {
            revoked_assertion_key_ids.insert(record.key_id());
        }
    }

    let enrollment = state.enrollment();
    let snapshot = VerifiedTrustSnapshot::from_authenticated_parts(
        enrollment.enrollment_id(),
        PrincipalRef::new(enrollment.principal_id()),
        state.trust_root_id(),
        state.state_generation(),
        assertion_keys,
        revoked_assertion_key_ids,
        authenticated.trust_state_digest(),
    )?;
    Ok(EnrolledPrincipalHandle::from_verified_snapshot(&snapshot))
}

fn random_uuid_v4(random: &mut impl SecureRandom) -> Result<Uuid, IdentityError> {
    let mut bytes = [0_u8; 16];
    random.fill(&mut bytes)?;
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Ok(Uuid::from_bytes(bytes))
}

fn ensure_bootstrap_backend_available(backend: &impl TrustBackend) -> Result<(), IdentityError> {
    match backend.status()? {
        TrustBackendStatus::Available => Ok(()),
        TrustBackendStatus::Locked => Err(IdentityError::new(
            IdentityErrorCategory::TrustBackend,
            IdentityErrorCode::TrustRootLocked,
            Some("local_bootstrap_backend"),
        )),
        TrustBackendStatus::Unavailable => Err(IdentityError::new(
            IdentityErrorCategory::TrustBackend,
            IdentityErrorCode::TrustRootUnavailable,
            Some("local_bootstrap_backend"),
        )),
    }
}

fn incomplete_bootstrap_error() -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::Bootstrap,
        IdentityErrorCode::BootstrapIncomplete,
        Some("incomplete_bootstrap"),
    )
}

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        collections::HashMap,
        env, fs,
        path::PathBuf,
        process,
    };

    use ecra_core::{EpochMillis, PrincipalId};

    use super::{
        EnrollmentRecord, ProtectedEnrollmentV1, bootstrap_or_reopen_local_principal,
    };
    use crate::backend::{
        DeterministicSecureRandom, TrustBackend, TrustBackendCapabilities, TrustBackendKind,
        TrustBackendSecretRef, TrustBackendStatus,
    };
    use crate::store::{BootstrapStoreLocation, ProtectedTrustStateStore};
    use crate::{
        EnrollmentId, IdentityError, IdentityErrorCategory, IdentityErrorCode, SensitiveBytes,
        TrustRootId,
    };

    struct TestBackend {
        status: TrustBackendStatus,
        secrets: RefCell<HashMap<TrustBackendSecretRef, Vec<u8>>>,
    }

    impl TestBackend {
        fn available() -> Self {
            Self {
                status: TrustBackendStatus::Available,
                secrets: RefCell::new(HashMap::new()),
            }
        }

        fn unavailable() -> Self {
            Self {
                status: TrustBackendStatus::Unavailable,
                secrets: RefCell::new(HashMap::new()),
            }
        }
    }

    impl TrustBackend for TestBackend {
        fn capabilities(&self) -> TrustBackendCapabilities {
            TrustBackendCapabilities::new(TrustBackendKind::MacosDataProtectionKeychain)
        }

        fn status(&self) -> Result<TrustBackendStatus, IdentityError> {
            Ok(self.status)
        }

        fn protect_secret(
            &self,
            secret_ref: TrustBackendSecretRef,
            secret: &SensitiveBytes,
        ) -> Result<(), IdentityError> {
            let mut secrets = self.secrets.borrow_mut();
            if secrets.contains_key(&secret_ref) {
                return Err(IdentityError::new(
                    IdentityErrorCategory::TrustBackend,
                    IdentityErrorCode::BackendInvariantViolation,
                    Some("duplicate_test_secret"),
                ));
            }
            secrets.insert(secret_ref, secret.as_slice().to_vec());
            Ok(())
        }

        fn open_protected_secret(
            &self,
            secret_ref: TrustBackendSecretRef,
        ) -> Result<SensitiveBytes, IdentityError> {
            self.secrets
                .borrow()
                .get(&secret_ref)
                .cloned()
                .map(SensitiveBytes::new)
                .ok_or_else(|| {
                    IdentityError::new(
                        IdentityErrorCategory::TrustBackend,
                        IdentityErrorCode::KeyNotFound,
                        Some("test_backend_secret"),
                    )
                })
        }

        fn delete_backend_material(
            &self,
            secret_ref: TrustBackendSecretRef,
        ) -> Result<(), IdentityError> {
            self.secrets.borrow_mut().remove(&secret_ref);
            Ok(())
        }
    }

    fn timestamp(value: i64) -> EpochMillis {
        EpochMillis::new(value).unwrap()
    }

    fn test_location(name: &str) -> (PathBuf, BootstrapStoreLocation) {
        let directory = env::temp_dir().join(format!(
            "ecra-identity-t041a-{}-{name}",
            process::id()
        ));
        let _ = fs::remove_dir_all(&directory);
        fs::create_dir_all(&directory).unwrap();
        let location =
            BootstrapStoreLocation::new(directory.join("protected-trust-state.json")).unwrap();
        (directory, location)
    }

    fn deterministic_random() -> DeterministicSecureRandom {
        let bytes = (0..512)
            .map(|index| ((index * 37 + 11) % 251) as u8)
            .collect();
        DeterministicSecureRandom::new(bytes)
    }

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

    #[test]
    fn complete_bootstrap_reopens_same_principal_without_reminting() {
        let (directory, location) = test_location("complete-reopen");
        let backend = TestBackend::available();
        let mut random = deterministic_random();
        let first = bootstrap_or_reopen_local_principal(
            &location,
            &backend,
            &mut random,
            timestamp(1_000),
        )
        .unwrap();
        assert_eq!(backend.secrets.borrow().len(), 2);
        assert!(ProtectedTrustStateStore::store_exists(location.path()).unwrap());
        assert!(!ProtectedTrustStateStore::bootstrap_marker_exists(location.path()).unwrap());

        let mut no_more_randomness = DeterministicSecureRandom::new(Vec::new());
        let reopened = bootstrap_or_reopen_local_principal(
            &location,
            &backend,
            &mut no_more_randomness,
            timestamp(2_000),
        )
        .unwrap();
        assert_eq!(reopened.principal(), first.principal());
        assert_eq!(reopened.enrollment_id(), first.enrollment_id());
        assert_eq!(reopened.trust_root_id(), first.trust_root_id());
        assert_eq!(
            reopened.trust_state_digest(),
            first.trust_state_digest()
        );
        assert_eq!(backend.secrets.borrow().len(), 2);

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn partial_marker_blocks_silent_identity_remint() {
        let (directory, location) = test_location("incomplete");
        ProtectedTrustStateStore::write_bootstrap_marker(location.path()).unwrap();
        let backend = TestBackend::available();
        let mut random = deterministic_random();
        let error = bootstrap_or_reopen_local_principal(
            &location,
            &backend,
            &mut random,
            timestamp(1_000),
        )
        .err()
        .unwrap();
        assert_eq!(error.category(), IdentityErrorCategory::Bootstrap);
        assert_eq!(error.code(), IdentityErrorCode::BootstrapIncomplete);
        assert_eq!(error.safe_context(), Some("incomplete_bootstrap"));
        assert!(backend.secrets.borrow().is_empty());
        assert!(!ProtectedTrustStateStore::store_exists(location.path()).unwrap());

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn unavailable_backend_fails_before_creating_partial_marker() {
        let (directory, location) = test_location("backend-unavailable");
        let backend = TestBackend::unavailable();
        let mut random = deterministic_random();
        let error = bootstrap_or_reopen_local_principal(
            &location,
            &backend,
            &mut random,
            timestamp(1_000),
        )
        .err()
        .unwrap();
        assert_eq!(error.code(), IdentityErrorCode::TrustRootUnavailable);
        assert!(!ProtectedTrustStateStore::bootstrap_marker_exists(location.path()).unwrap());
        assert!(!ProtectedTrustStateStore::store_exists(location.path()).unwrap());

        fs::remove_dir_all(directory).unwrap();
    }
}