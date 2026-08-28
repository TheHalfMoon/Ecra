use ecra_core::{IdentityAssertionId, PrincipalId, SchemaVersion, to_jcs_vec};
use ecra_identity::{
    IdentityAssertionV1, canonical_assertion_signing_input, identity_assertion_digest_bytes,
};
use serde::Serialize;

#[derive(Serialize)]
struct AssertionPayload {
    version: SchemaVersion,
    assertion_id: IdentityAssertionId,
    subject_principal_id: PrincipalId,
}

fn payload() -> AssertionPayload {
    AssertionPayload {
        version: SchemaVersion::new(1, 0),
        assertion_id: IdentityAssertionId::parse_str("00000000-0000-0000-0000-000000000001")
            .expect("assertion id"),
        subject_principal_id: PrincipalId::parse_str("00000000-0000-0000-0000-000000000004")
            .expect("principal id"),
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn assertion_payload_and_digest_match_fixed_goldens() {
    let payload = payload();
    let canonical = to_jcs_vec(&payload).expect("JCS");
    assert_eq!(
        canonical,
        include_bytes!("../../../contracts/ecra-identity-v1/expected/assertion-payload.jcs")
    );

    let signing_input = canonical_assertion_signing_input(&payload).expect("signing input");
    assert_eq!(
        signing_input,
        include_bytes!("../../../contracts/ecra-identity-v1/expected/assertion-signing-input.txt")
    );

    let digest = identity_assertion_digest_bytes(&payload).expect("assertion digest");
    assert_eq!(
        hex(&digest),
        include_str!("../../../contracts/ecra-identity-v1/expected/assertion-digest.sha256").trim()
    );
}

#[test]
fn phase3_invalid_wire_corpus_fails_before_identity_context_creation() {
    for fixture in [
        include_bytes!("../../../contracts/ecra-identity-v1/invalid/assertion-unknown-field.json")
            .as_slice(),
        include_bytes!("../../../contracts/ecra-identity-v1/invalid/assertion-unsupported-version.json")
            .as_slice(),
        include_bytes!("../../../contracts/ecra-identity-v1/invalid/assertion-malformed-signature.json")
            .as_slice(),
    ] {
        assert!(IdentityAssertionV1::from_json_slice(fixture).is_err());
    }
}
