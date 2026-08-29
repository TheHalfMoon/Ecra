use std::{collections::BTreeMap, fmt};

use ecra_core::{
    ActorId, EpochMillis, IdentityAssertionId, PrincipalId, SchemaVersion, to_jcs_vec,
};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, MapAccess, Visitor},
};
use sha2::{Digest, Sha256};

use crate::{
    AssertionNonceId, DelegationId, ECR_031_CONTRACT_VERSION, IdentityError, IdentityErrorCategory,
    IdentityErrorCode, KeyId, SignatureAlgorithm, TrustRootId, validate_ecr031_version,
};

pub const MAX_IDENTITY_ASSERTION_WIRE_BYTES: usize = 8 * 1024;
pub const MAX_JSON_DEPTH: usize = 16;
pub const MAX_ASSERTION_ATTRIBUTES: usize = 32;
pub const MAX_ASSERTION_ATTRIBUTE_KEY_BYTES: usize = 64;
pub const MAX_ASSERTION_ATTRIBUTE_VALUE_BYTES: usize = 256;
pub const MAX_AUDIENCE_INSTANCE_BYTES: usize = 128;
pub const MAX_ASSERTION_VALIDITY_MILLIS: i64 = 5 * 60 * 1000;

pub const IDENTITY_ASSERTION_SIGNING_DOMAIN: &[u8] = b"ecra.identity-assertion.v1\n";
pub const IDENTITY_ASSERTION_DIGEST_DOMAIN: &[u8] = b"ecra.identity-assertion-digest.v1\n";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssertionAudienceService {
    #[serde(rename = "ecra_policy_local")]
    EcraPolicyLocal,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct AudienceInstanceId(String);

impl AudienceInstanceId {
    pub fn new(value: String) -> Result<Self, IdentityError> {
        if value.is_empty()
            || value.len() > MAX_AUDIENCE_INSTANCE_BYTES
            || value.bytes().any(|byte| byte.is_ascii_control())
        {
            return Err(invalid_input("audience_instance_id"));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for AudienceInstanceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssertionAudience {
    service: AssertionAudienceService,
    instance_id: Option<AudienceInstanceId>,
}

impl AssertionAudience {
    #[must_use]
    pub const fn new(
        service: AssertionAudienceService,
        instance_id: Option<AudienceInstanceId>,
    ) -> Self {
        Self {
            service,
            instance_id,
        }
    }

    #[must_use]
    pub const fn service(&self) -> AssertionAudienceService {
        self.service
    }

    #[must_use]
    pub fn instance_id(&self) -> Option<&AudienceInstanceId> {
        self.instance_id.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct AssertionAttributes(BTreeMap<String, String>);

impl AssertionAttributes {
    pub fn new(values: BTreeMap<String, String>) -> Result<Self, IdentityError> {
        if values.len() > MAX_ASSERTION_ATTRIBUTES {
            return Err(IdentityError::new(
                IdentityErrorCategory::InvalidInput,
                IdentityErrorCode::CollectionLimitExceeded,
                Some("assertion_attributes"),
            ));
        }
        for (key, value) in &values {
            if key.is_empty()
                || key.len() > MAX_ASSERTION_ATTRIBUTE_KEY_BYTES
                || key.bytes().any(|byte| byte.is_ascii_control())
            {
                return Err(invalid_input("assertion_attribute_key"));
            }
            if value.len() > MAX_ASSERTION_ATTRIBUTE_VALUE_BYTES
                || value
                    .bytes()
                    .any(|byte| byte.is_ascii_control() && !byte.is_ascii_whitespace())
            {
                return Err(invalid_input("assertion_attribute_value"));
            }
        }
        Ok(Self(values))
    }

    #[must_use]
    pub const fn empty() -> Self {
        Self(BTreeMap::new())
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }
}

impl<'de> Deserialize<'de> for AssertionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AttributesVisitor;

        impl<'de> Visitor<'de> for AttributesVisitor {
            type Value = AssertionAttributes;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a bounded identity attribute object")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut values = BTreeMap::new();
                while let Some((key, value)) = access.next_entry::<String, String>()? {
                    if values.insert(key, value).is_some() {
                        return Err(de::Error::custom("duplicate assertion attribute key"));
                    }
                    if values.len() > MAX_ASSERTION_ATTRIBUTES {
                        return Err(de::Error::custom("assertion attribute count exceeded"));
                    }
                }
                AssertionAttributes::new(values).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_map(AttributesVisitor)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssertionIssuer {
    trust_root_id: TrustRootId,
    key_id: KeyId,
}

impl AssertionIssuer {
    #[must_use]
    pub const fn new(trust_root_id: TrustRootId, key_id: KeyId) -> Self {
        Self {
            trust_root_id,
            key_id,
        }
    }

    #[must_use]
    pub const fn trust_root_id(self) -> TrustRootId {
        self.trust_root_id
    }

    #[must_use]
    pub const fn key_id(self) -> KeyId {
        self.key_id
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActorBinding {
    actor_id: ActorId,
}

impl ActorBinding {
    #[must_use]
    pub const fn new(actor_id: ActorId) -> Self {
        Self { actor_id }
    }

    #[must_use]
    pub const fn actor_id(self) -> ActorId {
        self.actor_id
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OnBehalfOfBinding {
    principal_id: PrincipalId,
    delegation_id: DelegationId,
}

impl OnBehalfOfBinding {
    #[must_use]
    pub const fn new(principal_id: PrincipalId, delegation_id: DelegationId) -> Self {
        Self {
            principal_id,
            delegation_id,
        }
    }

    #[must_use]
    pub const fn principal_id(self) -> PrincipalId {
        self.principal_id
    }

    #[must_use]
    pub const fn delegation_id(self) -> DelegationId {
        self.delegation_id
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssertionSignature {
    algorithm: SignatureAlgorithm,
    key_id: KeyId,
    bytes_b64url: String,
}

impl AssertionSignature {
    #[must_use]
    pub fn from_bytes(key_id: KeyId, bytes: [u8; 64]) -> Self {
        Self {
            algorithm: SignatureAlgorithm::Ed25519,
            key_id,
            bytes_b64url: base64url_encode(&bytes),
        }
    }

    #[must_use]
    pub const fn algorithm(&self) -> SignatureAlgorithm {
        self.algorithm
    }

    #[must_use]
    pub const fn key_id(&self) -> KeyId {
        self.key_id
    }

    #[must_use]
    pub fn bytes_b64url(&self) -> &str {
        &self.bytes_b64url
    }

    pub(crate) fn decoded_bytes(&self) -> Result<[u8; 64], IdentityError> {
        decode_signature(&self.bytes_b64url)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AssertionSignatureWire {
    algorithm: SignatureAlgorithm,
    key_id: KeyId,
    bytes_b64url: String,
}

impl<'de> Deserialize<'de> for AssertionSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = AssertionSignatureWire::deserialize(deserializer)?;
        decode_signature(&wire.bytes_b64url).map_err(de::Error::custom)?;
        Ok(Self {
            algorithm: wire.algorithm,
            key_id: wire.key_id,
            bytes_b64url: wire.bytes_b64url,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityAssertionPayloadV1 {
    version: SchemaVersion,
    assertion_id: IdentityAssertionId,
    issuer: AssertionIssuer,
    subject_principal_id: PrincipalId,
    actor_binding: ActorBinding,
    on_behalf_of: Option<OnBehalfOfBinding>,
    audience: AssertionAudience,
    issued_at: EpochMillis,
    not_before: Option<EpochMillis>,
    expires_at: EpochMillis,
    nonce: Option<AssertionNonceId>,
    attributes: AssertionAttributes,
}

impl IdentityAssertionPayloadV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        assertion_id: IdentityAssertionId,
        issuer: AssertionIssuer,
        subject_principal_id: PrincipalId,
        actor_binding: ActorBinding,
        on_behalf_of: Option<OnBehalfOfBinding>,
        audience: AssertionAudience,
        issued_at: EpochMillis,
        not_before: Option<EpochMillis>,
        expires_at: EpochMillis,
        nonce: Option<AssertionNonceId>,
        attributes: AssertionAttributes,
    ) -> Result<Self, IdentityError> {
        let payload = Self {
            version: ECR_031_CONTRACT_VERSION,
            assertion_id,
            issuer,
            subject_principal_id,
            actor_binding,
            on_behalf_of,
            audience,
            issued_at,
            not_before,
            expires_at,
            nonce,
            attributes,
        };
        payload.validate()?;
        Ok(payload)
    }

    fn validate(&self) -> Result<(), IdentityError> {
        validate_ecr031_version(self.version)?;
        if self.expires_at.get() < self.issued_at.get() {
            return Err(temporal_error());
        }
        if let Some(not_before) = self.not_before
            && not_before.get() > self.expires_at.get()
        {
            return Err(temporal_error());
        }
        if self.expires_at.get() - self.issued_at.get() > MAX_ASSERTION_VALIDITY_MILLIS {
            return Err(temporal_error());
        }
        Ok(())
    }

    #[must_use]
    pub const fn assertion_id(&self) -> IdentityAssertionId {
        self.assertion_id
    }

    #[must_use]
    pub const fn issuer(&self) -> AssertionIssuer {
        self.issuer
    }

    #[must_use]
    pub const fn subject_principal_id(&self) -> PrincipalId {
        self.subject_principal_id
    }

    #[must_use]
    pub const fn actor_binding(&self) -> ActorBinding {
        self.actor_binding
    }

    #[must_use]
    pub const fn on_behalf_of(&self) -> Option<OnBehalfOfBinding> {
        self.on_behalf_of
    }

    #[must_use]
    pub fn audience(&self) -> &AssertionAudience {
        &self.audience
    }

    #[must_use]
    pub const fn issued_at(&self) -> EpochMillis {
        self.issued_at
    }

    #[must_use]
    pub const fn not_before(&self) -> Option<EpochMillis> {
        self.not_before
    }

    #[must_use]
    pub const fn expires_at(&self) -> EpochMillis {
        self.expires_at
    }

    #[must_use]
    pub const fn nonce(&self) -> Option<AssertionNonceId> {
        self.nonce
    }

    #[must_use]
    pub fn attributes(&self) -> &AssertionAttributes {
        &self.attributes
    }

    pub fn signing_input(&self) -> Result<Vec<u8>, IdentityError> {
        canonical_assertion_signing_input(self)
    }

    pub fn digest(&self) -> Result<IdentityAssertionDigest, IdentityError> {
        identity_assertion_digest_bytes(self).map(IdentityAssertionDigest::from_bytes)
    }

    pub(crate) fn into_signed(
        self,
        signature: AssertionSignature,
    ) -> Result<IdentityAssertionV1, IdentityError> {
        if signature.key_id() != self.issuer.key_id() {
            return Err(IdentityError::new(
                IdentityErrorCategory::InvalidInput,
                IdentityErrorCode::AssertionSignatureInvalid,
                Some("signature_key_binding"),
            ));
        }
        Ok(IdentityAssertionV1 {
            payload: self,
            signature,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdentityAssertionV1 {
    payload: IdentityAssertionPayloadV1,
    signature: AssertionSignature,
}

impl IdentityAssertionV1 {
    pub fn from_json_slice(input: &[u8]) -> Result<Self, IdentityError> {
        validate_json_limits(input, MAX_IDENTITY_ASSERTION_WIRE_BYTES, MAX_JSON_DEPTH)?;
        let wire: IdentityAssertionWire = serde_json::from_slice(input).map_err(|_| {
            IdentityError::new(
                IdentityErrorCategory::InvalidInput,
                IdentityErrorCode::InvalidJson,
                Some("identity_assertion"),
            )
        })?;
        let payload = IdentityAssertionPayloadV1 {
            version: wire.version,
            assertion_id: wire.assertion_id,
            issuer: wire.issuer,
            subject_principal_id: wire.subject_principal_id,
            actor_binding: wire.actor_binding,
            on_behalf_of: wire.on_behalf_of,
            audience: wire.audience,
            issued_at: wire.issued_at,
            not_before: wire.not_before,
            expires_at: wire.expires_at,
            nonce: wire.nonce,
            attributes: wire.attributes,
        };
        payload.validate()?;
        payload.into_signed(wire.signature)
    }

    #[must_use]
    pub fn payload(&self) -> &IdentityAssertionPayloadV1 {
        &self.payload
    }

    #[must_use]
    pub fn signature(&self) -> &AssertionSignature {
        &self.signature
    }

    pub fn signing_input(&self) -> Result<Vec<u8>, IdentityError> {
        self.payload.signing_input()
    }

    pub fn digest(&self) -> Result<IdentityAssertionDigest, IdentityError> {
        self.payload.digest()
    }
}

#[derive(Serialize)]
struct IdentityAssertionSerializeRef<'a> {
    version: SchemaVersion,
    assertion_id: IdentityAssertionId,
    issuer: AssertionIssuer,
    subject_principal_id: PrincipalId,
    actor_binding: ActorBinding,
    on_behalf_of: Option<OnBehalfOfBinding>,
    audience: &'a AssertionAudience,
    issued_at: EpochMillis,
    not_before: Option<EpochMillis>,
    expires_at: EpochMillis,
    nonce: Option<AssertionNonceId>,
    attributes: &'a AssertionAttributes,
    signature: &'a AssertionSignature,
}

impl Serialize for IdentityAssertionV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        IdentityAssertionSerializeRef {
            version: self.payload.version,
            assertion_id: self.payload.assertion_id,
            issuer: self.payload.issuer,
            subject_principal_id: self.payload.subject_principal_id,
            actor_binding: self.payload.actor_binding,
            on_behalf_of: self.payload.on_behalf_of,
            audience: &self.payload.audience,
            issued_at: self.payload.issued_at,
            not_before: self.payload.not_before,
            expires_at: self.payload.expires_at,
            nonce: self.payload.nonce,
            attributes: &self.payload.attributes,
            signature: &self.signature,
        }
        .serialize(serializer)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct IdentityAssertionWire {
    version: SchemaVersion,
    assertion_id: IdentityAssertionId,
    issuer: AssertionIssuer,
    subject_principal_id: PrincipalId,
    actor_binding: ActorBinding,
    on_behalf_of: Option<OnBehalfOfBinding>,
    audience: AssertionAudience,
    issued_at: EpochMillis,
    not_before: Option<EpochMillis>,
    expires_at: EpochMillis,
    nonce: Option<AssertionNonceId>,
    attributes: AssertionAttributes,
    signature: AssertionSignature,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentityAssertionDigest([u8; 32]);

impl IdentityAssertionDigest {
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for IdentityAssertionDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&sha256_text(&self.0))
    }
}

impl fmt::Debug for IdentityAssertionDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

impl Serialize for IdentityAssertionDigest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&sha256_text(&self.0))
    }
}

/// Reject gross input limits before JSON allocation or cryptographic work.
pub fn validate_json_limits(
    input: &[u8],
    max_bytes: usize,
    max_depth: usize,
) -> Result<(), IdentityError> {
    if input.len() > max_bytes {
        return Err(IdentityError::new(
            IdentityErrorCategory::InvalidInput,
            IdentityErrorCode::WireLimitExceeded,
            Some("json_bytes"),
        ));
    }

    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;

    for byte in input {
        if in_string {
            if escaped {
                escaped = false;
                continue;
            }
            match byte {
                b'\\' => escaped = true,
                b'"' => in_string = false,
                _ => {}
            }
            continue;
        }

        match byte {
            b'"' => in_string = true,
            b'{' | b'[' => {
                depth = depth.saturating_add(1);
                if depth > max_depth {
                    return Err(IdentityError::new(
                        IdentityErrorCategory::InvalidInput,
                        IdentityErrorCode::JsonDepthExceeded,
                        Some("json_depth"),
                    ));
                }
            }
            b'}' | b']' => depth = depth.saturating_sub(1),
            _ => {}
        }
    }

    Ok(())
}

pub fn validate_collection_count(
    count: usize,
    max_count: usize,
    context: &'static str,
) -> Result<(), IdentityError> {
    if count > max_count {
        return Err(IdentityError::new(
            IdentityErrorCategory::InvalidInput,
            IdentityErrorCode::CollectionLimitExceeded,
            Some(context),
        ));
    }
    Ok(())
}

pub fn canonical_assertion_signing_input<T>(payload: &T) -> Result<Vec<u8>, IdentityError>
where
    T: Serialize,
{
    let canonical = to_jcs_vec(payload).map_err(|_| {
        IdentityError::new(
            IdentityErrorCategory::InvalidInput,
            IdentityErrorCode::CanonicalizationFailed,
            Some("identity_assertion_payload"),
        )
    })?;
    let mut output = Vec::with_capacity(IDENTITY_ASSERTION_SIGNING_DOMAIN.len() + canonical.len());
    output.extend_from_slice(IDENTITY_ASSERTION_SIGNING_DOMAIN);
    output.extend_from_slice(&canonical);
    Ok(output)
}

pub fn identity_assertion_digest_bytes<T>(payload: &T) -> Result<[u8; 32], IdentityError>
where
    T: Serialize,
{
    let canonical = to_jcs_vec(payload).map_err(|_| {
        IdentityError::new(
            IdentityErrorCategory::InvalidInput,
            IdentityErrorCode::CanonicalizationFailed,
            Some("identity_assertion_payload"),
        )
    })?;
    let mut hasher = Sha256::new();
    hasher.update(IDENTITY_ASSERTION_DIGEST_DOMAIN);
    hasher.update(canonical);
    let digest = hasher.finalize();
    let mut output = [0u8; 32];
    output.copy_from_slice(&digest);
    Ok(output)
}

fn decode_signature(input: &str) -> Result<[u8; 64], IdentityError> {
    let decoded = base64url_decode(input)?;
    let bytes: [u8; 64] = decoded.try_into().map_err(|_| {
        IdentityError::new(
            IdentityErrorCategory::InvalidInput,
            IdentityErrorCode::AssertionSignatureInvalid,
            Some("signature_length"),
        )
    })?;
    if base64url_encode(&bytes) != input {
        return Err(IdentityError::new(
            IdentityErrorCategory::InvalidInput,
            IdentityErrorCode::AssertionSignatureInvalid,
            Some("signature_encoding"),
        ));
    }
    Ok(bytes)
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
    if input.is_empty() || input.contains('=') || input.len() % 4 == 1 {
        return Err(signature_encoding_error());
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
            _ => return Err(signature_encoding_error()),
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
        return Err(signature_encoding_error());
    }
    Ok(output)
}

fn signature_encoding_error() -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::InvalidInput,
        IdentityErrorCode::AssertionSignatureInvalid,
        Some("signature_encoding"),
    )
}

fn invalid_input(context: &'static str) -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::InvalidInput,
        IdentityErrorCode::InvalidJson,
        Some(context),
    )
}

fn temporal_error() -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::InvalidInput,
        IdentityErrorCode::AssertionTemporalInvalid,
        Some("assertion_temporal_range"),
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
