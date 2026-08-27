use std::collections::BTreeSet;

use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{RunError, RunErrorCategory, RunErrorCode};

pub const MAX_BUDGET_AMOUNT: u64 = 9_007_199_254_740_991;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct BudgetAmount(u64);

impl BudgetAmount {
    pub fn new(value: u64) -> Result<Self, RunError> {
        if value > MAX_BUDGET_AMOUNT {
            return Err(RunError::new(
                RunErrorCategory::Budget,
                RunErrorCode::InvalidBudget,
                "budget amount exceeds the I-JSON safe integer maximum",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    pub fn checked_add(self, other: Self) -> Result<Self, RunError> {
        let value = self.0.checked_add(other.0).ok_or_else(|| {
            RunError::new(
                RunErrorCategory::Budget,
                RunErrorCode::BudgetOverflow,
                "budget addition overflowed u64",
            )
        })?;
        if value > MAX_BUDGET_AMOUNT {
            return Err(RunError::new(
                RunErrorCategory::Budget,
                RunErrorCode::BudgetOverflow,
                "budget addition exceeds the I-JSON safe integer maximum",
            ));
        }
        Ok(Self(value))
    }
}

impl<'de> Deserialize<'de> for BudgetAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Self::new(value).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BudgetDimension {
    ActiveWallMillis,
    Steps,
    ToolCalls,
    ModelCalls,
    InputTokens,
    OutputTokens,
    CostMicrounits,
    ProcessCount,
    ProcessMillis,
    OutputBytes,
    NetworkRequests,
    NetworkBytes,
    StorageBytes,
    RecursionDepth,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BudgetLimit {
    dimension: BudgetDimension,
    soft: Option<BudgetAmount>,
    hard: BudgetAmount,
}

impl BudgetLimit {
    pub fn new(
        dimension: BudgetDimension,
        soft: Option<BudgetAmount>,
        hard: BudgetAmount,
    ) -> Result<Self, RunError> {
        if soft.is_some_and(|value| value > hard) {
            return Err(RunError::invalid_budget(
                "budget soft limit must not exceed the hard limit",
            ));
        }
        Ok(Self {
            dimension,
            soft,
            hard,
        })
    }

    #[must_use]
    pub const fn dimension(&self) -> BudgetDimension {
        self.dimension
    }

    #[must_use]
    pub const fn soft(&self) -> Option<BudgetAmount> {
        self.soft
    }

    #[must_use]
    pub const fn hard(&self) -> BudgetAmount {
        self.hard
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BudgetLimitWire {
    dimension: BudgetDimension,
    soft: Option<BudgetAmount>,
    hard: BudgetAmount,
}

impl<'de> Deserialize<'de> for BudgetLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = BudgetLimitWire::deserialize(deserializer)?;
        Self::new(wire.dimension, wire.soft, wire.hard).map_err(de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RunBudget {
    limits: Vec<BudgetLimit>,
}

impl RunBudget {
    pub fn new(limits: Vec<BudgetLimit>) -> Result<Self, RunError> {
        if limits.is_empty() {
            return Err(RunError::invalid_budget(
                "run budget requires at least one explicit limit",
            ));
        }
        let mut dimensions = BTreeSet::new();
        for limit in &limits {
            if !dimensions.insert(limit.dimension()) {
                return Err(RunError::invalid_budget(
                    "run budget contains a duplicate dimension",
                ));
            }
        }
        Ok(Self { limits })
    }

    #[must_use]
    pub fn limits(&self) -> &[BudgetLimit] {
        &self.limits
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RunBudgetWire {
    limits: Vec<BudgetLimit>,
}

impl<'de> Deserialize<'de> for RunBudget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::new(RunBudgetWire::deserialize(deserializer)?.limits).map_err(de::Error::custom)
    }
}
