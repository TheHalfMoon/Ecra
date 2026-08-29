use std::{collections::BTreeSet, fmt};

use ecra_core::{EpochMillis, PrincipalRef, SchemaVersion};
use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use crate::bootstrap::ProtectedEnrollmentV1;
use crate::{
    ECR_031_CONTRACT_VERSION, EnrollmentId, IdentityError, IdentityErrorCategory,
    IdentityErrorCode, KeyId, SignatureAlgorithm, TrustBackendKind, TrustRootId,
    validate_ecr031_version,
};

pub const MAX_PROTECTED_TRUST_STATE_KEYS: usize = 128;
pub const MAX_REVOKED_KEY_IDS: usize = 128;
pub const MAX_I_JSON_U64: u64 = 9_007_199_254_740_991;

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

/// Closed algorithm/suite identifier for persisted ECR-031 key metadata.
///
/// Signing keys use the already-frozen Ed25519 signature algorithm. The
/// protected-envelope root is not a signing key; its high-entropy secret is
/// backend-protected and used only as v1 HKDF-SHA-256 input by the protected
/// envelope implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyRecordAlgorithm {
    #[serde(rename = "ed25519")]
    Ed25519,
    #[serde(rename = "ecra_protected_envelope_root_v1")]
    EcraProtectedEnvelopeRootV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrustRootStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "revoked")]
    Revoked,
}

/// Rebuildable, non-authoritative trust-root metadata.
///
/// This record intentionally contains no backend secret locator, key bytes or
/// lifecycle generation. Security-critical lifecycle authority lives only in
/// authenticated `ProtectedTrustStateV1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrustRootRecord {
    version: SchemaVersion,
    trust_root_id: TrustRootId,
    backend_kind: TrustBackendKind,
    created_at: EpochMillis,
    status: TrustRootStatus,
}

impl TrustRootRecord {
    #[must_use]
    pub const fn new(
        trust_root_id: TrustRootId,
        backend_kind: TrustBackendKind,
        created_at: EpochMillis,
        status: TrustRootStatus,
    ) -> Self {
        Self {
            version: ECR_031_CONTRACT_VERSION,
            trust_root_id,
            backend_kind,
            created_at,
            status,
        }
    }

    pub fn validate(&self) -> Result<(), IdentityError> {
        validate_ecr031_version(self.version)
    }

    #[must_use]
    pub const fn trust_root_id(self) -> TrustRootId {
        self.trust_root_id
    }

    #[must_use]
    pub const fn backend_kind(self) -> TrustBackendKind {
        self.backend_kind
    }

    #[must_use]
    pub const fn created_at(self) -> EpochMillis {
        self.created_at
    }

    #[must_use]
    pub const fn status(self) -> TrustRootStatus {
        self.status
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TrustRootRecordWire {
    version: SchemaVersion,
    trust_root_id: TrustRootId,
    backend_kind: TrustBackendKind,
    created_at: EpochMillis,
    status: TrustRootStatus,
}

impl<'de> Deserialize<'de> for TrustRootRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = TrustRootRecordWire::deserialize(deserializer)?;
        validate_ecr031_version(wire.version).map_err(de::Error::custom)?;
        Ok(Self {
            version: wire.version,
            trust_root_id: wire.trust_root_id,
            backend_kind: wire.backend_kind,
            created_at: wire.created_at,
            status: wire.status,
        })
    }
}

/// Strict public lifecycle metadata for one ECR-031 key generation.
///
/// The only serialized cryptographic bytes are optional Ed25519 public
/// verification material. Private signing seeds, protected-envelope root
/// secrets and derived symmetric keys have no field or constructor path here.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KeyRecord {
    version: SchemaVersion,
    key_id: KeyId,
    trust_root_id: TrustRootId,
    purpose: KeyPurpose,
    algorithm: KeyRecordAlgorithm,
    generation: u64,
    status: KeyStatus,
    public_material_b64url: Option<String>,
    created_at: EpochMillis,
    activated_at: EpochMillis,
    retired_at: Option<EpochMillis>,
    revoked_at: Option<EpochMillis>,
}

