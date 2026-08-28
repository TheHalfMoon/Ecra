use serde::{Deserialize, Serialize};

use crate::error::{IdentityError, IdentityErrorCategory, IdentityErrorCode};

/// Explicit randomness boundary for identity/bootstrap cryptographic material.
///
/// Production callers use [`SystemSecureRandom`]. Deterministic material is
/// available only to crate-internal tests and cannot be selected by a
/// production configuration path.
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

/// Backend identity is descriptive evidence about the selected native custody
/// implementation. It is not an authorization grant and does not imply that a
/// backend is VERIFIED on the current platform.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrustBackendKind {
    #[serde(rename = "macos_data_protection_keychain")]
    MacosDataProtectionKeychain,
    #[serde(rename = "windows_dpapi")]
    WindowsDpapi,
    #[serde(rename = "linux_secret_service")]
    LinuxSecretService,
}

#[cfg(test)]
mod tests {
    use super::{DeterministicSecureRandom, SecureRandom, SystemSecureRandom};
    use crate::error::{IdentityErrorCategory, IdentityErrorCode};

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
        assert_eq!(error.safe_context(), Some("secure_random"));
    }

    #[test]
    fn deterministic_provider_is_compiled_only_for_tests() {
        let source = include_str!("backend.rs");
        let production = source.split("#[cfg(test)]").next().unwrap();
        assert!(!production.contains("DeterministicSecureRandom"));
        assert!(source.contains(
            "#[cfg(test)]\n#[derive(Debug)]\npub(crate) struct DeterministicSecureRandom"
        ));
    }
}
