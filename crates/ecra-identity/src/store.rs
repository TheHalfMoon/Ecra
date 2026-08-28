use std::{
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use ecra_core::{EpochMillis, SchemaVersion, to_jcs_vec};
use ed25519_dalek::SigningKey;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use uuid::Uuid;
use zeroize::Zeroizing;

use crate::backend::{SecureRandom, TrustBackend, TrustBackendSecretRef, TrustBackendStatus};
use crate::envelope::{open_envelope, protect_envelope};
use crate::key::{KeyRecord, KeyStatus, MAX_I_JSON_U64, ProtectedTrustStateV1};
use crate::{
    EnvelopeKeyRef, IdentityError, IdentityErrorCategory, IdentityErrorCode, KeyId, KeyPurpose,
    MAX_JSON_DEPTH, MAX_PROTECTED_ENVELOPE_WIRE_BYTES, ProtectedEnvelopeV1,
    ProtectedInformationClass, ProtectedObjectId, ProtectedPurpose, SensitiveBytes,
    TrustStateDigest, validate_ecr031_version, validate_json_limits,
};

const TRUST_STATE_PURPOSE: ProtectedPurpose = ProtectedPurpose::TrustState;
const TRUST_STATE_INFORMATION_CLASS: ProtectedInformationClass =
    ProtectedInformationClass::Sensitive;
const BOOTSTRAP_MARKER_BYTES: &[u8] = br#"{"state":"in_progress","version":{"major":1,"minor":0}}"#;
const ROTATION_SECRET_BYTES: usize = 32;

/// Filesystem location owned by the protected store boundary rather than by
/// identity bootstrap logic. The location is operational metadata only and can
/// never be converted into a principal, trust root, enrollment or key id.
#[derive(Clone, Debug)]
pub(crate) struct BootstrapStoreLocation {
    path: PathBuf,
}

impl BootstrapStoreLocation {
    pub(crate) fn new(path: PathBuf) -> Result<Self, IdentityError> {
        if path.file_name().is_none() {
            return Err(store_input_error("trust_state_store_path"));
        }
        Ok(Self { path })
    }

    #[must_use]
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }
}

/// Authenticated protected trust state returned only after backend secret open,
/// AEAD authentication and strict lifecycle validation all succeed.
///
/// This wrapper is intentionally crate-private: ordinary filesystem bytes are
/// never upgraded into identity authority merely because they parse as JSON.
#[derive(Debug)]
pub(crate) struct AuthenticatedTrustState {
    state: ProtectedTrustStateV1,
    trust_state_digest: TrustStateDigest,
}

impl AuthenticatedTrustState {
    #[must_use]
    pub(crate) const fn state(&self) -> &ProtectedTrustStateV1 {
        &self.state
    }

    #[must_use]
    pub(crate) const fn trust_state_digest(&self) -> TrustStateDigest {
        self.trust_state_digest
    }

    #[must_use]
    pub(crate) fn into_state(self) -> ProtectedTrustStateV1 {
        self.state
    }
}

/// Tiny ECR-031-owned durable store for the single authoritative protected
/// trust-state envelope.
///
/// The file contains only a `ProtectedEnvelopeV1`. Any ordinary indexes or
/// projections remain rebuildable and cannot override the authenticated state
/// returned by `open_authenticated`.
#[derive(Debug)]
pub(crate) struct ProtectedTrustStateStore {
    path: PathBuf,
    object_id: ProtectedObjectId,
}

impl ProtectedTrustStateStore {
    pub(crate) fn new(path: PathBuf, object_id: ProtectedObjectId) -> Result<Self, IdentityError> {
        if path.file_name().is_none() {
            return Err(store_input_error("trust_state_store_path"));
        }
        Ok(Self { path, object_id })
    }

    pub(crate) fn open_existing(
        path: PathBuf,
        backend: &impl TrustBackend,
    ) -> Result<(Self, AuthenticatedTrustState), IdentityError> {
        if path.file_name().is_none() {
            return Err(store_input_error("trust_state_store_path"));
        }
        let wire = read_bounded_path(&path)?;
        let envelope = ProtectedEnvelopeV1::from_json_slice(&wire)?;
        if envelope.purpose() != TRUST_STATE_PURPOSE
            || envelope.information_class() != TRUST_STATE_INFORMATION_CLASS
        {
            return Err(store_corruption_error("trust_state_envelope_role"));
        }
        let store = Self {
            path,
            object_id: envelope.object_id(),
        };
        let authenticated = store.open_authenticated(backend)?;
        Ok((store, authenticated))
    }

    #[must_use]
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn exists(&self) -> Result<bool, IdentityError> {
        Self::store_exists(&self.path)
    }

    pub(crate) fn store_exists(path: &Path) -> Result<bool, IdentityError> {
        path_is_regular_file_if_present(path, "trust_state_store_file_type")
    }

    pub(crate) fn bootstrap_marker_exists(path: &Path) -> Result<bool, IdentityError> {
        let marker = bootstrap_marker_path(path)?;
        path_is_regular_file_if_present(&marker, "bootstrap_marker_file_type")
    }

    pub(crate) fn write_bootstrap_marker(path: &Path) -> Result<(), IdentityError> {
        if Self::bootstrap_marker_exists(path)? {
            return Ok(());
        }
        let marker = bootstrap_marker_path(path)?;
        atomic_replace_path(&marker, BOOTSTRAP_MARKER_BYTES)
    }

