fn block<'a>(source: &'a str, start: &str, end: &str) -> &'a str {
    let start_index = source.find(start).expect("start marker");
    let tail = &source[start_index..];
    let end_index = tail.find(end).expect("end marker");
    &tail[..end_index]
}

#[test]
fn validated_context_exposes_identity_evidence_only() {
    let source = include_str!("../src/validation.rs");
    let validated = block(
        source,
        "pub struct ValidatedIdentityContext {",
        "impl ValidatedIdentityContext",
    );
    for forbidden in [
        "CapabilityGrant",
        "AuthorizationDecision",
        "DeclassificationDecision",
        "Approval",
        "ExecutionLease",
        "SensitiveBytes",
        "SigningKey",
    ] {
        assert!(
            !validated.contains(forbidden),
            "forbidden authority/secret field: {forbidden}"
        );
    }
}

#[test]
fn verified_snapshot_cannot_be_deserialized_or_publicly_fabricated() {
    let source = include_str!("../src/key.rs");
    let snapshot = block(
        source,
        "pub struct VerifiedTrustSnapshot {",
        "impl VerifiedTrustSnapshot",
    );
    assert!(!snapshot.contains("Deserialize"));

    let implementation = block(source, "impl VerifiedTrustSnapshot {", "fn lifecycle_error");
    assert!(implementation.contains("pub(crate) fn from_authenticated_parts"));
    assert!(!implementation.contains("pub fn from_authenticated_parts"));
}

#[test]
fn handles_and_sessions_are_non_serializable_process_local_capabilities() {
    let bootstrap = include_str!("../src/bootstrap.rs");
    let handle = block(
        bootstrap,
        "pub struct EnrolledPrincipalHandle {",
        "impl EnrolledPrincipalHandle",
    );
    assert!(!handle.contains("Serialize"));
    assert!(!handle.contains("Deserialize"));

    let issuance = include_str!("../src/issuance.rs");
    let session = block(issuance, "pub struct IssuerSession {", "impl IssuerSession");
    assert!(!session.contains("Serialize"));
    assert!(!session.contains("Deserialize"));
    assert!(session.contains("SigningKey"));
}

#[test]
fn pure_validation_has_no_ambient_io_clock_or_authorization_dependencies() {
    let source = include_str!("../src/validation.rs");
    let production = source.split("#[cfg(test)]").next().unwrap();
    for forbidden in [
        "SystemTime",
        "Instant::now",
        "std::env",
        "std::fs",
        "std::net",
        "reqwest",
        "tokio",
        "CapabilityGrant",
        "AuthorizationDecision",
    ] {
        assert!(
            !production.contains(forbidden),
            "ambient/authority dependency: {forbidden}"
        );
    }
}

#[test]
fn free_form_metadata_cannot_become_principal_identity() {
    let sources = [
        include_str!("../src/bootstrap.rs"),
        include_str!("../src/assertion.rs"),
        include_str!("../src/issuance.rs"),
        include_str!("../src/validation.rs"),
    ];

    for source in sources {
        let production = source.split("#[cfg(test)]").next().unwrap();
        assert!(
            !production.contains("PrincipalId::parse_str"),
            "production identity code must not derive PrincipalId from text"
        );
        for forbidden in [
            "username",
            "user_name",
            "email",
            "display_label",
            "PathBuf",
            "std::path",
            "protocol_subject",
        ] {
            assert!(
                !production.contains(forbidden),
                "free-form principal derivation surface: {forbidden}"
            );
        }
    }
}
