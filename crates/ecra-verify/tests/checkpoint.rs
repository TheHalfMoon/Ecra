use ecra_core::{
    ActorId, ClaimRef, EvidenceId, EvidenceKind, EvidenceRef, VerificationId, VerificationMethod,
    VerificationOutcome, VerificationReceipt, VerificationTarget,
};
use ecra_verify::{
    CheckpointId, MAX_CHECKPOINT_REQUIREMENTS, VerificationAggregateStateV1,
    VerificationAggregateViewV1, VerificationCheckpointFieldsV1, VerificationCheckpointV1,
    VerificationRequirementV1, VerifyErrorCode,
};
use serde::Deserialize;

fn target(reference: &str) -> VerificationTarget {
    VerificationTarget::Claim(ClaimRef::new("checkpoint", reference).expect("claim target"))
}

fn checkpoint_id() -> CheckpointId {
    CheckpointId::parse_str("00000000-0000-0000-0000-000000006001").expect("checkpoint id")
}

fn receipt_id(value: usize) -> VerificationId {
    VerificationId::parse_str(&format!("00000000-0000-0000-0000-{:012}", 6100 + value))
        .expect("verification id")
}

fn evidence_id(value: usize) -> EvidenceId {
    EvidenceId::parse_str(&format!("00000000-0000-0000-0000-{:012}", 6200 + value))
        .expect("evidence id")
}

fn receipt(
    value: usize,
    target: VerificationTarget,
    outcome: VerificationOutcome,
) -> VerificationReceipt {
    let evidence = if outcome == VerificationOutcome::NotEvaluated {
        Vec::new()
    } else {
        vec![EvidenceRef::new(
            evidence_id(value),
            EvidenceKind::Computation,
        )]
    };
    VerificationReceipt::new(
        receipt_id(value),
        ActorId::parse_str("00000000-0000-0000-0000-000000000001").expect("actor id"),
        target,
        VerificationMethod::DeterministicComputation,
        outcome,
        evidence,
    )
    .expect("verification receipt")
}

fn aggregate(
    target: VerificationTarget,
    outcomes: &[VerificationOutcome],
) -> VerificationAggregateViewV1 {
    let receipts = outcomes
        .iter()
        .enumerate()
        .map(|(index, outcome)| receipt(index + 1, target.clone(), *outcome))
        .collect::<Vec<_>>();
    VerificationAggregateViewV1::from_receipts(target, &receipts).expect("aggregate")
}

fn requirement(
    target: VerificationTarget,
    accepted: VerificationAggregateStateV1,
) -> VerificationRequirementV1 {
    VerificationRequirementV1::new(target, vec![accepted]).expect("requirement")
}