    pub(crate) fn clear_bootstrap_marker(path: &Path) -> Result<(), IdentityError> {
        let marker = bootstrap_marker_path(path)?;
        match fs::symlink_metadata(&marker) {
            Ok(metadata) => {
                if metadata.file_type().is_symlink() || !metadata.is_file() {
                    return Err(store_corruption_error("bootstrap_marker_file_type"));
                }
                fs::remove_file(&marker).map_err(|_| store_io_error("bootstrap_marker_remove"))?;
                let parent = marker
                    .parent()
                    .ok_or_else(|| store_input_error("bootstrap_marker_parent"))?;
                sync_parent_directory(parent)
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(_) => Err(store_io_error("bootstrap_marker_metadata")),
        }
    }

    pub(crate) fn publish(
        &self,
        backend: &impl TrustBackend,
        random: &mut impl SecureRandom,
        state: &ProtectedTrustStateV1,
    ) -> Result<(), IdentityError> {
        state.validate_schema_invariants()?;
        let active_envelope_root = state.active_key(KeyPurpose::ProtectedEnvelopeRoot)?;
        let key_ref = EnvelopeKeyRef::new(
            state.trust_root_id(),
            active_envelope_root.key_id(),
            active_envelope_root.generation(),
        )?;

        ensure_backend_available(backend)?;
        let secret_ref = TrustBackendSecretRef::new(
            key_ref.trust_root_id(),
            key_ref.key_id(),
            key_ref.generation(),
            KeyPurpose::ProtectedEnvelopeRoot,
        )?;
        let master_secret = backend.open_protected_secret(secret_ref)?;
        let canonical_state = to_jcs_vec(state).map_err(|_| {
            IdentityError::new(
                IdentityErrorCategory::ProtectedStorage,
                IdentityErrorCode::CanonicalizationFailed,
                Some("protected_trust_state_jcs"),
            )
        })?;
        validate_json_limits(
            &canonical_state,
            MAX_PROTECTED_ENVELOPE_WIRE_BYTES,
            MAX_JSON_DEPTH,
        )?;
        let plaintext = SensitiveBytes::new(canonical_state);
        let envelope = protect_envelope(
            &master_secret,
            random,
            self.object_id,
            TRUST_STATE_PURPOSE,
            TRUST_STATE_INFORMATION_CLASS,
            key_ref,
            &plaintext,
        )?;
        let wire = to_jcs_vec(&envelope).map_err(|_| {
            IdentityError::new(
                IdentityErrorCategory::ProtectedStorage,
                IdentityErrorCode::CanonicalizationFailed,
                Some("protected_trust_state_envelope_jcs"),
            )
        })?;
        if wire.len() > MAX_PROTECTED_ENVELOPE_WIRE_BYTES {
            return Err(IdentityError::new(
                IdentityErrorCategory::ProtectedStorage,
                IdentityErrorCode::WireLimitExceeded,
                Some("protected_trust_state_envelope"),
            ));
        }
        self.atomic_replace(&wire)
    }

    /// Rotate one active purpose key through the authoritative protected state.
    ///
    /// The next secret is protected by the selected backend before any state is
    /// published. The prior generation is retired, the next generation becomes
    /// active and the complete transition is committed through the same
    /// crash-safe atomic replacement used by ordinary protected-state writes.
    /// Backend material created before a failed publish is non-authoritative
    /// orphan material and cannot become identity authority by itself.
    #[allow(dead_code)]
    pub(crate) fn rotate_key(
        &self,
        backend: &impl TrustBackend,
        random: &mut impl SecureRandom,
        purpose: KeyPurpose,
        transitioned_at: EpochMillis,
    ) -> Result<AuthenticatedTrustState, IdentityError> {
        let authenticated = self.open_authenticated(backend)?;
        let state = authenticated.state();
        state.validate_schema_invariants()?;
        if transitioned_at.get() < state.updated_at().get() {
            return Err(rotation_error("rotation_timestamp_before_state"));
        }

        let current = state.active_key(purpose)?;
        current.ensure_new_material_use_allowed()?;
        let next_generation = current
            .generation()
            .checked_add(1)
            .filter(|generation| *generation <= MAX_I_JSON_U64)
            .ok_or_else(|| rotation_error("rotation_key_generation_overflow"))?;
        let next_state_generation = state
            .state_generation()
            .checked_add(1)
            .filter(|generation| *generation <= MAX_I_JSON_U64)
            .ok_or_else(|| rotation_error("rotation_state_generation_overflow"))?;
        let next_key_id = random_rotation_key_id(random)?;
        if state.keys().iter().any(|key| key.key_id() == next_key_id) {
            return Err(rotation_error("rotation_key_id_collision"));
        }

        let next_key = protect_rotated_secret(
            backend,
            random,
            state.trust_root_id(),
            next_key_id,
            purpose,
            next_generation,
            transitioned_at,
        )?;
        let retired_current = current.retire(transitioned_at)?;
        let current_key_id = current.key_id();
        let mut keys = Vec::with_capacity(state.keys().len().saturating_add(1));
        for key in state.keys() {
            if key.key_id() == current_key_id {
                keys.push(retired_current.clone());
            } else {
                keys.push(key.clone());
            }
        }
        keys.push(next_key);

        let next_state = ProtectedTrustStateV1::new(
            state.trust_root_id(),
            state.enrollment(),
            next_state_generation,
            keys,
            state.revoked_key_ids().clone(),
            transitioned_at,
        )?;
        self.publish(backend, random, &next_state)?;
        self.open_authenticated(backend)
    }

    /// Revoke one existing generation in the authenticated protected state.
    ///
    /// Revocation is authoritative lifecycle state, not backend deletion. The
    /// protected secret is intentionally left in backend custody so callers do
    /// not confuse `revoked` with unavailable/destroyed material. An active
    /// protected-envelope root must be rotated first because the store still
    /// needs one active root to authenticate and publish the revocation state.
    #[allow(dead_code)]
    pub(crate) fn revoke_key(
        &self,
        backend: &impl TrustBackend,
        random: &mut impl SecureRandom,
        key_id: KeyId,
        revoked_at: EpochMillis,
    ) -> Result<AuthenticatedTrustState, IdentityError> {
        let authenticated = self.open_authenticated(backend)?;
        let state = authenticated.state();
        state.validate_schema_invariants()?;
        if revoked_at.get() < state.updated_at().get() {
            return Err(revocation_error("revocation_timestamp_before_state"));
        }

        let target = state
            .keys()
            .iter()
            .find(|key| key.key_id() == key_id)
            .ok_or_else(|| {
                IdentityError::new(
                    IdentityErrorCategory::KeyState,
                    IdentityErrorCode::KeyNotFound,
                    Some("revocation_key"),
                )
            })?;
        if matches!(target.status(), KeyStatus::Revoked) {
            return Err(IdentityError::new(
                IdentityErrorCategory::KeyState,
                IdentityErrorCode::KeyRevoked,
                Some("revocation_key"),
            ));
        }
        if target.purpose() == KeyPurpose::ProtectedEnvelopeRoot
            && matches!(target.status(), KeyStatus::Active)
        {
            return Err(IdentityError::new(
                IdentityErrorCategory::KeyState,
                IdentityErrorCode::KeyNotActive,
                Some("revoke_active_envelope_root_requires_rotation"),
            ));
        }

        let next_state_generation = state
            .state_generation()
            .checked_add(1)
            .filter(|generation| *generation <= MAX_I_JSON_U64)
            .ok_or_else(|| revocation_error("revocation_state_generation_overflow"))?;
        let revoked_record = revoked_key_record(target, revoked_at)?;
        let mut keys = Vec::with_capacity(state.keys().len());
        for key in state.keys() {
            if key.key_id() == key_id {
                keys.push(revoked_record.clone());
            } else {
                keys.push(key.clone());
            }
        }
        let mut revoked_key_ids = state.revoked_key_ids().clone();
        if !revoked_key_ids.insert(key_id) {
            return Err(IdentityError::new(
                IdentityErrorCategory::KeyState,
                IdentityErrorCode::KeyRevoked,
                Some("revocation_set"),
            ));
        }

        let next_state = ProtectedTrustStateV1::new(
            state.trust_root_id(),
            state.enrollment(),
            next_state_generation,
            keys,
            revoked_key_ids,
            revoked_at,
        )?;
        self.publish(backend, random, &next_state)?;
        self.open_authenticated(backend)
    }

    pub(crate) fn open_authenticated(
        &self,
        backend: &impl TrustBackend,
    ) -> Result<AuthenticatedTrustState, IdentityError> {
        let wire = self.read_bounded()?;
        let envelope = ProtectedEnvelopeV1::from_json_slice(&wire)?;
        if envelope.object_id() != self.object_id
            || envelope.purpose() != TRUST_STATE_PURPOSE
            || envelope.information_class() != TRUST_STATE_INFORMATION_CLASS
        {
            return Err(store_corruption_error("trust_state_envelope_binding"));
        }

        ensure_backend_available(backend)?;
        let key_ref = envelope.key_ref();
        let secret_ref = TrustBackendSecretRef::new(
            key_ref.trust_root_id(),
            key_ref.key_id(),
            key_ref.generation(),
            KeyPurpose::ProtectedEnvelopeRoot,
        )?;
        let master_secret = backend.open_protected_secret(secret_ref)?;
        let plaintext = open_envelope(
            &master_secret,
            &envelope,
            self.object_id,
            TRUST_STATE_PURPOSE,
            TRUST_STATE_INFORMATION_CLASS,
            key_ref,
        )?;
        let plaintext_bytes = plaintext.as_slice();
        validate_json_limits(
            plaintext_bytes,
            MAX_PROTECTED_ENVELOPE_WIRE_BYTES,
            MAX_JSON_DEPTH,
        )?;
        validate_protected_state_version(plaintext_bytes)?;
        let state: ProtectedTrustStateV1 = serde_json::from_slice(plaintext_bytes)
            .map_err(|_| store_corruption_error("protected_trust_state_json"))?;
        state.validate_schema_invariants()?;

        if state.trust_root_id() != key_ref.trust_root_id() {
            return Err(store_corruption_error("trust_state_root_binding"));
        }
        let active_envelope_root = state.active_key(KeyPurpose::ProtectedEnvelopeRoot)?;
        if active_envelope_root.key_id() != key_ref.key_id()
            || active_envelope_root.generation() != key_ref.generation()
        {
            return Err(store_corruption_error("trust_state_key_binding"));
        }

        let canonical_state = to_jcs_vec(&state).map_err(|_| {
            IdentityError::new(
                IdentityErrorCategory::ProtectedStorage,
                IdentityErrorCode::CanonicalizationFailed,
                Some("authenticated_trust_state_jcs"),
            )
        })?;
        let digest = Sha256::digest(&canonical_state);
        let mut digest_bytes = [0_u8; 32];
        digest_bytes.copy_from_slice(&digest);

        Ok(AuthenticatedTrustState {
            state,
            trust_state_digest: TrustStateDigest::from_bytes(digest_bytes),
        })
    }

    fn read_bounded(&self) -> Result<Vec<u8>, IdentityError> {
        read_bounded_path(&self.path)
    }

    fn atomic_replace(&self, bytes: &[u8]) -> Result<(), IdentityError> {
        atomic_replace_path(&self.path, bytes)
    }
}

fn protect_rotated_secret(
    backend: &impl TrustBackend,
    random: &mut impl SecureRandom,
    trust_root_id: crate::TrustRootId,
    key_id: KeyId,
    purpose: KeyPurpose,
    generation: u64,
    transitioned_at: EpochMillis,
) -> Result<KeyRecord, IdentityError> {
    let mut secret = Zeroizing::new([0_u8; ROTATION_SECRET_BYTES]);
    random.fill(&mut *secret)?;

    let record = match purpose {
        KeyPurpose::ProtectedEnvelopeRoot => KeyRecord::new_protected_envelope_root(
            key_id,
            trust_root_id,
            generation,
            KeyStatus::Active,
            transitioned_at,
            transitioned_at,
            None,
            None,
        )?,
        KeyPurpose::IdentityAssertionSigning | KeyPurpose::ProtectedAnchorSigning => {
            let signing_key = SigningKey::from_bytes(&secret);
            KeyRecord::new_ed25519(
                key_id,
                trust_root_id,
                purpose,
                generation,
                KeyStatus::Active,
                signing_key.verifying_key().to_bytes(),
                transitioned_at,
                transitioned_at,
                None,
                None,
            )?
        }
    };

    let secret_ref = TrustBackendSecretRef::new(trust_root_id, key_id, generation, purpose)?;
    backend.protect_secret(secret_ref, &SensitiveBytes::new(secret.to_vec()))?;
    Ok(record)
}

fn revoked_key_record(
    record: &KeyRecord,
    revoked_at: EpochMillis,
) -> Result<KeyRecord, IdentityError> {
    if matches!(record.status(), KeyStatus::Revoked) {
        return Err(IdentityError::new(
            IdentityErrorCategory::KeyState,
            IdentityErrorCode::KeyRevoked,
            Some("revoked_key_record"),
        ));
    }

    match record.purpose() {
        KeyPurpose::ProtectedEnvelopeRoot => KeyRecord::new_protected_envelope_root(
            record.key_id(),
            record.trust_root_id(),
            record.generation(),
            KeyStatus::Revoked,
            record.created_at(),
            record.activated_at(),
            record.retired_at(),
            Some(revoked_at),
        ),
        KeyPurpose::IdentityAssertionSigning | KeyPurpose::ProtectedAnchorSigning => {
            let public_key = record
                .ed25519_public_key()?
                .ok_or_else(|| revocation_error("revoked_signing_key_missing_public_material"))?;
            KeyRecord::new_ed25519(
                record.key_id(),
                record.trust_root_id(),
                record.purpose(),
                record.generation(),
                KeyStatus::Revoked,
                public_key,
                record.created_at(),
                record.activated_at(),
                record.retired_at(),
                Some(revoked_at),
            )
        }
    }
}

fn random_rotation_key_id(random: &mut impl SecureRandom) -> Result<KeyId, IdentityError> {
    let mut bytes = [0_u8; 16];
    random.fill(&mut bytes)?;
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    KeyId::from_uuid(Uuid::from_bytes(bytes))
}

fn rotation_error(context: &'static str) -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::KeyState,
        IdentityErrorCode::TrustSnapshotLifecycleInvalid,
        Some(context),
    )
}

