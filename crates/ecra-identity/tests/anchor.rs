use ecra_core::SchemaVersion;
use ecra_identity::{
    KeyId, SignatureAlgorithm, TrustRootId, canonical_protected_anchor_input,
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
