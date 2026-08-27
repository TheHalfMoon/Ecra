use serde::Serialize;

use crate::error::DomainError;

/// Serialize a value using RFC 8785 JSON Canonicalization Scheme (JCS).
///
/// This function performs no I/O. Callers that use canonical bytes as a
/// security binding must also use the contract-defined domain separator.
pub fn to_jcs_vec<T>(value: &T) -> Result<Vec<u8>, DomainError>
where
    T: Serialize + ?Sized,
{
    serde_jcs::to_vec(value).map_err(|error| DomainError::Canonicalization(error.to_string()))
}