fn revocation_error(context: &'static str) -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::KeyState,
        IdentityErrorCode::TrustSnapshotLifecycleInvalid,
        Some(context),
    )
}

#[derive(Deserialize)]
struct VersionProbe {
    version: SchemaVersion,
}

fn validate_protected_state_version(input: &[u8]) -> Result<(), IdentityError> {
    let probe: VersionProbe = serde_json::from_slice(input)
        .map_err(|_| store_corruption_error("protected_trust_state_version"))?;
    validate_ecr031_version(probe.version)
}

fn ensure_backend_available(backend: &impl TrustBackend) -> Result<(), IdentityError> {
    match backend.status()? {
        TrustBackendStatus::Available => Ok(()),
        TrustBackendStatus::Locked => Err(IdentityError::new(
            IdentityErrorCategory::TrustBackend,
            IdentityErrorCode::TrustRootLocked,
            Some("protected_trust_state_backend"),
        )),
        TrustBackendStatus::Unavailable => Err(IdentityError::new(
            IdentityErrorCategory::TrustBackend,
            IdentityErrorCode::TrustRootUnavailable,
            Some("protected_trust_state_backend"),
        )),
    }
}

fn path_is_regular_file_if_present(
    path: &Path,
    context: &'static str,
) -> Result<bool, IdentityError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.is_file() {
                return Err(store_corruption_error(context));
            }
            Ok(true)
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(_) => Err(store_io_error("trust_state_path_metadata")),
    }
}

