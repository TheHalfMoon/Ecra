use ecra_core::to_jcs_vec;
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::{IdentityError, IdentityErrorCategory, IdentityErrorCode};

pub const PROTECTED_ANCHOR_DOMAIN: &[u8] = b"ecra.protected-anchor.v1\n";

pub fn canonical_protected_anchor_input<T>(payload: &T) -> Result<Vec<u8>, IdentityError>
where
    T: Serialize,
{
    let canonical = to_jcs_vec(payload).map_err(|_| {
        IdentityError::new(
            IdentityErrorCategory::InvalidInput,
            IdentityErrorCode::CanonicalizationFailed,
            Some("protected_anchor_payload"),
        )
    })?;
    let mut output = Vec::with_capacity(PROTECTED_ANCHOR_DOMAIN.len() + canonical.len());
    output.extend_from_slice(PROTECTED_ANCHOR_DOMAIN);
    output.extend_from_slice(&canonical);
    Ok(output)
}

pub fn protected_anchor_input_digest_bytes<T>(payload: &T) -> Result<[u8; 32], IdentityError>
where
    T: Serialize,
{
    let input = canonical_protected_anchor_input(payload)?;
    let digest = Sha256::digest(input);
    let mut output = [0u8; 32];
    output.copy_from_slice(&digest);
    Ok(output)
}
