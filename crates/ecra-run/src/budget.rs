use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{RunError, RunErrorCategory, RunErrorCode};

pub const MAX_BUDGET_AMOUNT: u64 = 9_007_199_254_740_991;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct BudgetAmount(u64);

impl BudgetAmount {
    pub const ZERO: Self = Self(0);

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

impl BudgetDimension {
    pub const ALL: [Self; 14] = [
        Self::ActiveWallMillis,
        Self::Steps,
        Self::ToolCalls,
        Self::ModelCalls,
        Self::InputTokens,
        Self::OutputTokens,
        Self::CostMicrounits,
        Self::ProcessCount,
        Self::ProcessMillis,
        Self::OutputBytes,
        Self::NetworkRequests,
        Self::NetworkBytes,
        Self::StorageBytes,
        Self::RecursionDepth,
    ];
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

    #[must_use]
    pub fn limit(&self, dimension: BudgetDimension) -> Option<&BudgetLimit> {
        self.limits
            .iter()
            .find(|limit| limit.dimension() == dimension)
    }

    #[must_use]
    pub fn remaining(
        &self,
        usage: &BudgetUsage,
        dimension: BudgetDimension,
    ) -> Option<BudgetAmount> {
        let limit = self.limit(dimension)?;
        let current = usage.get(dimension).get();
        Some(BudgetAmount(
            limit.hard().get().saturating_sub(current),
        ))
    }

    pub fn preflight(
        &self,
        usage: &BudgetUsage,
        dimension: BudgetDimension,
        known_upper_bound: BudgetAmount,
    ) -> Result<(), RunError> {
        let Some(remaining) = self.remaining(usage, dimension) else {
            return Ok(());
        };
        if known_upper_bound > remaining {
            return Err(RunError::new(
                RunErrorCategory::Budget,
                RunErrorCode::BudgetPreflightExceeded,
                "known upper bound exceeds remaining configured hard budget",
            ));
        }
        Ok(())
    }

    #[must_use]
    pub fn soft_crossing(
        &self,
        dimension: BudgetDimension,
        previous: BudgetAmount,
        cumulative: BudgetAmount,
    ) -> Option<(BudgetAmount, BudgetAmount)> {
        let soft = self.limit(dimension)?.soft()?;
        (previous < soft && cumulative >= soft).then_some((soft, cumulative))
    }

    #[must_use]
    pub fn hard_exhaustion(
        &self,
        dimension: BudgetDimension,
        cumulative: BudgetAmount,
    ) -> Option<(BudgetAmount, BudgetAmount)> {
        let hard = self.limit(dimension)?.hard();
        (cumulative >= hard).then_some((hard, cumulative))
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BudgetUsage(BTreeMap<BudgetDimension, BudgetAmount>);

impl BudgetUsage {
    #[must_use]
    pub fn get(&self, dimension: BudgetDimension) -> BudgetAmount {
        self.0
            .get(&dimension)
            .copied()
            .unwrap_or(BudgetAmount::ZERO)
    }

    #[must_use]
    pub fn recorded(&self, dimension: BudgetDimension) -> Option<BudgetAmount> {
        self.0.get(&dimension).copied()
    }

    #[must_use]
    pub fn amounts(&self) -> &BTreeMap<BudgetDimension, BudgetAmount> {
        &self.0
    }

    pub fn charge(
        &mut self,
        dimension: BudgetDimension,
        amount: BudgetAmount,
    ) -> Result<(BudgetAmount, BudgetAmount), RunError> {
        let previous = self.get(dimension);
        let cumulative = previous.checked_add(amount)?;
        self.0.insert(dimension, cumulative);
        Ok((previous, cumulative))
    }
}
