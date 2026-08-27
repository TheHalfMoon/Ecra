use std::{collections::BTreeSet, fmt::Debug, fs, path::PathBuf};

use ecra_core::{
    ActionAttemptRef, ActionIntent, ActionReceipt, Actor, ArtifactRef, CapabilityGrant,
    CapabilityRequest, EvidenceRef, Fact, FreshnessAssessment, IdentityAssertionRef,
    InformationClassification, InformationUse, Observation, PrincipalRef, ResourceRef, Scope,
    ScopeConstraint, VerificationReceipt, Versioned, WebOrigin, WorkspaceId,
};
use serde::{Serialize, de::DeserializeOwned};

const VALID_FIXTURES: &[&str] = &[
    "action-attempt-1.json",
    "action-attempt-2.json",
    "action-digest-golden.json",
    "action-irreversible-local.json",
    "action-keyed-idempotent.json",
    "action-read-only.json",
    "action-receipt-failure.json",
    "action-receipt-success.json",
    "action-receipt-unknown.json",
    "action-reversible-external.json",
    "action-unknown-conservative.json",
    "actor-agent.json",
    "artifact-classified.json",
    "capability-grant-delegated.json",
    "capability-grant-root.json",
    "capability-request-narrow.json",
    "classification-private.json",
    "classification-public.json",
    "classification-secret.json",
    "classification-sensitive.json",
    "classification-unknown.json",
    "evidence-snapshot.json",
    "fact-conflict.json",
    "fact-model-inferred.json",
    "freshness-basis.json",
    "identity-assertion-ref.json",
    "information-use-external-disclosure.json",
    "information-use-local-compute.json",
    "information-use-log-diagnostic.json",
    "information-use-model-context.json",
    "information-use-persist.json",
    "information-use-remote-provider.json",
    "observation-classified.json",
    "principal-ref.json",
    "resource-ref.json",
    "scope-explicit.json",
    "scope-one-of.json",
    "verification-inconclusive.json",
    "verification-not-evaluated.json",
    "verification-rejected.json",
    "verification-verified.json",
    "web-origin-opaque.json",
    "web-origin-tuple.json",
];

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../contracts/ecra-domain-v1/valid")
}

fn discovered_fixture_names() -> BTreeSet<String> {
    fs::read_dir(fixture_dir())
        .expect("read valid fixture directory")
        .map(|entry| entry.expect("valid fixture directory entry"))
        .filter(|entry| entry.path().extension().is_some_and(|extension| extension == "json"))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect()
}

fn fixture_text(name: &str) -> String {
    fs::read_to_string(fixture_dir().join(name))
        .unwrap_or_else(|error| panic!("read valid fixture {name}: {error}"))
}

fn round_trip_versioned<T>(name: &str, text: &str)
where
    T: Clone + Debug + DeserializeOwned + PartialEq + Serialize,
{
    let value: T = serde_json::from_str(text)
        .unwrap_or_else(|error| panic!("valid fixture {name} did not parse: {error}"));
    let encoded = serde_json::to_vec(&value)
        .unwrap_or_else(|error| panic!("valid fixture {name} did not serialize: {error}"));
    let round_trip: T = serde_json::from_slice(&encoded)
        .unwrap_or_else(|error| panic!("valid fixture {name} did not round-trip: {error}"));
    assert_eq!(round_trip, value, "semantic round-trip drift for {name}");

    let envelope = Versioned::v1(value.clone());
    let envelope_json = serde_json::to_vec(&envelope)
        .unwrap_or_else(|error| panic!("versioned fixture {name} did not serialize: {error}"));
    let versioned_round_trip: Versioned<T> = Versioned::from_json_slice(&envelope_json)
        .unwrap_or_else(|error| panic!("versioned fixture {name} did not parse: {error}"));
    assert_eq!(
        versioned_round_trip.into_value(),
        value,
        "versioned semantic round-trip drift for {name}"
    );
}

fn assert_valid_fixture(name: &str, text: &str) {
    match name {
        "action-attempt-1.json" | "action-attempt-2.json" => {
            round_trip_versioned::<ActionAttemptRef>(name, text);
        }
        "action-digest-golden.json"
        | "action-irreversible-local.json"
        | "action-keyed-idempotent.json"
        | "action-read-only.json"
        | "action-reversible-external.json"
        | "action-unknown-conservative.json" => {
            round_trip_versioned::<ActionIntent>(name, text);
        }
        "action-receipt-failure.json"
        | "action-receipt-success.json"
        | "action-receipt-unknown.json" => {
            round_trip_versioned::<ActionReceipt>(name, text);
        }
        "actor-agent.json" => round_trip_versioned::<Actor>(name, text),
        "artifact-classified.json" => round_trip_versioned::<ArtifactRef>(name, text),
        "capability-grant-delegated.json" | "capability-grant-root.json" => {
            round_trip_versioned::<CapabilityGrant>(name, text);
        }
        "capability-request-narrow.json" => round_trip_versioned::<CapabilityRequest>(name, text),
        "classification-private.json"
        | "classification-public.json"
        | "classification-secret.json"
        | "classification-sensitive.json"
        | "classification-unknown.json" => {
            round_trip_versioned::<InformationClassification>(name, text);
        }
        "evidence-snapshot.json" => round_trip_versioned::<EvidenceRef>(name, text),
        "fact-conflict.json" | "fact-model-inferred.json" => {
            round_trip_versioned::<Fact>(name, text);
        }
        "freshness-basis.json" => round_trip_versioned::<FreshnessAssessment>(name, text),
        "identity-assertion-ref.json" => round_trip_versioned::<IdentityAssertionRef>(name, text),
        "information-use-external-disclosure.json"
        | "information-use-local-compute.json"
        | "information-use-log-diagnostic.json"
        | "information-use-model-context.json"
        | "information-use-persist.json"
        | "information-use-remote-provider.json" => {
            round_trip_versioned::<InformationUse>(name, text);
        }
        "observation-classified.json" => round_trip_versioned::<Observation>(name, text),
        "principal-ref.json" => round_trip_versioned::<PrincipalRef>(name, text),
        "resource-ref.json" => round_trip_versioned::<ResourceRef>(name, text),
        "scope-explicit.json" => round_trip_versioned::<Scope>(name, text),
        "scope-one-of.json" => {
            round_trip_versioned::<ScopeConstraint<WorkspaceId>>(name, text);
        }
        "verification-inconclusive.json"
        | "verification-not-evaluated.json"
        | "verification-rejected.json"
        | "verification-verified.json" => {
            round_trip_versioned::<VerificationReceipt>(name, text);
        }
        "web-origin-opaque.json" | "web-origin-tuple.json" => {
            round_trip_versioned::<WebOrigin>(name, text);
        }
        other => panic!("valid fixture has no type manifest entry: {other}"),
    }
}

#[test]
fn every_committed_valid_fixture_is_typed_and_round_trips() {
    let discovered = discovered_fixture_names();
    let expected: BTreeSet<String> = VALID_FIXTURES.iter().map(|name| (*name).to_owned()).collect();
    assert_eq!(
        discovered, expected,
        "valid fixture directory and typed manifest diverged"
    );

    for name in VALID_FIXTURES {
        assert_valid_fixture(name, &fixture_text(name));
    }
}
