use ecra_run::{
    BudgetAmount, EventSequence, MAX_BUDGET_AMOUNT, MAX_EVENT_SEQUENCE, RunErrorCategory,
    RunErrorCode, RunEvent, RunEventEnvelope,
};

const ALL_CATEGORIES: [RunErrorCategory; 12] = [
    RunErrorCategory::Compatibility,
    RunErrorCategory::Event,
    RunErrorCategory::State,
    RunErrorCategory::Attempt,
    RunErrorCategory::Ledger,
    RunErrorCategory::Storage,
    RunErrorCategory::Migration,
    RunErrorCategory::Budget,
    RunErrorCategory::Archive,
    RunErrorCategory::Integrity,
    RunErrorCategory::Recovery,
    RunErrorCategory::Serialization,
];

const ALL_CODES: [RunErrorCode; 30] = [
    RunErrorCode::UnsupportedMajorVersion,
    RunErrorCode::UnsupportedMinorVersion,
    RunErrorCode::InvalidEventSequence,
    RunErrorCode::InvalidEvent,
    RunErrorCode::InvalidStateTransition,
    RunErrorCode::DuplicateAttempt,
    RunErrorCode::AttemptBindingMismatch,
    RunErrorCode::ReceiptBindingMismatch,
    RunErrorCode::UnresolvedAttempt,
    RunErrorCode::BlindRetryForbidden,
    RunErrorCode::LedgerHeadMismatch,
    RunErrorCode::LedgerChainInvalid,
    RunErrorCode::LedgerDigestMismatch,
    RunErrorCode::StoreConfigurationInvalid,
    RunErrorCode::StoreBusy,
    RunErrorCode::StorageError,
    RunErrorCode::UnsupportedStoreVersion,
    RunErrorCode::MigrationFailed,
    RunErrorCode::InvalidBudget,
    RunErrorCode::BudgetOverflow,
    RunErrorCode::BudgetPreflightExceeded,
    RunErrorCode::BudgetExhausted,
    RunErrorCode::ArchivePathInvalid,
    RunErrorCode::ArchiveDuplicateEntry,
    RunErrorCode::ArchiveFeatureUnsupported,
    RunErrorCode::ArchiveLimitExceeded,
    RunErrorCode::ArchiveManifestInvalid,
    RunErrorCode::ArchiveDigestMismatch,
    RunErrorCode::RecoveryRequired,
    RunErrorCode::SerializationFailed,
];

fn golden_envelope() -> RunEventEnvelope {
    RunEventEnvelope::from_json_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/run-created-envelope.v1.json"
    ))
    .expect("golden run-created envelope must parse")
}

#[test]
fn canonical_run_created_bytes_and_digest_match_golden() {
    let envelope = golden_envelope();
    assert_eq!(
        envelope
            .canonical_digest_material()
            .expect("canonical material"),
        include_bytes!("../../../contracts/ecra-run-v1/expected/run-created-golden.v1.jcs")
    );
    assert_eq!(
        envelope.event_digest().hex(),
        include_str!("../../../contracts/ecra-run-v1/expected/run-created-golden.sha256").trim()
    );
}

#[test]
fn every_v1_event_kind_has_a_strict_valid_fixture() {
    let events: Vec<RunEvent> = serde_json::from_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/all-event-kinds.v1.json"
    ))
    .expect("all valid event fixtures must parse");
    let kinds: Vec<&str> = events.iter().map(RunEvent::kind).collect();
    assert_eq!(
        kinds,
        [
            "run_created",
            "run_started",
            "run_suspended",
            "run_resumed",
            "cancellation_requested",
            "run_cancelled",
            "run_failed",
            "execution_completed",
            "attempt_prepared",
            "receipt_recorded",
            "recovery_boundary",
            "attempt_marked_unknown",
            "reconciliation_requested",
            "resource_usage_recorded",
            "budget_soft_limit_reached",
            "budget_exhausted",
            "intervention_recorded",
        ]
    );

    for event in events {
        let encoded = serde_json::to_vec(&event).expect("serialize event fixture");
        let reparsed: RunEvent = serde_json::from_slice(&encoded).expect("round-trip event fixture");
        assert_eq!(event, reparsed);
    }
}

