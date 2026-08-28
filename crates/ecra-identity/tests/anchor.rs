use ecra_core::SchemaVersion;
use ecra_identity::{
    KeyId, ProtectedAnchorPayloadDigest, ProtectedAnchorPurpose, ProtectedAnchorV1,
    SignatureAlgorithm, TrustRootId, canonical_protected_anchor_input,
    protected_anchor_input_digest_bytes,
};
use serde::Serialize;

#[derive(Serialize)]
struct AnchorPayload<'a> {
    version: SchemaVersion,
    trust_root_id: TrustRootId,
    key_id: KeyId,
    purpose: &'a str,
    payload_digest: &'a str,
    algorithm: SignatureAlgorithm,
}

fn payload() -> AnchorPayload<'static> {
    AnchorPayload {
        version: SchemaVersion::new(1, 0),
        trust_root_id: TrustRootId::parse_str("00000000-0000-0000-0000-000000000002")
            .expect("trust root"),
        key_id: KeyId::parse_str("00000000-0000-0000-0000-000000000021").expect("key"),
        purpose: "run_ledger_head",
        payload_digest: "sha256:0000000000000000000000000000000000000000000000000000000000000000",
        algorithm: SignatureAlgorithm::Ed25519,
    }
}

fn valid_anchor() -> String {
    r#"{
  "version":{"major":1,"minor":0},
  "anchor_id":"00000000-0000-0000-0000-000000000020",
  "trust_root_id":"00000000-0000-0000-0000-000000000002",
  "key_id":"00000000-0000-0000-0000-000000000021",
  "purpose":"run_ledger_head",
  "payload_digest":"sha256:0000000000000000000000000000000000000000000000000000000000000000",
  "algorithm":"ed25519",
  "signature_or_mac_b64url":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
}"#
    .to_owned()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn protected_anchor_input_matches_fixed_goldens() {
    let input = canonical_protected_anchor_input(&payload()).expect("anchor input");
    assert_eq!(
        input,
        include_bytes!("../../../contracts/ecra-identity-v1/expected/protected-anchor-input.txt")
    );
    let digest = protected_anchor_input_digest_bytes(&payload()).expect("anchor input digest");
    assert_eq!(
        hex(&digest),
        include_str!("../../../contracts/ecra-identity-v1/expected/protected-anchor-input.sha256")
            .trim()
    );
}

#[test]
fn strict_protected_anchor_matches_frozen_signing_input() {
    let anchor = ProtectedAnchorV1::from_json_slice(valid_anchor().as_bytes()).unwrap();
    assert_eq!(anchor.purpose(), ProtectedAnchorPurpose::RunLedgerHead);
    assert_eq!(anchor.algorithm(), SignatureAlgorithm::Ed25519);
    assert_eq!(
        anchor.payload_digest(),
        ProtectedAnchorPayloadDigest::from_bytes([0_u8; 32])
    );
    assert_eq!(
        anchor.signing_input().unwrap(),
        include_bytes!("../../../contracts/ecra-identity-v1/expected/protected-anchor-input.txt")
    );
}

#[test]
fn protected_anchor_rejects_unknown_fields_versions_purposes_algorithms_and_signature_encoding() {
    let invalid = [
        valid_anchor().replace("\"minor\":0", "\"minor\":1"),
        valid_anchor().replace(
            "\"purpose\":\"run_ledger_head\"",
            "\"purpose\":\"unknown\"",
        ),
        valid_anchor().replace("\"algorithm\":\"ed25519\"", "\"algorithm\":\"rsa\""),
        valid_anchor().replace(
            "\"signature_or_mac_b64url\":",
            "\"unexpected\":true,\"signature_or_mac_b64url\":",
        ),
        valid_anchor().replace("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"", "AAAA\""),
        valid_anchor().replace("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"", "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=\""),
    ];

    for wire in invalid {
        assert!(ProtectedAnchorV1::from_json_slice(wire.as_bytes()).is_err());
    }
}

#[test]
fn protected_anchor_payload_digest_is_strict_lowercase_sha256() {
    assert!(
        ProtectedAnchorPayloadDigest::parse_str(
            "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        )
        .is_ok()
    );
    for invalid in [
        "sha256:ABCDEF6789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "sha512:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "sha256:00",
    ] {
        assert!(ProtectedAnchorPayloadDigest::parse_str(invalid).is_err());
    }
}
