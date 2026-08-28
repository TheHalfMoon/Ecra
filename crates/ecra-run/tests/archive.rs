use std::io::{Cursor, Read, Write};
use std::path::PathBuf;

use ecra_core::{ContentDigest, EpochMillis, to_jcs_vec};
use ecra_run::{
    ArchiveBlob, MAX_ARCHIVE_ENTRIES, MAX_EVENT_COUNT, MAX_SINGLE_BLOB_BYTES, RunErrorCode,
    RunEvent, RunEventEnvelope, export_ecra, read_ecra,
};
use sha2::{Digest, Sha256};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, DateTime, System, ZipArchive, ZipWriter};

fn genesis() -> RunEventEnvelope {
    RunEventEnvelope::from_json_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/run-created-envelope.v1.json"
    ))
    .expect("genesis fixture")
}

fn fixture_event(kind: &str) -> RunEvent {
    let events: Vec<RunEvent> = serde_json::from_slice(include_bytes!(
        "../../../contracts/ecra-run-v1/valid/all-event-kinds.v1.json"
    ))
    .expect("event fixtures");
    events
        .into_iter()
        .find(|candidate| candidate.kind() == kind)
        .unwrap_or_else(|| panic!("missing fixture {kind}"))
}

fn history() -> Vec<RunEventEnvelope> {
    let created = genesis();
    let started = RunEventEnvelope::new(
        created.run_id(),
        created.sequence().checked_next().expect("next sequence"),
        EpochMillis::new(created.recorded_at().get() + 1).expect("timestamp"),
        Some(created.event_digest().clone()),
        fixture_event("run_started"),
    )
    .expect("started envelope");
    vec![created, started]
}

fn sha256_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(64);
    for byte in digest {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}

fn blob(bytes: &[u8]) -> ArchiveBlob {
    let digest = ContentDigest::new("sha256", sha256_hex(bytes)).expect("digest");
    ArchiveBlob::new(digest, bytes.to_vec()).expect("archive blob")
}

fn options() -> SimpleFileOptions {
    SimpleFileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .last_modified_time(DateTime::DEFAULT)
        .system(System::Unix)
        .unix_permissions(0o600)
}

fn zip_entries(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    for (name, bytes) in entries {
        writer.start_file(*name, options()).expect("start entry");
        writer.write_all(bytes).expect("write entry");
    }
    writer.finish().expect("finish zip").into_inner()
}

fn zip_names(bytes: &[u8]) -> Vec<String> {
    let mut zip = ZipArchive::new(Cursor::new(bytes)).expect("open zip");
    (0..zip.len())
        .map(|index| zip.by_index(index).expect("entry").name().to_owned())
        .collect()
}

fn repack_with_manifest(source: &[u8], manifest_bytes: &[u8]) -> Vec<u8> {
    let mut input = ZipArchive::new(Cursor::new(source)).expect("input archive");
    let mut entries = Vec::new();
    for index in 0..input.len() {
        let mut file = input.by_index(index).expect("input entry");
        let name = file.name().to_owned();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).expect("read input entry");
        if name == "manifest.v1.json" {
            entries.push((name, manifest_bytes.to_vec()));
        } else {
            entries.push((name, bytes));
        }
    }
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    for (name, bytes) in entries {
        writer
            .start_file(name, options())
            .expect("start repacked entry");
        writer.write_all(&bytes).expect("write repacked entry");
    }
    writer.finish().expect("finish repack").into_inner()
}

fn patch_u16_all(bytes: &mut [u8], local_offset: usize, central_offset: usize, value: u16) {
    let encoded = value.to_le_bytes();
    let mut index = 0;
    while index + 12 <= bytes.len() {
        if bytes[index..].starts_with(b"PK\x03\x04") {
            bytes[index + local_offset..index + local_offset + 2].copy_from_slice(&encoded);
        } else if bytes[index..].starts_with(b"PK\x01\x02") {
            bytes[index + central_offset..index + central_offset + 2].copy_from_slice(&encoded);
        }
        index += 1;
    }
}

fn patch_first_entry_uncompressed_size(bytes: &mut [u8], value: u32) {
    let encoded = value.to_le_bytes();
    let local = bytes
        .windows(4)
        .position(|window| window == b"PK\x03\x04")
        .expect("local header");
    bytes[local + 22..local + 26].copy_from_slice(&encoded);
    let central = bytes
        .windows(4)
        .position(|window| window == b"PK\x01\x02")
        .expect("central header");
    bytes[central + 24..central + 28].copy_from_slice(&encoded);
}

