use crate::{IdentityError, IdentityErrorCategory, IdentityErrorCode};

/// ECR-031 v1 intentionally ships no Linux Secret Service backend until the
/// exact dependency/live acceptance path is implemented and verified. Callers
/// must fail closed; no plaintext/file/environment/memory substitute exists,
/// and future lookup attributes may contain only non-sensitive opaque metadata.
pub(crate) fn unsupported_backend() -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::PlatformUnavailable,
        IdentityErrorCode::BackendUnsupported,
        Some("linux_secret_service_unverified"),
    )
}

#[cfg(test)]
mod tests {
    use super::unsupported_backend;
    use crate::IdentityErrorCode;

    #[test]
    fn linux_v1_backend_is_explicitly_unsupported_until_native_acceptance() {
        assert_eq!(unsupported_backend().code(), IdentityErrorCode::BackendUnsupported);
    }
}