impl KeyRecord {
    #[allow(clippy::too_many_arguments)]
    pub fn new_ed25519(
        key_id: KeyId,
        trust_root_id: TrustRootId,
        purpose: KeyPurpose,
        generation: u64,
        status: KeyStatus,
        public_key: [u8; 32],
        created_at: EpochMillis,
        activated_at: EpochMillis,
        retired_at: Option<EpochMillis>,
        revoked_at: Option<EpochMillis>,
    ) -> Result<Self, IdentityError> {
        if !matches!(
            purpose,
            KeyPurpose::IdentityAssertionSigning | KeyPurpose::ProtectedAnchorSigning
        ) {
            return Err(key_record_error("ed25519_key_purpose"));
        }
        VerifyingKey::from_bytes(&public_key)
            .map_err(|_| key_record_error("ed25519_public_material"))?;
        Self::from_parts(
            ECR_031_CONTRACT_VERSION,
            key_id,
            trust_root_id,
            purpose,
            KeyRecordAlgorithm::Ed25519,
            generation,
            status,
            Some(base64url_encode(&public_key)),
            created_at,
            activated_at,
            retired_at,
            revoked_at,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_protected_envelope_root(
        key_id: KeyId,
        trust_root_id: TrustRootId,
        generation: u64,
        status: KeyStatus,
        created_at: EpochMillis,
        activated_at: EpochMillis,
        retired_at: Option<EpochMillis>,
        revoked_at: Option<EpochMillis>,
    ) -> Result<Self, IdentityError> {
        Self::from_parts(
            ECR_031_CONTRACT_VERSION,
            key_id,
            trust_root_id,
            KeyPurpose::ProtectedEnvelopeRoot,
            KeyRecordAlgorithm::EcraProtectedEnvelopeRootV1,
            generation,
            status,
            None,
            created_at,
            activated_at,
            retired_at,
            revoked_at,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn from_parts(
        version: SchemaVersion,
        key_id: KeyId,
        trust_root_id: TrustRootId,
        purpose: KeyPurpose,
        algorithm: KeyRecordAlgorithm,
        generation: u64,
        status: KeyStatus,
        public_material_b64url: Option<String>,
        created_at: EpochMillis,
        activated_at: EpochMillis,
        retired_at: Option<EpochMillis>,
        revoked_at: Option<EpochMillis>,
    ) -> Result<Self, IdentityError> {
        validate_ecr031_version(version)?;
        validate_key_generation(generation)?;
        validate_key_algorithm_and_public_material(
            purpose,
            algorithm,
            public_material_b64url.as_deref(),
        )?;
        validate_lifecycle_timestamps(status, created_at, activated_at, retired_at, revoked_at)?;
        Ok(Self {
            version,
            key_id,
            trust_root_id,
            purpose,
            algorithm,
            generation,
            status,
            public_material_b64url,
            created_at,
            activated_at,
            retired_at,
            revoked_at,
        })
    }

    /// Retire one currently-active generation for historical verification or
    /// decryption only. V1 has no reactivation transition.
    pub fn retire(&self, retired_at: EpochMillis) -> Result<Self, IdentityError> {
        match self.status {
            KeyStatus::Active => {}
            KeyStatus::RetiredVerifyOrDecryptOnly => {
                return Err(IdentityError::new(
                    IdentityErrorCategory::KeyState,
                    IdentityErrorCode::KeyNotActive,
                    Some("retire_key_not_active"),
                ));
            }
            KeyStatus::Revoked => {
                return Err(IdentityError::new(
                    IdentityErrorCategory::KeyState,
                    IdentityErrorCode::KeyRevoked,
                    Some("retire_key_revoked"),
                ));
            }
        }

        Self::from_parts(
            self.version,
            self.key_id,
            self.trust_root_id,
            self.purpose,
            self.algorithm,
            self.generation,
            KeyStatus::RetiredVerifyOrDecryptOnly,
            self.public_material_b64url.clone(),
            self.created_at,
            self.activated_at,
            Some(retired_at),
            None,
        )
    }

    /// Enforce the lifecycle rule for creating a new signature, envelope or
    /// other purpose-owned protected artifact. Retired generations are never
    /// valid for new use and revoked generations fail closed distinctly.
    pub fn ensure_new_material_use_allowed(&self) -> Result<(), IdentityError> {
        match self.status {
            KeyStatus::Active => Ok(()),
            KeyStatus::RetiredVerifyOrDecryptOnly => Err(IdentityError::new(
                IdentityErrorCategory::KeyState,
                IdentityErrorCode::KeyNotActive,
                Some("retired_key_new_use"),
            )),
            KeyStatus::Revoked => Err(IdentityError::new(
                IdentityErrorCategory::KeyState,
                IdentityErrorCode::KeyRevoked,
                Some("revoked_key_new_use"),
            )),
        }
    }

    /// Enforce the v1 historical compatibility rule after the caller has
    /// already matched the key purpose to the existing artifact. Active and
    /// retired generations may verify/decrypt existing artifacts; revoked
    /// generations cannot be used through this compatibility path.
    pub fn ensure_historical_use_allowed(&self) -> Result<(), IdentityError> {
        match self.status {
            KeyStatus::Active | KeyStatus::RetiredVerifyOrDecryptOnly => Ok(()),
            KeyStatus::Revoked => Err(IdentityError::new(
                IdentityErrorCategory::KeyState,
                IdentityErrorCode::KeyRevoked,
                Some("revoked_key_historical_use"),
            )),
        }
    }

    #[must_use]
    pub const fn key_id(&self) -> KeyId {
        self.key_id
    }

    #[must_use]
    pub const fn trust_root_id(&self) -> TrustRootId {
        self.trust_root_id
    }

    #[must_use]
    pub const fn purpose(&self) -> KeyPurpose {
        self.purpose
    }

    #[must_use]
    pub const fn algorithm(&self) -> KeyRecordAlgorithm {
        self.algorithm
    }

    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    #[must_use]
    pub const fn status(&self) -> KeyStatus {
        self.status
    }

    #[must_use]
    pub fn public_material_b64url(&self) -> Option<&str> {
        self.public_material_b64url.as_deref()
    }

    pub fn ed25519_public_key(&self) -> Result<Option<[u8; 32]>, IdentityError> {
        match self.public_material_b64url.as_deref() {
            Some(encoded) => decode_ed25519_public_key(encoded).map(Some),
            None => Ok(None),
        }
    }

    #[must_use]
    pub const fn created_at(&self) -> EpochMillis {
        self.created_at
    }

    #[must_use]
    pub const fn activated_at(&self) -> EpochMillis {
        self.activated_at
    }

    #[must_use]
    pub const fn retired_at(&self) -> Option<EpochMillis> {
        self.retired_at
    }

    #[must_use]
    pub const fn revoked_at(&self) -> Option<EpochMillis> {
        self.revoked_at
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct KeyRecordWire {
    version: SchemaVersion,
    key_id: KeyId,
    trust_root_id: TrustRootId,
    purpose: KeyPurpose,
    algorithm: KeyRecordAlgorithm,
    generation: u64,
    status: KeyStatus,
    public_material_b64url: Option<String>,
    created_at: EpochMillis,
    activated_at: EpochMillis,
    retired_at: Option<EpochMillis>,
    revoked_at: Option<EpochMillis>,
}

impl<'de> Deserialize<'de> for KeyRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = KeyRecordWire::deserialize(deserializer)?;
        Self::from_parts(
            wire.version,
            wire.key_id,
            wire.trust_root_id,
            wire.purpose,
            wire.algorithm,
            wire.generation,
            wire.status,
            wire.public_material_b64url,
            wire.created_at,
            wire.activated_at,
            wire.retired_at,
            wire.revoked_at,
        )
        .map_err(de::Error::custom)
    }
}

/// Authenticated plaintext lifecycle model stored only inside the protected
/// trust-state envelope.
///
/// This type is lifecycle authority only after the owning store has
/// authenticated/opened the surrounding protected envelope. Deserializing these
/// bytes alone does not create a `VerifiedTrustSnapshot`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProtectedTrustStateV1 {
    version: SchemaVersion,
    trust_root_id: TrustRootId,
    enrollment: ProtectedEnrollmentV1,
    state_generation: u64,
    keys: Vec<KeyRecord>,
    revoked_key_ids: BTreeSet<KeyId>,
    updated_at: EpochMillis,
}

impl ProtectedTrustStateV1 {
    pub fn new(
        trust_root_id: TrustRootId,
        enrollment: ProtectedEnrollmentV1,
        state_generation: u64,
        keys: Vec<KeyRecord>,
        revoked_key_ids: BTreeSet<KeyId>,
        updated_at: EpochMillis,
    ) -> Result<Self, IdentityError> {
        let state = Self {
            version: ECR_031_CONTRACT_VERSION,
            trust_root_id,
            enrollment,
            state_generation,
            keys,
            revoked_key_ids,
            updated_at,
        };
        state.validate_schema_invariants()?;
        Ok(state)
    }

    pub fn validate_schema_invariants(&self) -> Result<(), IdentityError> {
        validate_ecr031_version(self.version)?;
        if self.state_generation == 0 || self.state_generation > MAX_I_JSON_U64 {
            return Err(lifecycle_error("trust_state_generation"));
        }
        if self.keys.len() > MAX_PROTECTED_TRUST_STATE_KEYS {
            return Err(lifecycle_error("trust_state_key_count"));
        }
        if self.revoked_key_ids.len() > MAX_REVOKED_KEY_IDS {
            return Err(lifecycle_error("trust_state_revocation_count"));
        }

        let mut seen = BTreeSet::new();
        let mut active_identity_assertion_signing = None;
        let mut active_protected_envelope_root = None;
        let mut active_protected_anchor_signing = None;
        for key in &self.keys {
            if key.trust_root_id() != self.trust_root_id {
                return Err(lifecycle_error("key_trust_root_mismatch"));
            }
            if key.generation() > self.state_generation {
                return Err(lifecycle_error("key_generation_ahead_of_state"));
            }
            if key.created_at().get() > self.updated_at.get()
                || key.activated_at().get() > self.updated_at.get()
                || key
                    .retired_at()
                    .is_some_and(|timestamp| timestamp.get() > self.updated_at.get())
                || key
                    .revoked_at()
                    .is_some_and(|timestamp| timestamp.get() > self.updated_at.get())
            {
                return Err(lifecycle_error("key_timestamp_ahead_of_state"));
            }
            if !seen.insert(key.key_id()) {
                return Err(lifecycle_error("duplicate_key_id"));
            }
            if matches!(key.status(), KeyStatus::Active) {
                let active_slot = match key.purpose() {
                    KeyPurpose::IdentityAssertionSigning => &mut active_identity_assertion_signing,
                    KeyPurpose::ProtectedEnvelopeRoot => &mut active_protected_envelope_root,
                    KeyPurpose::ProtectedAnchorSigning => &mut active_protected_anchor_signing,
                };
                if active_slot.replace(key.key_id()).is_some() {
                    return Err(lifecycle_error("multiple_active_keys_for_purpose"));
                }
            }
            let listed_revoked = self.revoked_key_ids.contains(&key.key_id());
            if listed_revoked != matches!(key.status(), KeyStatus::Revoked) {
                return Err(lifecycle_error("revocation_set_mismatch"));
            }
        }
        if self
            .revoked_key_ids
            .iter()
            .any(|key_id| !seen.contains(key_id))
        {
            return Err(lifecycle_error("unknown_revoked_key"));
        }

        Ok(())
    }

    #[must_use]
    pub const fn trust_root_id(&self) -> TrustRootId {
        self.trust_root_id
    }

    #[must_use]
    pub const fn enrollment(&self) -> ProtectedEnrollmentV1 {
        self.enrollment
    }

    #[must_use]
    pub const fn state_generation(&self) -> u64 {
        self.state_generation
    }

    #[must_use]
    pub fn keys(&self) -> &[KeyRecord] {
        &self.keys
    }

    pub fn active_key(&self, purpose: KeyPurpose) -> Result<&KeyRecord, IdentityError> {
        let mut active = self
            .keys
            .iter()
            .filter(|key| key.purpose() == purpose && matches!(key.status(), KeyStatus::Active));
        let selected = active.next().ok_or_else(|| {
            IdentityError::new(
                IdentityErrorCategory::KeyState,
                IdentityErrorCode::KeyNotActive,
                Some("active_key_missing"),
            )
        })?;
        if active.next().is_some() {
            return Err(lifecycle_error("multiple_active_keys_for_purpose"));
        }
        Ok(selected)
    }

    #[must_use]
    pub fn revoked_key_ids(&self) -> &BTreeSet<KeyId> {
        &self.revoked_key_ids
    }

    #[must_use]
    pub const fn updated_at(&self) -> EpochMillis {
        self.updated_at
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProtectedTrustStateWire {
    version: SchemaVersion,
    trust_root_id: TrustRootId,
    enrollment: ProtectedEnrollmentV1,
    state_generation: u64,
    keys: Vec<KeyRecord>,
    revoked_key_ids: Vec<KeyId>,
    updated_at: EpochMillis,
}

impl<'de> Deserialize<'de> for ProtectedTrustStateV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ProtectedTrustStateWire::deserialize(deserializer)?;
        if wire.revoked_key_ids.len() > MAX_REVOKED_KEY_IDS {
            return Err(de::Error::custom(lifecycle_error(
                "trust_state_revocation_count",
            )));
        }
        let revoked_key_ids = wire
            .revoked_key_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        if revoked_key_ids.len() != wire.revoked_key_ids.len() {
            return Err(de::Error::custom(lifecycle_error(
                "duplicate_revoked_key_id",
            )));
        }
        let state = Self {
            version: wire.version,
            trust_root_id: wire.trust_root_id,
            enrollment: wire.enrollment,
            state_generation: wire.state_generation,
            keys: wire.keys,
            revoked_key_ids,
            updated_at: wire.updated_at,
        };
        state
            .validate_schema_invariants()
            .map_err(de::Error::custom)?;
        Ok(state)
    }
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
    // Phase 4 authenticated trust-state parsing is the production caller.
    #[allow(dead_code)]
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
    // Phase 4 protected-state authentication is the production caller; no public constructor.
    #[allow(dead_code)]
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
        self.assertion_keys
            .iter()
            .find(|key| key.key_id() == key_id)
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

fn validate_key_generation(generation: u64) -> Result<(), IdentityError> {
    if generation == 0 || generation > MAX_I_JSON_U64 {
        return Err(key_record_error("key_generation"));
    }
    Ok(())
}

fn validate_key_algorithm_and_public_material(
    purpose: KeyPurpose,
    algorithm: KeyRecordAlgorithm,
    public_material_b64url: Option<&str>,
) -> Result<(), IdentityError> {
    match (purpose, algorithm, public_material_b64url) {
        (
            KeyPurpose::IdentityAssertionSigning | KeyPurpose::ProtectedAnchorSigning,
            KeyRecordAlgorithm::Ed25519,
            Some(encoded),
        ) => {
            decode_ed25519_public_key(encoded)?;
            Ok(())
        }
        (
            KeyPurpose::ProtectedEnvelopeRoot,
            KeyRecordAlgorithm::EcraProtectedEnvelopeRootV1,
            None,
        ) => Ok(()),
        _ => Err(key_record_error("key_algorithm_purpose_binding")),
    }
}

fn validate_lifecycle_timestamps(
    status: KeyStatus,
    created_at: EpochMillis,
    activated_at: EpochMillis,
    retired_at: Option<EpochMillis>,
    revoked_at: Option<EpochMillis>,
) -> Result<(), IdentityError> {
    if activated_at.get() < created_at.get() {
        return Err(key_record_error("key_activation_before_creation"));
    }

    match status {
        KeyStatus::Active => {
            if retired_at.is_some() || revoked_at.is_some() {
                return Err(key_record_error("active_key_terminal_timestamp"));
            }
        }
        KeyStatus::RetiredVerifyOrDecryptOnly => {
            let retired_at =
                retired_at.ok_or_else(|| key_record_error("retired_key_missing_timestamp"))?;
            if retired_at.get() < activated_at.get() || revoked_at.is_some() {
                return Err(key_record_error("retired_key_timestamp"));
            }
        }
        KeyStatus::Revoked => {
            let revoked_at =
                revoked_at.ok_or_else(|| key_record_error("revoked_key_missing_timestamp"))?;
            if revoked_at.get() < activated_at.get() {
                return Err(key_record_error("revoked_key_timestamp"));
            }
            if retired_at.is_some_and(|retired_at| {
                retired_at.get() < activated_at.get() || retired_at.get() > revoked_at.get()
            }) {
                return Err(key_record_error("revoked_key_retirement_timestamp"));
            }
        }
    }
    Ok(())
}

fn decode_ed25519_public_key(input: &str) -> Result<[u8; 32], IdentityError> {
    let decoded = base64url_decode(input)?;
    let bytes: [u8; 32] = decoded
        .try_into()
        .map_err(|_| key_record_error("ed25519_public_material_length"))?;
    if base64url_encode(&bytes) != input {
        return Err(key_record_error("ed25519_public_material_encoding"));
    }
    VerifyingKey::from_bytes(&bytes).map_err(|_| key_record_error("ed25519_public_material"))?;
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
        return Err(key_record_error("ed25519_public_material_encoding"));
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
            _ => return Err(key_record_error("ed25519_public_material_encoding")),
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
        return Err(key_record_error("ed25519_public_material_encoding"));
    }
    Ok(output)
}

fn key_record_error(context: &'static str) -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::KeyState,
        IdentityErrorCode::TrustSnapshotLifecycleInvalid,
        Some(context),
    )
}

#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use ecra_core::{EpochMillis, PrincipalId};
    use ed25519_dalek::SigningKey;

    use super::{
        KeyPurpose, KeyRecord, KeyRecordAlgorithm, KeyStatus, ProtectedTrustStateV1,
        TrustRootRecord, TrustRootStatus,
    };
    use crate::bootstrap::ProtectedEnrollmentV1;
    use crate::{EnrollmentId, IdentityErrorCode, KeyId, TrustBackendKind, TrustRootId};

    const ROOT: &str = "00000000-0000-0000-0000-000000000002";
    const SIGNING_KEY: &str = "00000000-0000-0000-0000-000000000003";
    const SIGNING_KEY_2: &str = "00000000-0000-0000-0000-000000000013";
    const ENVELOPE_KEY: &str = "00000000-0000-0000-0000-000000000011";
    const PRINCIPAL: &str = "00000000-0000-0000-0000-000000000004";
    const ENROLLMENT: &str = "00000000-0000-0000-0000-000000000030";

    fn timestamp(value: i64) -> EpochMillis {
        EpochMillis::new(value).unwrap()
    }

    fn root_id() -> TrustRootId {
        TrustRootId::parse_str(ROOT).unwrap()
    }

    fn signing_record(status: KeyStatus) -> KeyRecord {
        signing_record_with(SIGNING_KEY, 1, status, 7)
    }

    fn signing_record_with(
        key_id: &str,
        generation: u64,
        status: KeyStatus,
        seed_byte: u8,
    ) -> KeyRecord {
        let signing_key = SigningKey::from_bytes(&[seed_byte; 32]);
        let (retired_at, revoked_at) = match status {
            KeyStatus::Active => (None, None),
            KeyStatus::RetiredVerifyOrDecryptOnly => (Some(timestamp(1_100)), None),
            KeyStatus::Revoked => (None, Some(timestamp(1_100))),
        };
        KeyRecord::new_ed25519(
            KeyId::parse_str(key_id).unwrap(),
            root_id(),
            KeyPurpose::IdentityAssertionSigning,
            generation,
            status,
            signing_key.verifying_key().to_bytes(),
            timestamp(900),
            timestamp(1_000),
            retired_at,
            revoked_at,
        )
        .unwrap()
    }

    fn protected_enrollment() -> ProtectedEnrollmentV1 {
        ProtectedEnrollmentV1::new(
            EnrollmentId::parse_str(ENROLLMENT).unwrap(),
            PrincipalId::parse_str(PRINCIPAL).unwrap(),
        )
    }

    #[test]
    fn trust_root_projection_is_strict_and_contains_no_secret_locator() {
        let record = TrustRootRecord::new(
            root_id(),
            TrustBackendKind::MacosDataProtectionKeychain,
            timestamp(900),
            TrustRootStatus::Active,
        );
        let value = serde_json::to_value(record).unwrap();
        let object = value.as_object().unwrap();
        assert!(!object.contains_key("backend_locator_ref"));
        assert!(!object.contains_key("secret"));
        assert!(!object.contains_key("private_key"));
        assert!(!object.contains_key("current_generation_by_purpose"));

        let mut invalid = value;
        invalid["backend_locator_ref"] = "do-not-accept-free-form-native-locators".into();
        assert!(serde_json::from_value::<TrustRootRecord>(invalid).is_err());
    }

    #[test]
    fn key_record_serializes_only_public_signing_material() {
        let record = signing_record(KeyStatus::Active);
        assert_eq!(record.algorithm(), KeyRecordAlgorithm::Ed25519);
        assert!(record.ed25519_public_key().unwrap().is_some());

        let json = serde_json::to_value(&record).unwrap();
        assert!(json["public_material_b64url"].as_str().is_some());
        assert!(json.get("private_key").is_none());
        assert!(json.get("secret").is_none());
        assert!(json.get("seed").is_none());
        assert!(json.get("symmetric_key").is_none());
    }

    #[test]
    fn envelope_root_record_has_no_serialized_secret_or_public_material() {
        let record = KeyRecord::new_protected_envelope_root(
            KeyId::parse_str(ENVELOPE_KEY).unwrap(),
            root_id(),
            1,
            KeyStatus::Active,
            timestamp(900),
            timestamp(1_000),
            None,
            None,
        )
        .unwrap();

        let json = serde_json::to_value(&record).unwrap();
        assert_eq!(json["algorithm"], "ecra_protected_envelope_root_v1");
        assert!(json["public_material_b64url"].is_null());
        assert!(json.get("secret").is_none());
        assert!(json.get("root_key").is_none());
    }

    #[test]
    fn retirement_is_one_way_and_timestamp_bounded() {
        let active = signing_record(KeyStatus::Active);
        let retired = active.retire(timestamp(1_100)).unwrap();
        assert_eq!(retired.status(), KeyStatus::RetiredVerifyOrDecryptOnly);
        assert_eq!(retired.retired_at(), Some(timestamp(1_100)));
        assert_eq!(retired.key_id(), active.key_id());
        assert_eq!(retired.generation(), active.generation());

        let already_retired = retired.retire(timestamp(1_200)).unwrap_err();
        assert_eq!(already_retired.code(), IdentityErrorCode::KeyNotActive);

        let invalid_time = active.retire(timestamp(999)).unwrap_err();
        assert_eq!(
            invalid_time.code(),
            IdentityErrorCode::TrustSnapshotLifecycleInvalid
        );
    }

    #[test]
    fn retirement_blocks_new_use_but_preserves_historical_compatibility() {
        let active = signing_record(KeyStatus::Active);
        active.ensure_new_material_use_allowed().unwrap();
        active.ensure_historical_use_allowed().unwrap();

        let retired = active.retire(timestamp(1_100)).unwrap();
        let new_use = retired.ensure_new_material_use_allowed().unwrap_err();
        assert_eq!(new_use.code(), IdentityErrorCode::KeyNotActive);
        retired.ensure_historical_use_allowed().unwrap();

        let revoked = signing_record(KeyStatus::Revoked);
        assert_eq!(
            revoked
                .ensure_new_material_use_allowed()
                .unwrap_err()
                .code(),
            IdentityErrorCode::KeyRevoked
        );
        assert_eq!(
            revoked.ensure_historical_use_allowed().unwrap_err().code(),
            IdentityErrorCode::KeyRevoked
        );
    }

    #[test]
    fn protected_state_rejects_wrong_root_duplicate_and_revocation_drift() {
        let enrollment = protected_enrollment();
        let signing = signing_record(KeyStatus::Active);
        let state = ProtectedTrustStateV1::new(
            root_id(),
            enrollment,
            1,
            vec![signing.clone()],
            BTreeSet::new(),
            timestamp(1_200),
        )
        .unwrap();
        assert_eq!(state.keys().len(), 1);

        let duplicate = ProtectedTrustStateV1::new(
            root_id(),
            enrollment,
            1,
            vec![signing.clone(), signing],
            BTreeSet::new(),
            timestamp(1_200),
        )
        .unwrap_err();
        assert_eq!(
            duplicate.code(),
            IdentityErrorCode::TrustSnapshotLifecycleInvalid
        );

        let other_root = TrustRootId::parse_str("00000000-0000-0000-0000-000000000012").unwrap();
        let wrong_root_key = KeyRecord::new_protected_envelope_root(
            KeyId::parse_str(ENVELOPE_KEY).unwrap(),
            other_root,
            1,
            KeyStatus::Active,
            timestamp(900),
            timestamp(1_000),
            None,
            None,
        )
        .unwrap();
        assert!(
            ProtectedTrustStateV1::new(
                root_id(),
                enrollment,
                1,
                vec![wrong_root_key],
                BTreeSet::new(),
                timestamp(1_200),
            )
            .is_err()
        );

        let revoked = signing_record(KeyStatus::Revoked);
        assert!(
            ProtectedTrustStateV1::new(
                root_id(),
                enrollment,
                1,
                vec![revoked],
                BTreeSet::new(),
                timestamp(1_200),
            )
            .is_err()
        );
    }

    #[test]
    fn protected_state_deserialization_rejects_duplicate_revoked_ids() {
        let revoked = signing_record(KeyStatus::Revoked);
        let revoked_id = revoked.key_id();
        let enrollment = protected_enrollment();
        let state = ProtectedTrustStateV1::new(
            root_id(),
            enrollment,
            1,
            vec![revoked],
            BTreeSet::from([revoked_id]),
            timestamp(1_200),
        )
        .unwrap();
        let mut value = serde_json::to_value(state).unwrap();
        value["revoked_key_ids"] =
            serde_json::json!([revoked_id.to_string(), revoked_id.to_string()]);
        assert!(serde_json::from_value::<ProtectedTrustStateV1>(value).is_err());
    }

    #[test]
    fn protected_state_allows_one_active_key_per_distinct_purpose() {
        let envelope = KeyRecord::new_protected_envelope_root(
            KeyId::parse_str(ENVELOPE_KEY).unwrap(),
            root_id(),
            1,
            KeyStatus::Active,
            timestamp(900),
            timestamp(1_000),
            None,
            None,
        )
        .unwrap();
        let state = ProtectedTrustStateV1::new(
            root_id(),
            protected_enrollment(),
            1,
            vec![signing_record(KeyStatus::Active), envelope],
            BTreeSet::new(),
            timestamp(1_200),
        )
        .unwrap();

        assert_eq!(
            state
                .active_key(KeyPurpose::IdentityAssertionSigning)
                .unwrap()
                .key_id(),
            KeyId::parse_str(SIGNING_KEY).unwrap()
        );
        assert_eq!(
            state
                .active_key(KeyPurpose::ProtectedEnvelopeRoot)
                .unwrap()
                .key_id(),
            KeyId::parse_str(ENVELOPE_KEY).unwrap()
        );
    }

    #[test]
    fn protected_state_rejects_two_active_keys_for_same_purpose() {
        let first = signing_record_with(SIGNING_KEY, 1, KeyStatus::Active, 7);
        let second = signing_record_with(SIGNING_KEY_2, 2, KeyStatus::Active, 8);

        let error = ProtectedTrustStateV1::new(
            root_id(),
            protected_enrollment(),
            2,
            vec![first, second],
            BTreeSet::new(),
            timestamp(1_200),
        )
        .unwrap_err();
        assert_eq!(
            error.code(),
            IdentityErrorCode::TrustSnapshotLifecycleInvalid
        );
    }

    #[test]
    fn active_key_selects_active_generation_and_fails_closed_when_absent() {
        let retired = signing_record_with(SIGNING_KEY, 1, KeyStatus::RetiredVerifyOrDecryptOnly, 7);
        let active = signing_record_with(SIGNING_KEY_2, 2, KeyStatus::Active, 8);
        let state = ProtectedTrustStateV1::new(
            root_id(),
            protected_enrollment(),
            2,
            vec![retired, active],
            BTreeSet::new(),
            timestamp(1_200),
        )
        .unwrap();

        assert_eq!(
            state
                .active_key(KeyPurpose::IdentityAssertionSigning)
                .unwrap()
                .key_id(),
            KeyId::parse_str(SIGNING_KEY_2).unwrap()
        );
        let missing = state
            .active_key(KeyPurpose::ProtectedEnvelopeRoot)
            .unwrap_err();
        assert_eq!(missing.code(), IdentityErrorCode::KeyNotActive);
    }
}
