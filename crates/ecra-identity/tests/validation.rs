use std::any::TypeId;

use ecra_core::{ActorId, PrincipalId, SchemaVersion};
use ecra_identity::{
    AeadAlgorithm, AssertionNonceId, DelegationId, EnrollmentId, IdentityError,
    IdentityErrorCategory, IdentityErrorCode, KeyId, KeyPurpose, KeyStatus,
    MAX_IDENTITY_ASSERTION_WIRE_BYTES, MAX_JSON_DEPTH, MAX_PROTECTED_TRUST_STATE_KEYS,
    MAX_REVOKED_KEY_IDS, ProtectedObjectId, ProtectedPurpose, SignatureAlgorithm, TrustBackendKind,
    TrustRootId, validate_collection_count, validate_ecr031_version, validate_json_limits,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrimitiveFixture {
    version: SchemaVersion,
    trust_root_id: TrustRootId,
    key_id: KeyId,
    protected_object_id: ProtectedObjectId,
    enrollment_id: EnrollmentId,
    assertion_nonce_id: AssertionNonceId,
    delegation_id: DelegationId,
    signature_algorithm: SignatureAlgorithm,
    aead_algorithm: AeadAlgorithm,
    key_purpose: KeyPurpose,
    key_status: KeyStatus,
    protected_purpose: ProtectedPurpose,
    backend_kind: TrustBackendKind,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CountFixture {
    key_count: usize,
    revocation_count: usize,
}

fn parse_primitive(input: &[u8]) -> Result<PrimitiveFixture, IdentityError> {
    validate_json_limits(
        input,
        MAX_IDENTITY_ASSERTION_WIRE_BYTES,
        MAX_JSON_DEPTH,
    )?;
    let value: PrimitiveFixture = serde_json::from_slice(input).map_err(|_| {
        IdentityError::new(
            IdentityErrorCategory::InvalidInput,
            IdentityErrorCode::InvalidJson,
            Some("primitive_fixture"),
        )
    })?;
    validate_ecr031_version(value.version)?;
    Ok(value)
}

#[test]
fn strong_ids_are_distinct_types() {
    let types = [
        TypeId::of::<ActorId>(),
        TypeId::of::<PrincipalId>(),
        TypeId::of::<TrustRootId>(),
        TypeId::of::<KeyId>(),
        TypeId::of::<ProtectedObjectId>(),
        TypeId::of::<EnrollmentId>(),
        TypeId::of::<AssertionNonceId>(),
        TypeId::of::<DelegationId>(),
    ];
    for (index, left) in types.iter().enumerate() {
        for right in types.iter().skip(index + 1) {
            assert_ne!(left, right);
        }
    }
}

#[test]
fn ids_require_non_nil_canonical_uuid_text() {
    assert!(TrustRootId::parse_str("00000000-0000-0000-0000-000000000002").is_ok());
    assert!(TrustRootId::parse_str("00000000-0000-0000-0000-000000000000").is_err());
    assert!(TrustRootId::parse_str("00000000000000000000000000000002").is_err());
    assert!(TrustRootId::parse_str("00000000-0000-0000-0000-00000000000A").is_err());
    assert!(TrustRootId::parse_str("not-a-uuid").is_err());
}

#[test]
fn v1_version_is_exact_and_fail_closed() {
    assert!(validate_ecr031_version(SchemaVersion::new(1, 0)).is_ok());
    assert!(validate_ecr031_version(SchemaVersion::new(1, 1)).is_err());
    assert!(validate_ecr031_version(SchemaVersion::new(2, 0)).is_err());
}

#[test]
fn valid_primitive_fixture_round_trips_closed_values() {
    let fixture = include_bytes!("../../../contracts/ecra-identity-v1/valid/primitive-v1.json");
    let parsed = parse_primitive(fixture).expect("valid primitive fixture");
    assert_eq!(parsed.version, SchemaVersion::new(1, 0));
    assert_eq!(parsed.signature_algorithm, SignatureAlgorithm::Ed25519);
    assert_eq!(
        parsed.aead_algorithm,
        AeadAlgorithm::ChaCha20Poly1305Rfc8439
    );
    assert_eq!(parsed.key_purpose, KeyPurpose::IdentityAssertionSigning);
    assert_eq!(parsed.key_status, KeyStatus::Active);
    assert_eq!(parsed.protected_purpose, ProtectedPurpose::TrustState);
    assert_eq!(
        parsed.backend_kind,
        TrustBackendKind::MacosDataProtectionKeychain
    );
    assert_eq!(
        parsed.trust_root_id.to_string(),
        "00000000-0000-0000-0000-000000000002"
    );
    assert_eq!(
        parsed.key_id.to_string(),
        "00000000-0000-0000-0000-000000000003"
    );
    assert_eq!(
        parsed.protected_object_id.to_string(),
        "00000000-0000-0000-0000-000000000010"
    );
    assert_eq!(
        parsed.enrollment_id.to_string(),
        "00000000-0000-0000-0000-000000000030"
    );
    assert_eq!(
        parsed.assertion_nonce_id.to_string(),
        "00000000-0000-0000-0000-000000000031"
    );
    assert_eq!(
        parsed.delegation_id.to_string(),
        "00000000-0000-0000-0000-000000000032"
    );
}

#[test]
fn invalid_primitive_fixtures_fail_closed() {
    for fixture in [
        include_bytes!("../../../contracts/ecra-identity-v1/invalid/unknown-field.json").as_slice(),
        include_bytes!("../../../contracts/ecra-identity-v1/invalid/duplicate-field.json").as_slice(),
        include_bytes!("../../../contracts/ecra-identity-v1/invalid/nil-id.json").as_slice(),
        include_bytes!("../../../contracts/ecra-identity-v1/invalid/unsupported-signature-algorithm.json").as_slice(),
        include_bytes!("../../../contracts/ecra-identity-v1/invalid/unsupported-aead-algorithm.json").as_slice(),
        include_bytes!("../../../contracts/ecra-identity-v1/invalid/unsupported-version.json").as_slice(),
    ] {
        assert!(parse_primitive(fixture).is_err());
    }
}

#[test]
fn gross_wire_depth_and_count_limits_precede_semantic_parsing() {
    let depth_fixture = include_bytes!("../../../contracts/ecra-identity-v1/invalid/depth-breach.json");
    let depth_error = validate_json_limits(
        depth_fixture,
        MAX_IDENTITY_ASSERTION_WIRE_BYTES,
        MAX_JSON_DEPTH,
    )
    .expect_err("depth limit must reject");
    assert_eq!(depth_error.code(), IdentityErrorCode::JsonDepthExceeded);

    let seed = include_bytes!("../../../contracts/ecra-identity-v1/invalid/oversized-seed.txt");
    let oversized = seed.repeat((MAX_IDENTITY_ASSERTION_WIRE_BYTES / seed.len()) + 2);
    let size_error = validate_json_limits(
        &oversized,
        MAX_IDENTITY_ASSERTION_WIRE_BYTES,
        MAX_JSON_DEPTH,
    )
    .expect_err("byte limit must reject");
    assert_eq!(size_error.code(), IdentityErrorCode::WireLimitExceeded);

    let counts: CountFixture = serde_json::from_slice(include_bytes!(
        "../../../contracts/ecra-identity-v1/invalid/count-breach.json"
    ))
    .expect("count fixture syntax");
    assert!(
        validate_collection_count(
            counts.key_count,
            MAX_PROTECTED_TRUST_STATE_KEYS,
            "protected_trust_state_keys"
        )
        .is_err()
    );
    assert!(
        validate_collection_count(
            counts.revocation_count,
            MAX_REVOKED_KEY_IDS,
            "revoked_key_ids"
        )
        .is_err()
    );
}

#[test]
fn public_error_formatting_contains_only_closed_safe_context() {
    let error = IdentityError::invalid_identifier("trust_root_id");
    let rendered = format!("{error:?} / {error}");
    assert!(rendered.contains("trust_root_id"));
    assert!(!rendered.contains("not-a-uuid"));

    let bootstrap = IdentityError::new(
        IdentityErrorCategory::Bootstrap,
        IdentityErrorCode::BootstrapIncomplete,
        Some("protected_state_publish"),
    );
    assert_eq!(bootstrap.code(), IdentityErrorCode::BootstrapIncomplete);
    let issuance = IdentityError::new(
        IdentityErrorCategory::Issuance,
        IdentityErrorCode::IssuerSessionUnavailable,
        Some("issuer_session"),
    );
    assert_eq!(issuance.code(), IdentityErrorCode::IssuerSessionUnavailable);
    let snapshot = IdentityError::new(
        IdentityErrorCategory::IdentityValidation,
        IdentityErrorCode::TrustSnapshotAuthenticationFailed,
        Some("verified_trust_snapshot"),
    );
    assert_eq!(
        snapshot.code(),
        IdentityErrorCode::TrustSnapshotAuthenticationFailed
    );
}
