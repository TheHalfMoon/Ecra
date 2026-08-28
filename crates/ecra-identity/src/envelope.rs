use std::fmt;

use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

/// Owned sensitive bytes with bounded in-process exposure semantics.
///
/// The owned allocation is zeroized when this value is dropped, and formatting
/// never renders the contained bytes. This narrows accidental in-process
/// exposure; it does not claim process-wide, allocator-wide, swap, crash-dump,
/// debugger, kernel, or OS memory secrecy.
pub struct SensitiveBytes(Zeroizing<Vec<u8>>);

impl SensitiveBytes {
    #[must_use]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(Zeroizing::new(bytes))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Debug for SensitiveBytes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SensitiveBytes([REDACTED])")
    }
}

impl fmt::Display for SensitiveBytes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

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