#[test]
fn deterministic_export_roundtrip_and_golden_hash() {
    let history = history();
    let blobs = vec![blob(b"synthetic archive blob")];
    let first = export_ecra(&history, &blobs).expect("first export");
    let second = export_ecra(&history, &blobs).expect("second export");
    assert_eq!(first, second);

    let validated = read_ecra(&first).expect("strict import");
    assert_eq!(validated.events(), history.as_slice());
    assert_eq!(validated.blobs(), blobs.as_slice());
    assert_eq!(
        zip_names(&first),
        vec![
            "manifest.v1.json".to_owned(),
            "events/0000000000000001.json".to_owned(),
            "events/0000000000000002.json".to_owned(),
            format!("blobs/sha256/{}", blobs[0].content_digest().hex()),
        ]
    );

    let expected_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../contracts/ecra-run-v1/expected");
    let archive_path = expected_dir.join("archive-golden.v1.ecra");
    let digest_path = expected_dir.join("archive-golden.v1.sha256");
    let digest = sha256_hex(&first);
    if std::env::var_os("ECRA_WRITE_ARCHIVE_GOLDEN").is_some() {
        std::fs::create_dir_all(&expected_dir).expect("create expected dir");
        std::fs::write(&archive_path, &first).expect("write archive golden");
        std::fs::write(&digest_path, format!("{digest}\n")).expect("write archive digest");
        return;
    }
    assert_eq!(std::fs::read(&archive_path).expect("archive golden"), first);
    assert_eq!(
        std::fs::read_to_string(&digest_path)
            .expect("archive digest")
            .trim(),
        digest
    );
}

#[test]
fn writer_uses_strict_stored_metadata_profile() {
    let bytes = export_ecra(&history(), &[]).expect("export");
    let mut zip = ZipArchive::new(Cursor::new(&bytes)).expect("open archive");
    assert!(zip.comment().is_empty());
    assert_eq!(zip.offset(), 0);
    for index in 0..zip.len() {
        let file = zip.by_index(index).expect("entry");
        assert_eq!(file.compression(), CompressionMethod::Stored);
        assert!(!file.encrypted());
        assert!(!file.is_dir());
        assert!(!file.is_symlink());
        assert!(file.is_file());
        assert!(file.comment().is_empty());
        assert!(file.extra_data().is_none_or(|data| data.is_empty()));
        assert_eq!(file.last_modified(), Some(DateTime::DEFAULT));
        assert_eq!(file.unix_mode().map(|mode| mode & 0o7777), Some(0o600));
    }
}

#[test]
fn malicious_paths_duplicates_symlink_compression_and_encryption_fail_closed() {
    for name in [
        "/absolute",
        "../traversal",
        "events/../traversal",
        "events\\backslash",
        "bad\0nul",
        "C:/absolute",
    ] {
        let bytes = zip_entries(&[(name, b"x")]);
        let error = read_ecra(&bytes).expect_err("malicious path must fail");
        assert_eq!(error.code(), RunErrorCode::ArchivePathInvalid, "{name:?}");
    }

    let mut duplicate = zip_entries(&[("manifest.v1.json", b"{}"), ("manifest.v2.json", b"{}")]);
    let old_name = b"manifest.v2.json";
    let new_name = b"manifest.v1.json";
    let mut replacements = 0_usize;
    let mut index = 0_usize;
    while index + old_name.len() <= duplicate.len() {
        if &duplicate[index..index + old_name.len()] == old_name {
            duplicate[index..index + old_name.len()].copy_from_slice(new_name);
            replacements += 1;
            index += old_name.len();
        } else {
            index += 1;
        }
    }
    assert_eq!(
        replacements, 2,
        "local and central ZIP names must both be patched"
    );
    let error = read_ecra(&duplicate).expect_err("duplicate entry must fail");
    assert_eq!(error.code(), RunErrorCode::ArchiveDuplicateEntry);

    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    writer
        .add_symlink("manifest.v1.json", "target", options())
        .expect("add symlink");
    let symlink = writer.finish().expect("finish symlink zip").into_inner();
    let error = read_ecra(&symlink).expect_err("symlink must fail");
    assert_eq!(error.code(), RunErrorCode::ArchiveFeatureUnsupported);

    let mut unsupported = zip_entries(&[("manifest.v1.json", b"{}")]);
    patch_u16_all(&mut unsupported, 8, 10, 8);
    let error = read_ecra(&unsupported).expect_err("unsupported compression must fail");
    assert_eq!(error.code(), RunErrorCode::ArchiveFeatureUnsupported);

    let mut encrypted = zip_entries(&[("manifest.v1.json", b"{}")]);
    patch_u16_all(&mut encrypted, 6, 8, 1);
    let error = read_ecra(&encrypted).expect_err("encrypted flag must fail");
    assert_eq!(error.code(), RunErrorCode::ArchiveFeatureUnsupported);
}

