use ecra_core::{
    Actor, ActorId, ActorKind, IdentityAssertionId, IdentityAssertionRef, OpaqueOriginId,
    PrincipalId, PrincipalRef, ResourceId, ResourceKind, ResourceRef, Scope, ScopeConstraint,
    WebOrigin, WorkspaceId,
};
use uuid::Uuid;

fn uuid(value: &str) -> Uuid {
    Uuid::parse_str(value).expect("fixture UUID")
}

#[test]
fn actor_attribution_and_principal_reference_are_independent() {
    let actor = Actor::new(
        ActorId::from_uuid(uuid("00000000-0000-0000-0000-000000000001")),
        ActorKind::Agent,
        Some("admin@example.invalid".to_owned()),
    );
    let principal = PrincipalRef::new(PrincipalId::from_uuid(uuid(
        "00000000-0000-0000-0000-000000000002",
    )));
    let assertion = IdentityAssertionRef::new(
        IdentityAssertionId::from_uuid(uuid("00000000-0000-0000-0000-000000000003")),
        principal.id(),
    );

    assert_eq!(actor.kind(), ActorKind::Agent);
    assert_ne!(actor.id().to_string(), principal.id().to_string());
    assert_eq!(assertion.principal(), principal.id());
    assert_eq!(actor.label(), Some("admin@example.invalid"));
}

#[test]
fn tuple_web_origin_normalizes_host_scheme_and_default_port() {
    let origin = WebOrigin::from_url_str("HTTPS://Example.COM:443/a/path?q=1#fragment")
        .expect("valid tuple origin");

    assert_eq!(origin.scheme(), Some("https"));
    assert_eq!(origin.host(), Some("example.com"));
    assert_eq!(origin.port(), None);

    let equivalent = WebOrigin::tuple("https", "example.com", None).expect("canonical origin");
    assert_eq!(origin, equivalent);
}

#[test]
fn opaque_origins_do_not_collapse_together() {
    let first = WebOrigin::opaque(OpaqueOriginId::from_uuid(uuid(
        "00000000-0000-0000-0000-000000000010",
    )));
    let second = WebOrigin::opaque(OpaqueOriginId::from_uuid(uuid(
        "00000000-0000-0000-0000-000000000011",
    )));

    assert_ne!(first, second);
}

#[test]
fn scope_requires_explicit_wildcard_and_rejects_empty_one_of() {
    let workspace = WorkspaceId::from_uuid(uuid("00000000-0000-0000-0000-000000000020"));
    let scope = Scope::not_applicable().with_workspace(ScopeConstraint::exact(workspace));

    assert!(matches!(scope.workspace(), ScopeConstraint::Exact(value) if *value == workspace));

    let any: ScopeConstraint<WorkspaceId> =
        serde_json::from_str(r#"{"kind":"any_explicit"}"#).expect("explicit wildcard");
    assert!(matches!(any, ScopeConstraint::AnyExplicit));

    let empty = serde_json::from_str::<ScopeConstraint<WorkspaceId>>(
        r#"{"kind":"one_of","value":[]}"#,
    );
    assert!(empty.is_err());
}

#[test]
fn missing_scope_dimensions_are_not_implicitly_unrestricted() {
    let partial = r#"{
        "workspace":{"kind":"any_explicit"},
        "browser_space":{"kind":"not_applicable"}
    }"#;

    assert!(serde_json::from_str::<Scope>(partial).is_err());
}

#[test]
fn resource_locator_is_metadata_not_resource_identity() {
    let id = ResourceId::from_uuid(uuid("00000000-0000-0000-0000-000000000030"));
    let first = ResourceRef::new(
        id,
        ResourceKind::LocalResource,
        Some("/workspace/report.txt".to_owned()),
        None,
    )
    .expect("valid resource");
    let moved = ResourceRef::new(
        id,
        ResourceKind::LocalResource,
        Some("/workspace/archive/report.txt".to_owned()),
        None,
    )
    .expect("valid resource");

    assert_eq!(first.id(), moved.id());
    assert_ne!(first.locator(), moved.locator());
}

#[test]
fn empty_resource_locator_is_rejected() {
    let id = ResourceId::from_uuid(uuid("00000000-0000-0000-0000-000000000031"));
    assert!(ResourceRef::new(id, ResourceKind::Abstract, Some(String::new()), None).is_err());
}
