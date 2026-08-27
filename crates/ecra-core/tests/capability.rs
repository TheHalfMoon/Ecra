use ecra_core::{
    CapabilityGrant, CapabilityRequest, EpochMillis, EvaluationContext, OperationRef,
};

#[test]
fn operation_ref_rejects_empty_components() {
    assert!(OperationRef::new("", "read").is_err());
    assert!(OperationRef::new("browser", "").is_err());
}

#[test]
fn valid_capability_fixtures_parse_and_remain_distinct() {
    let request: CapabilityRequest = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/capability-request-narrow.json"
    ))
    .expect("valid capability request");
    let root: CapabilityGrant = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/capability-grant-root.json"
    ))
    .expect("valid root grant");
    let delegated: CapabilityGrant = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/capability-grant-delegated.json"
    ))
    .expect("valid delegated grant");

    assert_ne!(request.id().to_string(), root.id().to_string());
    assert_eq!(delegated.delegation().expect("delegation").depth(), 1);
}

#[test]
fn invalid_capability_fixtures_fail_closed() {
    assert!(
        serde_json::from_str::<CapabilityGrant>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/capability-request-as-grant.json"
        ))
        .is_err()
    );
    assert!(
        serde_json::from_str::<CapabilityRequest>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/capability-invalid-temporal.json"
        ))
        .is_err()
    );
    assert!(
        serde_json::from_str::<CapabilityRequest>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/capability-invalid-scope.json"
        ))
        .is_err()
    );
    assert!(
        serde_json::from_str::<CapabilityGrant>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/capability-invalid-delegation.json"
        ))
        .is_err()
    );
}

#[test]
fn temporal_evaluation_uses_only_caller_supplied_context() {
    let request: CapabilityRequest = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/capability-request-narrow.json"
    ))
    .expect("valid capability request");

    let before = EvaluationContext::new(EpochMillis::new(999).expect("safe time"));
    let during = EvaluationContext::new(EpochMillis::new(1_500).expect("safe time"));
    let after = EvaluationContext::new(EpochMillis::new(2_001).expect("safe time"));

    assert!(!request.is_temporally_valid_at(before));
    assert!(request.is_temporally_valid_at(during));
    assert!(!request.is_temporally_valid_at(after));
}
