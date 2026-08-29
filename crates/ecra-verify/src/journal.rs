use std::fmt;

use ecra_core::{SchemaVersion, VerificationReceipt};
use serde::{Deserialize, Deserializer, Serialize, de};
use sha2::{Digest, Sha256};

use crate::{
    ReconciliationRecordV1, VerificationCheckpointV1, VerifyError, VerifyErrorCategory,
    VerifyErrorCode,
};

const VERIFICATION_JOURNAL_V1_DOMAIN: &[u8] = b"ecra/verification-journal/v1\0";
const SHA256_HEX_LEN: usize = 64;
const HEX: &[u8; 16] = b"0123456789abcdef";
pub const MAX_VERIFICATION_JOURNAL_SEQUENCE: u64 = 9_007_199_254_740_991;
pub const MAX_VERIFICATION_JOURNAL_ENTRY_BYTES: usize = 65_536;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct VerificationJournalSequence(u64);

impl VerificationJournalSequence {
    pub fn new(value: u64) -> Result<Self, VerifyError> {
        if value == 0 || value > MAX_VERIFICATION_JOURNAL_SEQUENCE {
            return Err(VerifyError::new(
                VerifyErrorCategory::Validation,
                VerifyErrorCode::JournalSequenceMismatch,
                "verification journal sequence must be in the v1 positive safe-integer range",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    pub fn checked_next(self) -> Result<Self, VerifyError> {
        Self::new(self.0.checked_add(1).ok_or_else(|| {
            VerifyError::new(
                VerifyErrorCategory::ResourceLimit,
                VerifyErrorCode::ResourceLimitExceeded,
                "verification journal sequence increment overflowed",
            )
        })?)
    }
}

impl<'de> Deserialize<'de> for VerificationJournalSequence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Self::new(value).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationJournalDigestAlgorithm {
    Sha256,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationJournalDigest {
    algorithm: VerificationJournalDigestAlgorithm,
    hex: String,
}

impl VerificationJournalDigest {
    pub fn new_sha256(hex: impl Into<String>) -> Result<Self, VerifyError> {
        let hex = hex.into();
        if hex.len() != SHA256_HEX_LEN
            || !hex
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(VerifyError::new(
                VerifyErrorCategory::Validation,
                VerifyErrorCode::JournalDigestMismatch,
                "verification journal sha256 digest must be exactly 64 lowercase hexadecimal characters",
            ));
        }
        Ok(Self {
            algorithm: VerificationJournalDigestAlgorithm::Sha256,
            hex,
        })
    }

    #[must_use]
    pub const fn algorithm(&self) -> VerificationJournalDigestAlgorithm {
        self.algorithm
    }

    #[must_use]
    pub fn hex(&self) -> &str {
        &self.hex
    }

    fn for_material(material: &[u8]) -> Self {
        let mut preimage =
            Vec::with_capacity(VERIFICATION_JOURNAL_V1_DOMAIN.len() + material.len());
        preimage.extend_from_slice(VERIFICATION_JOURNAL_V1_DOMAIN);
        preimage.extend_from_slice(material);
        let digest = Sha256::digest(preimage);
        let mut hex = String::with_capacity(SHA256_HEX_LEN);
        for byte in digest {
            hex.push(char::from(HEX[usize::from(byte >> 4)]));
            hex.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
        Self {
            algorithm: VerificationJournalDigestAlgorithm::Sha256,
            hex,
        }
    }
}

impl fmt::Display for VerificationJournalDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.hex)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VerificationJournalDigestWire {
    algorithm: VerificationJournalDigestAlgorithm,
    hex: String,
}

impl<'de> Deserialize<'de> for VerificationJournalDigest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = VerificationJournalDigestWire::deserialize(deserializer)?;
        match wire.algorithm {
            VerificationJournalDigestAlgorithm::Sha256 => Self::new_sha256(wire.hex),
        }
        .map_err(de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum VerificationJournalBodyV1 {
    VerificationReceipt {
        receipt: VerificationReceipt,
    },
    CheckpointDefined {
        checkpoint: VerificationCheckpointV1,
    },
    ReconciliationRecorded {
        record: ReconciliationRecordV1,
    },
}

#[derive(Serialize)]
struct VerificationJournalDigestMaterial<'a> {
    version: SchemaVersion,
    sequence: VerificationJournalSequence,
    previous_digest: Option<&'a VerificationJournalDigest>,
    body: &'a VerificationJournalBodyV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationJournalEntryV1 {
    version: SchemaVersion,
    sequence: VerificationJournalSequence,
    previous_digest: Option<VerificationJournalDigest>,
    body: VerificationJournalBodyV1,
    entry_digest: VerificationJournalDigest,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VerificationJournalEntryWire {
    version: SchemaVersion,
    sequence: VerificationJournalSequence,
    previous_digest: Option<VerificationJournalDigest>,
    body: VerificationJournalBodyV1,
    entry_digest: VerificationJournalDigest,
}

impl VerificationJournalEntryV1 {
    pub fn new(
        sequence: VerificationJournalSequence,
        previous_digest: Option<VerificationJournalDigest>,
        body: VerificationJournalBodyV1,
    ) -> Result<Self, VerifyError> {
        validate_previous_digest(sequence, previous_digest.as_ref())?;
        let material = canonical_material(sequence, previous_digest.as_ref(), &body)?;
        let entry_digest = VerificationJournalDigest::for_material(&material);
        Ok(Self {
            version: SchemaVersion::V1_0,
            sequence,
            previous_digest,
            body,
            entry_digest,
        })
    }

    fn validate_wire(wire: VerificationJournalEntryWire) -> Result<Self, VerifyError> {
        if wire.version.validate_supported().is_err() || wire.version != SchemaVersion::V1_0 {
            return Err(VerifyError::new(
                VerifyErrorCategory::Compatibility,
                VerifyErrorCode::UnsupportedVersion,
                "verification journal entry version is not supported",
            ));
        }
        validate_previous_digest(wire.sequence, wire.previous_digest.as_ref())?;
        let material =
            canonical_material(wire.sequence, wire.previous_digest.as_ref(), &wire.body)?;
        let expected = VerificationJournalDigest::for_material(&material);
        if expected != wire.entry_digest {
            return Err(VerifyError::new(
                VerifyErrorCategory::Persistence,
                VerifyErrorCode::JournalDigestMismatch,
                "verification journal entry digest does not match canonical entry material",
            ));
        }
        Ok(Self {
            version: wire.version,
            sequence: wire.sequence,
            previous_digest: wire.previous_digest,
            body: wire.body,
            entry_digest: wire.entry_digest,
        })
    }

    pub fn from_json_slice(input: &[u8]) -> Result<Self, VerifyError> {
        if input.len() > MAX_VERIFICATION_JOURNAL_ENTRY_BYTES {
            return Err(VerifyError::new(
                VerifyErrorCategory::ResourceLimit,
                VerifyErrorCode::ResourceLimitExceeded,
                "verification journal entry exceeds the v1 byte limit",
            ));
        }
        let wire: VerificationJournalEntryWire = serde_json::from_slice(input).map_err(|_| {
            VerifyError::new(
                VerifyErrorCategory::Persistence,
                VerifyErrorCode::StoreCorrupt,
                "verification journal entry JSON is malformed or contains unsupported fields",
            )
        })?;
        Self::validate_wire(wire)
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, VerifyError> {
        serde_jcs::to_vec(self).map_err(|_| {
            VerifyError::new(
                VerifyErrorCategory::Persistence,
                VerifyErrorCode::StoreCorrupt,
                "verification journal entry could not be canonicalized",
            )
        })
    }

    #[must_use]
    pub const fn version(&self) -> SchemaVersion {
        self.version
    }

    #[must_use]
    pub const fn sequence(&self) -> VerificationJournalSequence {
        self.sequence
    }

    #[must_use]
    pub const fn previous_digest(&self) -> Option<&VerificationJournalDigest> {
        self.previous_digest.as_ref()
    }

    #[must_use]
    pub const fn body(&self) -> &VerificationJournalBodyV1 {
        &self.body
    }

    #[must_use]
    pub const fn entry_digest(&self) -> &VerificationJournalDigest {
        &self.entry_digest
    }
}

impl<'de> Deserialize<'de> for VerificationJournalEntryV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = VerificationJournalEntryWire::deserialize(deserializer)?;
        Self::validate_wire(wire).map_err(de::Error::custom)
    }
}

fn validate_previous_digest(
    sequence: VerificationJournalSequence,
    previous_digest: Option<&VerificationJournalDigest>,
) -> Result<(), VerifyError> {
    let valid = if sequence.get() == 1 {
        previous_digest.is_none()
    } else {
        previous_digest.is_some()
    };
    if !valid {
        return Err(VerifyError::new(
            VerifyErrorCategory::Persistence,
            VerifyErrorCode::JournalSequenceMismatch,
            "verification journal genesis must have no previous digest and successors must have one",
        ));
    }
    Ok(())
}

fn canonical_material(
    sequence: VerificationJournalSequence,
    previous_digest: Option<&VerificationJournalDigest>,
    body: &VerificationJournalBodyV1,
) -> Result<Vec<u8>, VerifyError> {
    serde_jcs::to_vec(&VerificationJournalDigestMaterial {
        version: SchemaVersion::V1_0,
        sequence,
        previous_digest,
        body,
    })
    .map_err(|_| {
        VerifyError::new(
            VerifyErrorCategory::Persistence,
            VerifyErrorCode::StoreCorrupt,
            "verification journal digest material could not be canonicalized",
        )
    })
}
