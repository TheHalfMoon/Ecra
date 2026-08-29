use ecra_core::{
    ActorId, ClaimRef, VerificationId, VerificationMethod, VerificationOutcome,
    VerificationReceipt, VerificationTarget,
};
use ecra_verify::{
    MAX_VERIFICATION_JOURNAL_SEQUENCE, VerificationJournalBodyV1, VerificationJournalDigest,
    VerificationJournalEntryV1, VerificationJournalSequence, VerifyErrorCode,
};
use sha2::{Digest, Sha256};

const JOURNAL_DOMAIN: &[u8] = b"ecra/verification-journal/v1\0";

fn golden_receipt() -> VerificationReceipt {
    VerificationReceipt::new(
        VerificationId::parse_str("00000000-0000-0000-0000-000000090001").expect("verification id"),
        ActorId::parse_str("00000000-0000-0000-0000-000000000001").expect("actor id"),
        VerificationTarget::Claim(ClaimRef::new("journal", "golden").expect("claim target")),
        VerificationMethod::Other,
        VerificationOutcome::NotEvaluated,
        Vec::new(),
    )
    .expect("golden receipt")
}

fn golden_entry() -> VerificationJournalEntryV1 {
    VerificationJournalEntryV1::new(
        VerificationJournalSequence::new(1).expect("genesis sequence"),
        None,
        VerificationJournalBodyV1::VerificationReceipt {
            receipt: golden_receipt(),
        },
    )
    .expect("golden journal entry")
}

#[test]
fn canonical_genesis_digest_matches_fixed_golden() {
    let material =
        include_bytes!("../../../contracts/ecra-verify-v1/expected/journal-genesis-material.json");
    let material = material
        .strip_suffix(b"\n")
        .expect("golden material has one trailing newline");
    let expected =
        include_str!("../../../contracts/ecra-verify-v1/expected/journal-genesis.sha256").trim();

    let mut preimage = Vec::with_capacity(JOURNAL_DOMAIN.len() + material.len());
    preimage.extend_from_slice(JOURNAL_DOMAIN);
    preimage.extend_from_slice(material);
    let direct = Sha256::digest(preimage);
    let direct_hex = direct
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    assert_eq!(direct_hex, expected);

    let entry = golden_entry();
    assert_eq!(entry.entry_digest().hex(), expected);
    let parsed = VerificationJournalEntryV1::from_json_slice(
        &entry.canonical_bytes().expect("canonical entry bytes"),
    )
    .expect("round-trip entry");
    assert_eq!(parsed, entry);
}

#[test]
fn version_sequence_previous_body_and_digest_mutations_fail_closed() {
    let entry = golden_entry();

    let mut version = serde_json::to_value(&entry).expect("entry json");
    version["version"]["major"] = 2.into();
    let error = VerificationJournalEntryV1::from_json_slice(
        &serde_json::to_vec(&version).expect("version mutation"),
    )
    .expect_err("unsupported version must fail");
    assert_eq!(error.code(), VerifyErrorCode::UnsupportedVersion);

    let mut sequence = serde_json::to_value(&entry).expect("entry json");
    sequence["sequence"] = 2.into();
    let error = VerificationJournalEntryV1::from_json_slice(
        &serde_json::to_vec(&sequence).expect("sequence mutation"),
    )
    .expect_err("successor without previous digest must fail");
    assert_eq!(error.code(), VerifyErrorCode::JournalSequenceMismatch);

    let mut previous = serde_json::to_value(&entry).expect("entry json");
    previous["previous_digest"] = serde_json::json!({
        "algorithm":"sha256",
        "hex":"1111111111111111111111111111111111111111111111111111111111111111"
    });
    let error = VerificationJournalEntryV1::from_json_slice(
        &serde_json::to_vec(&previous).expect("previous digest mutation"),
    )
    .expect_err("genesis with previous digest must fail");
    assert_eq!(error.code(), VerifyErrorCode::JournalSequenceMismatch);

    let mut body = serde_json::to_value(&entry).expect("entry json");
    body["body"]["receipt"]["target"]["value"]["reference"] = "mutated".into();
    let error = VerificationJournalEntryV1::from_json_slice(
        &serde_json::to_vec(&body).expect("body mutation"),
    )
    .expect_err("body substitution must fail digest validation");
    assert_eq!(error.code(), VerifyErrorCode::JournalDigestMismatch);

    let mut digest = serde_json::to_value(&entry).expect("entry json");
    digest["entry_digest"]["hex"] =
        "0000000000000000000000000000000000000000000000000000000000000000".into();
    let error = VerificationJournalEntryV1::from_json_slice(
        &serde_json::to_vec(&digest).expect("digest mutation"),
    )
    .expect_err("entry digest substitution must fail");
    assert_eq!(error.code(), VerifyErrorCode::JournalDigestMismatch);
}

#[test]
fn journal_bounds_and_digest_encoding_are_strict() {
    assert!(VerificationJournalSequence::new(0).is_err());
    assert!(VerificationJournalSequence::new(MAX_VERIFICATION_JOURNAL_SEQUENCE).is_ok());
    assert!(VerificationJournalSequence::new(MAX_VERIFICATION_JOURNAL_SEQUENCE + 1).is_err());

    assert!(
        VerificationJournalDigest::new_sha256(
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
        )
        .is_err()
    );
    assert!(VerificationJournalDigest::new_sha256("abc").is_err());
}

#[test]
fn successor_requires_previous_digest() {
    let first = golden_entry();
    let second = VerificationJournalEntryV1::new(
        VerificationJournalSequence::new(2).expect("successor sequence"),
        Some(first.entry_digest().clone()),
        VerificationJournalBodyV1::VerificationReceipt {
            receipt: golden_receipt(),
        },
    )
    .expect("valid successor");
    assert_eq!(second.previous_digest(), Some(first.entry_digest()));
}
