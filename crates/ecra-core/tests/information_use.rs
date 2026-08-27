use std::any::TypeId;

use ecra_core::{CapabilityGrant, InformationUse, InformationUseKind};

#[test]
fn all_phase6_use_kinds_parse_as_declarations() {
    let fixtures = [
        (
            include_str!("../../../contracts/ecra-domain-v1/valid/information-use-local-compute.json"),
            InformationUseKind::LocalCompute,
        ),
        (
            include_str!("../../../contracts/ecra-domain-v1/valid/information-use-model-context.json"),
            InformationUseKind::ModelContext,
        ),
        (
            include_str!("../../../contracts/ecra-domain-v1/valid/information-use-persist.json"),
            InformationUseKind::Persist,
        ),
        (
            include_str!("../../../contracts/ecra-domain-v1/valid/information-use-log-diagnostic.json"),
            InformationUseKind::LogOrDiagnostic,
        ),
        (
            include_str!("../../../contracts/ecra-domain-v1/valid/information-use-external-disclosure.json"),
            InformationUseKind::ExternalDisclosure,
        ),
        (
            include_str!("../../../contracts/ecra-domain-v1/valid/information-use-remote-provider.json"),
            InformationUseKind::RemoteProvider,
        ),
    ];

    for (fixture, expected_kind) in fixtures {
        let declared_use: InformationUse = serde_json::from_str(fixture).expect("valid use");
        assert_eq!(declared_use.kind(), expected_kind);
        assert!(!declared_use.sources().is_empty());
    }
}

#[test]
fn invalid_information_use_fixtures_fail_closed() {
    assert!(
        serde_json::from_str::<InformationUse>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/information-use-empty-sources.json"
        ))
        .is_err()
    );
    assert!(
        serde_json::from_str::<InformationUse>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/information-use-invalid-destination.json"
        ))
        .is_err()
    );
}

#[test]
fn declaration_is_not_authorization_or_capability_grant() {
    assert_ne!(TypeId::of::<InformationUse>(), TypeId::of::<CapabilityGrant>());

    let declared_use: InformationUse = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/information-use-external-disclosure.json"
    ))
    .expect("valid disclosure declaration");
    let serialized = serde_json::to_string(&declared_use).expect("serialize declaration");

    assert!(!serialized.contains("capability_grant"));
    assert!(!serialized.contains("authorization"));
}

#[test]
fn separate_capabilities_do_not_encode_source_to_sink_authorization() {
    let read_grant: CapabilityGrant = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/capability-grant-root.json"
    ))
    .expect("valid capability grant");
    let write_grant: CapabilityGrant = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/capability-grant-delegated.json"
    ))
    .expect("valid capability grant");
    let declared_use: InformationUse = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/information-use-remote-provider.json"
    ))
    .expect("valid remote-provider declaration");

    let read_json = serde_json::to_string(&read_grant).expect("serialize read grant");
    let write_json = serde_json::to_string(&write_grant).expect("serialize write grant");
    let use_json = serde_json::to_string(&declared_use).expect("serialize information use");

    assert!(!read_json.contains("sources"));
    assert!(!write_json.contains("sources"));
    assert!(use_json.contains("sources"));
    assert!(use_json.contains("remote_provider"));
}
