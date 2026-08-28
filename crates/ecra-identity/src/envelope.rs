use serde::{Deserialize, Serialize};

pub const MAX_PROTECTED_ENVELOPE_WIRE_BYTES: usize = 8 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProtectedPurpose {
    #[serde(rename = "identity_state")]
    IdentityState,
    #[serde(rename = "trust_state")]
    TrustState,
    #[serde(rename = "assertion_signing_key")]
    AssertionSigningKey,
    #[serde(rename = "protected_anchor_signing_key")]
    ProtectedAnchorSigningKey,
    #[serde(rename = "consumer_sensitive_blob")]
    ConsumerSensitiveBlob,
    #[serde(rename = "ledger_anchor_material")]
    LedgerAnchorMaterial,
}
