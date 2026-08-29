use ecra_core::{
    ActorId, ClaimRef, VerificationId, VerificationMethod, VerificationOutcome,
    VerificationReceipt, VerificationTarget,
};
use ecra_verify::{
    MAX_MATERIALIZED_JOURNAL_ENTRIES, MAX_VERIFICATION_JOURNAL_ENTRY_BYTES,
    MAX_VERIFICATION_JOURNAL_SEQUENCE, VerificationJournalBodyV1, VerificationJournalDigest,
    VerificationJournalEntryV1, VerificationJournalSequence, VerificationStore, VerifyErrorCode,
};
use proptest::prelude::*;
use rusqlite::{Connection, params};
use sha2::{Digest, Sha256};
use tempfile::tempdir;

const JOURNAL_DOMAIN: &[u8] = b"ecra/verification-journal/v1\0";

fn receipt_with_tail(tail: u64) -> VerificationReceipt {
    VerificationReceipt::new(
        VerificationId::parse_str(&format!("00000000-0000-0000-0000-{tail:012}"))
            .expect("verification id"),
        ActorId::parse_str("00000000-0000-0000-0000-000000000001").expect("actor id"),
        VerificationTarget::Claim(ClaimRef::new("journal", "golden").expect("claim target")),
        VerificationMethod::Other,
        VerificationOutcome::NotEvaluated,
        Vec::new(),
    )
    .expect("verification receipt")
}

fn golden_receipt() -> VerificationReceipt {
    receipt_with_tail(90_001)
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

#[test]
fn journal_entry_byte_limit_accepts_exact_boundary_and_rejects_max_plus_one_typed() {
    let exact = vec![b' '; MAX_VERIFICATION_JOURNAL_ENTRY_BYTES];
    let exact_error = VerificationJournalEntryV1::from_json_slice(&exact)
        .expect_err("non-JSON exact-boundary input still fails parsing");
    assert_ne!(
        exact_error.code(),
        VerifyErrorCode::ResourceLimitExceeded,
        "exact byte ceiling must reach parsing rather than resource rejection"
    );

    let over = vec![b' '; MAX_VERIFICATION_JOURNAL_ENTRY_BYTES + 1];
    let error = VerificationJournalEntryV1::from_json_slice(&over)
        .expect_err("journal byte max+1 must fail");
    assert_eq!(error.code(), VerifyErrorCode::ResourceLimitExceeded);
}

#[test]
fn query_materialization_exact_max_is_accepted_and_max_plus_one_is_typed() {
    let directory = tempdir().expect("tempdir");
    let path = directory.path().join("materialization.sqlite");
    drop(VerificationStore::open(&path).expect("initialize store"));

    let mut connection = Connection::open(&path).expect("open direct fixture connection");
    let transaction = connection.transaction().expect("fixture transaction");
    let mut previous = None;
    for index in 0..MAX_MATERIALIZED_JOURNAL_ENTRIES {
        let sequence =
            VerificationJournalSequence::new(u64::try_from(index + 1).expect("sequence fits u64"))
                .expect("journal sequence");
        let entry = VerificationJournalEntryV1::new(
            sequence,
            previous.clone(),
            VerificationJournalBodyV1::VerificationReceipt {
                receipt: receipt_with_tail(200_000 + u64::try_from(index).expect("index")),
            },
        )
        .expect("materialization fixture entry");
        let json = String::from_utf8(entry.canonical_bytes().expect("entry bytes"))
            .expect("canonical UTF-8");
        transaction
            .execute(
                "INSERT INTO verification_journal (sequence, entry_json, entry_digest) VALUES (?1, ?2, ?3)",
                params![
                    i64::try_from(sequence.get()).expect("SQLite sequence"),
                    json,
                    entry.entry_digest().hex()
                ],
            )
            .expect("insert fixture entry");
        previous = Some(entry.entry_digest().clone());
    }
    transaction.commit().expect("commit exact-max fixture");
    drop(connection);

    let store = VerificationStore::open(&path).expect("reopen exact-max store");
    assert_eq!(
        store
            .load_entries()
            .expect("exact max materialization")
            .len(),
        MAX_MATERIALIZED_JOURNAL_ENTRIES
    );
    drop(store);

    let sequence = VerificationJournalSequence::new(
        u64::try_from(MAX_MATERIALIZED_JOURNAL_ENTRIES + 1).expect("sequence fits u64"),
    )
    .expect("over-limit sequence");
    let over_entry = VerificationJournalEntryV1::new(
        sequence,
        previous,
        VerificationJournalBodyV1::VerificationReceipt {
            receipt: receipt_with_tail(300_000),
        },
    )
    .expect("over-limit fixture entry");
    let over_json = String::from_utf8(over_entry.canonical_bytes().expect("entry bytes"))
        .expect("canonical UTF-8");
    let connection = Connection::open(&path).expect("open direct fixture connection");
    connection
        .execute(
            "INSERT INTO verification_journal (sequence, entry_json, entry_digest) VALUES (?1, ?2, ?3)",
            params![
                i64::try_from(sequence.get()).expect("SQLite sequence"),
                over_json,
                over_entry.entry_digest().hex()
            ],
        )
        .expect("insert max+1 fixture entry");
    drop(connection);

    let store = VerificationStore::open(&path).expect("reopen over-limit store");
    let error = store
        .load_entries()
        .expect_err("materialization max+1 must fail");
    assert_eq!(error.code(), VerifyErrorCode::ResourceLimitExceeded);
}

proptest! {
    #[test]
    fn arbitrary_bounded_journal_json_never_panics(bytes in proptest::collection::vec(any::<u8>(), 0..=4_096)) {
        let _ = VerificationJournalEntryV1::from_json_slice(&bytes);
    }
}