#[test]
fn version_sequence_unknown_field_and_digest_fail_with_typed_codes() {
    let base: serde_json::Value = serde_json::from_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/run-created-envelope.v1.json"
    ))
    .expect("valid base json");

    let mut unsupported_major = base.clone();
    unsupported_major["schema_version"]["major"] = 2.into();
    let error = RunEventEnvelope::from_json_slice(
        &serde_json::to_vec(&unsupported_major).expect("json"),
    )
    .expect_err("major must fail");
    assert_eq!(error.category(), RunErrorCategory::Compatibility);
    assert_eq!(error.code(), RunErrorCode::UnsupportedMajorVersion);

    let mut unsupported_minor = base.clone();
    unsupported_minor["schema_version"]["minor"] = 1.into();
    let error = RunEventEnvelope::from_json_slice(
        &serde_json::to_vec(&unsupported_minor).expect("json"),
    )
    .expect_err("minor must fail");
    assert_eq!(error.code(), RunErrorCode::UnsupportedMinorVersion);

    let mut zero_sequence = base.clone();
    zero_sequence["sequence"] = 0.into();
    let error = RunEventEnvelope::from_json_slice(
        &serde_json::to_vec(&zero_sequence).expect("json"),
    )
    .expect_err("zero sequence must fail");
    assert_eq!(error.code(), RunErrorCode::InvalidEventSequence);

    let mut unknown_field = base.clone();
    unknown_field["unexpected"] = true.into();
    let error = RunEventEnvelope::from_json_slice(
        &serde_json::to_vec(&unknown_field).expect("json"),
    )
    .expect_err("unknown field must fail");
    assert_eq!(error.category(), RunErrorCategory::Serialization);
    assert_eq!(error.code(), RunErrorCode::SerializationFailed);

    let mut digest_mismatch = base;
    digest_mismatch["event_digest"]["hex"] = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into();
    let error = RunEventEnvelope::from_json_slice(
        &serde_json::to_vec(&digest_mismatch).expect("json"),
    )
    .expect_err("digest mismatch must fail");
    assert_eq!(error.category(), RunErrorCategory::Ledger);
    assert_eq!(error.code(), RunErrorCode::LedgerDigestMismatch);
}

#[test]
fn genesis_and_successor_chain_rules_fail_closed() {
    let mut genesis_with_previous: serde_json::Value = serde_json::from_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/run-created-envelope.v1.json"
    ))
    .expect("valid base json");
    genesis_with_previous["previous_digest"] = serde_json::json!({
        "algorithm":"sha256",
        "hex":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    });
    let error = RunEventEnvelope::from_json_slice(
        &serde_json::to_vec(&genesis_with_previous).expect("json"),
    )
    .expect_err("genesis previous digest must fail");
    assert_eq!(error.code(), RunErrorCode::LedgerChainInvalid);

    let previous = golden_envelope();
    let cross_run = RunEventEnvelope::from_json_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/invalid/cross-run-successor.v1.json"
    ))
    .expect("cross-run fixture is structurally valid before contextual linkage");
    let error = cross_run
        .validate_successor(&previous)
        .expect_err("cross-run successor must fail contextual chain validation");
    assert_eq!(error.code(), RunErrorCode::LedgerChainInvalid);
}

#[test]
fn strict_event_bodies_reject_unknown_fields_and_oversized_diagnostics() {
    let unknown = br#"{"kind":"run_started","value":{"unexpected":true}}"#;
    assert!(serde_json::from_slice::<RunEvent>(unknown).is_err());

    let oversized = "x".repeat(4_097);
    let json = serde_json::json!({
        "kind":"intervention_recorded",
        "value":{
            "actor":"00000000-0000-0000-0000-000000000001",
            "kind":"note",
            "note":oversized
        }
    });
    assert!(serde_json::from_value::<RunEvent>(json).is_err());
}

#[test]
fn integer_wrappers_enforce_i_json_boundaries_and_checked_arithmetic() {
    assert!(EventSequence::new(1).is_ok());
    assert!(EventSequence::new(MAX_EVENT_SEQUENCE).is_ok());
    assert_eq!(
        EventSequence::new(0).expect_err("zero must fail").code(),
        RunErrorCode::InvalidEventSequence
    );
    assert_eq!(
        EventSequence::new(MAX_EVENT_SEQUENCE + 1)
            .expect_err("too large must fail")
            .code(),
        RunErrorCode::InvalidEventSequence
    );

    assert!(BudgetAmount::new(0).is_ok());
    assert!(BudgetAmount::new(MAX_BUDGET_AMOUNT).is_ok());
    assert_eq!(
        BudgetAmount::new(MAX_BUDGET_AMOUNT + 1)
            .expect_err("too large must fail")
            .code(),
        RunErrorCode::InvalidBudget
    );
    assert_eq!(
        BudgetAmount::new(MAX_BUDGET_AMOUNT)
            .expect("max")
            .checked_add(BudgetAmount::new(1).expect("one"))
            .expect_err("checked addition must fail")
            .code(),
        RunErrorCode::BudgetOverflow
    );
}

#[test]
fn every_error_category_and_code_is_machine_readable_without_display_parsing() {
    for category in ALL_CATEGORIES {
        let encoded = serde_json::to_string(&category).expect("serialize category");
        let decoded: RunErrorCategory = serde_json::from_str(&encoded).expect("parse category");
        assert_eq!(decoded, category);
    }
    for code in ALL_CODES {
        let encoded = serde_json::to_string(&code).expect("serialize code");
        let decoded: RunErrorCode = serde_json::from_str(&encoded).expect("parse code");
        assert_eq!(decoded, code);
    }
}
