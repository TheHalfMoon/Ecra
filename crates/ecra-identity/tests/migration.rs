use ecra_identity::{IdentityErrorCode, ProtectedEnvelopeV1, ProtectedPurpose};

const CURRENT_V1: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../contracts/ecra-identity-v1/migrations/protected-trust-state-envelope-v1.json"
));
const FUTURE_V2: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../contracts/ecra-identity-v1/migrations/protected-trust-state-envelope-v2-unsupported.json"
));
const CORRUPT: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../contracts/ecra-identity-v1/migrations/protected-trust-state-envelope-corrupt.json"
));

#[test]
fn current_v1_protected_trust_state_store_envelope_is_accepted() {
    let envelope = ProtectedEnvelopeV1::from_json_slice(CURRENT_V1).unwrap();
    assert_eq!(envelope.purpose(), ProtectedPurpose::TrustState);
}

#[test]
fn newer_store_envelope_version_fails_closed() {
    let error = ProtectedEnvelopeV1::from_json_slice(FUTURE_V2).unwrap_err();
    assert_eq!(error.code(), IdentityErrorCode::UnsupportedVersion);
}

#[test]
fn corrupt_store_fixture_fails_before_authenticated_open() {
    let error = ProtectedEnvelopeV1::from_json_slice(CORRUPT).unwrap_err();
    assert_eq!(error.code(), IdentityErrorCode::ProtectedEnvelopeInvalid);
}
