use ecra_core::InformationClass;
use ecra_identity::{
    IdentityErrorCode, PROTECTED_ENVELOPE_AAD_DOMAIN, ProtectedEnvelopeV1,
    ProtectedInformationClass,
};

const VALID_ENVELOPE: &str = r#"{
  "version":{"major":1,"minor":0},
  "object_id":"00000000-0000-0000-0000-000000000010",
  "purpose":"identity_state",
  "information_class":"sensitive",
  "key_ref":{
    "trust_root_id":"00000000-0000-0000-0000-000000000002",
    "key_id":"00000000-0000-0000-0000-000000000011",
    "generation":1
  },
  "algorithm":"chacha20_poly1305_rfc8439",
  "nonce_b64url":"AAAAAAAAAAAAAAAA",
  "ciphertext_b64url":"AAAAAAAAAAAAAAAAAAAAAA"
}"#;

#[test]
fn strict_envelope_parses_and_aad_matches_frozen_contract() {
    let envelope = ProtectedEnvelopeV1::from_json_slice(VALID_ENVELOPE.as_bytes()).unwrap();
    let expected_jcs = concat!(
        "{\"algorithm\":\"chacha20_poly1305_rfc8439\",",
        "\"information_class\":\"sensitive\",",
        "\"key_ref\":{\"generation\":1,",
        "\"key_id\":\"00000000-0000-0000-0000-000000000011\",",
        "\"trust_root_id\":\"00000000-0000-0000-0000-000000000002\"},",
        "\"object_id\":\"00000000-0000-0000-0000-000000000010\",",
        "\"purpose\":\"identity_state\",",
        "\"version\":{\"major\":1,\"minor\":0}}"
    );
    let mut expected = PROTECTED_ENVELOPE_AAD_DOMAIN.to_vec();
    expected.extend_from_slice(expected_jcs.as_bytes());
    assert_eq!(envelope.aad_bytes().unwrap(), expected);
    assert_eq!(
        envelope.information_class().information_class(),
        InformationClass::Sensitive
    );
}

#[test]
fn interpretation_critical_fields_are_all_bound_into_aad() {
    let baseline = ProtectedEnvelopeV1::from_json_slice(VALID_ENVELOPE.as_bytes())
        .unwrap()
        .aad_bytes()
        .unwrap();
    for (needle, replacement) in [
        ("identity_state", "trust_state"),
        ("\"sensitive\"", "\"secret\""),
        ("000000000010", "000000000012"),
        ("000000000011", "000000000013"),
    ] {
        let candidate = VALID_ENVELOPE.replace(needle, replacement);
        let aad = ProtectedEnvelopeV1::from_json_slice(candidate.as_bytes())
            .unwrap()
            .aad_bytes()
            .unwrap();
        assert_ne!(aad, baseline, "AAD must bind changed field {needle}");
    }
}

#[test]
fn strict_envelope_rejects_malformed_or_incompatible_wire() {
    let invalid = [
        VALID_ENVELOPE.replace("\"minor\":0", "\"minor\":1"),
        VALID_ENVELOPE.replace("\"generation\":1", "\"generation\":0"),
        VALID_ENVELOPE.replace("AAAAAAAAAAAAAAAA\"", "AAAA\""),
        VALID_ENVELOPE.replace("AAAAAAAAAAAAAAAAAAAAAA\"", "AAAA\""),
        VALID_ENVELOPE.replace("AAAAAAAAAAAAAAAA\"", "AAAAAAAAAAAAAAAA=\""),
        VALID_ENVELOPE.replace(
            "\"algorithm\":\"chacha20_poly1305_rfc8439\"",
            "\"algorithm\":\"unsupported_aead\"",
        ),
        VALID_ENVELOPE.replace(
            "\"ciphertext_b64url\":",
            "\"unexpected\":true,\"ciphertext_b64url\":",
        ),
    ];

    for input in invalid {
        assert!(ProtectedEnvelopeV1::from_json_slice(input.as_bytes()).is_err());
    }
}

#[test]
fn unsupported_version_keeps_compatibility_error_code() {
    let input = VALID_ENVELOPE.replace("\"major\":1", "\"major\":2");
    let error = ProtectedEnvelopeV1::from_json_slice(input.as_bytes()).unwrap_err();
    assert_eq!(error.code(), IdentityErrorCode::UnsupportedVersion);
}

#[test]
fn protected_classification_is_storage_metadata_not_authority() {
    assert_eq!(
        ProtectedInformationClass::Private.information_class(),
        InformationClass::Private
    );
    assert_eq!(
        ProtectedInformationClass::Secret.information_class(),
        InformationClass::Secret
    );
}
