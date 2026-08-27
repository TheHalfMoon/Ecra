use serde::{Deserialize, Deserializer, Serialize, de};

use crate::error::DomainError;

/// Largest integer exactly representable by I-JSON / IEEE-754 binary64.
pub const I_JSON_MAX_SAFE_INTEGER: i64 = 9_007_199_254_740_991;
pub const I_JSON_MIN_SAFE_INTEGER: i64 = -I_JSON_MAX_SAFE_INTEGER;

/// Caller-supplied epoch milliseconds. The core never reads an ambient clock.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct EpochMillis(i64);

impl EpochMillis {
    pub const fn new(value: i64) -> Result<Self, DomainError> {
        if value < I_JSON_MIN_SAFE_INTEGER || value > I_JSON_MAX_SAFE_INTEGER {
            return Err(DomainError::InvalidEpochMillis { value });
        }
        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}

impl TryFrom<i64> for EpochMillis {
    type Error = DomainError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'de> Deserialize<'de> for EpochMillis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i64::deserialize(deserializer)?;
        Self::new(value).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TemporalValidity {
    not_before: Option<EpochMillis>,
    expires_at: Option<EpochMillis>,
}

impl TemporalValidity {
    pub const fn new(
        not_before: Option<EpochMillis>,
        expires_at: Option<EpochMillis>,
    ) -> Result<Self, DomainError> {
        if let (Some(start), Some(end)) = (not_before, expires_at)
            && start.get() > end.get()
        {
            return Err(DomainError::InvalidTemporalRange);
        }
        Ok(Self {
            not_before,
            expires_at,
        })
    }

    #[must_use]
    pub const fn not_before(self) -> Option<EpochMillis> {
        self.not_before
    }

    #[must_use]
    pub const fn expires_at(self) -> Option<EpochMillis> {
        self.expires_at
    }

    #[must_use]
    pub const fn contains(self, context: EvaluationContext) -> bool {
        if let Some(start) = self.not_before
            && context.now.get() < start.get()
        {
            return false;
        }
        if let Some(end) = self.expires_at
            && context.now.get() > end.get()
        {
            return false;
        }
        true
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TemporalValidityWire {
    not_before: Option<EpochMillis>,
    expires_at: Option<EpochMillis>,
}

impl<'de> Deserialize<'de> for TemporalValidity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = TemporalValidityWire::deserialize(deserializer)?;
        Self::new(wire.not_before, wire.expires_at).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvaluationContext {
    now: EpochMillis,
}

impl EvaluationContext {
    #[must_use]
    pub const fn new(now: EpochMillis) -> Self {
        Self { now }
    }

    #[must_use]
    pub const fn now(self) -> EpochMillis {
        self.now
    }
}
