use std::{collections::BTreeSet, fmt};

use ecra_core::{SchemaVersion, VerificationTarget};
use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{
    CheckpointId, VerificationAggregateStateV1, VerificationAggregateViewV1, VerifyError,
    VerifyErrorCategory, VerifyErrorCode,
};

pub const MAX_CHECKPOINT_LABEL_BYTES: usize = 256;
pub const MAX_CHECKPOINT_REQUIREMENTS: usize = 128;
pub const MAX_ACCEPTED_STATES_PER_REQUIREMENT: usize = 2;
pub const MAX_VERIFICATION_CHECKPOINT_BYTES: usize = 256 * 1024;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationRequirementV1 {
    target: VerificationTarget,
    accepted_states: Vec<VerificationAggregateStateV1>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VerificationRequirementWire {
    target: VerificationTarget,
    #[serde(deserialize_with = "deserialize_accepted_states")]
    accepted_states: Vec<VerificationAggregateStateV1>,
}

fn deserialize_accepted_states<'de, D>(
    deserializer: D,
) -> Result<Vec<VerificationAggregateStateV1>, D::Error>
where
    D: Deserializer<'de>,
{
    struct AcceptedStatesVisitor;

    impl<'de> de::Visitor<'de> for AcceptedStatesVisitor {
        type Value = Vec<VerificationAggregateStateV1>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a bounded list of verification aggregate states")
        }

        fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut values = Vec::with_capacity(
                sequence
                    .size_hint()
                    .unwrap_or(0)
                    .min(MAX_ACCEPTED_STATES_PER_REQUIREMENT),
            );
            while let Some(value) = sequence.next_element()? {
                if values.len() >= MAX_ACCEPTED_STATES_PER_REQUIREMENT {
                    return Err(de::Error::custom(
                        "verification requirement accepted_states exceeds the v1 limit",
                    ));
                }
                values.push(value);
            }
            Ok(values)
        }
    }

    deserializer.deserialize_seq(AcceptedStatesVisitor)
}

impl VerificationRequirementV1 {
    pub fn new(
        target: VerificationTarget,
        accepted_states: Vec<VerificationAggregateStateV1>,
    ) -> Result<Self, VerifyError> {
        if accepted_states.is_empty() {
            return Err(VerifyError::new(
                VerifyErrorCategory::Validation,
                VerifyErrorCode::InvalidTarget,
                "verification requirement accepted_states must be non-empty",
            ));
        }
        if accepted_states.len() > MAX_ACCEPTED_STATES_PER_REQUIREMENT {
            return Err(VerifyError::new(
                VerifyErrorCategory::ResourceLimit,
                VerifyErrorCode::ResourceLimitExceeded,
                "verification requirement accepted_states exceeds the v1 limit",
            ));
        }
        let mut states = BTreeSet::new();
        for state in accepted_states {
            let key = match state {
                VerificationAggregateStateV1::Verified => 1_u8,
                VerificationAggregateStateV1::Rejected => 2_u8,
                VerificationAggregateStateV1::Absent
                | VerificationAggregateStateV1::Inconclusive
                | VerificationAggregateStateV1::Conflicted => {
                    return Err(VerifyError::new(
                        VerifyErrorCategory::Validation,
                        VerifyErrorCode::InvalidTarget,
                        "verification requirement contains a prohibited satisfying state",
                    ));
                }
            };
            if !states.insert(key) {
                return Err(VerifyError::new(
                    VerifyErrorCategory::Validation,
                    VerifyErrorCode::DuplicateId,
                    "verification requirement contains a duplicate accepted state",
                ));
            }
        }
        let accepted_states = states
            .into_iter()
            .map(|key| match key {
                1 => VerificationAggregateStateV1::Verified,
                2 => VerificationAggregateStateV1::Rejected,
                _ => unreachable!(),
            })
            .collect();
        Ok(Self {
            target,
            accepted_states,
        })
    }

    #[must_use]
    pub const fn target(&self) -> &VerificationTarget {
        &self.target
    }

    #[must_use]
    pub fn accepted_states(&self) -> &[VerificationAggregateStateV1] {
        &self.accepted_states
    }

    #[must_use]
    pub fn accepts(&self, state: VerificationAggregateStateV1) -> bool {
        self.accepted_states.contains(&state)
    }
}

