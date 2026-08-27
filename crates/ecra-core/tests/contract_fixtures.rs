use ecra_core::{
    Actor, IdentityAssertionRef, PrincipalRef, ResourceRef, Scope, ScopeConstraint, WebOrigin,
    WorkspaceId,
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
