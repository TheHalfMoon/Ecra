use crate::{IdentityError, IdentityErrorCategory, IdentityErrorCode};

/// ECR-031 v1 intentionally ships no Windows secret backend until the exact
/// DPAPI dependency/native acceptance path is implemented and verified on a
/// trusted Windows runner. Callers must fail closed; no file, environment,
/// memory, cross-machine, or hardware-signing substitute is permitted.
pub(crate) fn unsupported_backend() -> IdentityError {
    IdentityError::new(
        IdentityErrorCategory::PlatformUnavailable,
        IdentityErrorCode::BackendUnsupported,
        Some("windows_dpapi_unverified"),
    )
}

#[cfg(test)]
mod tests {
    use super::unsupported_backend;
    use crate::IdentityErrorCode;

    #[test]
    fn windows_v1_backend_is_explicitly_unsupported_until_native_acceptance() {
        assert_eq!(
            unsupported_backend().code(),
            IdentityErrorCode::BackendUnsupported
        );
    }
}
