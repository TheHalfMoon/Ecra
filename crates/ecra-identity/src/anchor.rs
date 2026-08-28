use std::fmt;

use ecra_core::{SchemaVersion, to_jcs_vec};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use sha2::{Digest, Sha256};
use zeroize::Zeroizing;

use crate::backend::{TrustBackend, TrustBackendSecretRef, TrustBackendStatus};
use crate::key::{KeyPurpose, KeyRecord, KeyRecordAlgorithm};
use crate::{
    ECR_031_CONTRACT_VERSION, IdentityError, IdentityErrorCategory, IdentityErrorCode, KeyId,
    MAX_JSON_DEPTH, ProtectedObjectId, SignatureAlgorithm, TrustRootId, validate_ecr031_version,
    validate_json_limits,
};

pub const PROTECTED_ANCHOR_DOMAIN: &[u8] = b"ecra.protected-anchor.v1\n";
pub const MAX_PROTECTED_ANCHOR_WIRE_BYTES: usize = 64 * 1024;
const ED25519_SIGNATURE_BYTES: usize = 64;
const ED25519_SEED_BYTES: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProtectedAnchorPurpose {
    #[serde(rename = "run_ledger_head")]
    RunLedgerHead,
    #[serde(rename = "artifact_manifest")]
    ArtifactManifest,
    #[serde(rename = "trust_state_snapshot")]
    TrustStateSnapshot,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProtectedAnchorPayloadDigest([u8; 32]);

impl ProtectedAnchorPayloadDigest {
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn parse_str(value: &str) -> Result<Self, IdentityError> {
        let hex = value
            .strip_prefix("sha256:")
            .ok_or_else(anchor_digest_error)?;
        if hex.len() != 64
            || !hex
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(anchor_digest_error());
        }

        let mut bytes = [0_u8; 32];
        for (index, pair) in hex.as_bytes().chunks_exact(2).enumerate() {
            bytes[index] = (hex_nibble(pair[0])? << 4) | hex_nibble(pair[1])?;
        }
        Ok(Self(bytes))
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for ProtectedAnchorPayloadDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ProtectedAnchorPayloadDigest")
            .field(&self.to_string())
            .finish()
    }
}

