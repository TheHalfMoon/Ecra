use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignatureAlgorithm {
    #[serde(rename = "ed25519")]
    Ed25519,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AeadAlgorithm {
    #[serde(rename = "chacha20_poly1305_rfc8439")]
    ChaCha20Poly1305Rfc8439,
}
