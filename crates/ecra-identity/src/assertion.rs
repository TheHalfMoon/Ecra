use ecra_core::to_jcs_vec;
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::{IdentityError, IdentityErrorCategory, IdentityErrorCode};

pub const MAX_IDENTITY_ASSERTION_WIRE_BYTES: usize = 8 * 1024;
pub const MAX_JSON_DEPTH: usize = 16;
pub const MAX_ASSERTION_ATTRIBUTES: usize = 32;

pub const IDENTITY_ASSERTION_SIGNING_DOMAIN: &[u8] = b"ecra.identity-assertion.v1\n";
pub const IDENTITY_ASSERTION_DIGEST_DOMAIN: &[u8] = b"ecra.identity-assertion-digest.v1\n";

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