impl<'de> Deserialize<'de> for VerificationRequirementV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = VerificationRequirementWire::deserialize(deserializer)?;
        Self::new(wire.target, wire.accepted_states).map_err(de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerificationCheckpointFieldsV1 {
    pub id: CheckpointId,
    pub label: String,
    pub requirements: Vec<VerificationRequirementV1>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationCheckpointV1 {
    version: SchemaVersion,
    id: CheckpointId,
    label: String,
    requirements: Vec<VerificationRequirementV1>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VerificationCheckpointWire {
    version: SchemaVersion,
    id: CheckpointId,
    label: String,
    #[serde(deserialize_with = "deserialize_requirements")]
    requirements: Vec<VerificationRequirementV1>,
}

fn deserialize_requirements<'de, D>(
    deserializer: D,
) -> Result<Vec<VerificationRequirementV1>, D::Error>
where
    D: Deserializer<'de>,
{
    struct RequirementsVisitor;

    impl<'de> de::Visitor<'de> for RequirementsVisitor {
        type Value = Vec<VerificationRequirementV1>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a bounded list of verification requirements")
        }

        fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut values = Vec::with_capacity(
                sequence
                    .size_hint()
                    .unwrap_or(0)
                    .min(MAX_CHECKPOINT_REQUIREMENTS),
            );
            while let Some(value) = sequence.next_element()? {
                if values.len() >= MAX_CHECKPOINT_REQUIREMENTS {
                    return Err(de::Error::custom(
                        "verification checkpoint requirements exceeds the v1 limit",
                    ));
                }
                values.push(value);
            }
            Ok(values)
        }
    }

    deserializer.deserialize_seq(RequirementsVisitor)
}

impl VerificationCheckpointV1 {
    pub fn from_fields(fields: VerificationCheckpointFieldsV1) -> Result<Self, VerifyError> {
        Self::validate(SchemaVersion::V1_0, fields)
    }

    fn validate(
        version: SchemaVersion,
        mut fields: VerificationCheckpointFieldsV1,
    ) -> Result<Self, VerifyError> {
        if version.validate_supported().is_err() || version != SchemaVersion::V1_0 {
            return Err(VerifyError::new(
                VerifyErrorCategory::Compatibility,
                VerifyErrorCode::UnsupportedVersion,
                "verification checkpoint version is not supported",
            ));
        }
        if fields.label.is_empty() {
            return Err(VerifyError::new(
                VerifyErrorCategory::Validation,
                VerifyErrorCode::InvalidTarget,
                "verification checkpoint label must be non-empty",
            ));
        }
        if fields.label.len() > MAX_CHECKPOINT_LABEL_BYTES {
            return Err(VerifyError::new(
                VerifyErrorCategory::ResourceLimit,
                VerifyErrorCode::ResourceLimitExceeded,
                "verification checkpoint label exceeds the v1 byte limit",
            ));
        }
        if fields.requirements.is_empty() {
            return Err(VerifyError::new(
                VerifyErrorCategory::Validation,
                VerifyErrorCode::InvalidTarget,
                "verification checkpoint requirements must be non-empty",
            ));
        }
        if fields.requirements.len() > MAX_CHECKPOINT_REQUIREMENTS {
            return Err(VerifyError::new(
                VerifyErrorCategory::ResourceLimit,
                VerifyErrorCode::ResourceLimitExceeded,
                "verification checkpoint requirement count exceeds the v1 limit",
            ));
        }

        let mut keyed = Vec::with_capacity(fields.requirements.len());
        let mut targets = BTreeSet::new();
        for requirement in fields.requirements.drain(..) {
            let key = target_key(requirement.target())?;
            if !targets.insert(key.clone()) {
                return Err(VerifyError::new(
                    VerifyErrorCategory::Validation,
                    VerifyErrorCode::DuplicateId,
                    "verification checkpoint contains a duplicate exact target",
                ));
            }
            keyed.push((key, requirement));
        }
        keyed.sort_by(|left, right| left.0.cmp(&right.0));
        fields.requirements = keyed
            .into_iter()
            .map(|(_, requirement)| requirement)
            .collect();

        let value = Self {
            version,
            id: fields.id,
            label: fields.label,
            requirements: fields.requirements,
        };
        let serialized = serde_json::to_vec(&value).map_err(|_| {
            VerifyError::new(
                VerifyErrorCategory::Validation,
                VerifyErrorCode::InvalidTarget,
                "verification checkpoint could not be size-checked",
            )
        })?;
        if serialized.len() > MAX_VERIFICATION_CHECKPOINT_BYTES {
            return Err(VerifyError::new(
                VerifyErrorCategory::ResourceLimit,
                VerifyErrorCode::ResourceLimitExceeded,
                "verification checkpoint exceeds the complete v1 byte limit",
            ));
        }
        Ok(value)
    }

