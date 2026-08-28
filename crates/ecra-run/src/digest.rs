use ecra_core::{EpochMillis, RunId, SchemaVersion, to_jcs_vec};
use serde::{Deserialize, Deserializer, Serialize, de};
use sha2::{Digest, Sha256};

use crate::{EventSequence, RunError, RunEvent};

const RUN_EVENT_V1_DOMAIN: &[u8] = b"ecra/run-event/v1\0";
const SHA256_HEX_LEN: usize = 64;
const HEX: &[u8; 16] = b"0123456789abcdef";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LedgerDigestAlgorithm {
    Sha256,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LedgerDigest {
    algorithm: LedgerDigestAlgorithm,
    hex: String,
}

impl LedgerDigest {
    pub fn new_sha256(hex: impl Into<String>) -> Result<Self, RunError> {
        let hex = hex.into();
        if hex.len() != SHA256_HEX_LEN
            || !hex
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(RunError::ledger_digest_mismatch(
                "ledger sha256 digest must be exactly 64 lowercase hexadecimal characters",
            ));
        }
        Ok(Self {
            algorithm: LedgerDigestAlgorithm::Sha256,
            hex,
        })
    }

    #[must_use]
    pub const fn algorithm(&self) -> LedgerDigestAlgorithm {
        self.algorithm
    }

    #[must_use]
    pub fn hex(&self) -> &str {
        &self.hex
    }

    pub fn for_event(
        schema_version: SchemaVersion,
        run_id: RunId,
        sequence: EventSequence,
        recorded_at: EpochMillis,
        previous_digest: Option<&Self>,
        event: &RunEvent,
    ) -> Result<Self, RunError> {
        let canonical = canonical_event_material(
            schema_version,
            run_id,
            sequence,
            recorded_at,
            previous_digest,
            event,
        )?;
        let mut preimage = Vec::with_capacity(RUN_EVENT_V1_DOMAIN.len() + canonical.len());
        preimage.extend_from_slice(RUN_EVENT_V1_DOMAIN);
        preimage.extend_from_slice(&canonical);
        let digest = Sha256::digest(preimage);
        let mut hex = String::with_capacity(SHA256_HEX_LEN);
        for byte in digest {
            hex.push(char::from(HEX[usize::from(byte >> 4)]));
            hex.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
        Ok(Self {
            algorithm: LedgerDigestAlgorithm::Sha256,
            hex,
        })
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LedgerDigestWire {
    algorithm: LedgerDigestAlgorithm,
    hex: String,
}

impl<'de> Deserialize<'de> for LedgerDigest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = LedgerDigestWire::deserialize(deserializer)?;
        match wire.algorithm {
            LedgerDigestAlgorithm::Sha256 => Self::new_sha256(wire.hex),
        }
        .map_err(de::Error::custom)
    }
}

#[derive(Serialize)]
struct EventDigestMaterial<'a> {
    schema_version: SchemaVersion,
    run_id: RunId,
    sequence: EventSequence,
    recorded_at: EpochMillis,
    previous_digest: Option<&'a LedgerDigest>,
    event: &'a RunEvent,
}

pub(crate) fn canonical_event_material(
    schema_version: SchemaVersion,
    run_id: RunId,
    sequence: EventSequence,
    recorded_at: EpochMillis,
    previous_digest: Option<&LedgerDigest>,
    event: &RunEvent,
) -> Result<Vec<u8>, RunError> {
    to_jcs_vec(&EventDigestMaterial {
        schema_version,
        run_id,
        sequence,
        recorded_at,
        previous_digest,
        event,
    })
    .map_err(|error| RunError::serialization(error.to_string()))
}
