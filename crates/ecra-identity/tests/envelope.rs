use chacha20poly1305::{
    ChaCha20Poly1305,
    aead::{Aead, KeyInit, Payload, array::Array},
};
use ecra_core::InformationClass;
use ecra_identity::{
    IdentityErrorCode, PROTECTED_ENVELOPE_AAD_DOMAIN, ProtectedEnvelopeV1,
    ProtectedInformationClass,
};
use hkdf::Hkdf;
use sha2::{Digest, Sha256};

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

fn hex_nibble(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        b'A'..=b'F' => byte - b'A' + 10,
        _ => panic!("invalid test hex"),
    }
}

fn decode_hex(input: &str) -> Vec<u8> {
    assert!(input.len().is_multiple_of(2));
    input
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| (hex_nibble(pair[0]) << 4) | hex_nibble(pair[1]))
        .collect()
}

fn encode_hex(input: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(input.len() * 2);
    for byte in input {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn base64url_encode(input: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut output = String::with_capacity(input.len().div_ceil(3) * 4);
    let mut index = 0usize;
    while index + 3 <= input.len() {
        let chunk = ((input[index] as u32) << 16)
            | ((input[index + 1] as u32) << 8)
            | input[index + 2] as u32;
        output.push(ALPHABET[((chunk >> 18) & 0x3f) as usize] as char);
        output.push(ALPHABET[((chunk >> 12) & 0x3f) as usize] as char);
        output.push(ALPHABET[((chunk >> 6) & 0x3f) as usize] as char);
        output.push(ALPHABET[(chunk & 0x3f) as usize] as char);
        index += 3;
    }
    match input.len() - index {
        1 => {
            let chunk = (input[index] as u32) << 16;
            output.push(ALPHABET[((chunk >> 18) & 0x3f) as usize] as char);
            output.push(ALPHABET[((chunk >> 12) & 0x3f) as usize] as char);
        }
        2 => {
            let chunk = ((input[index] as u32) << 16) | ((input[index + 1] as u32) << 8);
            output.push(ALPHABET[((chunk >> 18) & 0x3f) as usize] as char);
            output.push(ALPHABET[((chunk >> 12) & 0x3f) as usize] as char);
            output.push(ALPHABET[((chunk >> 6) & 0x3f) as usize] as char);
        }
        _ => {}
    }
    output
}

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

#[test]
fn rfc8439_dependency_vector_matches_section_2_8_2() {
    let key: Vec<u8> = (0x80_u8..=0x9f).collect();
    let nonce: [u8; 12] = decode_hex("070000004041424344454647").try_into().unwrap();
    let aad = decode_hex("50515253c0c1c2c3c4c5c6c7");
    let plaintext = b"Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.";
    let expected = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../contracts/ecra-identity-v1/expected/rfc8439-aead-ciphertext-tag.hex"
    ))
    .trim();

    let cipher = ChaCha20Poly1305::new_from_slice(&key).unwrap();
    let actual = cipher
        .encrypt(
            &Array(nonce),
            Payload {
                msg: plaintext,
                aad: &aad,
            },
        )
        .unwrap();
    assert_eq!(encode_hex(&actual), expected);
}

#[test]
fn ecra_canonical_hkdf_and_envelope_goldens_match_frozen_contract() {
    let mut trust_root_bytes = [0_u8; 16];
    trust_root_bytes[15] = 0x02;
    let mut key_id_bytes = [0_u8; 16];
    key_id_bytes[15] = 0x11;

    let mut salt_hasher = Sha256::new();
    salt_hasher.update(b"ecra.protected-envelope-hkdf-salt.v1\n");
    salt_hasher.update(trust_root_bytes);
    salt_hasher.update(key_id_bytes);
    salt_hasher.update(1_u64.to_be_bytes());
    let salt = salt_hasher.finalize();

    let info = concat!(
        "ecra.protected-envelope-key.v1\n",
        "{\"algorithm\":\"chacha20_poly1305_rfc8439\",",
        "\"object_domain\":\"protected_envelope\",\"purpose\":\"identity_state\"}"
    );
    let master_secret = [0x42_u8; 32];
    let hkdf = Hkdf::<Sha256>::new(Some(&salt), &master_secret);
    let mut derived_key = [0_u8; 32];
    hkdf.expand(info.as_bytes(), &mut derived_key).unwrap();
    assert_eq!(
        encode_hex(&derived_key),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../contracts/ecra-identity-v1/expected/protected-envelope-dek.hex"
        ))
        .trim()
    );

    let aad = concat!(
        "ecra.protected-envelope-aad.v1\n",
        "{\"algorithm\":\"chacha20_poly1305_rfc8439\",",
        "\"information_class\":\"sensitive\",",
        "\"key_ref\":{\"generation\":1,",
        "\"key_id\":\"00000000-0000-0000-0000-000000000011\",",
        "\"trust_root_id\":\"00000000-0000-0000-0000-000000000002\"},",
        "\"object_id\":\"00000000-0000-0000-0000-000000000010\",",
        "\"purpose\":\"identity_state\",",
        "\"version\":{\"major\":1,\"minor\":0}}"
    );
    let nonce: [u8; 12] = decode_hex("000102030405060708090a0b").try_into().unwrap();
    let plaintext = b"ecra-t049-canonical-envelope";
    let cipher = ChaCha20Poly1305::new_from_slice(&derived_key).unwrap();
    let ciphertext_with_tag = cipher
        .encrypt(
            &Array(nonce),
            Payload {
                msg: plaintext,
                aad: aad.as_bytes(),
            },
        )
        .unwrap();
    assert_eq!(
        encode_hex(&ciphertext_with_tag),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../contracts/ecra-identity-v1/expected/protected-envelope-ciphertext-tag.hex"
        ))
        .trim()
    );

    let golden_json = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../contracts/ecra-identity-v1/expected/protected-envelope-v1.json"
    ));
    let envelope = ProtectedEnvelopeV1::from_json_slice(golden_json.as_bytes()).unwrap();
    assert_eq!(envelope.aad_bytes().unwrap(), aad.as_bytes());
    assert_eq!(envelope.nonce_b64url(), base64url_encode(&nonce));
    assert_eq!(
        envelope.ciphertext_b64url(),
        base64url_encode(&ciphertext_with_tag)
    );
}