impl fmt::Display for ProtectedAnchorPayloadDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("sha256:")?;
        for byte in self.0 {
            write!(formatter, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl Serialize for ProtectedAnchorPayloadDigest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ProtectedAnchorPayloadDigest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse_str(&value).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct ProtectedAnchorSigningPayloadV1 {
    version: SchemaVersion,
    trust_root_id: TrustRootId,
    key_id: KeyId,
    purpose: ProtectedAnchorPurpose,
    payload_digest: ProtectedAnchorPayloadDigest,
    algorithm: SignatureAlgorithm,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProtectedAnchorV1 {
    version: SchemaVersion,
    anchor_id: ProtectedObjectId,
    trust_root_id: TrustRootId,
    key_id: KeyId,
    purpose: ProtectedAnchorPurpose,
    payload_digest: ProtectedAnchorPayloadDigest,
    algorithm: SignatureAlgorithm,
    signature_or_mac_b64url: String,
}

impl ProtectedAnchorV1 {
    pub fn from_json_slice(input: &[u8]) -> Result<Self, IdentityError> {
        validate_json_limits(input, MAX_PROTECTED_ANCHOR_WIRE_BYTES, MAX_JSON_DEPTH)?;
        let wire: ProtectedAnchorWire =
            serde_json::from_slice(input).map_err(|_| anchor_wire_error())?;
        Self::from_encoded_parts(
            wire.version,
            wire.anchor_id,
            wire.trust_root_id,
            wire.key_id,
            wire.purpose,
            wire.payload_digest,
            wire.algorithm,
            wire.signature_or_mac_b64url,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn from_encoded_parts(
        version: SchemaVersion,
        anchor_id: ProtectedObjectId,
        trust_root_id: TrustRootId,
        key_id: KeyId,
        purpose: ProtectedAnchorPurpose,
        payload_digest: ProtectedAnchorPayloadDigest,
        algorithm: SignatureAlgorithm,
        signature_or_mac_b64url: String,
    ) -> Result<Self, IdentityError> {
        validate_ecr031_version(version)?;
        let decoded = base64url_decode(&signature_or_mac_b64url)?;
        if decoded.len() != ED25519_SIGNATURE_BYTES
            || base64url_encode(&decoded) != signature_or_mac_b64url
        {
            return Err(anchor_wire_error());
        }
        Ok(Self {
            version,
            anchor_id,
            trust_root_id,
            key_id,
            purpose,
            payload_digest,
            algorithm,
            signature_or_mac_b64url,
        })
    }

    pub(crate) fn from_signature_bytes(
        anchor_id: ProtectedObjectId,
        trust_root_id: TrustRootId,
        key_id: KeyId,
        purpose: ProtectedAnchorPurpose,
        payload_digest: ProtectedAnchorPayloadDigest,
        signature: [u8; ED25519_SIGNATURE_BYTES],
    ) -> Result<Self, IdentityError> {
        Self::from_encoded_parts(
            ECR_031_CONTRACT_VERSION,
            anchor_id,
            trust_root_id,
            key_id,
            purpose,
            payload_digest,
            SignatureAlgorithm::Ed25519,
            base64url_encode(&signature),
        )
    }

    #[must_use]
    pub const fn anchor_id(&self) -> ProtectedObjectId {
        self.anchor_id
    }

    #[must_use]
    pub const fn trust_root_id(&self) -> TrustRootId {
        self.trust_root_id
    }

    #[must_use]
    pub const fn key_id(&self) -> KeyId {
        self.key_id
    }

    #[must_use]
    pub const fn purpose(&self) -> ProtectedAnchorPurpose {
        self.purpose
    }

    #[must_use]
    pub const fn payload_digest(&self) -> ProtectedAnchorPayloadDigest {
        self.payload_digest
    }

    #[must_use]
    pub const fn algorithm(&self) -> SignatureAlgorithm {
        self.algorithm
    }

    #[must_use]
    pub fn signature_or_mac_b64url(&self) -> &str {
        &self.signature_or_mac_b64url
    }

    pub fn signing_input(&self) -> Result<Vec<u8>, IdentityError> {
        canonical_protected_anchor_input(&self.signing_payload())
    }

    pub fn verify_with_key_record(&self, key_record: &KeyRecord) -> Result<(), IdentityError> {
        if self.trust_root_id != key_record.trust_root_id()
            || self.key_id != key_record.key_id()
            || key_record.purpose() != KeyPurpose::ProtectedAnchorSigning
            || key_record.algorithm() != KeyRecordAlgorithm::Ed25519
        {
            return Err(anchor_authentication_error());
        }
        key_record.ensure_historical_use_allowed()?;
        let public_key = key_record
            .ed25519_public_key()?
            .ok_or_else(anchor_authentication_error)?;
        let verifying_key =
            VerifyingKey::from_bytes(&public_key).map_err(|_| anchor_authentication_error())?;
        let signature = Signature::from_slice(&self.decoded_signature()?)
            .map_err(|_| anchor_authentication_error())?;
        verifying_key
            .verify(&self.signing_input()?, &signature)
            .map_err(|_| anchor_authentication_error())
    }

    fn signing_payload(&self) -> ProtectedAnchorSigningPayloadV1 {
        ProtectedAnchorSigningPayloadV1 {
            version: self.version,
            trust_root_id: self.trust_root_id,
            key_id: self.key_id,
            purpose: self.purpose,
            payload_digest: self.payload_digest,
            algorithm: self.algorithm,
        }
    }

    pub(crate) fn decoded_signature(&self) -> Result<[u8; ED25519_SIGNATURE_BYTES], IdentityError> {
        base64url_decode(&self.signature_or_mac_b64url)?
            .try_into()
            .map_err(|_| anchor_wire_error())
    }
}

#[allow(dead_code)]
pub(crate) fn sign_protected_anchor(
    backend: &impl TrustBackend,
    key_record: &KeyRecord,
    anchor_id: ProtectedObjectId,
    purpose: ProtectedAnchorPurpose,
    payload_digest: ProtectedAnchorPayloadDigest,
) -> Result<ProtectedAnchorV1, IdentityError> {
    if key_record.purpose() != KeyPurpose::ProtectedAnchorSigning
        || key_record.algorithm() != KeyRecordAlgorithm::Ed25519
    {
        return Err(IdentityError::new(
            IdentityErrorCategory::KeyState,
            IdentityErrorCode::KeyNotActive,
            Some("protected_anchor_signing_key"),
        ));
    }
    key_record.ensure_new_material_use_allowed()?;
    match backend.status()? {
        TrustBackendStatus::Available => {}
        TrustBackendStatus::Locked => {
            return Err(IdentityError::new(
                IdentityErrorCategory::TrustBackend,
                IdentityErrorCode::TrustRootLocked,
                Some("protected_anchor_backend"),
            ));
        }
        TrustBackendStatus::Unavailable => {
            return Err(IdentityError::new(
                IdentityErrorCategory::PlatformUnavailable,
                IdentityErrorCode::TrustRootUnavailable,
                Some("protected_anchor_backend"),
            ));
        }
    }

    let secret_ref = TrustBackendSecretRef::new(
        key_record.trust_root_id(),
        key_record.key_id(),
        key_record.generation(),
        KeyPurpose::ProtectedAnchorSigning,
    )?;
    let opened = backend.open_protected_secret(secret_ref)?;
    if opened.len() != ED25519_SEED_BYTES {
        return Err(anchor_backend_invariant_error());
    }
    let mut seed = Zeroizing::new([0_u8; ED25519_SEED_BYTES]);
    seed.copy_from_slice(opened.as_slice());
    let signing_key = SigningKey::from_bytes(&seed);
    let expected_public = key_record
        .ed25519_public_key()?
        .ok_or_else(anchor_backend_invariant_error)?;
    if signing_key.verifying_key().to_bytes() != expected_public {
        return Err(anchor_backend_invariant_error());
    }

    let payload = ProtectedAnchorSigningPayloadV1 {
        version: ECR_031_CONTRACT_VERSION,
        trust_root_id: key_record.trust_root_id(),
        key_id: key_record.key_id(),
        purpose,
        payload_digest,
        algorithm: SignatureAlgorithm::Ed25519,
    };
    let input = canonical_protected_anchor_input(&payload)?;
    let signature = signing_key.sign(&input).to_bytes();
    ProtectedAnchorV1::from_signature_bytes(
        anchor_id,
        key_record.trust_root_id(),
        key_record.key_id(),
        purpose,
        payload_digest,
        signature,
    )
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProtectedAnchorWire {
    version: SchemaVersion,
    anchor_id: ProtectedObjectId,
    trust_root_id: TrustRootId,
    key_id: KeyId,
    purpose: ProtectedAnchorPurpose,
    payload_digest: ProtectedAnchorPayloadDigest,
    algorithm: SignatureAlgorithm,
    signature_or_mac_b64url: String,
}

pub fn canonical_protected_anchor_input<T>(payload: &T) -> Result<Vec<u8>, IdentityError>
where
    T: Serialize,
{
    let canonical = to_jcs_vec(payload).map_err(|_| {
        IdentityError::new(
            IdentityErrorCategory::InvalidInput,
            IdentityErrorCode::CanonicalizationFailed,
            Some("protected_anchor_payload"),
        )
    })?;
    let mut output = Vec::with_capacity(PROTECTED_ANCHOR_DOMAIN.len() + canonical.len());
    output.extend_from_slice(PROTECTED_ANCHOR_DOMAIN);
    output.extend_from_slice(&canonical);
    Ok(output)
}

pub fn protected_anchor_input_digest_bytes<T>(payload: &T) -> Result<[u8; 32], IdentityError>
where
    T: Serialize,
{
    let input = canonical_protected_anchor_input(payload)?;
    let digest = Sha256::digest(input);
    let mut output = [0u8; 32];
    output.copy_from_slice(&digest);
    Ok(output)
}

fn hex_nibble(byte: u8) -> Result<u8, IdentityError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(anchor_digest_error()),
    }
}

fn anchor_digest_error() -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::InvalidInput,
        IdentityErrorCode::InvalidJson,
        Some("protected_anchor_payload_digest"),
    )
}

fn anchor_wire_error() -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::InvalidInput,
        IdentityErrorCode::InvalidJson,
        Some("protected_anchor_wire"),
    )
}

