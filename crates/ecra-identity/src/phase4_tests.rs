use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap},
    env, fs, process,
};

use ecra_core::{ActorId, EpochMillis, IdentityAssertionId, PrincipalId, PrincipalRef};
use ed25519_dalek::SigningKey;

use crate::backend::{
    DeterministicSecureRandom, TrustBackend, TrustBackendCapabilities, TrustBackendKind,
    TrustBackendSecretRef, TrustBackendStatus,
};
use crate::bootstrap::bootstrap_or_reopen_local_principal;
use crate::store::{BootstrapStoreLocation, ProtectedTrustStateStore};
use crate::{
    AssertionAttributes, AssertionAudience, AssertionAudienceService, AssertionIssuanceRequest,
    EnrolledPrincipalHandle, EnrollmentId, IdentityError, IdentityErrorCode,
    IdentityValidationContext, IssuerSession, KeyId, KeyPurpose, KeyStatus, ReplayValidationInput,
    SensitiveBytes, TrustRootId, TrustStateDigest, VerifiedAssertionKey, VerifiedTrustSnapshot,
    validate_identity_assertion,
};

struct TestBackend {
    secrets: RefCell<HashMap<TrustBackendSecretRef, Vec<u8>>>,
}

impl TestBackend {
    fn available() -> Self {
        Self {
            secrets: RefCell::new(HashMap::new()),
        }
    }
}

impl TrustBackend for TestBackend {
    fn capabilities(&self) -> TrustBackendCapabilities {
        TrustBackendCapabilities::new(TrustBackendKind::MacosDataProtectionKeychain)
    }

    fn status(&self) -> Result<TrustBackendStatus, IdentityError> {
        Ok(TrustBackendStatus::Available)
    }