fn read_bounded_path(path: &Path) -> Result<Vec<u8>, IdentityError> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            store_corruption_error("trust_state_store_missing")
        } else {
            store_io_error("trust_state_store_metadata")
        }
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(store_corruption_error("trust_state_store_file_type"));
    }
    let length = usize::try_from(metadata.len())
        .map_err(|_| store_corruption_error("trust_state_store_length"))?;
    if length == 0 || length > MAX_PROTECTED_ENVELOPE_WIRE_BYTES {
        return Err(IdentityError::new(
            IdentityErrorCategory::Corruption,
            IdentityErrorCode::WireLimitExceeded,
            Some("trust_state_store_wire"),
        ));
    }

    let file = File::open(path).map_err(|_| store_io_error("trust_state_store_open"))?;
    let mut wire = Vec::with_capacity(length);
    file.take((MAX_PROTECTED_ENVELOPE_WIRE_BYTES + 1) as u64)
        .read_to_end(&mut wire)
        .map_err(|_| store_io_error("trust_state_store_read"))?;
    if wire.is_empty() || wire.len() > MAX_PROTECTED_ENVELOPE_WIRE_BYTES {
        return Err(IdentityError::new(
            IdentityErrorCategory::Corruption,
            IdentityErrorCode::WireLimitExceeded,
            Some("trust_state_store_wire"),
        ));
    }
    Ok(wire)
}

