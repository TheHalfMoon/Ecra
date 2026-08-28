use serde::{Deserialize, Serialize};

use crate::error::{IdentityError, IdentityErrorCategory, IdentityErrorCode};
use crate::{KeyId, KeyPurpose, MAX_I_JSON_U64, SensitiveBytes, TrustRootId};

pub trait SecureRandom {
    fn fill(&mut self, destination: &mut [u8]) -> Result<(), IdentityError>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SystemSecureRandom;

impl SecureRandom for SystemSecureRandom {
    fn fill(&mut self, destination: &mut [u8]) -> Result<(), IdentityError> {
        getrandom::fill(destination).map_err(|_| secure_random_failure())
    }
}

fn secure_random_failure() -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::PlatformUnavailable,
        IdentityErrorCode::BackendInvariantViolation,
        Some("secure_random"),
    )
}

#[cfg(test)]
#[derive(Debug)]
pub(crate) struct DeterministicSecureRandom {
    bytes: Vec<u8>,
    offset: usize,
}

#[cfg(test)]
impl DeterministicSecureRandom {
    pub(crate) fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, offset: 0 }
    }
}

#[cfg(test)]
impl SecureRandom for DeterministicSecureRandom {
    fn fill(&mut self, destination: &mut [u8]) -> Result<(), IdentityError> {
        let end = self
            .offset
            .checked_add(destination.len())
            .ok_or_else(secure_random_failure)?;
        let source = self
            .bytes
            .get(self.offset..end)
            .ok_or_else(secure_random_failure)?;
        destination.copy_from_slice(source);
        self.offset = end;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrustBackendKind {
    #[serde(rename = "macos_data_protection_keychain")]
    MacosDataProtectionKeychain,
    #[serde(rename = "windows_dpapi")]
    WindowsDpapi,
    #[serde(rename = "linux_secret_service")]
    LinuxSecretService,
}

/// Select the only production-native backend candidate for this compilation
/// target. The choice accepts no caller input, configuration string, path or
/// environment value, so plaintext/file/memory substitutes cannot enter the
/// production selection path.
pub fn production_trust_backend_kind() -> Result<TrustBackendKind, IdentityError> {
    #[cfg(target_os = "macos")]
    {
        Ok(TrustBackendKind::MacosDataProtectionKeychain)
    }
    #[cfg(target_os = "windows")]
    {
        Ok(TrustBackendKind::WindowsDpapi)
    }
    #[cfg(target_os = "linux")]
    {
        Ok(TrustBackendKind::LinuxSecretService)
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        Err(IdentityError::new(
            IdentityErrorCategory::PlatformUnavailable,
            IdentityErrorCode::BackendUnsupported,
            Some("production_trust_backend"),
        ))
    }
}

/// Marker for the isolated test-only backend family used by later lifecycle
/// fixtures. It cannot exist in a production build or production selector.
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TestTrustBackendKind {
    InMemory,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrustBackendCapabilities {
    backend_kind: TrustBackendKind,
    user_scoped: bool,
    machine_bound: bool,
    hardware_backed_private_operations: bool,
    non_exportable_private_key: bool,
    user_presence_gate: bool,
    biometric_gate: bool,
    locked_state_observable: bool,
    synchronizing_store: bool,
}

impl TrustBackendCapabilities {
    #[allow(dead_code)]
    pub(crate) const fn new(backend_kind: TrustBackendKind) -> Self {
        Self {
            backend_kind,
            user_scoped: false,
            machine_bound: false,
            hardware_backed_private_operations: false,
            non_exportable_private_key: false,
            user_presence_gate: false,
            biometric_gate: false,
            locked_state_observable: false,
            synchronizing_store: false,
        }
    }

    #[allow(dead_code)]
    pub(crate) const fn with_user_scoped(mut self, value: bool) -> Self {
        self.user_scoped = value;
        self
    }

    #[allow(dead_code)]
    pub(crate) const fn with_machine_bound(mut self, value: bool) -> Self {
        self.machine_bound = value;
        self
    }

    #[allow(dead_code)]
    pub(crate) const fn with_hardware_backed_private_operations(mut self, value: bool) -> Self {
        self.hardware_backed_private_operations = value;
        self
    }

    #[allow(dead_code)]
    pub(crate) const fn with_non_exportable_private_key(mut self, value: bool) -> Self {
        self.non_exportable_private_key = value;
        self
    }

    #[allow(dead_code)]
    pub(crate) const fn with_user_presence_gate(mut self, value: bool) -> Self {
        self.user_presence_gate = value;
        self
    }

    #[allow(dead_code)]
    pub(crate) const fn with_biometric_gate(mut self, value: bool) -> Self {
        self.biometric_gate = value;
        self
    }

    #[allow(dead_code)]
    pub(crate) const fn with_locked_state_observable(mut self, value: bool) -> Self {
        self.locked_state_observable = value;
        self
    }

    #[allow(dead_code)]
    pub(crate) const fn with_synchronizing_store(mut self, value: bool) -> Self {
        self.synchronizing_store = value;
        self
    }

    #[must_use]
    pub const fn backend_kind(self) -> TrustBackendKind {
        self.backend_kind
    }

    #[must_use]
    pub const fn user_scoped(self) -> bool {
        self.user_scoped
    }

    #[must_use]
    pub const fn machine_bound(self) -> bool {
        self.machine_bound
    }

    #[must_use]
    pub const fn hardware_backed_private_operations(self) -> bool {
        self.hardware_backed_private_operations
    }

    #[must_use]
    pub const fn non_exportable_private_key(self) -> bool {
        self.non_exportable_private_key
    }

    #[must_use]
    pub const fn user_presence_gate(self) -> bool {
        self.user_presence_gate
    }

    #[must_use]
    pub const fn biometric_gate(self) -> bool {
        self.biometric_gate
    }

    #[must_use]
    pub const fn locked_state_observable(self) -> bool {
        self.locked_state_observable
    }

    #[must_use]
    pub const fn synchronizing_store(self) -> bool {
        self.synchronizing_store
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TrustBackendStatus {
    Available,
    Locked,
    Unavailable,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct TrustBackendSecretRef {
    trust_root_id: TrustRootId,
    key_id: KeyId,
    generation: u64,
    purpose: KeyPurpose,
}

impl TrustBackendSecretRef {
    #[allow(dead_code)]
    pub(crate) fn new(
        trust_root_id: TrustRootId,
        key_id: KeyId,
        generation: u64,
        purpose: KeyPurpose,
    ) -> Result<Self, IdentityError> {
        if generation == 0 || generation > MAX_I_JSON_U64 {
            return Err(IdentityError::new(
                IdentityErrorCategory::KeyState,
                IdentityErrorCode::BackendInvariantViolation,
                Some("backend_secret_generation"),
            ));
        }
        Ok(Self {
            trust_root_id,
            key_id,
            generation,
            purpose,
        })
    }

    #[allow(dead_code)]
    pub(crate) const fn trust_root_id(self) -> TrustRootId {
        self.trust_root_id
    }

    #[allow(dead_code)]
    pub(crate) const fn key_id(self) -> KeyId {
        self.key_id
    }

    #[allow(dead_code)]
    pub(crate) const fn generation(self) -> u64 {
        self.generation
    }

    #[allow(dead_code)]
    pub(crate) const fn purpose(self) -> KeyPurpose {
        self.purpose
    }
}

#[allow(dead_code)]
pub(crate) trait TrustBackend {
    fn capabilities(&self) -> TrustBackendCapabilities;
    fn status(&self) -> Result<TrustBackendStatus, IdentityError>;
    fn protect_secret(
        &self,
        secret_ref: TrustBackendSecretRef,
        secret: &SensitiveBytes,
    ) -> Result<(), IdentityError>;
    fn open_protected_secret(
        &self,
        secret_ref: TrustBackendSecretRef,
    ) -> Result<SensitiveBytes, IdentityError>;
    fn delete_backend_material(
        &self,
        secret_ref: TrustBackendSecretRef,
    ) -> Result<(), IdentityError>;
}

#[cfg(test)]
mod tests {
    use super::{
        DeterministicSecureRandom, SecureRandom, SystemSecureRandom, TrustBackendCapabilities,
        TrustBackendKind, TrustBackendSecretRef, production_trust_backend_kind,
    };
    use crate::error::{IdentityErrorCategory, IdentityErrorCode};
    use crate::{KeyId, KeyPurpose, TrustRootId};

    #[test]
    fn system_secure_random_fills_requested_bytes() {
        let mut random = SystemSecureRandom;
        let mut bytes = [0_u8; 32];
        random
            .fill(&mut bytes)
            .expect("system CSPRNG must be available on the trusted test runner");
    }

    #[test]
    fn deterministic_secure_random_is_exact_and_bounded() {
        let mut random = DeterministicSecureRandom::new(vec![1, 2, 3, 4, 5, 6]);
        let mut first = [0_u8; 4];
        let mut second = [0_u8; 2];
        random.fill(&mut first).unwrap();
        random.fill(&mut second).unwrap();
        assert_eq!(first, [1, 2, 3, 4]);
        assert_eq!(second, [5, 6]);

        let mut exhausted = [0_u8; 1];
        let error = random.fill(&mut exhausted).unwrap_err();
        assert_eq!(error.category(), IdentityErrorCategory::PlatformUnavailable);
        assert_eq!(error.code(), IdentityErrorCode::BackendInvariantViolation);
    }

    #[test]
    fn deterministic_provider_is_compiled_only_for_tests() {
        let source = include_str!("backend.rs");
        let production = source.split("#[cfg(test)]").next().unwrap();
        assert!(!production.contains("DeterministicSecureRandom"));
    }

    #[test]
    fn backend_capabilities_start_from_fail_closed_assurance() {
        let baseline = TrustBackendCapabilities::new(TrustBackendKind::MacosDataProtectionKeychain);
        assert_eq!(
            baseline.backend_kind(),
            TrustBackendKind::MacosDataProtectionKeychain
        );
        assert!(!baseline.user_scoped());
        assert!(!baseline.machine_bound());
        assert!(!baseline.hardware_backed_private_operations());
        assert!(!baseline.non_exportable_private_key());
        assert!(!baseline.user_presence_gate());
        assert!(!baseline.biometric_gate());
        assert!(!baseline.locked_state_observable());
        assert!(!baseline.synchronizing_store());

        let evidenced = baseline
            .with_user_scoped(true)
            .with_locked_state_observable(true);
        assert!(evidenced.user_scoped());
        assert!(evidenced.locked_state_observable());
        assert!(!evidenced.hardware_backed_private_operations());
        assert!(!evidenced.non_exportable_private_key());
    }

    #[test]
    fn backend_secret_reference_is_typed_and_generation_bounded() {
        let trust_root_id = TrustRootId::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
        let key_id = KeyId::parse_str("00000000-0000-0000-0000-000000000011").unwrap();
        let secret_ref =
            TrustBackendSecretRef::new(trust_root_id, key_id, 1, KeyPurpose::ProtectedEnvelopeRoot)
                .unwrap();
        assert_eq!(secret_ref.trust_root_id(), trust_root_id);
        assert_eq!(secret_ref.key_id(), key_id);
        assert_eq!(secret_ref.generation(), 1);
        assert_eq!(secret_ref.purpose(), KeyPurpose::ProtectedEnvelopeRoot);

        assert!(
            TrustBackendSecretRef::new(
                trust_root_id,
                key_id,
                0,
                KeyPurpose::ProtectedEnvelopeRoot,
            )
            .is_err()
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn production_selection_is_macos_keychain_on_macos() {
        assert_eq!(
            production_trust_backend_kind().unwrap(),
            TrustBackendKind::MacosDataProtectionKeychain
        );
    }
}
