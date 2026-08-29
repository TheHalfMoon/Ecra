use ecra_core::{
    ActorId, ClaimRef, EvidenceId, EvidenceKind, EvidenceRef, VerificationId, VerificationMethod,
    VerificationOutcome, VerificationReceipt, VerificationTarget,
};
use ecra_verify::{VerificationAggregateStateV1, VerificationAggregateViewV1};
use serde::Deserialize;

fn verification_id(value: usize) -> VerificationId {
    VerificationId::parse_str(&format!("00000000-0000-0000-0000-{:012}", 4000 + value))
        .expect("verification id")
}

fn evidence_id(value: usize) -> EvidenceId {
    EvidenceId::parse_str(&format!("00000000-0000-0000-0000-{:012}", 5000 + value))
        .expect("evidence id")
}

fn target() -> VerificationTarget {
    VerificationTarget::Claim(ClaimRef::new("aggregate", "phase3").expect("claim target"))
}

fn receipt(value: usize, outcome: VerificationOutcome) -> VerificationReceipt {
    let evidence = if outcome == VerificationOutcome::NotEvaluated {
        Vec::new()
    } else {
        vec![EvidenceRef::new(
            evidence_id(value),
            EvidenceKind::Computation,
        )]
    };
    VerificationReceipt::new(
        verification_id(value),
        ActorId::parse_str("00000000-0000-0000-0000-000000000001").expect("actor id"),
        target(),
        VerificationMethod::DeterministicComputation,
        outcome,
        evidence,
    )
    .expect("verification receipt")
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AggregateFixture {
    version: String,
    cases: Vec<AggregateCase>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AggregateCase {
    name: String,
    outcomes: Vec<String>,
    expected_state: String,
}

fn outcome(value: &str) -> VerificationOutcome {
    match value {
        "verified" => VerificationOutcome::Verified,
        "rejected" => VerificationOutcome::Rejected,
        "inconclusive" => VerificationOutcome::Inconclusive,
        "not_evaluated" => VerificationOutcome::NotEvaluated,
        other => panic!("unexpected fixture outcome: {other}"),
    }
}

fn state(value: &str) -> VerificationAggregateStateV1 {
    match value {
        "absent" => VerificationAggregateStateV1::Absent,
        "verified" => VerificationAggregateStateV1::Verified,
        "rejected" => VerificationAggregateStateV1::Rejected,
        "inconclusive" => VerificationAggregateStateV1::Inconclusive,
        "conflicted" => VerificationAggregateStateV1::Conflicted,
        other => panic!("unexpected fixture state: {other}"),
    }
}

#[test]
fn aggregation_fixture_matrix_preserves_conflict_and_not_evaluated_semantics() {
    let fixture: AggregateFixture = serde_json::from_str(include_str!(
        "../../../contracts/ecra-verify-v1/valid/aggregate-cases.json"
    ))
    .expect("aggregate fixture");
    assert_eq!(fixture.version, "1.0");

    for (case_index, case) in fixture.cases.iter().enumerate() {
        let receipts = case
            .outcomes
            .iter()
            .enumerate()
            .map(|(index, value)| receipt(case_index * 10 + index + 1, outcome(value)))
            .collect::<Vec<_>>();
        let aggregate = VerificationAggregateViewV1::from_receipts(target(), &receipts)
            .unwrap_or_else(|error| panic!("fixture {} failed: {error}", case.name));
        assert_eq!(
            aggregate.state(),
            state(&case.expected_state),
            "{}",
            case.name
        );
        assert_eq!(
            aggregate.receipt_ids().len(),
            receipts.len(),
            "{}",
            case.name
        );
    }
}

#[test]
fn verified_and_rejected_is_conflicted_with_both_receipts_retained() {
    let verified = receipt(101, VerificationOutcome::Verified);
    let rejected = receipt(102, VerificationOutcome::Rejected);
    let aggregate =
        VerificationAggregateViewV1::from_receipts(target(), &[verified.clone(), rejected.clone()])
            .expect("aggregate");

    assert_eq!(aggregate.state(), VerificationAggregateStateV1::Conflicted);
    assert_eq!(aggregate.verified_ids(), &[verified.id()]);
    assert_eq!(aggregate.rejected_ids(), &[rejected.id()]);
    assert_eq!(aggregate.receipt_ids(), &[verified.id(), rejected.id()]);
}

#[test]
fn not_evaluated_alone_is_absent_and_never_satisfies_verification() {
    let receipt = receipt(103, VerificationOutcome::NotEvaluated);
    let aggregate =
        VerificationAggregateViewV1::from_receipts(target(), std::slice::from_ref(&receipt))
            .expect("aggregate");

    assert_eq!(aggregate.state(), VerificationAggregateStateV1::Absent);
    assert!(aggregate.verified_ids().is_empty());
    assert!(aggregate.rejected_ids().is_empty());
    assert!(aggregate.inconclusive_ids().is_empty());
    assert_eq!(aggregate.not_evaluated_ids(), &[receipt.id()]);
}

#[test]
fn receipt_order_permutations_are_byte_equivalent() {
    let a = receipt(201, VerificationOutcome::Verified);
    let b = receipt(202, VerificationOutcome::Inconclusive);
    let c = receipt(203, VerificationOutcome::NotEvaluated);
    let permutations = [
        vec![a.clone(), b.clone(), c.clone()],
        vec![a.clone(), c.clone(), b.clone()],
        vec![b.clone(), a.clone(), c.clone()],
        vec![b.clone(), c.clone(), a.clone()],
        vec![c.clone(), a.clone(), b.clone()],
        vec![c, b, a],
    ];

    let expected = serde_jcs::to_vec(
        &VerificationAggregateViewV1::from_receipts(target(), &permutations[0]).expect("aggregate"),
    )
    .expect("canonical aggregate");

    for receipts in permutations.iter().skip(1) {
        let aggregate =
            VerificationAggregateViewV1::from_receipts(target(), receipts).expect("aggregate");
        let bytes = serde_jcs::to_vec(&aggregate).expect("canonical aggregate");
        assert_eq!(bytes, expected);
    }
}

#[test]
fn one_thousand_identical_aggregate_evaluations_are_byte_equivalent() {
    let receipts = vec![
        receipt(301, VerificationOutcome::Verified),
        receipt(302, VerificationOutcome::Inconclusive),
        receipt(303, VerificationOutcome::NotEvaluated),
    ];
    let expected = serde_jcs::to_vec(
        &VerificationAggregateViewV1::from_receipts(target(), &receipts).expect("aggregate"),
    )
    .expect("canonical aggregate");

    for _ in 0..1_000 {
        let aggregate =
            VerificationAggregateViewV1::from_receipts(target(), &receipts).expect("aggregate");
        assert_eq!(
            serde_jcs::to_vec(&aggregate).expect("canonical aggregate"),
            expected
        );
    }
}
