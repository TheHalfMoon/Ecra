use std::collections::BTreeSet;

use ecra_core::{EpochMillis, PrincipalId};
use ecra_identity::bootstrap::ProtectedEnrollmentV1;
use ecra_identity::key::{KeyRecord, ProtectedTrustStateV1};
use ecra_identity::{EnrollmentId, IdentityErrorCode, KeyId, KeyPurpose, KeyStatus, TrustRootId};
use ed25519_dalek::SigningKey;
use serde::Deserialize;

fn timestamp(value: i64) -> EpochMillis {
    EpochMillis::new(value).unwrap()
}

fn root_id() -> TrustRootId {
    TrustRootId::parse_str("00000000-0000-0000-0000-000000000002").unwrap()
}

fn enrollment() -> ProtectedEnrollmentV1 {
    ProtectedEnrollmentV1::new(
        EnrollmentId::parse_str("00000000-0000-0000-0000-000000000030").unwrap(),
        PrincipalId::parse_str("00000000-0000-0000-0000-000000000004").unwrap(),
    )
}

fn signing_record(key_text: &str, seed: u8, generation: u64, status: KeyStatus) -> KeyRecord {
    let signing_key = SigningKey::from_bytes(&[seed; 32]);
    let terminal = match status {
        KeyStatus::Active => (None, None),
        KeyStatus::RetiredVerifyOrDecryptOnly => (Some(timestamp(1_100)), None),
        KeyStatus::Revoked => (None, Some(timestamp(1_100))),
    };
    KeyRecord::new_ed25519(
        KeyId::parse_str(key_text).unwrap(),
        root_id(),
        KeyPurpose::IdentityAssertionSigning,
        generation,
        status,
        signing_key.verifying_key().to_bytes(),
        timestamp(900),
        timestamp(1_000),
        terminal.0,
        terminal.1,
    )
    .unwrap()
}

#[test]
fn retirement_blocks_new_material_but_preserves_historical_compatibility() {
    let active = signing_record(
        "00000000-0000-0000-0000-000000000003",
        7,
        1,
        KeyStatus::Active,
    );
    let retired = active.retire(timestamp(1_100)).unwrap();
    assert_eq!(retired.status(), KeyStatus::RetiredVerifyOrDecryptOnly);
    assert_eq!(retired.retired_at(), Some(timestamp(1_100)));
    assert_eq!(
        retired.ensure_new_material_use_allowed().unwrap_err().code(),
        IdentityErrorCode::KeyNotActive
    );
    retired.ensure_historical_use_allowed().unwrap();
    assert_eq!(
        retired.retire(timestamp(1_200)).unwrap_err().code(),
        IdentityErrorCode::KeyNotActive
    );
}

#[test]
fn duplicate_active_generation_for_one_purpose_fails_closed() {
    let first = signing_record(
        "00000000-0000-0000-0000-000000000003",
        7,
        1,
        KeyStatus::Active,
    );
    let second = signing_record(
        "00000000-0000-0000-0000-000000000004",
        8,
        1,
        KeyStatus::Active,
    );
    assert!(
        ProtectedTrustStateV1::new(
            root_id(),
            enrollment(),
            1,
            vec![first, second],
            BTreeSet::new(),
            timestamp(1_100),
        )
        .is_err()
    );
}

#[test]
fn revocation_set_is_authoritative_and_cannot_be_unsignedly_reactivated() {
    let revoked = signing_record(
        "00000000-0000-0000-0000-000000000003",
        7,
        1,
        KeyStatus::Revoked,
    );
    assert_eq!(
        revoked.ensure_new_material_use_allowed().unwrap_err().code(),
        IdentityErrorCode::KeyRevoked
    );
    assert_eq!(
        revoked.ensure_historical_use_allowed().unwrap_err().code(),
        IdentityErrorCode::KeyRevoked
    );

    assert!(
        ProtectedTrustStateV1::new(
            root_id(),
            enrollment(),
            1,
            vec![revoked.clone()],
            BTreeSet::new(),
            timestamp(1_100),
        )
        .is_err()
    );

    let mut revoked_ids = BTreeSet::new();
    revoked_ids.insert(revoked.key_id());
    let state = ProtectedTrustStateV1::new(
        root_id(),
        enrollment(),
        1,
        vec![revoked],
        revoked_ids,
        timestamp(1_100),
    )
    .unwrap();

    let mut stale_metadata = serde_json::to_value(&state).unwrap();
    stale_metadata["keys"][0]["status"] = "active".into();
    stale_metadata["keys"][0]["revoked_at"] = serde_json::Value::Null;
    assert!(serde_json::from_value::<ProtectedTrustStateV1>(stale_metadata).is_err());
}

#[derive(Deserialize)]
struct RollbackBoundaryFixture {
    claim: String,
    older: ProtectedTrustStateV1,
    newer: ProtectedTrustStateV1,
}

#[test]
fn rollback_fixture_explicitly_proves_no_monotonic_counter_claim() {
    let fixture: RollbackBoundaryFixture = serde_json::from_slice(include_bytes!(
        "../../../contracts/ecra-identity-v1/valid/rollback-boundary-v1.json"
    ))
    .unwrap();
    fixture.older.validate_schema_invariants().unwrap();
    fixture.newer.validate_schema_invariants().unwrap();
    assert!(fixture.older.state_generation() < fixture.newer.state_generation());
    assert_eq!(fixture.claim, "no_monotonic_rollback_resistance");
}

#[test]
fn issuance_and_validation_sources_require_current_authenticated_lifecycle() {
    let issuance = include_str!("../src/issuance.rs");
    assert!(issuance.contains("snapshot.active_assertion_key()"));
    assert!(issuance.contains("matches!(active_key.status(), KeyStatus::Active)"));

    let validation = include_str!("../src/validation.rs");
    let revoked_check = validation.find("snapshot.is_revoked(key.key_id())").unwrap();
    let signature_verify = validation.find("verifying_key").unwrap();
    assert!(revoked_check < signature_verify);
    assert!(validation.contains("IdentityErrorCode::KeyRevoked"));
}
