use std::collections::HashSet;

use ecra_core::{
    ActorId, EvidenceId, EvidenceKind, EvidenceRef, PrincipalId, PrincipalRef, ReceiptId,
    SchemaVersion, VerificationId, VerificationMethod, VerificationOutcome, VerificationTarget,
};
use ecra_verify::{
    CheckpointId, MAX_EVIDENCE_REFS_PER_REQUEST, ReconciliationId, VerificationRequestFieldsV1,
    VerificationRequestV1, VerifyErrorCode,
};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize)]
struct InvalidCase {
    name: String,
    request: Value,
}

fn target_kind(target: &VerificationTarget) -> &'static str {
    match target {
        VerificationTarget::Action(_) => "action",
        VerificationTarget::ActionAttempt(_) => "action_attempt",
        VerificationTarget::Receipt(_) => "receipt",
        VerificationTarget::Fact(_) => "fact",
        VerificationTarget::Artifact(_) => "artifact",
        VerificationTarget::Claim(_) => "claim",
    }
}

fn parse_receipt_id(value: &str) -> ReceiptId {
    ReceiptId::parse_str(value).expect("valid receipt id")
}

fn evidence(index: usize) -> EvidenceRef {
    let id = EvidenceId::parse_str(&format!("00000000-0000-0000-0000-{:012}", 500 + index))
        .expect("valid evidence id");
    EvidenceRef::new(id, EvidenceKind::Other)
}

#[test]
fn ecr004_ids_reject_nil_uuid() {
    let checkpoint = CheckpointId::from_uuid(Uuid::nil()).expect_err("nil checkpoint id rejects");
    let reconciliation =
        ReconciliationId::from_uuid(Uuid::nil()).expect_err("nil reconciliation id rejects");
    assert_eq!(checkpoint.code(), VerifyErrorCode::InvalidIdentifier);
    assert_eq!(reconciliation.code(), VerifyErrorCode::InvalidIdentifier);
}

#[test]
fn valid_fixture_covers_all_targets_methods_and_outcomes() {
    let requests: Vec<VerificationRequestV1> = serde_json::from_str(include_str!(
        "../../../contracts/ecra-verify-v1/valid/request-coverage.json"
    ))
    .expect("valid request fixture matrix");

    let mut targets = HashSet::new();
    let mut methods = HashSet::new();
    let mut outcomes = HashSet::new();
    for request in &requests {
        assert_eq!(request.version(), SchemaVersion::V1_0);
        targets.insert(target_kind(request.target()));
        methods.insert(
            serde_json::to_value(request.method())
                .expect("serialize method")
                .as_str()
                .expect("method string")
                .to_owned(),
        );
        outcomes.insert(
            serde_json::to_value(request.proposed_outcome())
                .expect("serialize outcome")
                .as_str()
                .expect("outcome string")
                .to_owned(),
        );
    }

    assert_eq!(targets.len(), 6);
    assert_eq!(methods.len(), 8);
    assert_eq!(outcomes.len(), 4);
}

#[test]
fn invalid_fixture_cases_fail_closed() {
    let cases: Vec<InvalidCase> = serde_json::from_str(include_str!(
        "../../../contracts/ecra-verify-v1/invalid/request-cases.json"
    ))
    .expect("invalid fixture container");

    for case in cases {
        let encoded = serde_json::to_vec(&case.request).expect("serialize invalid fixture case");
        let result = VerificationRequestV1::from_json_slice(&encoded);
        assert!(
            result.is_err(),
            "invalid fixture unexpectedly accepted: {}",
            case.name
        );
    }
}

#[test]
fn unsupported_version_and_duplicate_evidence_have_machine_codes() {
    let cases: Vec<InvalidCase> = serde_json::from_str(include_str!(
        "../../../contracts/ecra-verify-v1/invalid/request-cases.json"
    ))
    .expect("invalid fixture container");

    for (name, expected) in [
        ("unsupported_version", VerifyErrorCode::UnsupportedVersion),
        ("duplicate_evidence", VerifyErrorCode::DuplicateId),
    ] {
        let case = cases.iter().find(|case| case.name == name).expect("case");
        let encoded = serde_json::to_vec(&case.request).expect("serialize case");
        let error = VerificationRequestV1::from_json_slice(&encoded).expect_err("must reject");
        assert_eq!(error.code(), expected);
    }
}

#[test]
fn evidence_count_limit_is_checked_before_receipt_construction() {
    let fields = VerificationRequestFieldsV1 {
        receipt_id: VerificationId::parse_str("00000000-0000-0000-0000-000000000601")
            .expect("verification id"),
        verifier: ActorId::parse_str("00000000-0000-0000-0000-000000000001").expect("actor id"),
        verifier_principal: None,
        target: VerificationTarget::Receipt(parse_receipt_id(
            "00000000-0000-0000-0000-000000000602",
        )),
        method: VerificationMethod::Other,
        evidence: (0..=MAX_EVIDENCE_REFS_PER_REQUEST).map(evidence).collect(),
        proposed_outcome: VerificationOutcome::NotEvaluated,
        evaluated_at: None,
        rule_id: "limit_check".to_owned(),
        notes: None,
    };

    let error = VerificationRequestV1::from_fields(fields).expect_err("over limit rejects");
    assert_eq!(error.code(), VerifyErrorCode::ResourceLimitExceeded);
}

#[test]
fn validated_request_constructs_only_the_canonical_receipt_shape() {
    let evidence = evidence(0);
    let fields = VerificationRequestFieldsV1 {
        receipt_id: VerificationId::parse_str("00000000-0000-0000-0000-000000000701")
            .expect("verification id"),
        verifier: ActorId::parse_str("00000000-0000-0000-0000-000000000001").expect("actor id"),
        verifier_principal: Some(PrincipalRef::new(
            PrincipalId::parse_str("00000000-0000-0000-0000-000000000002").expect("principal id"),
        )),
        target: VerificationTarget::Receipt(parse_receipt_id(
            "00000000-0000-0000-0000-000000000702",
        )),
        method: VerificationMethod::StructuredExternalState,
        evidence: vec![evidence],
        proposed_outcome: VerificationOutcome::Verified,
        evaluated_at: Some(ecra_core::EpochMillis::new(42).expect("time")),
        rule_id: "exact_rule".to_owned(),
        notes: Some("bounded note".to_owned()),
    };
    let request = VerificationRequestV1::from_fields(fields).expect("valid request");
    let receipt = request.to_receipt().expect("canonical receipt");
    let request_json = serde_json::to_value(&request).expect("serialize request");
    let receipt_json = serde_json::to_value(&receipt).expect("serialize receipt");

    assert_eq!(receipt_json["id"], request_json["receipt_id"]);
    assert_eq!(receipt_json["verifier"], request_json["verifier"]);
    assert_eq!(
        receipt_json["verifier_principal"],
        request_json["verifier_principal"]
    );
    assert_eq!(receipt_json["target"], request_json["target"]);
    assert_eq!(receipt_json["method"], request_json["method"]);
    assert_eq!(receipt_json["evidence"], request_json["evidence"]);
    assert_eq!(receipt_json["outcome"], request_json["proposed_outcome"]);
    assert_eq!(receipt_json["evaluated_at"], request_json["evaluated_at"]);
    assert_eq!(receipt_json["notes"], request_json["notes"]);
    assert!(receipt_json.get("rule_id").is_none());
    assert!(receipt_json.get("verified").is_none());
}