fn atomic_replace_path(path: &Path, bytes: &[u8]) -> Result<(), IdentityError> {
    if bytes.is_empty() || bytes.len() > MAX_PROTECTED_ENVELOPE_WIRE_BYTES {
        return Err(IdentityError::new(
            IdentityErrorCategory::ProtectedStorage,
            IdentityErrorCode::WireLimitExceeded,
            Some("trust_state_store_atomic_write"),
        ));
    }
    let parent = path
        .parent()
        .ok_or_else(|| store_input_error("trust_state_store_parent"))?;
    fs::create_dir_all(parent).map_err(|_| store_io_error("trust_state_store_create_parent"))?;

    let temp_path = temp_path_for(path)?;
    match fs::symlink_metadata(&temp_path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.is_file() {
                return Err(store_corruption_error("trust_state_store_temp_file_type"));
            }
            fs::remove_file(&temp_path)
                .map_err(|_| store_io_error("trust_state_store_remove_stale_temp"))?;
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(_) => return Err(store_io_error("trust_state_store_temp_metadata")),
    }

    let write_result = (|| -> Result<(), IdentityError> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)
            .map_err(|_| store_io_error("trust_state_store_create_temp"))?;
        file.write_all(bytes)
            .map_err(|_| store_io_error("trust_state_store_write_temp"))?;
        file.sync_all()
            .map_err(|_| store_io_error("trust_state_store_flush_temp"))?;
        drop(file);

        fs::rename(&temp_path, path)
            .map_err(|_| store_io_error("trust_state_store_atomic_rename"))?;
        sync_parent_directory(parent)?;
        Ok(())
    })();

    if write_result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }
    write_result
}

fn temp_path_for(path: &Path) -> Result<PathBuf, IdentityError> {
    let file_name = path
        .file_name()
        .ok_or_else(|| store_input_error("trust_state_store_file_name"))?;
    let mut temp_name = OsString::from(".");
    temp_name.push(file_name);
    temp_name.push(".tmp");
    Ok(path.with_file_name(temp_name))
}

fn bootstrap_marker_path(path: &Path) -> Result<PathBuf, IdentityError> {
    let file_name = path
        .file_name()
        .ok_or_else(|| store_input_error("bootstrap_marker_file_name"))?;
    let mut marker_name = OsString::from(".");
    marker_name.push(file_name);
    marker_name.push(".bootstrap-in-progress");
    Ok(path.with_file_name(marker_name))
}

fn store_input_error(context: &'static str) -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::InvalidInput,
        IdentityErrorCode::ProtectedEnvelopeInvalid,
        Some(context),
    )
}

fn store_corruption_error(context: &'static str) -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::Corruption,
        IdentityErrorCode::ProtectedEnvelopeInvalid,
        Some(context),
    )
}

fn store_io_error(context: &'static str) -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::ProtectedStorage,
        IdentityErrorCode::BackendInvariantViolation,
        Some(context),
    )
}

#[cfg(unix)]
fn sync_parent_directory(parent: &Path) -> Result<(), IdentityError> {
    File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| store_io_error("trust_state_store_flush_parent"))
}

