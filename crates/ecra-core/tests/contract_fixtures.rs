use ecra_core::{
    ActionIntent, Actor, IdentityAssertionRef, PrincipalRef, ResourceRef, Scope, ScopeConstraint,
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
        include_str!(
            "../../../contracts/ecra-domain-v1/invalid/action-invalid-key-missing.json"
        ),
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
