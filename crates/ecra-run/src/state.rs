use ecra_core::ActionAttemptRef;
use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{BudgetDimension, RunError, RunErrorCategory, RunErrorCode};

pub const MAX_SUSPENSION_OTHER_CODE_BYTES: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunPhase {
    Created,
    Running,
    Suspended,
    CancellationRequested,
    Cancelled,
    Failed,
    ExecutionCompleted,
}

impl RunPhase {
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Cancelled | Self::Failed | Self::ExecutionCompleted
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SuspensionReason {
    UserPause,
    BudgetExhausted { dimension: BudgetDimension },
    ReconciliationRequired { attempt: ActionAttemptRef },
    CancellationInProgress,
    RuntimeInterruption,
    Other { code: String },
}

impl SuspensionReason {
    pub fn other(code: impl Into<String>) -> Result<Self, RunError> {
        let code = code.into();
        if code.is_empty() || code.len() > MAX_SUSPENSION_OTHER_CODE_BYTES {
            return Err(RunError::new(
                RunErrorCategory::State,
                RunErrorCode::InvalidStateTransition,
                "suspension other code must be 1..=256 UTF-8 bytes",
            ));
        }
        Ok(Self::Other { code })
    }

    #[must_use]
    pub const fn is_directly_resumable(&self) -> bool {
        matches!(self, Self::UserPause | Self::RuntimeInterruption)
    }
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum SuspensionReasonWire {
    UserPause,
    BudgetExhausted { dimension: BudgetDimension },
    ReconciliationRequired { attempt: ActionAttemptRef },
    CancellationInProgress,
    RuntimeInterruption,
    Other { code: String },
}

impl<'de> Deserialize<'de> for SuspensionReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match SuspensionReasonWire::deserialize(deserializer)? {
            SuspensionReasonWire::UserPause => Ok(Self::UserPause),
            SuspensionReasonWire::BudgetExhausted { dimension } => {
                Ok(Self::BudgetExhausted { dimension })
            }
            SuspensionReasonWire::ReconciliationRequired { attempt } => {
                Ok(Self::ReconciliationRequired { attempt })
            }
            SuspensionReasonWire::CancellationInProgress => Ok(Self::CancellationInProgress),
            SuspensionReasonWire::RuntimeInterruption => Ok(Self::RuntimeInterruption),
            SuspensionReasonWire::Other { code } => Self::other(code).map_err(de::Error::custom),
        }
    }
}