fn checkpoint(requirements: Vec<VerificationRequirementV1>) -> VerificationCheckpointV1 {
    VerificationCheckpointV1::from_fields(VerificationCheckpointFieldsV1 {
        id: checkpoint_id(),
        label: "release-critical verification".to_owned(),
        requirements,
    })
    .expect("checkpoint")
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CheckpointFixture {
    version: String,
    cases: Vec<CheckpointCase>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CheckpointCase {
    name: String,
    states: Vec<String>,
    expected_satisfied: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct InvalidCheckpointCase {
    name: String,
    checkpoint: serde_json::Value,
}

#[test]
fn checkpoint_fixture_matrix_is_fail_closed() {
    let fixture: CheckpointFixture = serde_json::from_str(include_str!(
        "../../../contracts/ecra-verify-v1/valid/checkpoint-cases.json"
    ))
    .expect("checkpoint fixture");
    assert_eq!(fixture.version, "1.0");

    for (case_index, case) in fixture.cases.iter().enumerate() {
        let mut requirements = Vec::new();
        let mut aggregates = Vec::new();
        for (state_index, state) in case.states.iter().enumerate() {
            let case_target = target(&format!("fixture-{case_index}-{state_index}"));
            requirements.push(requirement(
                case_target.clone(),
                VerificationAggregateStateV1::Verified,
            ));
            match state.as_str() {
                "absent" => {}
                "verified" => aggregates.push(aggregate(
                    case_target,
                    &[VerificationOutcome::Verified],
                )),
                "rejected" => aggregates.push(aggregate(
                    case_target,
                    &[VerificationOutcome::Rejected],
                )),
                "inconclusive" => aggregates.push(aggregate(
                    case_target,
                    &[VerificationOutcome::Inconclusive],
                )),
                "conflicted" => aggregates.push(aggregate(
                    case_target,
                    &[VerificationOutcome::Verified, VerificationOutcome::Rejected],
                )),
                other => panic!("unexpected checkpoint fixture state: {other}"),
            }
        }
        let evaluation = checkpoint(requirements)
            .evaluate(&aggregates)
            .unwrap_or_else(|error| panic!("fixture {} failed: {error}", case.name));
        assert_eq!(
            evaluation.satisfied(),
            case.expected_satisfied,
            "{}",
            case.name
        );
    }
}

#[test]
fn invalid_checkpoint_fixtures_fail_closed() {
    let cases: Vec<InvalidCheckpointCase> = serde_json::from_str(include_str!(
        "../../../contracts/ecra-verify-v1/invalid/checkpoint-cases.json"
    ))
    .expect("invalid checkpoint fixtures");

    for case in cases {
        let bytes = serde_json::to_vec(&case.checkpoint).expect("fixture bytes");
        assert!(
            VerificationCheckpointV1::from_json_slice(&bytes).is_err(),
            "invalid fixture unexpectedly accepted: {}",
            case.name
        );
    }
}

#[test]
fn all_verified_requirements_satisfy_checkpoint() {
    let a = target("a");
    let b = target("b");
    let checkpoint = checkpoint(vec![
        requirement(a.clone(), VerificationAggregateStateV1::Verified),
        requirement(b.clone(), VerificationAggregateStateV1::Verified),
    ]);
    let aggregates = vec![
        aggregate(a, &[VerificationOutcome::Verified]),
        aggregate(b, &[VerificationOutcome::Verified]),
    ];

    let evaluation = checkpoint.evaluate(&aggregates).expect("evaluation");
    assert!(evaluation.satisfied());
    assert_eq!(evaluation.satisfied_targets().len(), 2);
    assert!(evaluation.unsatisfied_targets().is_empty());
    assert!(evaluation.conflicted_targets().is_empty());
}

#[test]
fn absent_rejected_and_inconclusive_do_not_satisfy_verified_requirement() {
    let absent = target("absent");
    let rejected = target("rejected");
    let inconclusive = target("inconclusive");
    let checkpoint = checkpoint(vec![
        requirement(absent.clone(), VerificationAggregateStateV1::Verified),
        requirement(rejected.clone(), VerificationAggregateStateV1::Verified),
        requirement(
            inconclusive.clone(),
            VerificationAggregateStateV1::Verified,
        ),
    ]);
    let aggregates = vec![
        aggregate(rejected, &[VerificationOutcome::Rejected]),
        aggregate(inconclusive, &[VerificationOutcome::Inconclusive]),
    ];

    let evaluation = checkpoint.evaluate(&aggregates).expect("evaluation");
    assert!(!evaluation.satisfied());
    assert_eq!(evaluation.unsatisfied_targets().len(), 3);
    assert!(evaluation.conflicted_targets().is_empty());
}

#[test]
fn conflict_is_reported_and_never_satisfies_checkpoint() {
    let conflicted = target("conflicted");
    let checkpoint = checkpoint(vec![requirement(
        conflicted.clone(),
        VerificationAggregateStateV1::Verified,
    )]);
    let aggregate = aggregate(
        conflicted.clone(),
        &[VerificationOutcome::Verified, VerificationOutcome::Rejected],
    );

    let evaluation = checkpoint.evaluate(&[aggregate]).expect("evaluation");
    assert!(!evaluation.satisfied());
    assert_eq!(evaluation.unsatisfied_targets(), &[conflicted.clone()]);
    assert_eq!(evaluation.conflicted_targets(), &[conflicted]);
}

#[test]
fn specialized_negative_requirement_may_accept_rejected() {
    let negative = target("must-not-exist");
    let checkpoint = checkpoint(vec![requirement(
        negative.clone(),
        VerificationAggregateStateV1::Rejected,
    )]);
    let aggregate = aggregate(negative, &[VerificationOutcome::Rejected]);

    let evaluation = checkpoint.evaluate(&[aggregate]).expect("evaluation");
    assert!(evaluation.satisfied());
}

#[test]
fn prohibited_satisfying_states_fail_closed() {
    for state in [
        VerificationAggregateStateV1::Absent,
        VerificationAggregateStateV1::Inconclusive,
        VerificationAggregateStateV1::Conflicted,
    ] {
        let error = VerificationRequirementV1::new(target("prohibited"), vec![state])
            .expect_err("prohibited state must fail");
        assert_eq!(error.code(), VerifyErrorCode::InvalidTarget);
    }
}

#[test]
fn duplicate_exact_targets_fail_closed() {
    let duplicate = target("duplicate");
    let error = VerificationCheckpointV1::from_fields(VerificationCheckpointFieldsV1 {
        id: checkpoint_id(),
        label: "duplicate target".to_owned(),
        requirements: vec![
            requirement(duplicate.clone(), VerificationAggregateStateV1::Verified),
            requirement(duplicate, VerificationAggregateStateV1::Rejected),
        ],
    })
    .expect_err("duplicate exact targets must fail");
    assert_eq!(error.code(), VerifyErrorCode::DuplicateId);
}

#[test]
fn requirement_count_limit_is_enforced() {
    let requirements = (0..=MAX_CHECKPOINT_REQUIREMENTS)
        .map(|index| {
            requirement(
                target(&format!("over-limit-{index}")),
                VerificationAggregateStateV1::Verified,
            )
        })
        .collect();
    let error = VerificationCheckpointV1::from_fields(VerificationCheckpointFieldsV1 {
        id: checkpoint_id(),
        label: "over limit".to_owned(),
        requirements,
    })
    .expect_err("checkpoint requirement limit must be enforced");
    assert_eq!(error.code(), VerifyErrorCode::ResourceLimitExceeded);
}

#[test]
fn strict_json_rejects_unknown_fields_and_invalid_requirement_states() {
    let unknown = br#"{
        "version":{"major":1,"minor":0},
        "id":"00000000-0000-0000-0000-000000006001",
        "label":"strict",
        "requirements":[{
            "target":{"kind":"claim","value":{"namespace":"checkpoint","reference":"strict"}},
            "accepted_states":["verified"]
        }],
        "unexpected":true
    }"#;
    assert!(VerificationCheckpointV1::from_json_slice(unknown).is_err());

    let invalid_state = br#"{
        "version":{"major":1,"minor":0},
        "id":"00000000-0000-0000-0000-000000006001",
        "label":"strict",
        "requirements":[{
            "target":{"kind":"claim","value":{"namespace":"checkpoint","reference":"strict"}},
            "accepted_states":["conflicted"]
        }]
    }"#;
    assert!(VerificationCheckpointV1::from_json_slice(invalid_state).is_err());
}

#[test]
fn requirement_input_order_has_canonical_checkpoint_bytes() {
    let a = requirement(target("a"), VerificationAggregateStateV1::Verified);
    let b = requirement(target("b"), VerificationAggregateStateV1::Rejected);
    let left = checkpoint(vec![a.clone(), b.clone()]);
    let right = checkpoint(vec![b, a]);

    assert_eq!(
        serde_jcs::to_vec(&left).expect("canonical left"),
        serde_jcs::to_vec(&right).expect("canonical right")
    );
}
