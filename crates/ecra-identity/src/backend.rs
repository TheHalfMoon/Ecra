use serde::{Deserialize, Serialize};

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
