use std::{
    collections::{BTreeSet, HashSet},
    fs,
    path::PathBuf,
};

use ecra_core::{
    ActionAttemptRef, ActionIntent, ActionReceipt, ActionRef, ArtifactRef, CapabilityGrant,
    DomainError, ErrorCategory, ErrorCode, EvidenceRef, Fact, FreshnessAssessment,
    InformationClassification, InformationRef, InformationUse, LineageRef, Origin, PrincipalRef,
    ResourceRef, Scope, ScopeConstraint, VerificationReceipt, Versioned, WebOrigin, WorkspaceId,
};
use serde::Deserialize;

#[rustfmt::skip]
const INVALID_FIXTURES: &[(&str, &str, ErrorCode)] = &[
    ("action-attempt-wrong-ref.json", "action_attempt", ErrorCode::InvalidAttempt),
    ("action-invalid-key-missing.json", "action_intent", ErrorCode::SerializationFailed),
    ("action-invalid-local-not-applicable.json", "action_intent", ErrorCode::SerializationFailed),
    ("action-invalid-non-idempotent-safe.json", "action_intent", ErrorCode::SerializationFailed),
    ("action-invalid-none-reversible.json", "action_intent", ErrorCode::SerializationFailed),
    ("action-invalid-unknown-safe.json", "action_intent", ErrorCode::SerializationFailed),
    ("action-receipt-invalid-timing.json", "action_receipt", ErrorCode::SerializationFailed),
    ("action-receipt-type-confusion.json", "verification_receipt", ErrorCode::SerializationFailed),
    ("action-ref-wrong-digest.json", "action_ref_binding", ErrorCode::InvalidAction),
    ("artifact-invalid-byte-size.json", "artifact_ref", ErrorCode::SerializationFailed),
    ("artifact-invalid-digest.json", "artifact_ref", ErrorCode::SerializationFailed),
    ("capability-invalid-delegation.json", "capability_grant", ErrorCode::SerializationFailed),
    ("capability-invalid-scope.json", "capability_grant", ErrorCode::SerializationFailed),
    ("capability-invalid-temporal.json", "capability_grant", ErrorCode::SerializationFailed),
    ("capability-request-as-grant.json", "capability_grant", ErrorCode::SerializationFailed),
    ("classification-empty-tag.json", "information_classification", ErrorCode::SerializationFailed),
    ("classification-invalid-class.json", "information_classification", ErrorCode::SerializationFailed),
    ("evidence-empty-external-ref.json", "evidence_ref", ErrorCode::SerializationFailed),
    ("fact-integer-outside-ijson.json", "fact", ErrorCode::SerializationFailed),
    ("fact-invalid-decimal.json", "fact", ErrorCode::SerializationFailed),
    ("fact-verified-flag.json", "fact", ErrorCode::SerializationFailed),
    ("freshness-unpaired-basis.json", "freshness_assessment", ErrorCode::SerializationFailed),
    ("information-ref-unknown-field.json", "information_ref", ErrorCode::SerializationFailed),
    ("information-use-empty-sources.json", "information_use", ErrorCode::SerializationFailed),
    ("information-use-invalid-destination.json", "information_use", ErrorCode::SerializationFailed),
    ("lineage-ref-unknown-field.json", "lineage_ref", ErrorCode::SerializationFailed),
    ("origin-unknown-field.json", "origin", ErrorCode::SerializationFailed),
    ("principal-actor-field-mismatch.json", "principal_ref", ErrorCode::SerializationFailed),
    ("resource-empty-locator.json", "resource_ref", ErrorCode::SerializationFailed),
    ("scope-implicit-wildcard.json", "scope", ErrorCode::SerializationFailed),
    ("scope-one-of-empty.json", "scope_constraint", ErrorCode::SerializationFailed),
    ("verification-missing-evidence.json", "verification_receipt", ErrorCode::SerializationFailed),
    ("verification-missing-target.json", "verification_receipt", ErrorCode::SerializationFailed),
    ("verification-missing-verifier.json", "verification_receipt", ErrorCode::SerializationFailed),
    ("verification-verified-empty-evidence.json", "verification_receipt", ErrorCode::SerializationFailed),
    ("version-unknown-field.json", "versioned_actor", ErrorCode::SerializationFailed),
    ("version-unsupported-major.json", "versioned_actor", ErrorCode::UnsupportedMajorVersion),
    ("version-unsupported-minor.json", "versioned_actor", ErrorCode::UnsupportedMinorVersion),
    ("web-origin-empty-host.json", "web_origin", ErrorCode::SerializationFailed),
];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionRefFixture {
    action: ActionIntent,
    reference: ActionRef,
}