fn anchor_authentication_error() -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::CryptographicAuthentication,
        IdentityErrorCode::AuthenticationFailed,
        Some("protected_anchor_authentication"),
    )
}

fn anchor_backend_invariant_error() -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::TrustBackend,
        IdentityErrorCode::BackendInvariantViolation,
        Some("protected_anchor_signing_material"),
    )
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

fn base64url_decode(input: &str) -> Result<Vec<u8>, IdentityError> {
    if input.contains('=') {
        return Err(anchor_wire_error());
    }
    let mut output = Vec::with_capacity(input.len() * 3 / 4 + 3);
    let mut accumulator = 0_u32;
    let mut bits = 0_u8;
    for byte in input.bytes() {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'-' => 62,
            b'_' => 63,
            _ => return Err(anchor_wire_error()),
        };
        accumulator = (accumulator << 6) | u32::from(value);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            output.push(((accumulator >> bits) & 0xff) as u8);
        }
    }
    if bits > 0 && (accumulator & ((1_u32 << bits) - 1)) != 0 {
        return Err(anchor_wire_error());
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use ecra_core::EpochMillis;

    use super::*;
    use crate::backend::{TrustBackendCapabilities, TrustBackendKind};
    use crate::{KeyStatus, SensitiveBytes};

    struct AnchorBackend {
        secret_ref: TrustBackendSecretRef,
        seed: [u8; ED25519_SEED_BYTES],
        status: TrustBackendStatus,
    }

    impl TrustBackend for AnchorBackend {
        fn capabilities(&self) -> TrustBackendCapabilities {
            TrustBackendCapabilities::new(TrustBackendKind::MacosDataProtectionKeychain)
        }

        fn status(&self) -> Result<TrustBackendStatus, IdentityError> {
            Ok(self.status)
        }

        fn protect_secret(
            &self,
            _secret_ref: TrustBackendSecretRef,
            _secret: &SensitiveBytes,
        ) -> Result<(), IdentityError> {
            Err(anchor_backend_invariant_error())
        }

        fn open_protected_secret(
            &self,
            secret_ref: TrustBackendSecretRef,
        ) -> Result<SensitiveBytes, IdentityError> {
            if secret_ref != self.secret_ref {
                return Err(anchor_backend_invariant_error());
            }
            Ok(SensitiveBytes::new(self.seed.to_vec()))
        }

        fn delete_backend_material(
            &self,
            _secret_ref: TrustBackendSecretRef,
        ) -> Result<(), IdentityError> {
            Err(anchor_backend_invariant_error())
        }
    }

    fn fixture(status: KeyStatus) -> (AnchorBackend, KeyRecord) {
        let trust_root_id = TrustRootId::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
        let key_id = KeyId::parse_str("00000000-0000-0000-0000-000000000021").unwrap();
        let seed = [7_u8; ED25519_SEED_BYTES];
        let signing_key = SigningKey::from_bytes(&seed);
        let created = EpochMillis::new(1_000).unwrap();
        let retired_at = matches!(status, KeyStatus::RetiredVerifyOrDecryptOnly)
            .then(|| EpochMillis::new(2_000).unwrap());
        let revoked_at =
            matches!(status, KeyStatus::Revoked).then(|| EpochMillis::new(2_000).unwrap());
        let record = KeyRecord::new_ed25519(
            key_id,
            trust_root_id,
            KeyPurpose::ProtectedAnchorSigning,
            1,
            status,
            signing_key.verifying_key().to_bytes(),
            created,
            created,
            retired_at,
            revoked_at,
        )
        .unwrap();
        let secret_ref = TrustBackendSecretRef::new(
            trust_root_id,
            key_id,
            1,
            KeyPurpose::ProtectedAnchorSigning,
        )
        .unwrap();
        (
            AnchorBackend {
                secret_ref,
                seed,
                status: TrustBackendStatus::Available,
            },
            record,
        )
    }

    #[test]
    fn protected_anchor_signing_uses_backend_opened_active_purpose_key() {
        let (backend, record) = fixture(KeyStatus::Active);
        let anchor = sign_protected_anchor(
            &backend,
            &record,
            ProtectedObjectId::parse_str("00000000-0000-0000-0000-000000000020").unwrap(),
            ProtectedAnchorPurpose::RunLedgerHead,
            ProtectedAnchorPayloadDigest::from_bytes([9_u8; 32]),
        )
        .unwrap();
        anchor.verify_with_key_record(&record).unwrap();
    }

    #[test]
    fn retired_and_revoked_anchor_keys_cannot_create_new_anchor() {
        for (status, expected) in [
            (
                KeyStatus::RetiredVerifyOrDecryptOnly,
                IdentityErrorCode::KeyNotActive,
            ),
            (KeyStatus::Revoked, IdentityErrorCode::KeyRevoked),
        ] {
            let (backend, record) = fixture(status);
            let error = sign_protected_anchor(
                &backend,
                &record,
                ProtectedObjectId::parse_str("00000000-0000-0000-0000-000000000020").unwrap(),
                ProtectedAnchorPurpose::RunLedgerHead,
                ProtectedAnchorPayloadDigest::from_bytes([9_u8; 32]),
            )
            .unwrap_err();
            assert_eq!(error.code(), expected);
        }
    }

    #[test]
    fn protected_anchor_signing_rejects_backend_seed_public_key_mismatch() {
        let (mut backend, record) = fixture(KeyStatus::Active);
        backend.seed = [8_u8; ED25519_SEED_BYTES];
        let error = sign_protected_anchor(
            &backend,
            &record,
            ProtectedObjectId::parse_str("00000000-0000-0000-0000-000000000020").unwrap(),
            ProtectedAnchorPurpose::RunLedgerHead,
            ProtectedAnchorPayloadDigest::from_bytes([9_u8; 32]),
        )
        .unwrap_err();
        assert_eq!(error.code(), IdentityErrorCode::BackendInvariantViolation);
    }
}
