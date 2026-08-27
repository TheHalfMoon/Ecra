use ecra_core::{
    ActionAttemptRef, ActionIntent, ActionOutcome, ActionReceipt, Actor, IdentityAssertionRef,
    PrincipalRef, ResourceRef, Scope, ScopeConstraint, VerificationOutcome, VerificationReceipt,
    WebOrigin, WorkspaceId,
};

#[test]
fn phase3_valid_contract_fixtures_parse() {
    serde_json::from_str::<Actor>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/actor-agent.json"
    ))
    .expect("valid actor fixture");
    serde_json::from_str::<PrincipalRef>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/principal-ref.json"
    ))
    .expect("valid principal fixture");
    serde_json::from_str::<IdentityAssertionRef>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/identity-assertion-ref.json"
    ))
    .expect("valid identity assertion fixture");
    serde_json::from_str::<WebOrigin>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/web-origin-tuple.json"
    ))
    .expect("valid tuple origin fixture");
    serde_json::from_str::<WebOrigin>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/web-origin-opaque.json"
    ))
    .expect("valid opaque origin fixture");
    serde_json::from_str::<ResourceRef>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/resource-ref.json"
    ))
    .expect("valid resource fixture");
    serde_json::from_str::<Scope>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/scope-explicit.json"
    ))
    .expect("valid explicit scope fixture");
    serde_json::from_str::<ScopeConstraint<WorkspaceId>>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/scope-one-of.json"
    ))
    .expect("valid non-empty one_of fixture");
}

#[test]
fn phase3_invalid_contract_fixtures_fail_closed() {
    assert!(
        serde_json::from_str::<WebOrigin>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/web-origin-empty-host.json"
        ))
        .is_err()
    );
    assert!(
        serde_json::from_str::<ResourceRef>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/resource-empty-locator.json"
        ))
        .is_err()
    );
    assert!(
        serde_json::from_str::<ScopeConstraint<WorkspaceId>>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/scope-one-of-empty.json"
        ))
        .is_err()
    );
    assert!(
        serde_json::from_str::<Scope>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/scope-implicit-wildcard.json"
        ))
        .is_err()
    );
    assert!(
        serde_json::from_str::<PrincipalRef>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/principal-actor-field-mismatch.json"
        ))
        .is_err()
    );
}

#[test]
fn phase7_valid_action_fixtures_parse() {
    for fixture in [
        include_str!("../../../contracts/ecra-domain-v1/valid/action-read-only.json"),
        include_str!("../../../contracts/ecra-domain-v1/valid/action-irreversible-local.json"),
        include_str!("../../../contracts/ecra-domain-v1/valid/action-reversible-external.json"),
        include_str!("../../../contracts/ecra-domain-v1/valid/action-keyed-idempotent.json"),
        include_str!("../../../contracts/ecra-domain-v1/valid/action-unknown-conservative.json"),
        include_str!("../../../contracts/ecra-domain-v1/valid/action-digest-golden.json"),
    ] {
        serde_json::from_str::<ActionIntent>(fixture).expect("valid phase 7 action fixture");
    }
}

#[test]
fn phase7_invalid_action_fixtures_fail_closed() {
    for fixture in [
        include_str!(
            "../../../contracts/ecra-domain-v1/invalid/action-invalid-none-reversible.json"
        ),
        include_str!(
            "../../../contracts/ecra-domain-v1/invalid/action-invalid-local-not-applicable.json"
        ),
        include_str!("../../../contracts/ecra-domain-v1/invalid/action-invalid-key-missing.json"),
        include_str!(
            "../../../contracts/ecra-domain-v1/invalid/action-invalid-non-idempotent-safe.json"
        ),
        include_str!("../../../contracts/ecra-domain-v1/invalid/action-invalid-unknown-safe.json"),
    ] {
        assert!(
            serde_json::from_str::<ActionIntent>(fixture).is_err(),
            "invalid phase 7 action fixture must fail closed"
        );
    }
}

#[test]
fn phase8_two_attempts_bind_one_exact_action_ref() {
    let intent = serde_json::from_str::<ActionIntent>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/action-digest-golden.json"
    ))
    .expect("golden action intent");
    let first = serde_json::from_str::<ActionAttemptRef>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/action-attempt-1.json"
    ))
    .expect("first action attempt");
    let second = serde_json::from_str::<ActionAttemptRef>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/action-attempt-2.json"
    ))
    .expect("second action attempt");

    assert_ne!(first.id(), second.id());
    assert_eq!(first.action(), second.action());
    first
        .validate_for(&intent)
        .expect("first exact action binding");
    second
        .validate_for(&intent)
        .expect("second exact action binding");
}

