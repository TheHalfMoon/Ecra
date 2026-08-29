use ecra_core::{
    ArtifactRef, DisputeState, ErrorCode, EvidenceRef, Fact, FactValue, FreshnessAssessment,
    FreshnessState, I_JSON_MAX_SAFE_INTEGER, I_JSON_MIN_SAFE_INTEGER, InformationClass,
    InformationClassification, Observation, Provenance,
};

#[test]
fn all_information_classes_are_explicit_and_round_trip() {
    let fixtures = [
        include_str!("../../../contracts/ecra-domain-v1/valid/classification-public.json"),
        include_str!("../../../contracts/ecra-domain-v1/valid/classification-private.json"),
        include_str!("../../../contracts/ecra-domain-v1/valid/classification-sensitive.json"),
        include_str!("../../../contracts/ecra-domain-v1/valid/classification-secret.json"),
        include_str!("../../../contracts/ecra-domain-v1/valid/classification-unknown.json"),
    ];
    let expected = [
        InformationClass::Public,
        InformationClass::Private,
        InformationClass::Sensitive,
        InformationClass::Secret,
        InformationClass::Unknown,
    ];

    for (fixture, expected_class) in fixtures.into_iter().zip(expected) {
        let classification: InformationClassification =
            serde_json::from_str(fixture).expect("valid classification");
        assert_eq!(classification.class(), expected_class);
        let encoded = serde_json::to_string(&classification).expect("serialize classification");
        let decoded: InformationClassification =
            serde_json::from_str(&encoded).expect("round trip classification");
        assert_eq!(decoded.class(), expected_class);
    }
}

#[test]
fn observation_fact_and_artifact_keep_security_axes_separate() {
    let observation: Observation = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/observation-classified.json"
    ))
    .expect("valid observation");
    assert_eq!(
        observation.classification().class(),
        InformationClass::Sensitive
    );

    let fact: Fact = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/fact-model-inferred.json"
    ))
    .expect("valid fact");
    assert_eq!(fact.provenance(), Provenance::ModelInferred);
    assert_eq!(fact.classification().class(), InformationClass::Sensitive);
    assert_eq!(fact.freshness().state(), FreshnessState::Stale);
    assert_eq!(fact.dispute(), DisputeState::Contradicted);

    let conflict: Fact = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/fact-conflict.json"
    ))
    .expect("valid conflict fact");
    assert_eq!(conflict.provenance(), Provenance::Retrieved);
    assert_eq!(conflict.dispute(), DisputeState::Disputed);

    let artifact: ArtifactRef = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/artifact-classified.json"
    ))
    .expect("valid artifact");
    assert_eq!(
        artifact.classification().class(),
        InformationClass::Sensitive
    );
    assert_eq!(artifact.byte_size_decimal(), Some("42"));
}

#[test]
fn fact_numeric_construction_is_wire_safe() {
    for value in [I_JSON_MIN_SAFE_INTEGER, I_JSON_MAX_SAFE_INTEGER] {
        let fact_value = FactValue::integer(value).expect("safe integer must construct");
        let json = serde_json::to_string(&fact_value).expect("safe integer must serialize");
        let decoded: FactValue =
            serde_json::from_str(&json).expect("serialized safe integer must deserialize");
        assert_eq!(decoded, fact_value);
    }

    for value in [I_JSON_MIN_SAFE_INTEGER - 1, I_JSON_MAX_SAFE_INTEGER + 1] {
        let error = FactValue::integer(value).expect_err("unsafe integer must not construct");
        assert_eq!(error.code(), ErrorCode::InvalidInformation);
    }

    for value in ["0", "1", "-1", "1.25", "0.001"] {
        let fact_value = FactValue::decimal(value).expect("canonical decimal must construct");
        let json = serde_json::to_string(&fact_value).expect("canonical decimal must serialize");
        let decoded: FactValue =
            serde_json::from_str(&json).expect("serialized canonical decimal must deserialize");
        assert_eq!(decoded, fact_value);
    }

    for value in ["", "+1", "01", "-0", "1.", "1e3"] {
        let error =
            FactValue::decimal(value).expect_err("non-canonical decimal must not construct");
        assert_eq!(error.code(), ErrorCode::InvalidInformation);
    }
}

#[test]
fn verification_cannot_be_embedded_as_fact_truth_flag() {
    let fact: Fact = serde_json::from_str(include_str!(
        "../../../contracts/ecra-domain-v1/valid/fact-model-inferred.json"
    ))
    .expect("valid fact");
    let json = serde_json::to_string(&fact).expect("serialize fact");
    assert!(!json.contains("verified"));

    assert!(
        serde_json::from_str::<Fact>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/fact-verified-flag.json"
        ))
        .is_err()
    );
}

#[test]
fn freshness_and_evidence_valid_fixtures_parse() {
    serde_json::from_str::<FreshnessAssessment>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/freshness-basis.json"
    ))
    .expect("valid freshness");
    serde_json::from_str::<EvidenceRef>(include_str!(
        "../../../contracts/ecra-domain-v1/valid/evidence-snapshot.json"
    ))
    .expect("valid evidence");
}

#[test]
fn phase5_invalid_contract_fixtures_fail_closed() {
    assert!(
        serde_json::from_str::<InformationClassification>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/classification-invalid-class.json"
        ))
        .is_err()
    );
    assert!(
        serde_json::from_str::<InformationClassification>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/classification-empty-tag.json"
        ))
        .is_err()
    );
    assert!(
        serde_json::from_str::<FreshnessAssessment>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/freshness-unpaired-basis.json"
        ))
        .is_err()
    );
    assert!(
        serde_json::from_str::<EvidenceRef>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/evidence-empty-external-ref.json"
        ))
        .is_err()
    );
    assert!(
        serde_json::from_str::<ArtifactRef>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/artifact-invalid-digest.json"
        ))
        .is_err()
    );
    assert!(
        serde_json::from_str::<ArtifactRef>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/artifact-invalid-byte-size.json"
        ))
        .is_err()
    );
    assert!(
        serde_json::from_str::<Fact>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/fact-integer-outside-ijson.json"
        ))
        .is_err()
    );
    assert!(
        serde_json::from_str::<Fact>(include_str!(
            "../../../contracts/ecra-domain-v1/invalid/fact-invalid-decimal.json"
        ))
        .is_err()
    );
}

#[test]
fn evidence_ref_read_only_accessors_preserve_wire_and_canonical_bytes() {
    let fixture = include_str!("../../../contracts/ecra-domain-v1/valid/evidence-snapshot.json");
    let evidence: EvidenceRef = serde_json::from_str(fixture).expect("valid snapshot evidence");
    let json_before = serde_json::to_vec(&evidence).expect("serialize evidence before access");
    let jcs_before = ecra_core::to_jcs_vec(&evidence).expect("canonicalize evidence before access");

    assert!(evidence.artifact().is_none());
    assert!(evidence.observation().is_none());
    assert!(evidence.receipt().is_none());
    assert_eq!(evidence.external_ref(), Some("snapshot:example-report-v1"));
    assert_eq!(
        evidence.content_digest().expect("content digest").hex(),
        "0000000000000000000000000000000000000000000000000000000000000000"
    );
    assert_eq!(evidence.as_of().expect("as-of").get(), 1900);

    let json_after = serde_json::to_vec(&evidence).expect("serialize evidence after access");
    let jcs_after = ecra_core::to_jcs_vec(&evidence).expect("canonicalize evidence after access");
    assert_eq!(json_before, json_after);
    assert_eq!(jcs_before, jcs_after);
}
