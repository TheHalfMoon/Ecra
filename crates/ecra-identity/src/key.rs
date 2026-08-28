use std::{collections::BTreeSet, fmt};

use ecra_core::PrincipalRef;
use ed25519_dalek::VerifyingKey;
use serde::{Serialize, Serializer};

use crate::{
    EnrollmentId, IdentityError, IdentityErrorCategory, IdentityErrorCode, KeyId, SignatureAlgorithm,
    TrustRootId,
};

pub const MAX_PROTECTED_TRUST_STATE_KEYS: usize = 128;
pub const MAX_REVOKED_KEY_IDS: usize = 128;
pub const MAX_I_JSON_U64: u64 = 9_007_199_254_740_991;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, serde::Deserialize)]
pub enum KeyPurpose {
    #[serde(rename = "identity_assertion_signing")]
    IdentityAssertionSigning,
    #[serde(rename = "protected_envelope_root")]
    ProtectedEnvelopeRoot,
    #[serde(rename = "protected_anchor_signing")]
    ProtectedAnchorSigning,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, serde::Deserialize)]
pub enum KeyStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "retired_verify_or_decrypt_only")]
    RetiredVerifyOrDecryptOnly,
    #[serde(rename = "revoked")]
    Revoked,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrustStateDigest([u8; 32]);

impl TrustStateDigest {
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for TrustStateDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&sha256_text(&self.0))
    }
}

impl fmt::Debug for TrustStateDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

impl Serialize for TrustStateDigest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&sha256_text(&self.0))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VerifiedAssertionKey {
    key_id: KeyId,
    generation: u64,
    status: KeyStatus,
    algorithm: SignatureAlgorithm,
    public_key: [u8; 32],
}

impl VerifiedAssertionKey {
    pub(crate) fn new(
        key_id: KeyId,
        generation: u64,
        status: KeyStatus,
        public_key: [u8; 32],
    ) -> Result<Self, IdentityError> {
        if generation == 0 || generation > MAX_I_JSON_U64 {
            return Err(lifecycle_error("key_generation"));
        }
        VerifyingKey::from_bytes(&public_key).map_err(|_| {
            IdentityError::new(
                IdentityErrorCategory::IdentityValidation,
                IdentityErrorCode::TrustSnapshotLifecycleInvalid,
                Some("assertion_verifying_key"),
            )
        })?;
        Ok(Self {
            key_id,
            generation,
            status,
            algorithm: SignatureAlgorithm::Ed25519,
            public_key,
        })
    }

    #[must_use]
    pub const fn key_id(self) -> KeyId {
        self.key_id
    }

    #[must_use]
    pub const fn generation(self) -> u64 {
        self.generation
    }

    #[must_use]
    pub const fn status(self) -> KeyStatus {
        self.status
    }

    #[must_use]
    pub const fn algorithm(self) -> SignatureAlgorithm {
        self.algorithm
    }

    #[must_use]
    pub const fn public_key(self) -> [u8; 32] {
        self.public_key
    }
}

/// Immutable validation input created only inside ECR-031 after protected trust
/// state authentication. There is intentionally no public constructor and no
/// Deserialize implementation that could turn ordinary metadata into authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedTrustSnapshot {
    enrollment_id: EnrollmentId,
    principal: PrincipalRef,
    trust_root_id: TrustRootId,
    generation: u64,
    assertion_keys: Vec<VerifiedAssertionKey>,
    revoked_key_ids: BTreeSet<KeyId>,
    trust_state_digest: TrustStateDigest,
}

impl VerifiedTrustSnapshot {
    pub(crate) fn from_authenticated_parts(
        enrollment_id: EnrollmentId,
        principal: PrincipalRef,
        trust_root_id: TrustRootId,
        generation: u64,
        assertion_keys: Vec<VerifiedAssertionKey>,
        revoked_key_ids: BTreeSet<KeyId>,
        trust_state_digest: TrustStateDigest,
    ) -> Result<Self, IdentityError> {
        if generation == 0 || generation > MAX_I_JSON_U64 {
            return Err(lifecycle_error("trust_state_generation"));
        }
        if assertion_keys.len() > MAX_PROTECTED_TRUST_STATE_KEYS {
            return Err(lifecycle_error("trust_state_key_count"));
        }
        if revoked_key_ids.len() > MAX_REVOKED_KEY_IDS {
            return Err(lifecycle_error("trust_state_revocation_count"));
        }

        let mut seen = BTreeSet::new();
        let mut active_count = 0usize;
        for key in &assertion_keys {
            if !seen.insert(key.key_id()) {
                return Err(lifecycle_error("duplicate_assertion_key"));
            }
            if key.generation() > generation {
                return Err(lifecycle_error("key_generation_ahead_of_state"));
            }
            let listed_revoked = revoked_key_ids.contains(&key.key_id());
            if listed_revoked != matches!(key.status(), KeyStatus::Revoked) {
                return Err(lifecycle_error("revocation_set_mismatch"));
            }
            if matches!(key.status(), KeyStatus::Active) {
                active_count = active_count.saturating_add(1);
            }
        }
        if active_count > 1 {
            return Err(lifecycle_error("multiple_active_assertion_keys"));
        }
        if revoked_key_ids.iter().any(|key_id| !seen.contains(key_id)) {
            return Err(lifecycle_error("unknown_revoked_key"));
        }

        Ok(Self {
            enrollment_id,
            principal,
            trust_root_id,
            generation,
            assertion_keys,
            revoked_key_ids,
            trust_state_digest,
        })
    }

    #[must_use]
    pub const fn enrollment_id(&self) -> EnrollmentId {
        self.enrollment_id
    }

    #[must_use]
    pub const fn principal(&self) -> PrincipalRef {
        self.principal
    }

    #[must_use]
    pub const fn trust_root_id(&self) -> TrustRootId {
        self.trust_root_id
    }

    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    #[must_use]
    pub const fn trust_state_digest(&self) -> TrustStateDigest {
        self.trust_state_digest
    }

    #[must_use]
    pub fn assertion_key(&self, key_id: KeyId) -> Option<&VerifiedAssertionKey> {
        self.assertion_keys.iter().find(|key| key.key_id() == key_id)
    }

    #[must_use]
    pub fn active_assertion_key(&self) -> Option<&VerifiedAssertionKey> {
        self.assertion_keys
            .iter()
            .find(|key| matches!(key.status(), KeyStatus::Active))
    }

    #[must_use]
    pub fn is_revoked(&self, key_id: KeyId) -> bool {
        self.revoked_key_ids.contains(&key_id)
    }
}

fn lifecycle_error(context: &'static str) -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::IdentityValidation,
        IdentityErrorCode::TrustSnapshotLifecycleInvalid,
        Some(context),
    )
}

fn sha256_text(bytes: &[u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(71);
    output.push_str("sha256:");
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}