#[test]
fn archive_count_and_size_preflight_limits_fail_before_materialization() {
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    for index in 0..=MAX_ARCHIVE_ENTRIES {
        writer
            .start_file(format!("x/{index:05}"), options())
            .expect("start count entry");
    }
    let too_many = writer.finish().expect("finish count zip").into_inner();
    let error = read_ecra(&too_many).expect_err("entry count must fail");
    assert_eq!(error.code(), RunErrorCode::ArchiveLimitExceeded);

    let event_count_manifest = format!(
        "{{\"schema_version\":{{\"major\":1,\"minor\":0}},\"run_id\":\"00000000-0000-0000-0000-000000000001\",\"event_count\":{},\"head_digest\":{{\"algorithm\":\"sha256\",\"hex\":\"{}\"}},\"events\":[],\"blobs\":[]}}",
        MAX_EVENT_COUNT + 1,
        "0".repeat(64)
    );
    let bytes = zip_entries(&[("manifest.v1.json", event_count_manifest.as_bytes())]);
    let error = read_ecra(&bytes).expect_err("manifest event count limit must fail");
    assert!(matches!(
        error.code(),
        RunErrorCode::ArchiveLimitExceeded | RunErrorCode::ArchiveManifestInvalid
    ));

    let digest = "0".repeat(64);
    let path = format!("blobs/sha256/{digest}");
    let mut oversized = zip_entries(&[(path.as_str(), b"")]);
    patch_first_entry_uncompressed_size(
        &mut oversized,
        u32::try_from(MAX_SINGLE_BLOB_BYTES + 1).expect("fits u32"),
    );
    let error = read_ecra(&oversized).expect_err("oversized blob metadata must fail");
    assert_eq!(error.code(), RunErrorCode::ArchiveLimitExceeded);
}

#[test]
fn malformed_manifest_content_and_ledger_digests_fail_closed() {
    let malformed = zip_entries(&[("manifest.v1.json", b"{}")]);
    let error = read_ecra(&malformed).expect_err("malformed manifest must fail");
    assert_eq!(error.code(), RunErrorCode::ArchiveManifestInvalid);

    let exported = export_ecra(&history(), &[]).expect("export");
    let mut zip = ZipArchive::new(Cursor::new(&exported)).expect("open exported");
    let mut manifest_bytes = Vec::new();
    zip.by_name("manifest.v1.json")
        .expect("manifest")
        .read_to_end(&mut manifest_bytes)
        .expect("read manifest");
    let mut manifest: serde_json::Value =
        serde_json::from_slice(&manifest_bytes).expect("manifest JSON");
    manifest["events"][0]["content_digest"]["hex"] = serde_json::Value::String("0".repeat(64));
    let bad_content_manifest = to_jcs_vec(&manifest).expect("canonical bad content manifest");
    let bad_content = repack_with_manifest(&exported, &bad_content_manifest);
    let error = read_ecra(&bad_content).expect_err("content digest mismatch must fail");
    assert_eq!(error.code(), RunErrorCode::ArchiveDigestMismatch);

    let mut manifest: serde_json::Value =
        serde_json::from_slice(&manifest_bytes).expect("manifest JSON");
    manifest["head_digest"]["hex"] = serde_json::Value::String("1".repeat(64));
    let bad_head_manifest = to_jcs_vec(&manifest).expect("canonical bad head manifest");
    let bad_head = repack_with_manifest(&exported, &bad_head_manifest);
    let error = read_ecra(&bad_head).expect_err("head digest mismatch must fail");
    assert_eq!(error.code(), RunErrorCode::ArchiveDigestMismatch);
}

#[test]
fn archive_exports_only_logical_synthetic_content_never_sqlite_or_wal_files() {
    let bytes = export_ecra(&history(), &[blob(b"synthetic-only-fixture")]).expect("export");
    let names = zip_names(&bytes);
    assert!(names.iter().all(|name| !name.ends_with(".db")));
    assert!(names.iter().all(|name| !name.ends_with("-wal")));
    assert!(names.iter().all(|name| !name.ends_with("-shm")));
    assert!(
        !bytes
            .windows(b"SQLite format 3".len())
            .any(|window| window == b"SQLite format 3")
    );
    read_ecra(&bytes).expect("synthetic archive validates");
}
