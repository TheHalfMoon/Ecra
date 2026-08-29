use security_framework::{
    Error as SecurityFrameworkError,
    passwords::{
        PasswordOptions, delete_generic_password_options, generic_password,
        set_generic_password_options,
    },
};
use zeroize::Zeroizing;

use crate::backend::{
    TrustBackend, TrustBackendCapabilities, TrustBackendKind, TrustBackendSecretRef,
    TrustBackendStatus,
};
use crate::{IdentityError, IdentityErrorCategory, IdentityErrorCode, KeyPurpose, SensitiveBytes};

const ECRA_IDENTITY_KEYCHAIN_SERVICE: &str = "dev.ecra.identity.v1";
const STATUS_PROBE_ACCOUNT: &str = "__ecra_data_protection_keychain_status_probe_v1__";

// Security.framework OSStatus values. They are used only to normalize platform
// failures into Ecra's closed redacted error vocabulary; raw native errors are
// never retained or rendered.
const ERR_SEC_ITEM_NOT_FOUND: i32 = -25_300;
const ERR_SEC_NOT_AVAILABLE: i32 = -25_291;
const ERR_SEC_AUTH_FAILED: i32 = -25_293;
const ERR_SEC_INTERACTION_NOT_ALLOWED: i32 = -25_308;
const ERR_SEC_MISSING_ENTITLEMENT: i32 = -34_018;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct MacosDataProtectionKeychainBackend;

impl MacosDataProtectionKeychainBackend {
    fn password_options(secret_ref: TrustBackendSecretRef) -> PasswordOptions {
        let account = secret_account(secret_ref);
        let mut options = PasswordOptions::new_generic_password(ECRA_IDENTITY_KEYCHAIN_SERVICE, &account);
        options.use_protected_keychain();
        options.set_access_synchronized(Some(false));
        options
    }

    fn status_options() -> PasswordOptions {
        let mut options = PasswordOptions::new_generic_password(
            ECRA_IDENTITY_KEYCHAIN_SERVICE,
            STATUS_PROBE_ACCOUNT,
        );
        options.use_protected_keychain();
        options.set_access_synchronized(Some(false));
        options
    }
}

impl TrustBackend for MacosDataProtectionKeychainBackend {
    fn capabilities(&self) -> TrustBackendCapabilities {
        TrustBackendCapabilities::new(TrustBackendKind::MacosDataProtectionKeychain)
            .with_user_scoped(true)
            .with_locked_state_observable(true)
    }

    fn status(&self) -> Result<TrustBackendStatus, IdentityError> {
        match generic_password(Self::status_options()) {
            Ok(bytes) => {
                drop(Zeroizing::new(bytes));
                Ok(TrustBackendStatus::Available)
            }
            Err(error) if error.code() == ERR_SEC_ITEM_NOT_FOUND => Ok(TrustBackendStatus::Available),
            Err(error)
                if matches!(
                    error.code(),
                    ERR_SEC_AUTH_FAILED | ERR_SEC_INTERACTION_NOT_ALLOWED
                ) =>
            {
                Ok(TrustBackendStatus::Locked)
            }
            Err(error)
                if matches!(
                    error.code(),
                    ERR_SEC_NOT_AVAILABLE | ERR_SEC_MISSING_ENTITLEMENT
                ) =>
            {
                Ok(TrustBackendStatus::Unavailable)
            }
            Err(error) => Err(normalize_keychain_error(error, "macos_keychain_status")),
        }
    }

    fn protect_secret(
        &self,
        secret_ref: TrustBackendSecretRef,
        secret: &SensitiveBytes,
    ) -> Result<(), IdentityError> {
        if secret.is_empty() {
            return Err(IdentityError::new(
                IdentityErrorCategory::TrustBackend,
                IdentityErrorCode::BackendInvariantViolation,
                Some("macos_keychain_empty_secret"),
            ));
        }
        set_generic_password_options(secret.as_slice(), Self::password_options(secret_ref))
            .map_err(|error| normalize_keychain_error(error, "macos_keychain_store"))
    }

    fn open_protected_secret(
        &self,
        secret_ref: TrustBackendSecretRef,
    ) -> Result<SensitiveBytes, IdentityError> {
        generic_password(Self::password_options(secret_ref))
            .map(SensitiveBytes::new)
            .map_err(|error| normalize_keychain_error(error, "macos_keychain_open"))
    }

    fn delete_backend_material(
        &self,
        secret_ref: TrustBackendSecretRef,
    ) -> Result<(), IdentityError> {
        delete_generic_password_options(Self::password_options(secret_ref))
            .map_err(|error| normalize_keychain_error(error, "macos_keychain_delete"))
    }
}