    pub fn from_json_slice(input: &[u8]) -> Result<Self, VerifyError> {
        if input.len() > MAX_VERIFICATION_CHECKPOINT_BYTES {
            return Err(VerifyError::new(
                VerifyErrorCategory::ResourceLimit,
                VerifyErrorCode::ResourceLimitExceeded,
                "verification checkpoint JSON exceeds the complete v1 byte limit",
            ));
        }
        let wire: VerificationCheckpointWire = serde_json::from_slice(input).map_err(|_| {
            VerifyError::new(
                VerifyErrorCategory::Validation,
                VerifyErrorCode::InvalidTarget,
                "verification checkpoint JSON is malformed or contains unsupported fields",
            )
        })?;
        let fields = VerificationCheckpointFieldsV1 {
            id: wire.id,
            label: wire.label,
            requirements: wire.requirements,
        };
        Self::validate(wire.version, fields)
    }

    pub fn evaluate(
        &self,
        aggregates: &[VerificationAggregateViewV1],
    ) -> Result<CheckpointEvaluationV1, VerifyError> {
        let mut aggregate_targets = BTreeSet::new();
        for aggregate in aggregates {
            let key = target_key(aggregate.target())?;
            if !aggregate_targets.insert(key) {
                return Err(VerifyError::new(
                    VerifyErrorCategory::Aggregation,
                    VerifyErrorCode::DuplicateId,
                    "checkpoint evaluation contains duplicate aggregate targets",
                ));
            }
        }

        let mut satisfied_targets = Vec::new();
        let mut unsatisfied_targets = Vec::new();
        let mut conflicted_targets = Vec::new();

        for requirement in &self.requirements {
            let aggregate = aggregates
                .iter()
                .find(|candidate| candidate.target() == requirement.target());
            match aggregate {
                Some(value) if value.state() == VerificationAggregateStateV1::Conflicted => {
                    conflicted_targets.push(requirement.target().clone());
                    unsatisfied_targets.push(requirement.target().clone());
                }
                Some(value) if requirement.accepts(value.state()) => {
                    satisfied_targets.push(requirement.target().clone());
                }
                Some(_) | None => {
                    unsatisfied_targets.push(requirement.target().clone());
                }
            }
        }

        sort_targets(&mut satisfied_targets)?;
        sort_targets(&mut unsatisfied_targets)?;
        sort_targets(&mut conflicted_targets)?;
        let satisfied = unsatisfied_targets.is_empty() && conflicted_targets.is_empty();

        Ok(CheckpointEvaluationV1 {
            checkpoint_id: self.id,
            satisfied,
            satisfied_targets,
            unsatisfied_targets,
            conflicted_targets,
        })
    }

    #[must_use]
    pub const fn version(&self) -> SchemaVersion {
        self.version
    }

    #[must_use]
    pub const fn id(&self) -> CheckpointId {
        self.id
    }

    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub fn requirements(&self) -> &[VerificationRequirementV1] {
        &self.requirements
    }
}

impl<'de> Deserialize<'de> for VerificationCheckpointV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = VerificationCheckpointWire::deserialize(deserializer)?;
        let fields = VerificationCheckpointFieldsV1 {
            id: wire.id,
            label: wire.label,
            requirements: wire.requirements,
        };
        Self::validate(wire.version, fields).map_err(de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckpointEvaluationV1 {
    checkpoint_id: CheckpointId,
    satisfied: bool,
    satisfied_targets: Vec<VerificationTarget>,
    unsatisfied_targets: Vec<VerificationTarget>,
    conflicted_targets: Vec<VerificationTarget>,
}

impl CheckpointEvaluationV1 {
    #[must_use]
    pub const fn checkpoint_id(&self) -> CheckpointId {
        self.checkpoint_id
    }

    #[must_use]
    pub const fn satisfied(&self) -> bool {
        self.satisfied
    }

    #[must_use]
    pub fn satisfied_targets(&self) -> &[VerificationTarget] {
        &self.satisfied_targets
    }

    #[must_use]
    pub fn unsatisfied_targets(&self) -> &[VerificationTarget] {
        &self.unsatisfied_targets
    }

    #[must_use]
    pub fn conflicted_targets(&self) -> &[VerificationTarget] {
        &self.conflicted_targets
    }
}

fn target_key(target: &VerificationTarget) -> Result<Vec<u8>, VerifyError> {
    serde_jcs::to_vec(target).map_err(|_| {
        VerifyError::new(
            VerifyErrorCategory::Validation,
            VerifyErrorCode::InvalidTarget,
            "verification target could not be canonicalized",
        )
    })
}

fn sort_targets(targets: &mut [VerificationTarget]) -> Result<(), VerifyError> {
    let mut keyed = targets
        .iter()
        .map(|target| target_key(target).map(|key| (key, target.clone())))
        .collect::<Result<Vec<_>, _>>()?;
    keyed.sort_by(|left, right| left.0.cmp(&right.0));
    for (slot, (_, target)) in targets.iter_mut().zip(keyed) {
        *slot = target;
    }
    Ok(())
}