fn invalid_fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../contracts/ecra-domain-v1/invalid")
}

fn valid_fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../contracts/ecra-domain-v1/valid")
}

fn discovered_fixture_names() -> BTreeSet<String> {
    fs::read_dir(invalid_fixture_dir())
        .expect("read invalid fixture directory")
        .map(|entry| entry.expect("invalid fixture directory entry"))
        .filter(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|extension| extension == "json")
        })
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect()
}

fn invalid_fixture_text(name: &str) -> String {
    fs::read_to_string(invalid_fixture_dir().join(name))
        .unwrap_or_else(|error| panic!("read invalid fixture {name}: {error}"))
}

fn valid_fixture_text(name: &str) -> String {
    fs::read_to_string(valid_fixture_dir().join(name))
        .unwrap_or_else(|error| panic!("read valid fixture {name}: {error}"))
}

fn serialization_failure<T>(text: &str) -> DomainError
where
    T: serde::de::DeserializeOwned,
{
    match serde_json::from_str::<T>(text) {
        Ok(_) => panic!("invalid fixture unexpectedly parsed"),
        Err(error) => DomainError::Serialization(error.to_string()),
    }
}

fn invalid_error(name: &str, kind: &str, text: &str) -> DomainError {
    match kind {
        "action_attempt" => {
            let attempt: ActionAttemptRef = serde_json::from_str(text).unwrap_or_else(|error| {
                panic!("{name} should remain structurally parseable: {error}")
            });
            let intent: ActionIntent =
                serde_json::from_str(&valid_fixture_text("action-digest-golden.json"))
                    .expect("golden action intent");
            attempt
                .validate_for(&intent)
                .expect_err("wrong action-attempt binding must fail")
        }
        "action_intent" => serialization_failure::<ActionIntent>(text),
        "action_receipt" => serialization_failure::<ActionReceipt>(text),
        "action_ref_binding" => {
            let fixture: ActionRefFixture = serde_json::from_str(text).unwrap_or_else(|error| {
                panic!("{name} should remain structurally parseable: {error}")
            });
            fixture
                .reference
                .validate_for(&fixture.action)
                .expect_err("wrong ActionRef digest must fail")
        }
        "artifact_ref" => serialization_failure::<ArtifactRef>(text),
        "capability_grant" => serialization_failure::<CapabilityGrant>(text),
        "evidence_ref" => serialization_failure::<EvidenceRef>(text),
        "fact" => serialization_failure::<Fact>(text),
        "freshness_assessment" => serialization_failure::<FreshnessAssessment>(text),
        "information_classification" => serialization_failure::<InformationClassification>(text),
        "information_ref" => serialization_failure::<InformationRef>(text),
        "information_use" => serialization_failure::<InformationUse>(text),
        "lineage_ref" => serialization_failure::<LineageRef>(text),
        "origin" => serialization_failure::<Origin>(text),
        "principal_ref" => serialization_failure::<PrincipalRef>(text),
        "resource_ref" => serialization_failure::<ResourceRef>(text),
        "scope" => serialization_failure::<Scope>(text),
        "scope_constraint" => serialization_failure::<ScopeConstraint<WorkspaceId>>(text),
        "verification_receipt" => serialization_failure::<VerificationReceipt>(text),
        "versioned_actor" => Versioned::<ecra_core::Actor>::from_json_slice(text.as_bytes())
            .expect_err("invalid versioned actor fixture must fail"),
        "web_origin" => serialization_failure::<WebOrigin>(text),
        other => panic!("invalid fixture {name} has unknown type manifest entry {other}"),
    }
}

#[test]
fn every_committed_invalid_fixture_has_a_typed_machine_readable_failure() {
    let discovered = discovered_fixture_names();
    let expected: BTreeSet<String> = INVALID_FIXTURES
        .iter()
        .map(|(name, _, _)| (*name).to_owned())
        .collect();
    assert_eq!(
        discovered, expected,
        "invalid fixture directory and typed manifest diverged"
    );

    for (name, kind, expected_code) in INVALID_FIXTURES {
        let error = invalid_error(name, kind, &invalid_fixture_text(name));
        assert_eq!(
            error.code(),
            *expected_code,
            "unexpected machine-readable error code for {name}"
        );
        assert_eq!(
            error.category(),
            expected_code.category(),
            "unexpected machine-readable error category for {name}"
        );
    }
}