fn secret_account(secret_ref: TrustBackendSecretRef) -> String {
    format!(
        "{}:{}:{}:{}",
        secret_ref.trust_root_id(),
        secret_ref.key_id(),
        secret_ref.generation(),
        key_purpose_token(secret_ref.purpose())
    )
}

const fn key_purpose_token(purpose: KeyPurpose) -> &'static str {
    match purpose {
        KeyPurpose::IdentityAssertionSigning => "identity_assertion_signing",
        KeyPurpose::ProtectedEnvelopeRoot => "protected_envelope_root",
        KeyPurpose::ProtectedAnchorSigning => "protected_anchor_signing",
    }
}

fn normalize_keychain_error(
    error: SecurityFrameworkError,
    context: &'static str,
) -> IdentityError {
    match error.code() {
        ERR_SEC_ITEM_NOT_FOUND => IdentityError::new(
            IdentityErrorCategory::TrustBackend,
            IdentityErrorCode::KeyNotFound,
            Some(context),
        ),
        ERR_SEC_AUTH_FAILED | ERR_SEC_INTERACTION_NOT_ALLOWED => IdentityError::new(
            IdentityErrorCategory::TrustBackend,
            IdentityErrorCode::TrustRootLocked,
            Some(context),
        ),
        ERR_SEC_NOT_AVAILABLE | ERR_SEC_MISSING_ENTITLEMENT => IdentityError::new(
            IdentityErrorCategory::PlatformUnavailable,
            IdentityErrorCode::TrustRootUnavailable,
            Some(context),
        ),
        _ => IdentityError::new(
            IdentityErrorCategory::TrustBackend,
            IdentityErrorCode::BackendInvariantViolation,
            Some(context),
        ),
    }
}

#[cfg(test)]
mod tests {
    use std::process;

    use super::MacosDataProtectionKeychainBackend;
    use crate::backend::{TrustBackend, TrustBackendSecretRef, TrustBackendStatus};
    use crate::{IdentityErrorCode, KeyId, KeyPurpose, SensitiveBytes, TrustRootId};

    fn typed_ids(slot: u16) -> (TrustRootId, KeyId) {
        let suffix = format!("{:08x}{slot:04x}", process::id());
        let trust_root = TrustRootId::parse_str(&format!(
            "00000000-0000-4000-8000-{suffix}"
        ))
        .unwrap();
        let key_suffix = format!("{:08x}{:04x}", process::id(), slot.wrapping_add(0x100));
        let key = KeyId::parse_str(&format!("00000000-0000-4000-8000-{key_suffix}"))
            .unwrap();
        (trust_root, key)
    }

    fn secret_ref(slot: u16, purpose: KeyPurpose) -> TrustBackendSecretRef {
        let (trust_root_id, key_id) = typed_ids(slot);
        TrustBackendSecretRef::new(trust_root_id, key_id, 1, purpose).unwrap()
    }

    #[test]
    fn capabilities_match_portable_v1_macos_assurance() {
        let backend = MacosDataProtectionKeychainBackend;
        let capabilities = backend.capabilities();
        assert!(capabilities.user_scoped());
        assert!(capabilities.locked_state_observable());
        assert!(!capabilities.machine_bound());
        assert!(!capabilities.hardware_backed_private_operations());
        assert!(!capabilities.non_exportable_private_key());
        assert!(!capabilities.user_presence_gate());
        assert!(!capabilities.biometric_gate());
        assert!(!capabilities.synchronizing_store());
    }

    #[test]
    fn data_protection_keychain_roundtrips_all_v1_secret_purposes() {
        let backend = MacosDataProtectionKeychainBackend;
        assert_eq!(backend.status().unwrap(), TrustBackendStatus::Available);

        let cases = [
            (1, KeyPurpose::ProtectedEnvelopeRoot, vec![0x11; 32]),
            (2, KeyPurpose::IdentityAssertionSigning, vec![0x22; 32]),
            (3, KeyPurpose::ProtectedAnchorSigning, vec![0x33; 32]),
        ];

        for (slot, purpose, expected) in cases {
            let reference = secret_ref(slot, purpose);
            let _ = backend.delete_backend_material(reference);
            backend
                .protect_secret(reference, &SensitiveBytes::new(expected.clone()))
                .unwrap();
            let opened = backend.open_protected_secret(reference).unwrap();
            assert_eq!(opened.as_slice(), expected.as_slice());
            backend.delete_backend_material(reference).unwrap();
            let error = backend.open_protected_secret(reference).unwrap_err();
            assert_eq!(error.code(), IdentityErrorCode::KeyNotFound);
        }
    }
}
