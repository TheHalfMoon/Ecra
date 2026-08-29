use ecra_core::{
    ActionAttemptId, ActionAttemptRef, ActionDigest, ActionId, ActionRef, ActorId, ClaimRef,
    EvidenceId, EvidenceKind, EvidenceRef, RunId, SecurityDigest, VerificationId,
    VerificationMethod, VerificationOutcome, VerificationReceipt, VerificationTarget,
};
use ecra_verify::{
    ReconciliationId, ReconciliationOutcomeV1, ReconciliationRecordFieldsV1,
    ReconciliationRecordV1, VerificationAggregateViewV1, VerificationJournalBodyV1,
    VerificationJournalEntryV1, VerificationJournalSequence,
};

fn verification_id(tail: u64) -> VerificationId {
    VerificationId::parse_str(&format!("00000000-0000-0000-0000-{tail:012}"))
        .expect("verification id")
}

fn evidence_id(tail: u64) -> EvidenceId {
    EvidenceId::parse_str(&format!("00000000-0000-0000-0000-{tail:012}"))
        .expect("evidence id")
}

fn target() -> VerificationTarget {
    VerificationTarget::Claim(ClaimRef::new("portability", "stable").expect("claim target"))
}

fn receipt(tail: u64, outcome: VerificationOutcome) -> VerificationReceipt {
    let evidence = if outcome == VerificationOutcome::NotEvaluated {
        Vec::new()
    } else {
        vec![EvidenceRef::new(
            evidence_id(193_000 + tail),
            EvidenceKind::Computation,
        )]
    };
    VerificationReceipt::new(
        verification_id(tail),
        ActorId::parse_str("00000000-0000-0000-0000-000000000001").expect("actor id"),
        target(),
        VerificationMethod::Other,
        outcome,
        evidence,
    )
    .expect("verification receipt")
}

fn action() -> ActionRef {
    ActionRef::new(
        ActionId::parse_str("00000000-0000-0000-0000-000000093001").expect("action id"),
        ActionDigest::new(SecurityDigest::sha256(b"portability-action")),
    )
}

fn reconciliation(support: Vec<VerificationId>) -> ReconciliationRecordV1 {
    let action = action();
    let attempt = ActionAttemptRef::new(
        ActionAttemptId::parse_str("00000000-0000-0000-0000-000000093002")
            .expect("attempt id"),
        action.clone(),
    );
    ReconciliationRecordV1::from_fields(ReconciliationRecordFieldsV1 {
        id: ReconciliationId::parse_str("00000000-0000-0000-0000-000000093003")
            .expect("reconciliation id"),
        run_id: RunId::parse_str("00000000-0000-0000-0000-000000093004").expect("run id"),
        attempt,
        action,
        outcome: ReconciliationOutcomeV1::StillUnknown,
        verification_receipts: support,
        reconciled_at: None,
        notes: Some("portable reconciliation".to_owned()),
    })
    .expect("reconciliation record")
}

#[test]
fn journal_json_formatting_and_line_endings_preserve_digest_and_canonical_bytes() {
    let entry = VerificationJournalEntryV1::new(
        VerificationJournalSequence::new(1).expect("sequence"),
        None,
        VerificationJournalBodyV1::VerificationReceipt {
            receipt: receipt(93_101, VerificationOutcome::NotEvaluated),
        },
    )
    .expect("journal entry");

    let compact = serde_json::to_vec(&entry).expect("compact JSON");
    let pretty_crlf = format!(
        "\r\n{}\r\n",
        serde_json::to_string_pretty(&entry)
            .expect("pretty JSON")
            .replace('\n', "\r\n")
    );
    let compact_parsed =
        VerificationJournalEntryV1::from_json_slice(&compact).expect("compact parse");
    let crlf_parsed = VerificationJournalEntryV1::from_json_slice(pretty_crlf.as_bytes())
        .expect("CRLF parse");

    assert_eq!(compact_parsed, crlf_parsed);
    assert_eq!(compact_parsed.entry_digest(), crlf_parsed.entry_digest());
    assert_eq!(
        compact_parsed.canonical_bytes().expect("compact canonical"),
        crlf_parsed.canonical_bytes().expect("CRLF canonical")
    );
}

#[test]
fn aggregate_behavior_is_independent_of_receipt_input_order() {
    let verified = receipt(93_201, VerificationOutcome::Verified);
    let rejected = receipt(93_202, VerificationOutcome::Rejected);
    let inconclusive = receipt(93_203, VerificationOutcome::Inconclusive);

    let forward = VerificationAggregateViewV1::from_receipts(
        target(),
        &[verified.clone(), rejected.clone(), inconclusive.clone()],
    )
    .expect("forward aggregate");
    let reverse = VerificationAggregateViewV1::from_receipts(
        target(),
        &[inconclusive, rejected, verified],
    )
    .expect("reverse aggregate");

    assert_eq!(forward, reverse);
    assert_eq!(
        serde_jcs::to_vec(&forward).expect("forward JCS"),
        serde_jcs::to_vec(&reverse).expect("reverse JCS")
    );
}

#[test]
fn reconciliation_support_order_and_json_field_order_are_portable() {
    let first_id = verification_id(93_301);
    let second_id = verification_id(93_302);
    let forward = reconciliation(vec![first_id, second_id]);
    let reverse = reconciliation(vec![second_id, first_id]);
    assert_eq!(forward, reverse);
    assert_eq!(
        serde_jcs::to_vec(&forward).expect("forward JCS"),
        serde_jcs::to_vec(&reverse).expect("reverse JCS")
    );

    let value = serde_json::to_value(&forward).expect("record value");
    let reordered = format!(
        concat!(
            "{{\r\n",
            "  \"notes\":{},\r\n",
            "  \"reconciled_at\":{},\r\n",
            "  \"verification_receipts\":{},\r\n",
            "  \"outcome\":{},\r\n",
            "  \"action\":{},\r\n",
            "  \"attempt\":{},\r\n",
            "  \"run_id\":{},\r\n",
            "  \"id\":{},\r\n",
            "  \"version\":{}\r\n",
            "}}\r\n"
        ),
        serde_json::to_string(&value["notes"]).expect("notes"),
        serde_json::to_string(&value["reconciled_at"]).expect("reconciled_at"),
        serde_json::to_string(&value["verification_receipts"])
            .expect("verification_receipts"),
        serde_json::to_string(&value["outcome"]).expect("outcome"),
        serde_json::to_string(&value["action"]).expect("action"),
        serde_json::to_string(&value["attempt"]).expect("attempt"),
        serde_json::to_string(&value["run_id"]).expect("run_id"),
        serde_json::to_string(&value["id"]).expect("id"),
        serde_json::to_string(&value["version"]).expect("version"),
    );
    let parsed = ReconciliationRecordV1::from_json_slice(reordered.as_bytes())
        .expect("reordered strict JSON parse");
    assert_eq!(parsed, forward);
    assert_eq!(
        serde_jcs::to_vec(&parsed).expect("parsed JCS"),
        serde_jcs::to_vec(&forward).expect("forward JCS")
    );
}
