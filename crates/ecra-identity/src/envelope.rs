use std::fmt;

use ecra_core::{InformationClass, SchemaVersion, to_jcs_vec};
use serde::{Deserialize, Deserializer, Serialize, de};
use zeroize::Zeroizing;

use crate::{
    AeadAlgorithm, ECR_031_CONTRACT_VERSION, IdentityError, IdentityErrorCategory,
    IdentityErrorCode, KeyId, MAX_I_JSON_U64, ProtectedObjectId, TrustRootId,
    validate_ecr031_version, validate_json_limits,
};

pub const MAX_PROTECTED_ENVELOPE_WIRE_BYTES: usize = 8 * 1024 * 1024;
pub const PROTECTED_ENVELOPE_NONCE_BYTES: usize = 12;
pub const PROTECTED_ENVELOPE_TAG_BYTES: usize = 16;
pub const PROTECTED_ENVELOPE_AAD_DOMAIN: &[u8] = b"ecra.protected-envelope-aad.v1\n";

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

/// Storage-protection classification. This mirrors only ECR-001 classes that
/// require non-public local protection and never declassifies or authorizes use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProtectedInformationClass {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "sensitive")]
    Sensitive,
    #[serde(rename = "secret")]
    Secret,
}

impl ProtectedInformationClass {
    #[must_use]
    pub const fn information_class(self) -> InformationClass {
        match self {
            Self::Private => InformationClass::Private,
            Self::Sensitive => InformationClass::Sensitive,
            Self::Secret => InformationClass::Secret,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EnvelopeKeyRef {
    trust_root_id: TrustRootId,
    key_id: KeyId,
    generation: u64,
}

impl EnvelopeKeyRef {
    pub fn new(
        trust_root_id: TrustRootId,
        key_id: KeyId,
        generation: u64,
    ) -> Result<Self, IdentityError> {
        if generation == 0 || generation > MAX_I_JSON_U64 {
            return Err(protected_envelope_error("key_generation"));
        }
        Ok(Self {
            trust_root_id,
            key_id,
            generation,
        })
    }

    #[must_use]
    pub const fn trust_root_id(self) -> TrustRootId {
        self.trust_root_id
    }

    #[must_use]
    pub const fn key_id(self) -> KeyId {
        self.key_id
    }

    #[must_use]
    pub const fn generation(self) -> u64 {
        self.generation
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EnvelopeKeyRefWire {
    trust_root_id: TrustRootId,
    key_id: KeyId,
    generation: u64,
}

impl<'de> Deserialize<'de> for EnvelopeKeyRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = EnvelopeKeyRefWire::deserialize(deserializer)?;
        Self::new(wire.trust_root_id, wire.key_id, wire.generation).map_err(de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProtectedEnvelopeV1 {
    version: SchemaVersion,
    object_id: ProtectedObjectId,
    purpose: ProtectedPurpose,
    information_class: ProtectedInformationClass,
    key_ref: EnvelopeKeyRef,
    algorithm: AeadAlgorithm,
    nonce_b64url: String,
    ciphertext_b64url: String,
}

impl ProtectedEnvelopeV1 {
    pub fn from_json_slice(input: &[u8]) -> Result<Self, IdentityError> {
        validate_json_limits(
            input,
            MAX_PROTECTED_ENVELOPE_WIRE_BYTES,
            crate::MAX_JSON_DEPTH,
        )?;
        let wire: ProtectedEnvelopeWire = serde_json::from_slice(input)
            .map_err(|_| protected_envelope_error("protected_envelope_json"))?;
        Self::from_encoded_parts(
            wire.version,
            wire.object_id,
            wire.purpose,
            wire.information_class,
            wire.key_ref,
            wire.algorithm,
            wire.nonce_b64url,
            wire.ciphertext_b64url,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn from_encoded_parts(
        version: SchemaVersion,
        object_id: ProtectedObjectId,
        purpose: ProtectedPurpose,
        information_class: ProtectedInformationClass,
        key_ref: EnvelopeKeyRef,
        algorithm: AeadAlgorithm,
        nonce_b64url: String,
        ciphertext_b64url: String,
    ) -> Result<Self, IdentityError> {
        validate_ecr031_version(version)?;
        let nonce = base64url_decode(&nonce_b64url, "nonce_encoding")?;
        if nonce.len() != PROTECTED_ENVELOPE_NONCE_BYTES
            || base64url_encode(&nonce) != nonce_b64url
        {
            return Err(protected_envelope_error("nonce_encoding"));
        }
        let ciphertext = base64url_decode(&ciphertext_b64url, "ciphertext_encoding")?;
        if ciphertext.len() < PROTECTED_ENVELOPE_TAG_BYTES
            || base64url_encode(&ciphertext) != ciphertext_b64url
        {
            return Err(protected_envelope_error("ciphertext_encoding"));
        }
        Ok(Self {
            version,
            object_id,
            purpose,
            information_class,
            key_ref,
            algorithm,
            nonce_b64url,
            ciphertext_b64url,
        })
    }

    pub(crate) fn from_ciphertext(
        object_id: ProtectedObjectId,
        purpose: ProtectedPurpose,
        information_class: ProtectedInformationClass,
        key_ref: EnvelopeKeyRef,
        nonce: [u8; PROTECTED_ENVELOPE_NONCE_BYTES],
        ciphertext_with_tag: Vec<u8>,
    ) -> Result<Self, IdentityError> {
        if ciphertext_with_tag.len() < PROTECTED_ENVELOPE_TAG_BYTES {
            return Err(protected_envelope_error("ciphertext_length"));
        }
        Self::from_encoded_parts(
            ECR_031_CONTRACT_VERSION,
            object_id,
            purpose,
            information_class,
            key_ref,
            AeadAlgorithm::ChaCha20Poly1305Rfc8439,
            base64url_encode(&nonce),
            base64url_encode(&ciphertext_with_tag),
        )
    }

    #[must_use]
    pub const fn object_id(&self) -> ProtectedObjectId {
        self.object_id
    }

    #[must_use]
    pub const fn purpose(&self) -> ProtectedPurpose {
        self.purpose
    }

    #[must_use]
    pub const fn information_class(&self) -> ProtectedInformationClass {
        self.information_class
    }

    #[must_use]
    pub const fn key_ref(&self) -> EnvelopeKeyRef {
        self.key_ref
    }

    #[must_use]
    pub const fn algorithm(&self) -> AeadAlgorithm {
        self.algorithm
    }

    #[must_use]
    pub fn nonce_b64url(&self) -> &str {
        &self.nonce_b64url
    }

    #[must_use]
    pub fn ciphertext_b64url(&self) -> &str {
        &self.ciphertext_b64url
    }

    pub fn aad_bytes(&self) -> Result<Vec<u8>, IdentityError> {
        let aad = ProtectedEnvelopeAadV1 {
            version: self.version,
            object_id: self.object_id,
            purpose: self.purpose,
            information_class: self.information_class,
            key_ref: self.key_ref,
            algorithm: self.algorithm,
        };
        let canonical = to_jcs_vec(&aad).map_err(|_| {
            IdentityError::new(
                IdentityErrorCategory::InvalidInput,
                IdentityErrorCode::CanonicalizationFailed,
                Some("protected_envelope_aad"),
            )
        })?;
        let mut output = Vec::with_capacity(PROTECTED_ENVELOPE_AAD_DOMAIN.len() + canonical.len());
        output.extend_from_slice(PROTECTED_ENVELOPE_AAD_DOMAIN);
        output.extend_from_slice(&canonical);
        Ok(output)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProtectedEnvelopeWire {
    version: SchemaVersion,
    object_id: ProtectedObjectId,
    purpose: ProtectedPurpose,
    information_class: ProtectedInformationClass,
    key_ref: EnvelopeKeyRef,
    algorithm: AeadAlgorithm,
    nonce_b64url: String,
    ciphertext_b64url: String,
}

#[derive(Serialize)]
struct ProtectedEnvelopeAadV1 {
    version: SchemaVersion,
    object_id: ProtectedObjectId,
    purpose: ProtectedPurpose,
    information_class: ProtectedInformationClass,
    key_ref: EnvelopeKeyRef,
    algorithm: AeadAlgorithm,
}

fn protected_envelope_error(context: &'static str) -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::InvalidInput,
        IdentityErrorCode::ProtectedEnvelopeInvalid,
        Some(context),
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

fn base64url_decode(input: &str, context: &'static str) -> Result<Vec<u8>, IdentityError> {
    if input.is_empty() || input.contains('=') || input.len() % 4 == 1 {
        return Err(protected_envelope_error(context));
    }
    let mut output = Vec::with_capacity(input.len() * 3 / 4);
    let mut buffer = 0u32;
    let mut bits = 0u8;
    for byte in input.bytes() {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'-' => 62,
            b'_' => 63,
            _ => return Err(protected_envelope_error(context)),
        };
        buffer = (buffer << 6) | u32::from(value);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            output.push(((buffer >> bits) & 0xff) as u8);
            if bits == 0 {
                buffer = 0;
            } else {
                buffer &= (1u32 << bits) - 1;
            }
        }
    }
    if bits != 0 && buffer != 0 {
        return Err(protected_envelope_error(context));
    }
    Ok(output)
}