#[test]
fn every_domain_error_code_and_category_is_machine_readable_without_display_parsing() {
    let cases = [
        (
            DomainError::UnsupportedMajorVersion {
                supported: 1,
                actual: 2,
            },
            ErrorCode::UnsupportedMajorVersion,
            ErrorCategory::Compatibility,
        ),
        (
            DomainError::UnsupportedMinorVersion {
                supported: 0,
                actual: 1,
            },
            ErrorCode::UnsupportedMinorVersion,
            ErrorCategory::Compatibility,
        ),
        (
            DomainError::InvalidIdentifier {
                kind: "actor",
                value: "invalid".to_owned(),
            },
            ErrorCode::InvalidIdentifier,
            ErrorCategory::Identifier,
        ),
        (
            DomainError::InvalidEpochMillis { value: i64::MAX },
            ErrorCode::InvalidEpochMillis,
            ErrorCategory::Temporal,
        ),
        (
            DomainError::InvalidTemporalRange,
            ErrorCode::InvalidTemporalRange,
            ErrorCategory::Temporal,
        ),
        (
            DomainError::InvalidOrigin("invalid".to_owned()),
            ErrorCode::InvalidOrigin,
            ErrorCategory::Origin,
        ),
        (
            DomainError::InvalidResource("invalid".to_owned()),
            ErrorCode::InvalidResource,
            ErrorCategory::Resource,
        ),
        (
            DomainError::InvalidScope("invalid".to_owned()),
            ErrorCode::InvalidScope,
            ErrorCategory::Scope,
        ),
        (
            DomainError::InvalidCapability("invalid".to_owned()),
            ErrorCode::InvalidCapability,
            ErrorCategory::Capability,
        ),
        (
            DomainError::InvalidIdentity("invalid".to_owned()),
            ErrorCode::InvalidIdentity,
            ErrorCategory::Identity,
        ),
        (
            DomainError::InvalidInformation("invalid".to_owned()),
            ErrorCode::InvalidInformation,
            ErrorCategory::Information,
        ),
        (
            DomainError::Canonicalization("invalid".to_owned()),
            ErrorCode::CanonicalizationFailed,
            ErrorCategory::Canonicalization,
        ),
        (
            DomainError::InvalidContentDigest("invalid".to_owned()),
            ErrorCode::InvalidContentDigest,
            ErrorCategory::Digest,
        ),
        (
            DomainError::InvalidSecurityDigest("invalid".to_owned()),
            ErrorCode::InvalidSecurityDigest,
            ErrorCategory::Digest,
        ),
        (
            DomainError::InvalidAction("invalid".to_owned()),
            ErrorCode::InvalidAction,
            ErrorCategory::Action,
        ),
        (
            DomainError::InvalidAttempt("invalid".to_owned()),
            ErrorCode::InvalidAttempt,
            ErrorCategory::Attempt,
        ),
        (
            DomainError::InvalidReceipt("invalid".to_owned()),
            ErrorCode::InvalidReceipt,
            ErrorCategory::Receipt,
        ),
        (
            DomainError::InvalidVerification("invalid".to_owned()),
            ErrorCode::InvalidVerification,
            ErrorCategory::Verification,
        ),
        (
            DomainError::Serialization("invalid".to_owned()),
            ErrorCode::SerializationFailed,
            ErrorCategory::Serialization,
        ),
    ];

    let mut code_names = BTreeSet::new();
    let mut categories = HashSet::new();

    for (error, expected_code, expected_category) in cases {
        assert_eq!(error.code(), expected_code);
        assert_eq!(error.category(), expected_category);
        assert_eq!(expected_code.category(), expected_category);
        assert!(
            code_names.insert(expected_code.as_str()),
            "duplicate ErrorCode machine-readable name"
        );
        categories.insert(expected_category);
    }

    assert_eq!(code_names.len(), 19, "all ErrorCode variants must be covered");
    assert_eq!(
        categories.len(),
        16,
        "all ErrorCategory variants must be covered"
    );
}
