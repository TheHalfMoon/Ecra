use serde::{Deserialize, Serialize};

pub const MAX_PROTECTED_TRUST_STATE_KEYS: usize = 128;
pub const MAX_REVOKED_KEY_IDS: usize = 128;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyPurpose {
    #[serde(rename = "identity_assertion_signing")]
    IdentityAssertionSigning,
    #[serde(rename = "protected_envelope_root")]
    ProtectedEnvelopeRoot,
    #[serde(rename = "protected_anchor_signing")]
    ProtectedAnchorSigning,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "retired_verify_or_decrypt_only")]
    RetiredVerifyOrDecryptOnly,
    #[serde(rename = "revoked")]
    Revoked,
}