#[test]
fn phase8_receipts_bind_attempt_and_preserve_unknown() {
    let intent = serde_json::from_str::<ActionIntent>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/action-digest-golden.json"
    ))
    .expect("golden action intent");
    let unknown = serde_json::from_str::<ActionReceipt>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/action-receipt-unknown.json"
    ))
    .expect("unknown receipt");
    let success = serde_json::from_str::<ActionReceipt>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/action-receipt-success.json"
    ))
    .expect("executor success receipt");
    let failure = serde_json::from_str::<ActionReceipt>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/action-receipt-failure.json"
    ))
    .expect("executor failure receipt");

    unknown
        .validate_for(&intent)
        .expect("unknown exact binding");
    success
        .validate_for(&intent)
        .expect("success exact binding");
    failure
        .validate_for(&intent)
        .expect("failure exact binding");
    assert_eq!(unknown.outcome(), ActionOutcome::Unknown);
    assert_eq!(success.outcome(), ActionOutcome::ExecutorObservedSuccess);
    assert_eq!(failure.outcome(), ActionOutcome::ExecutorObservedFailure);
    assert_ne!(success.attempt().id(), failure.attempt().id());

    let serialized = serde_json::to_string(&unknown).expect("serialize unknown receipt");
    let round_trip =
        serde_json::from_str::<ActionReceipt>(&serialized).expect("round-trip receipt");
    assert_eq!(round_trip, unknown);
    assert_eq!(round_trip.outcome(), ActionOutcome::Unknown);
}

#[test]
fn phase8_verification_outcomes_are_independent_records() {
    let verified = serde_json::from_str::<VerificationReceipt>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/verification-verified.json"
    ))
    .expect("verified receipt");
    let rejected = serde_json::from_str::<VerificationReceipt>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/verification-rejected.json"
    ))
    .expect("rejected receipt");
    let inconclusive = serde_json::from_str::<VerificationReceipt>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/verification-inconclusive.json"
    ))
    .expect("inconclusive receipt");
    let not_evaluated = serde_json::from_str::<VerificationReceipt>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/verification-not-evaluated.json"
    ))
    .expect("not-evaluated receipt");

    assert_eq!(verified.outcome(), VerificationOutcome::Verified);
    assert_eq!(rejected.outcome(), VerificationOutcome::Rejected);
    assert_eq!(inconclusive.outcome(), VerificationOutcome::Inconclusive);
    assert_eq!(not_evaluated.outcome(), VerificationOutcome::NotEvaluated);
    assert!(!verified.evidence().is_empty());
    assert!(not_evaluated.evidence().is_empty());
}

#[test]
fn phase8_executor_success_is_not_verification() {
    let receipt_fixture = include_str!(
        "../../../contracts/ecra-domain-v1/invalid/action-receipt-type-confusion.json"
    );
    let receipt =
        serde_json::from_str::<ActionReceipt>(receipt_fixture).expect("valid receipt shape");
    assert_eq!(receipt.outcome(), ActionOutcome::ExecutorObservedSuccess);
    assert!(serde_json::from_str::<VerificationReceipt>(receipt_fixture).is_err());

    let receipt_json = serde_json::to_value(&receipt).expect("receipt json");
    assert_eq!(receipt_json["outcome"], "executor_observed_success");
    let verification = serde_json::from_str::<VerificationReceipt>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/verification-verified.json"
    ))
    .expect("verified receipt");
    let verification_json = serde_json::to_value(&verification).expect("verification json");
    assert_eq!(verification_json["outcome"], "verified");
}

#[test]
fn phase8_invalid_contracts_fail_closed() {
    let intent = serde_json::from_str::<ActionIntent>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/action-digest-golden.json"
    ))
    .expect("golden action intent");
    let wrong_attempt = serde_json::from_str::<ActionAttemptRef>(include_str!(
        "../../../contracts/ecra-domain-v1/invalid/action-attempt-wrong-ref.json"
    ))
    .expect("structurally valid attempt with wrong external binding");
    assert!(wrong_attempt.validate_for(&intent).is_err());

    assert!(
        serde_json::from_str::<ActionReceipt>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/action-receipt-invalid-timing.json"
        ))
        .is_err()
    );

    for fixture in [
        include_str!("../../../contracts/ecra-domain-v1/invalid/verification-missing-target.json"),
        include_str!(
            "../../../contracts/ecra-domain-v1/invalid/verification-missing-verifier.json"
        ),
        include_str!(
            "../../../contracts/ecra-domain-v1/invalid/verification-missing-evidence.json"
        ),
        include_str!(
            "../../../contracts/ecra-domain-v1/invalid/verification-verified-empty-evidence.json"
        ),
    ] {
        assert!(
            serde_json::from_str::<VerificationReceipt>(fixture).is_err(),
            "invalid verification fixture must fail closed"
        );
    }
}
