use serde::{Deserialize, Deserializer, Serialize, de};
use sha2::{Digest, Sha256};

use crate::error::DomainError;

const SHA256_HEX_LEN: usize = 64;
const HEX: &[u8; 16] = b"0123456789abcdef";

fn is_lower_hex(value: &str) -> bool {
    !value.is_empty()
        && value.len().is_multiple_of(2)
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn encode_lower_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}

/// Generic content checksum metadata. This type does not imply authenticity.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContentDigest {
    algorithm: String,
    hex: String,
}

impl ContentDigest {
    pub fn new(algorithm: impl Into<String>, hex: impl Into<String>) -> Result<Self, DomainError> {
        let algorithm = algorithm.into();
        let hex = hex.into();
        if algorithm.is_empty()
            || !algorithm
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
        {
            return Err(DomainError::InvalidContentDigest(
                "algorithm must be a non-empty ASCII token".to_owned(),
            ));
        }
        if !is_lower_hex(&hex) {
            return Err(DomainError::InvalidContentDigest(
                "digest must be non-empty even-length lowercase hexadecimal".to_owned(),
            ));
        }
        Ok(Self { algorithm, hex })
    }

    #[must_use]
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }

    #[must_use]
    pub fn hex(&self) -> &str {
        &self.hex
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ContentDigestWire {
    algorithm: String,
    hex: String,
}

impl<'de> Deserialize<'de> for ContentDigest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ContentDigestWire::deserialize(deserializer)?;
        Self::new(wire.algorithm, wire.hex).map_err(de::Error::custom)
    }
}

/// Algorithms permitted for security-binding digests in ECR-001 v1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityDigestAlgorithm {
    Sha256,
}

/// Strong security-binding digest. Unlike [`ContentDigest`], its algorithm and
/// encoding are constrained by the normative security contract.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SecurityDigest {
    algorithm: SecurityDigestAlgorithm,
    hex: String,
}

impl SecurityDigest {
    pub fn new_sha256(hex: impl Into<String>) -> Result<Self, DomainError> {
        let hex = hex.into();
        if hex.len() != SHA256_HEX_LEN || !is_lower_hex(&hex) {
            return Err(DomainError::InvalidSecurityDigest(
                "sha256 digest must be exactly 64 lowercase hexadecimal characters".to_owned(),
            ));
        }
        Ok(Self {
            algorithm: SecurityDigestAlgorithm::Sha256,
            hex,
        })
    }

    #[must_use]
    pub fn sha256(input: &[u8]) -> Self {
        let digest = Sha256::digest(input);
        Self {
            algorithm: SecurityDigestAlgorithm::Sha256,
            hex: encode_lower_hex(digest.as_ref()),
        }
    }

    #[must_use]
    pub const fn algorithm(&self) -> SecurityDigestAlgorithm {
        self.algorithm
    }

    #[must_use]
    pub fn hex(&self) -> &str {
        &self.hex
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SecurityDigestWire {
    algorithm: SecurityDigestAlgorithm,
    hex: String,
}

impl<'de> Deserialize<'de> for SecurityDigest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = SecurityDigestWire::deserialize(deserializer)?;
        match wire.algorithm {
            SecurityDigestAlgorithm::Sha256 => Self::new_sha256(wire.hex),
        }
        .map_err(de::Error::custom)
    }
}