#[cfg(not(unix))]
fn sync_parent_directory(_parent: &Path) -> Result<(), IdentityError> {
    // The accepted v1 live durability oracle is macOS. Other platform backends
    // are not marked verified until their native store and replacement behavior
    // receive platform-specific evidence.
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, collections::HashMap, env, fs, path::PathBuf, process};

    use ecra_core::{EpochMillis, PrincipalId, to_jcs_vec};
    use ed25519_dalek::SigningKey;

    use super::{
        BootstrapStoreLocation, ProtectedTrustStateStore, TRUST_STATE_INFORMATION_CLASS,
        TRUST_STATE_PURPOSE, temp_path_for,
    };
    use crate::backend::{
        DeterministicSecureRandom, TrustBackend, TrustBackendCapabilities, TrustBackendKind,
        TrustBackendSecretRef, TrustBackendStatus,
    };
    use crate::bootstrap::ProtectedEnrollmentV1;
    use crate::envelope::protect_envelope;
    use crate::key::{KeyRecord, KeyStatus, ProtectedTrustStateV1};
    use crate::{
        EnrollmentId, EnvelopeKeyRef, IdentityError, IdentityErrorCategory, IdentityErrorCode,
        KeyId, KeyPurpose, ProtectedObjectId, SensitiveBytes, TrustRootId,
    };

    const ROOT: &str = "00000000-0000-0000-0000-000000000002";
    const KEY: &str = "00000000-0000-0000-0000-000000000011";
    const SIGNING_KEY: &str = "00000000-0000-0000-0000-000000000003";
    const OBJECT: &str = "00000000-0000-0000-0000-000000000010";
    const PRINCIPAL: &str = "00000000-0000-0000-0000-000000000004";
    const ENROLLMENT: &str = "00000000-0000-0000-0000-000000000030";

    struct TestBackend {
        secret_ref: TrustBackendSecretRef,
        secret: Vec<u8>,
        status: TrustBackendStatus,
    }

    impl TrustBackend for TestBackend {
        fn capabilities(&self) -> TrustBackendCapabilities {
            TrustBackendCapabilities::new(TrustBackendKind::MacosDataProtectionKeychain)
        }

        fn status(&self) -> Result<TrustBackendStatus, IdentityError> {
            Ok(self.status)
        }

        fn protect_secret(
            &self,
            secret_ref: TrustBackendSecretRef,
            secret: &SensitiveBytes,
        ) -> Result<(), IdentityError> {
            assert_eq!(secret_ref, self.secret_ref);
            assert_eq!(secret.as_slice(), self.secret.as_slice());
            Ok(())
        }

        fn open_protected_secret(
            &self,
            secret_ref: TrustBackendSecretRef,
        ) -> Result<SensitiveBytes, IdentityError> {
            assert_eq!(secret_ref, self.secret_ref);
            Ok(SensitiveBytes::new(self.secret.clone()))
        }

        fn delete_backend_material(
            &self,
            secret_ref: TrustBackendSecretRef,
        ) -> Result<(), IdentityError> {
            assert_eq!(secret_ref, self.secret_ref);
            Ok(())
        }
    }

    struct RotationBackend {
        secrets: RefCell<HashMap<TrustBackendSecretRef, Vec<u8>>>,
    }

    impl RotationBackend {
        fn new() -> Self {
            Self {
                secrets: RefCell::new(HashMap::new()),
            }
        }

        fn insert(&self, secret_ref: TrustBackendSecretRef, secret: Vec<u8>) {
            self.secrets.borrow_mut().insert(secret_ref, secret);
        }
    }

    impl TrustBackend for RotationBackend {
        fn capabilities(&self) -> TrustBackendCapabilities {
            TrustBackendCapabilities::new(TrustBackendKind::MacosDataProtectionKeychain)
        }

        fn status(&self) -> Result<TrustBackendStatus, IdentityError> {
            Ok(TrustBackendStatus::Available)
        }

        fn protect_secret(
            &self,
            secret_ref: TrustBackendSecretRef,
            secret: &SensitiveBytes,
        ) -> Result<(), IdentityError> {
            let mut secrets = self.secrets.borrow_mut();
            if secrets.contains_key(&secret_ref) {
                return Err(IdentityError::new(
                    IdentityErrorCategory::TrustBackend,
                    IdentityErrorCode::BackendInvariantViolation,
                    Some("rotation_test_duplicate_secret"),
                ));
            }
            secrets.insert(secret_ref, secret.as_slice().to_vec());
            Ok(())
        }

        fn open_protected_secret(
            &self,
            secret_ref: TrustBackendSecretRef,
        ) -> Result<SensitiveBytes, IdentityError> {
            self.secrets
                .borrow()
                .get(&secret_ref)
                .cloned()
                .map(SensitiveBytes::new)
                .ok_or_else(|| {
                    IdentityError::new(
                        IdentityErrorCategory::TrustBackend,
                        IdentityErrorCode::KeyNotFound,
                        Some("rotation_test_secret"),
                    )
                })
        }

        fn delete_backend_material(
            &self,
            secret_ref: TrustBackendSecretRef,
        ) -> Result<(), IdentityError> {
            self.secrets.borrow_mut().remove(&secret_ref);
            Ok(())
        }
    }

    fn timestamp(value: i64) -> EpochMillis {
        EpochMillis::new(value).unwrap()
    }

    fn root_id() -> TrustRootId {
        TrustRootId::parse_str(ROOT).unwrap()
    }

    fn key_id() -> KeyId {
        KeyId::parse_str(KEY).unwrap()
    }

    fn signing_key_id() -> KeyId {
        KeyId::parse_str(SIGNING_KEY).unwrap()
    }

    fn object_id() -> ProtectedObjectId {
        ProtectedObjectId::parse_str(OBJECT).unwrap()
    }

    fn key_ref() -> EnvelopeKeyRef {
        EnvelopeKeyRef::new(root_id(), key_id(), 1).unwrap()
    }

    fn backend() -> TestBackend {
        TestBackend {
            secret_ref: TrustBackendSecretRef::new(
                root_id(),
                key_id(),
                1,
                KeyPurpose::ProtectedEnvelopeRoot,
            )
            .unwrap(),
            secret: vec![0x42; 32],
            status: TrustBackendStatus::Available,
        }
    }

    fn rotation_backend() -> RotationBackend {
        let backend = RotationBackend::new();
        backend.insert(
            TrustBackendSecretRef::new(root_id(), key_id(), 1, KeyPurpose::ProtectedEnvelopeRoot)
                .unwrap(),
            vec![0x42; 32],
        );
        backend.insert(
            TrustBackendSecretRef::new(
                root_id(),
                signing_key_id(),
                1,
                KeyPurpose::IdentityAssertionSigning,
            )
            .unwrap(),
            vec![0x07; 32],
        );
        backend
    }

    fn state(generation: u64, updated_at: i64) -> ProtectedTrustStateV1 {
        let root_key = KeyRecord::new_protected_envelope_root(
            key_id(),
            root_id(),
            1,
            KeyStatus::Active,
            timestamp(900),
            timestamp(1_000),
            None,
            None,
        )
        .unwrap();
        ProtectedTrustStateV1::new(
            root_id(),
            ProtectedEnrollmentV1::new(
                EnrollmentId::parse_str(ENROLLMENT).unwrap(),
                PrincipalId::parse_str(PRINCIPAL).unwrap(),
            ),
            generation,
            vec![root_key],
            Default::default(),
            timestamp(updated_at),
        )
        .unwrap()
    }

    fn rotation_state() -> ProtectedTrustStateV1 {
        let root_key = KeyRecord::new_protected_envelope_root(
            key_id(),
            root_id(),
            1,
            KeyStatus::Active,
            timestamp(900),
            timestamp(1_000),
            None,
            None,
        )
        .unwrap();
        let signing_key = SigningKey::from_bytes(&[0x07; 32]);
        let signing_record = KeyRecord::new_ed25519(
            signing_key_id(),
            root_id(),
            KeyPurpose::IdentityAssertionSigning,
            1,
            KeyStatus::Active,
            signing_key.verifying_key().to_bytes(),
            timestamp(900),
            timestamp(1_000),
            None,
            None,
        )
        .unwrap();
        ProtectedTrustStateV1::new(
            root_id(),
            ProtectedEnrollmentV1::new(
                EnrollmentId::parse_str(ENROLLMENT).unwrap(),
                PrincipalId::parse_str(PRINCIPAL).unwrap(),
            ),
            1,
            vec![root_key, signing_record],
            Default::default(),
            timestamp(1_200),
        )
        .unwrap()
    }

    fn test_store(name: &str) -> (PathBuf, ProtectedTrustStateStore) {
        let directory =
            env::temp_dir().join(format!("ecra-identity-t041-{}-{name}", process::id()));
        let _ = fs::remove_dir_all(&directory);
        fs::create_dir_all(&directory).unwrap();
        let path = directory.join("protected-trust-state.json");
        let store = ProtectedTrustStateStore::new(path, object_id()).unwrap();
        (directory, store)
    }

    #[test]
    fn store_location_is_operational_metadata_only() {
        let (directory, store) = test_store("location");
        let location = BootstrapStoreLocation::new(store.path().to_path_buf()).unwrap();
        assert_eq!(location.path(), store.path());
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn publish_reopen_and_replace_keep_plaintext_out_of_store() {
        let (directory, store) = test_store("publish-reopen");
        let backend = backend();
        let mut random = DeterministicSecureRandom::new(vec![0x11; 24]);

        let first = state(1, 1_200);
        store.publish(&backend, &mut random, &first).unwrap();
        assert!(store.exists().unwrap());
        let disk = fs::read(store.path()).unwrap();
        let disk_text = String::from_utf8(disk).unwrap();
        assert!(!disk_text.contains("principal_id"));
        assert!(!disk_text.contains("ecra_local_installation_principal"));
        assert_eq!(store.open_authenticated(&backend).unwrap().state(), &first);

        let second = state(2, 1_300);
        store.publish(&backend, &mut random, &second).unwrap();
        assert_eq!(store.open_authenticated(&backend).unwrap().state(), &second);
        assert!(!temp_path_for(store.path()).unwrap().exists());

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn rotation_protects_next_signing_generation_and_retires_previous() {
        let (directory, store) = test_store("rotate-signing");
        let backend = rotation_backend();
        let initial = rotation_state();
        let mut publish_random = DeterministicSecureRandom::new(vec![0x11; 12]);
        store
            .publish(&backend, &mut publish_random, &initial)
            .unwrap();

        let mut rotation_random = DeterministicSecureRandom::new(vec![0x22; 60]);
        let authenticated = store
            .rotate_key(
                &backend,
                &mut rotation_random,
                KeyPurpose::IdentityAssertionSigning,
                timestamp(1_300),
            )
            .unwrap();
        let rotated = authenticated.state();
        assert_eq!(rotated.state_generation(), 2);
        let old = rotated
            .keys()
            .iter()
            .find(|key| key.key_id() == signing_key_id())
            .unwrap();
        assert_eq!(old.status(), KeyStatus::RetiredVerifyOrDecryptOnly);
        assert_eq!(old.retired_at(), Some(timestamp(1_300)));
        let active = rotated
            .active_key(KeyPurpose::IdentityAssertionSigning)
            .unwrap();
        assert_eq!(active.generation(), 2);
        assert_ne!(active.key_id(), signing_key_id());
        let new_secret_ref = TrustBackendSecretRef::new(
            root_id(),
            active.key_id(),
            2,
            KeyPurpose::IdentityAssertionSigning,
        )
        .unwrap();
        assert!(backend.secrets.borrow().contains_key(&new_secret_ref));
        assert_eq!(backend.secrets.borrow().len(), 3);

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn rotation_of_envelope_root_reencrypts_state_under_new_active_root() {
        let (directory, store) = test_store("rotate-envelope-root");
        let backend = rotation_backend();
        let initial = rotation_state();
        let mut publish_random = DeterministicSecureRandom::new(vec![0x31; 12]);
        store
            .publish(&backend, &mut publish_random, &initial)
            .unwrap();

        let mut rotation_random = DeterministicSecureRandom::new(vec![0x44; 60]);
        let authenticated = store
            .rotate_key(
                &backend,
                &mut rotation_random,
                KeyPurpose::ProtectedEnvelopeRoot,
                timestamp(1_300),
            )
            .unwrap();
        let rotated = authenticated.state();
        let old = rotated
            .keys()
            .iter()
            .find(|key| key.key_id() == key_id())
            .unwrap();
        assert_eq!(old.status(), KeyStatus::RetiredVerifyOrDecryptOnly);
        let active = rotated
            .active_key(KeyPurpose::ProtectedEnvelopeRoot)
            .unwrap();
        assert_eq!(active.generation(), 2);
        assert_ne!(active.key_id(), key_id());
        let new_secret_ref = TrustBackendSecretRef::new(
            root_id(),
            active.key_id(),
            2,
            KeyPurpose::ProtectedEnvelopeRoot,
        )
        .unwrap();
        assert!(backend.secrets.borrow().contains_key(&new_secret_ref));
        assert_eq!(
            store
                .open_authenticated(&backend)
                .unwrap()
                .state()
                .active_key(KeyPurpose::ProtectedEnvelopeRoot)
                .unwrap()
                .key_id(),
            active.key_id()
        );

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn rotation_rejects_non_monotonic_transition_time_before_creating_secret() {
        let (directory, store) = test_store("rotate-non-monotonic");
        let backend = rotation_backend();
        let initial = rotation_state();
        let mut publish_random = DeterministicSecureRandom::new(vec![0x51; 12]);
        store
            .publish(&backend, &mut publish_random, &initial)
            .unwrap();
        let before = backend.secrets.borrow().len();
        let mut rotation_random = DeterministicSecureRandom::new(vec![0x61; 60]);
        let error = store
            .rotate_key(
                &backend,
                &mut rotation_random,
                KeyPurpose::IdentityAssertionSigning,
                timestamp(1_199),
            )
            .unwrap_err();
        assert_eq!(
            error.code(),
            IdentityErrorCode::TrustSnapshotLifecycleInvalid
        );
        assert_eq!(backend.secrets.borrow().len(), before);

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn revocation_persists_authoritatively_without_deleting_backend_secret() {
        let (directory, store) = test_store("revoke-signing");
        let backend = rotation_backend();
        let initial = rotation_state();
        let signing_secret_ref = TrustBackendSecretRef::new(
            root_id(),
            signing_key_id(),
            1,
            KeyPurpose::IdentityAssertionSigning,
        )
        .unwrap();
        let mut publish_random = DeterministicSecureRandom::new(vec![0x71; 12]);
        store
            .publish(&backend, &mut publish_random, &initial)
            .unwrap();
        let mut revoke_random = DeterministicSecureRandom::new(vec![0x72; 12]);
        let authenticated = store
            .revoke_key(
                &backend,
                &mut revoke_random,
                signing_key_id(),
                timestamp(1_300),
            )
            .unwrap();
        let revoked = authenticated.state();
        assert_eq!(revoked.state_generation(), 2);
        assert!(revoked.revoked_key_ids().contains(&signing_key_id()));
        let record = revoked
            .keys()
            .iter()
            .find(|key| key.key_id() == signing_key_id())
            .unwrap();
        assert_eq!(record.status(), KeyStatus::Revoked);
        assert_eq!(record.revoked_at(), Some(timestamp(1_300)));
        assert_eq!(
            revoked
                .active_key(KeyPurpose::IdentityAssertionSigning)
                .unwrap_err()
                .code(),
            IdentityErrorCode::KeyNotActive
        );
        assert!(backend.secrets.borrow().contains_key(&signing_secret_ref));

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn retired_generation_can_be_revoked_without_reactivating_it() {
        let (directory, store) = test_store("revoke-retired");
        let backend = rotation_backend();
        let initial = rotation_state();
        let mut publish_random = DeterministicSecureRandom::new(vec![0x81; 12]);
        store
            .publish(&backend, &mut publish_random, &initial)
            .unwrap();
        let mut rotation_random = DeterministicSecureRandom::new(vec![0x82; 60]);
        let rotated = store
            .rotate_key(
                &backend,
                &mut rotation_random,
                KeyPurpose::IdentityAssertionSigning,
                timestamp(1_300),
            )
            .unwrap();
        let active_key_id = rotated
            .state()
            .active_key(KeyPurpose::IdentityAssertionSigning)
            .unwrap()
            .key_id();
        let mut revoke_random = DeterministicSecureRandom::new(vec![0x83; 12]);
        let revoked = store
            .revoke_key(
                &backend,
                &mut revoke_random,
                signing_key_id(),
                timestamp(1_400),
            )
            .unwrap();
        let state = revoked.state();
        assert_eq!(state.state_generation(), 3);
        assert_eq!(
            state
                .active_key(KeyPurpose::IdentityAssertionSigning)
                .unwrap()
                .key_id(),
            active_key_id
        );
        let old = state
            .keys()
            .iter()
            .find(|key| key.key_id() == signing_key_id())
            .unwrap();
        assert_eq!(old.status(), KeyStatus::Revoked);
        assert_eq!(old.retired_at(), Some(timestamp(1_300)));
        assert_eq!(old.revoked_at(), Some(timestamp(1_400)));
        assert!(state.revoked_key_ids().contains(&signing_key_id()));

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn active_envelope_root_requires_rotation_before_revocation() {
        let (directory, store) = test_store("revoke-active-root");
        let backend = rotation_backend();
        let initial = rotation_state();
        let before = backend.secrets.borrow().len();
        let mut publish_random = DeterministicSecureRandom::new(vec![0x91; 12]);
        store
            .publish(&backend, &mut publish_random, &initial)
            .unwrap();
        let mut revoke_random = DeterministicSecureRandom::new(Vec::new());
        let error = store
            .revoke_key(&backend, &mut revoke_random, key_id(), timestamp(1_300))
            .unwrap_err();
        assert_eq!(error.code(), IdentityErrorCode::KeyNotActive);
        assert_eq!(backend.secrets.borrow().len(), before);
        assert_eq!(
            store
                .open_authenticated(&backend)
                .unwrap()
                .state()
                .active_key(KeyPurpose::ProtectedEnvelopeRoot)
                .unwrap()
                .key_id(),
            key_id()
        );

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn corruption_and_missing_state_fail_closed() {
        let (directory, store) = test_store("corruption");
        let backend = backend();
        assert!(!store.exists().unwrap());
        let missing = store.open_authenticated(&backend).unwrap_err();
        assert_eq!(missing.code(), IdentityErrorCode::ProtectedEnvelopeInvalid);

        fs::write(store.path(), b"{\"version\":").unwrap();
        let corrupted = store.open_authenticated(&backend).unwrap_err();
        assert_eq!(
            corrupted.code(),
            IdentityErrorCode::ProtectedEnvelopeInvalid
        );

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn authenticated_inner_future_version_fails_closed() {
        let (directory, store) = test_store("future-inner-version");
        let backend = backend();
        let mut value = serde_json::to_value(state(1, 1_200)).unwrap();
        value["version"]["major"] = 2_u64.into();
        let future_plaintext = SensitiveBytes::new(to_jcs_vec(&value).unwrap());
        let mut random = DeterministicSecureRandom::new(vec![0x33; 12]);
        let master_secret = SensitiveBytes::new(vec![0x42; 32]);
        let envelope = protect_envelope(
            &master_secret,
            &mut random,
            object_id(),
            TRUST_STATE_PURPOSE,
            TRUST_STATE_INFORMATION_CLASS,
            key_ref(),
            &future_plaintext,
        )
        .unwrap();
        store
            .atomic_replace(&to_jcs_vec(&envelope).unwrap())
            .unwrap();

        let error = store.open_authenticated(&backend).unwrap_err();
        assert_eq!(error.code(), IdentityErrorCode::UnsupportedVersion);

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn bootstrap_marker_is_durable_and_non_authoritative() {
        let (directory, store) = test_store("bootstrap-marker");
        assert!(!ProtectedTrustStateStore::bootstrap_marker_exists(store.path()).unwrap());
        ProtectedTrustStateStore::write_bootstrap_marker(store.path()).unwrap();
        assert!(ProtectedTrustStateStore::bootstrap_marker_exists(store.path()).unwrap());
        assert!(!store.exists().unwrap());
        ProtectedTrustStateStore::clear_bootstrap_marker(store.path()).unwrap();
        assert!(!ProtectedTrustStateStore::bootstrap_marker_exists(store.path()).unwrap());
        fs::remove_dir_all(directory).unwrap();
    }
}
