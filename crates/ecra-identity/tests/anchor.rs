use std::any::TypeId;

use ecra_core::{ContentDigest, EpochMillis, SchemaVersion, VerificationReceipt};
use ecra_identity::{
    KeyId, KeyPurpose, KeyRecord, KeyStatus, ProtectedAnchorPayloadDigest, ProtectedAnchorPurpose,
    ProtectedAnchorV1, SignatureAlgorithm, TrustRootId, canonical_protected_anchor_input,
    protected_anchor_input_digest_bytes,
};
use ed25519_dalek::SigningKey;
use serde::Serialize;

const ECR002_RUN_CREATED_LEDGER_DIGEST: &str =
    "6fa5235b4056ac201824cc878b4ee90aa3310fe125ea02fce6fa8d76cd64516b";

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

fn signed_ledger_anchor_fixture() -> &'static str {
    include_str!("../../../contracts/ecra-identity-v1/valid/protected-anchor-ledger-head-v1.json")
}

fn anchor_key_record() -> KeyRecord {
    let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
    let created = EpochMillis::new(1_000).unwrap();
    KeyRecord::new_ed25519(
        KeyId::parse_str("00000000-0000-0000-0000-000000000021").unwrap(),
        TrustRootId::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
        KeyPurpose::ProtectedAnchorSigning,
        1,
        KeyStatus::Active,
        signing_key.verifying_key().to_bytes(),
        created,
        created,
        None,
        None,
    )
    .unwrap()
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

#[test]
fn protected_anchor_is_a_distinct_type_from_generic_digest_and_verification_receipt() {
    assert_ne!(
        TypeId::of::<ProtectedAnchorV1>(),
        TypeId::of::<ContentDigest>()
    );
    assert_ne!(
        TypeId::of::<ProtectedAnchorV1>(),
        TypeId::of::<VerificationReceipt>()
    );
}

#[test]
fn signed_anchor_mutations_fail_verification() {
    let record = anchor_key_record();
    let baseline = signed_ledger_anchor_fixture();
    let anchor = ProtectedAnchorV1::from_json_slice(baseline.as_bytes()).unwrap();
    anchor.verify_with_key_record(&record).unwrap();

    let mutations = [
        baseline.replace(
            ECR002_RUN_CREATED_LEDGER_DIGEST,
            "7fa5235b4056ac201824cc878b4ee90aa3310fe125ea02fce6fa8d76cd64516b",
        ),
        baseline.replace("run_ledger_head", "artifact_manifest"),
        baseline.replace("000000000021", "000000000022"),
        baseline.replace("HyoR4gOi3bpH", "IyoR4gOi3bpH"),
    ];

    for mutation in mutations {
        let mutated = ProtectedAnchorV1::from_json_slice(mutation.as_bytes()).unwrap();
        assert!(mutated.verify_with_key_record(&record).is_err());
    }
}

#[test]
fn ledger_head_anchor_uses_ecr002_golden_digest_without_redefining_it() {
    let ecr002_digest =
        include_str!("../../../contracts/ecra-run-v1/expected/run-created-golden.sha256").trim();
    assert_eq!(ecr002_digest, ECR002_RUN_CREATED_LEDGER_DIGEST);

    let anchor =
        ProtectedAnchorV1::from_json_slice(signed_ledger_anchor_fixture().as_bytes()).unwrap();
    assert_eq!(
        anchor.payload_digest().to_string(),
        format!("sha256:{ECR002_RUN_CREATED_LEDGER_DIGEST}")
    );
    assert!(
        String::from_utf8(anchor.signing_input().unwrap())
            .unwrap()
            .contains(ECR002_RUN_CREATED_LEDGER_DIGEST)
    );
    anchor.verify_with_key_record(&anchor_key_record()).unwrap();
}
