use ecra_core::{ActionIntent, Versioned, to_jcs_vec};

const CORE_SOURCES: &[(&str, &str)] = &[
    ("action.rs", include_str!("../src/action.rs")),
    ("actor.rs", include_str!("../src/actor.rs")),
    ("artifact.rs", include_str!("../src/artifact.rs")),
    ("canonical.rs", include_str!("../src/canonical.rs")),
    ("capability.rs", include_str!("../src/capability.rs")),
    ("digest.rs", include_str!("../src/digest.rs")),
    ("error.rs", include_str!("../src/error.rs")),
    ("evidence.rs", include_str!("../src/evidence.rs")),
    ("id.rs", include_str!("../src/id.rs")),
    ("identity.rs", include_str!("../src/identity.rs")),
    ("information.rs", include_str!("../src/information.rs")),
    ("lib.rs", include_str!("../src/lib.rs")),
    ("origin.rs", include_str!("../src/origin.rs")),
    ("receipt.rs", include_str!("../src/receipt.rs")),
    ("resource.rs", include_str!("../src/resource.rs")),
    ("scope.rs", include_str!("../src/scope.rs")),
    ("time.rs", include_str!("../src/time.rs")),
    ("verification.rs", include_str!("../src/verification.rs")),
    ("version.rs", include_str!("../src/version.rs")),
];

#[test]
fn contract_semantics_are_independent_of_json_whitespace_and_line_endings() {
    let compact = include_str!("../../../contracts/ecra-domain-v1/valid/action-digest-golden.json");
    let json: serde_json::Value = serde_json::from_str(compact).expect("golden JSON");
    let pretty = serde_json::to_string_pretty(&json).expect("pretty JSON");
    let crlf = pretty.replace('\n', "\r\n");

    let compact_intent: ActionIntent = serde_json::from_str(compact).expect("compact intent");
    let crlf_intent: ActionIntent = serde_json::from_str(&crlf).expect("CRLF intent");
    assert_eq!(compact_intent, crlf_intent);
    assert_eq!(
        to_jcs_vec(&Versioned::v1(&compact_intent)).expect("compact JCS"),
        to_jcs_vec(&Versioned::v1(&crlf_intent)).expect("CRLF JCS")
    );
    assert_eq!(
        compact_intent.digest().expect("compact digest"),
        crlf_intent.digest().expect("CRLF digest")
    );
}

#[test]
fn trusted_core_source_does_not_consult_platform_environment_or_services() {
    const PROHIBITED: &[&str] = &[
        "std::env",
        "std::fs",
        "std::net",
        "std::process",
        "SystemTime",
        "Instant::now",
        "target_os",
        "target_arch",
    ];

    for (path, source) in CORE_SOURCES {
        for prohibited in PROHIBITED {
            assert!(
                !source.contains(prohibited),
                "trusted core source {path} must not inspect platform/environment via {prohibited}"
            );
        }
    }
}