    fn protect_secret(
        &self,
        secret_ref: TrustBackendSecretRef,
        secret: &SensitiveBytes,
    ) -> Result<(), IdentityError> {
        self.secrets
            .borrow_mut()
            .insert(secret_ref, secret.as_slice().to_vec());
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
                    crate::IdentityErrorCategory::TrustBackend,
                    IdentityErrorCode::KeyNotFound,
                    Some("phase4_test_secret"),
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

fn test_location(name: &str) -> (std::path::PathBuf, BootstrapStoreLocation) {
    let directory = env::temp_dir().join(format!("ecra-identity-t040-{}-{name}", process::id()));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).unwrap();
    let location = BootstrapStoreLocation::new(directory.join("protected-trust-state.json")).unwrap();
    (directory, location)
}

fn deterministic_random() -> DeterministicSecureRandom {
    let bytes = (0..512)
        .map(|index| ((index * 37 + 11) % 251) as u8)
        .collect();
    DeterministicSecureRandom::new(bytes)
}

fn signing_snapshot(status: KeyStatus, signing_key: &SigningKey) -> VerifiedTrustSnapshot {
    let key_id = KeyId::parse_str("00000000-0000-0000-0000-000000000003").unwrap();
    let mut revoked = BTreeSet::new();
    if matches!(status, KeyStatus::Revoked) {
        revoked.insert(key_id);
    }
    VerifiedTrustSnapshot::from_authenticated_parts(
        EnrollmentId::parse_str("00000000-0000-0000-0000-000000000030").unwrap(),
        PrincipalRef::new(
            PrincipalId::parse_str("00000000-0000-0000-0000-000000000004").unwrap(),
        ),
        TrustRootId::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
        1,
        vec![VerifiedAssertionKey::new(
            key_id,
            1,
            status,
            signing_key.verifying_key().to_bytes(),
        )
        .unwrap()],
        revoked,
        TrustStateDigest::from_bytes([9_u8; 32]),
    )
    .unwrap()
}

#[test]
fn orphan_backend_material_with_marker_never_remints_identity() {
    let (directory, location) = test_location("orphan-secret");
    let backend = TestBackend::available();
    ProtectedTrustStateStore::write_bootstrap_marker(location.path()).unwrap();
    let orphan_ref = TrustBackendSecretRef::new(
        TrustRootId::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
        KeyId::parse_str("00000000-0000-0000-0000-000000000011").unwrap(),
        1,
        KeyPurpose::ProtectedEnvelopeRoot,
    )
    .unwrap();
    backend
        .protect_secret(orphan_ref, &SensitiveBytes::new(vec![0x42; 32]))
        .unwrap();

    let mut no_randomness = DeterministicSecureRandom::new(Vec::new());
    let error = bootstrap_or_reopen_local_principal(
        &location,
        &backend,
        &mut no_randomness,
        timestamp(1_000),
    )
    .err()
    .unwrap();
    assert_eq!(error.code(), IdentityErrorCode::BootstrapIncomplete);
    assert_eq!(backend.secrets.borrow().len(), 1);
    assert!(!ProtectedTrustStateStore::store_exists(location.path()).unwrap());
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn published_state_with_stale_marker_reopens_same_identity_and_clears_marker() {
    let (directory, location) = test_location("post-publish-marker");
    let backend = TestBackend::available();
    let mut random = deterministic_random();
    let first = bootstrap_or_reopen_local_principal(
        &location,
        &backend,
        &mut random,
        timestamp(1_000),
    )
    .unwrap();
    ProtectedTrustStateStore::write_bootstrap_marker(location.path()).unwrap();

    let mut no_randomness = DeterministicSecureRandom::new(Vec::new());
    let reopened = bootstrap_or_reopen_local_principal(
        &location,
        &backend,
        &mut no_randomness,
        timestamp(2_000),
    )
    .unwrap();
    assert_eq!(reopened.principal(), first.principal());
    assert_eq!(reopened.trust_root_id(), first.trust_root_id());
    assert_eq!(reopened.enrollment_id(), first.enrollment_id());
    assert!(!ProtectedTrustStateStore::bootstrap_marker_exists(location.path()).unwrap());
    assert_eq!(backend.secrets.borrow().len(), 2);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn stale_or_revoked_signing_key_cannot_create_issuer_session() {
    let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
    for status in [KeyStatus::RetiredVerifyOrDecryptOnly, KeyStatus::Revoked] {
        let snapshot = signing_snapshot(status, &signing_key);
        let handle = EnrolledPrincipalHandle::from_verified_snapshot(&snapshot);
        let error = IssuerSession::from_verified_state(
            handle,
            &snapshot,
            signing_key.clone(),
            timestamp(1_000),
        )
        .err()
        .unwrap();
        assert_eq!(error.code(), IdentityErrorCode::KeyNotActive);
    }
}

#[test]
fn assertion_signed_before_revocation_is_rejected_by_revoked_snapshot() {
    let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
    let active_snapshot = signing_snapshot(KeyStatus::Active, &signing_key);
    let handle = EnrolledPrincipalHandle::from_verified_snapshot(&active_snapshot);
    let session = IssuerSession::from_verified_state(
        handle,
        &active_snapshot,
        signing_key.clone(),
        timestamp(1_000),
    )
    .unwrap();
    let actor = ActorId::parse_str("00000000-0000-0000-0000-000000000005").unwrap();
    let audience = AssertionAudience::new(AssertionAudienceService::EcraPolicyLocal, None);
    let assertion = session
        .issue(AssertionIssuanceRequest::new(
            IdentityAssertionId::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            actor,
            audience.clone(),
            timestamp(1_000),
            Some(timestamp(1_000)),
            timestamp(2_000),
            None,
            AssertionAttributes::empty(),
            None,
        ))
        .unwrap();

    let revoked_snapshot = signing_snapshot(KeyStatus::Revoked, &signing_key);
    let context = IdentityValidationContext::new(
        timestamp(1_500),
        actor,
        audience,
        Some(session.principal().id()),
        ReplayValidationInput::reusable_within_validity(),
        &revoked_snapshot,
    );
    let error = validate_identity_assertion(&assertion, &context).unwrap_err();
    assert_eq!(error.code(), IdentityErrorCode::KeyRevoked);
}
