use std::collections::BTreeSet;
use std::io::{Cursor, Read, Write};

use ecra_core::{ContentDigest, RunId, SchemaVersion, to_jcs_vec};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, DateTime, System, ZipArchive, ZipWriter};

use crate::{
    BudgetAmount, EventSequence, LedgerDigest, RunError, RunErrorCategory, RunErrorCode,
    RunEventEnvelope, RunReducer,
};

pub const MAX_ARCHIVE_ENTRIES: usize = 16_384;
pub const MAX_EVENT_COUNT: usize = 10_000;
pub const MAX_BLOB_COUNT: usize = 6_000;
pub const MAX_MANIFEST_BYTES: u64 = 8 * 1024 * 1024;
pub const MAX_EVENT_ENTRY_BYTES: u64 = 4 * 1024 * 1024;
pub const MAX_SINGLE_BLOB_BYTES: u64 = 64 * 1024 * 1024;
pub const MAX_TOTAL_UNCOMPRESSED_BYTES: u64 = 512 * 1024 * 1024;
pub const MAX_PATH_BYTES: usize = 512;

const MANIFEST_PATH: &str = "manifest.v1.json";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestEventEntry {
    sequence: EventSequence,
    path: String,
    ledger_digest: LedgerDigest,
    byte_size: BudgetAmount,
    content_digest: ContentDigest,
}

impl ManifestEventEntry {
    #[must_use]
    pub const fn sequence(&self) -> EventSequence {
        self.sequence
    }

    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    #[must_use]
    pub fn ledger_digest(&self) -> &LedgerDigest {
        &self.ledger_digest
    }

    #[must_use]
    pub const fn byte_size(&self) -> BudgetAmount {
        self.byte_size
    }

    #[must_use]
    pub const fn content_digest(&self) -> &ContentDigest {
        &self.content_digest
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestBlobEntry {
    path: String,
    content_digest: ContentDigest,
    byte_size: BudgetAmount,
}

impl ManifestBlobEntry {
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    #[must_use]
    pub const fn content_digest(&self) -> &ContentDigest {
        &self.content_digest
    }

    #[must_use]
    pub const fn byte_size(&self) -> BudgetAmount {
        self.byte_size
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcraRunManifest {
    schema_version: SchemaVersion,
    run_id: RunId,
    event_count: EventSequence,
    head_digest: LedgerDigest,
    events: Vec<ManifestEventEntry>,
    blobs: Vec<ManifestBlobEntry>,
}

impl EcraRunManifest {
    #[must_use]
    pub const fn schema_version(&self) -> SchemaVersion {
        self.schema_version
    }

    #[must_use]
    pub const fn run_id(&self) -> RunId {
        self.run_id
    }

    #[must_use]
    pub const fn event_count(&self) -> EventSequence {
        self.event_count
    }

    #[must_use]
    pub fn head_digest(&self) -> &LedgerDigest {
        &self.head_digest
    }

    #[must_use]
    pub fn events(&self) -> &[ManifestEventEntry] {
        &self.events
    }

    #[must_use]
    pub fn blobs(&self) -> &[ManifestBlobEntry] {
        &self.blobs
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArchiveBlob {
    content_digest: ContentDigest,
    bytes: Vec<u8>,
}

impl ArchiveBlob {
    pub fn new(content_digest: ContentDigest, bytes: Vec<u8>) -> Result<Self, RunError> {
        validate_sha256_content_digest(&content_digest)?;
        verify_content_digest(&content_digest, &bytes)?;
        if u64::try_from(bytes.len()).map_err(|_| archive_limit("blob length does not fit u64"))?
            > MAX_SINGLE_BLOB_BYTES
        {
            return Err(archive_limit("blob exceeds MAX_SINGLE_BLOB_BYTES"));
        }
        Ok(Self {
            content_digest,
            bytes,
        })
    }

    #[must_use]
    pub const fn content_digest(&self) -> &ContentDigest {
        &self.content_digest
    }

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatedEcraArchive {
    manifest: EcraRunManifest,
    events: Vec<RunEventEnvelope>,
    blobs: Vec<ArchiveBlob>,
}

impl ValidatedEcraArchive {
    #[must_use]
    pub const fn manifest(&self) -> &EcraRunManifest {
        &self.manifest
    }

    #[must_use]
    pub fn events(&self) -> &[RunEventEnvelope] {
        &self.events
    }

    #[must_use]
    pub fn blobs(&self) -> &[ArchiveBlob] {
        &self.blobs
    }
}

pub fn export_ecra(
    history: &[RunEventEnvelope],
    blobs: &[ArchiveBlob],
) -> Result<Vec<u8>, RunError> {
    if history.is_empty() {
        return Err(archive_manifest("archive history must be non-empty"));
    }
    if history.len() > MAX_EVENT_COUNT || blobs.len() > MAX_BLOB_COUNT {
        return Err(archive_limit(
            "logical archive entry count exceeds v1 limits",
        ));
    }
    let state = RunReducer::reduce(history)?;
    let event_count = EventSequence::new(
        u64::try_from(history.len()).map_err(|_| archive_limit("event count does not fit u64"))?,
    )?;

    let mut event_entries = Vec::with_capacity(history.len());
    let mut event_payloads = Vec::with_capacity(history.len());
    let mut total = 0_u64;
    for (index, envelope) in history.iter().enumerate() {
        let expected_sequence =
            u64::try_from(index + 1).map_err(|_| archive_limit("event index does not fit u64"))?;
        if envelope.run_id() != state.run_id() || envelope.sequence().get() != expected_sequence {
            return Err(archive_manifest(
                "archive history must exactly cover one run at sequences 1..=event_count",
            ));
        }
        let payload =
            to_jcs_vec(envelope).map_err(|error| RunError::serialization(error.to_string()))?;
        let byte_size = checked_entry_size(payload.len(), MAX_EVENT_ENTRY_BYTES, "event")?;
        total = checked_total(total, byte_size.get())?;
        let path = event_path(envelope.sequence());
        event_entries.push(ManifestEventEntry {
            sequence: envelope.sequence(),
            path,
            ledger_digest: envelope.event_digest().clone(),
            byte_size,
            content_digest: sha256_content_digest(&payload)?,
        });
        event_payloads.push(payload);
    }

    let mut sorted_blobs: Vec<&ArchiveBlob> = blobs.iter().collect();
    sorted_blobs.sort_by(|left, right| {
        blob_path(left.content_digest()).cmp(&blob_path(right.content_digest()))
    });
    let mut blob_entries = Vec::with_capacity(sorted_blobs.len());
    let mut seen_blob_paths = BTreeSet::new();
    for blob in &sorted_blobs {
        validate_sha256_content_digest(blob.content_digest())?;
        verify_content_digest(blob.content_digest(), blob.bytes())?;
        let byte_size = checked_entry_size(blob.bytes().len(), MAX_SINGLE_BLOB_BYTES, "blob")?;
        total = checked_total(total, byte_size.get())?;
        let path = blob_path(blob.content_digest());
        if !seen_blob_paths.insert(path.clone()) {
            return Err(archive_duplicate("duplicate content-addressed blob path"));
        }
        blob_entries.push(ManifestBlobEntry {
            path,
            content_digest: blob.content_digest().clone(),
            byte_size,
        });
    }

    let manifest = EcraRunManifest {
        schema_version: SchemaVersion::V1_0,
        run_id: state.run_id(),
        event_count,
        head_digest: state.last_digest().clone(),
        events: event_entries,
        blobs: blob_entries,
    };
    validate_manifest(&manifest)?;
    let manifest_bytes =
        to_jcs_vec(&manifest).map_err(|error| RunError::serialization(error.to_string()))?;
    let manifest_size = u64::try_from(manifest_bytes.len())
        .map_err(|_| archive_limit("manifest length does not fit u64"))?;
    if manifest_size > MAX_MANIFEST_BYTES {
        return Err(archive_limit("manifest exceeds MAX_MANIFEST_BYTES"));
    }
    let _ = checked_total(total, manifest_size)?;

    let cursor = Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);
    let options = canonical_file_options();
    write_entry(&mut writer, MANIFEST_PATH, &manifest_bytes, options)?;
    for (entry, payload) in manifest.events.iter().zip(event_payloads.iter()) {
        write_entry(&mut writer, &entry.path, payload, options)?;
    }
    for (entry, blob) in manifest.blobs.iter().zip(sorted_blobs.iter()) {
        write_entry(&mut writer, &entry.path, blob.bytes(), options)?;
    }
    let cursor = writer.finish().map_err(map_zip_error)?;
    Ok(cursor.into_inner())
}

fn declared_archive_entry_count(bytes: &[u8]) -> Result<usize, RunError> {
    const EOCD_LEN: usize = 22;
    const EOCD_SIGNATURE: [u8; 4] = [0x50, 0x4b, 0x05, 0x06];

    if bytes.len() < EOCD_LEN {
        return Err(archive_manifest(
            "ZIP end-of-central-directory record is missing",
        ));
    }
    let eocd = &bytes[bytes.len() - EOCD_LEN..];
    if eocd[..4] != EOCD_SIGNATURE {
        return Err(archive_feature(
            "ZIP comments, trailing bytes and non-canonical EOCD layout are unsupported",
        ));
    }
    let comment_len = u16::from_le_bytes([eocd[20], eocd[21]]);
    if comment_len != 0 {
        return Err(archive_feature("ZIP archive comments are unsupported"));
    }
    let entries_on_disk = u16::from_le_bytes([eocd[8], eocd[9]]);
    let total_entries = u16::from_le_bytes([eocd[10], eocd[11]]);
    if entries_on_disk != total_entries {
        return Err(archive_feature("multi-disk ZIP archives are unsupported"));
    }
    Ok(usize::from(total_entries))
}

pub fn read_ecra(bytes: &[u8]) -> Result<ValidatedEcraArchive, RunError> {
    let mut archive = ZipArchive::new(Cursor::new(bytes)).map_err(map_zip_error)?;
    if archive.offset() != 0 || !archive.comment().is_empty() {
        return Err(archive_feature(
            "archive prefix data and archive comments are unsupported",
        ));
    }
    let declared_entries = declared_archive_entry_count(bytes)?;
    if declared_entries > MAX_ARCHIVE_ENTRIES {
        return Err(archive_limit("archive exceeds MAX_ARCHIVE_ENTRIES"));
    }
    if declared_entries != archive.len() {
        return Err(archive_duplicate(
            "duplicate archive entry names collapsed during ZIP metadata parsing",
        ));
    }
    if archive.len() > MAX_ARCHIVE_ENTRIES {
        return Err(archive_limit("archive exceeds MAX_ARCHIVE_ENTRIES"));
    }

    let mut names = BTreeSet::new();
    let mut event_count = 0_usize;
    let mut blob_count = 0_usize;
    let mut total = 0_u64;
    for index in 0..archive.len() {
        let file = archive.by_index(index).map_err(map_zip_error)?;
        let raw_name = std::str::from_utf8(file.name_raw())
            .map_err(|_| archive_path("archive entry name is not UTF-8"))?;
        if raw_name != file.name() {
            return Err(archive_path("archive entry name is not canonical UTF-8"));
        }
        validate_archive_path(raw_name)?;
        if !names.insert(raw_name.to_owned()) {
            return Err(archive_duplicate("duplicate archive entry name"));
        }
        if file.compression() != CompressionMethod::Stored {
            return Err(archive_feature("only ZIP Stored entries are supported"));
        }
        if file.encrypted() || file.is_dir() || file.is_symlink() || !file.is_file() {
            return Err(archive_feature(
                "encrypted, directory, symlink or non-file entries are unsupported",
            ));
        }
        if !file.comment().is_empty() || file.extra_data().is_some_and(|data| !data.is_empty()) {
            return Err(archive_feature(
                "entry comments and extra fields are unsupported",
            ));
        }
        if file.last_modified() != Some(DateTime::DEFAULT) {
            return Err(archive_feature("entry timestamp is outside the v1 profile"));
        }
        if file.unix_mode().map(|mode| mode & 0o7777) != Some(0o600) {
            return Err(archive_feature(
                "entry permissions are outside the v1 profile",
            ));
        }

        let size = file.size();
        if raw_name == MANIFEST_PATH {
            if size > MAX_MANIFEST_BYTES {
                return Err(archive_limit("manifest exceeds MAX_MANIFEST_BYTES"));
            }
        } else if raw_name.starts_with("events/") {
            event_count = event_count
                .checked_add(1)
                .ok_or_else(|| archive_limit("event count overflow"))?;
            if event_count > MAX_EVENT_COUNT || size > MAX_EVENT_ENTRY_BYTES {
                return Err(archive_limit("event archive limits exceeded"));
            }
        } else if raw_name.starts_with("blobs/") {
            blob_count = blob_count
                .checked_add(1)
                .ok_or_else(|| archive_limit("blob count overflow"))?;
            if blob_count > MAX_BLOB_COUNT || size > MAX_SINGLE_BLOB_BYTES {
                return Err(archive_limit("blob archive limits exceeded"));
            }
        }
        total = checked_total(total, size)?;
    }

    if !names.contains(MANIFEST_PATH) {
        return Err(archive_manifest("manifest.v1.json is required"));
    }
    let manifest_bytes = read_named_entry(&mut archive, MANIFEST_PATH, MAX_MANIFEST_BYTES)?;
    let manifest: EcraRunManifest = serde_json::from_slice(&manifest_bytes)
        .map_err(|error| archive_manifest(error.to_string()))?;
    validate_manifest(&manifest)?;
    let canonical_manifest =
        to_jcs_vec(&manifest).map_err(|error| RunError::serialization(error.to_string()))?;
    if canonical_manifest != manifest_bytes {
        return Err(archive_manifest("manifest JSON is not RFC 8785 canonical"));
    }

    let mut allowed = BTreeSet::from([MANIFEST_PATH.to_owned()]);
    for entry in &manifest.events {
        if !allowed.insert(entry.path.clone()) {
            return Err(archive_duplicate("manifest declares duplicate entry path"));
        }
    }
    for entry in &manifest.blobs {
        if !allowed.insert(entry.path.clone()) {
            return Err(archive_duplicate("manifest declares duplicate entry path"));
        }
    }
    if allowed != names {
        return Err(archive_manifest(
            "archive entry set does not exactly match manifest declarations",
        ));
    }

    let mut events = Vec::with_capacity(manifest.events.len());
    for entry in &manifest.events {
        let payload = read_named_entry(&mut archive, &entry.path, MAX_EVENT_ENTRY_BYTES)?;
        verify_declared_entry(&payload, entry.byte_size, &entry.content_digest)?;
        let envelope = RunEventEnvelope::from_json_slice(&payload)?;
        let canonical =
            to_jcs_vec(&envelope).map_err(|error| RunError::serialization(error.to_string()))?;
        if canonical != payload {
            return Err(archive_manifest("event JSON is not RFC 8785 canonical"));
        }
        if envelope.run_id() != manifest.run_id
            || envelope.sequence() != entry.sequence
            || envelope.event_digest() != &entry.ledger_digest
        {
            return Err(archive_manifest(
                "manifest event binding does not match canonical event envelope",
            ));
        }
        events.push(envelope);
    }
    let state = RunReducer::reduce(&events)?;
    if state.run_id() != manifest.run_id || state.last_digest() != &manifest.head_digest {
        return Err(archive_digest(
            "manifest head digest does not match validated event history",
        ));
    }

    let mut blobs = Vec::with_capacity(manifest.blobs.len());
    for entry in &manifest.blobs {
        let payload = read_named_entry(&mut archive, &entry.path, MAX_SINGLE_BLOB_BYTES)?;
        verify_declared_entry(&payload, entry.byte_size, &entry.content_digest)?;
        blobs.push(ArchiveBlob::new(entry.content_digest.clone(), payload)?);
    }

    Ok(ValidatedEcraArchive {
        manifest,
        events,
        blobs,
    })
}

fn validate_manifest(manifest: &EcraRunManifest) -> Result<(), RunError> {
    if manifest.schema_version.major() != 1 {
        return Err(RunError::new(
            RunErrorCategory::Compatibility,
            RunErrorCode::UnsupportedMajorVersion,
            "unsupported archive manifest major version",
        ));
    }
    if manifest.schema_version.minor() > 0 {
        return Err(RunError::new(
            RunErrorCategory::Compatibility,
            RunErrorCode::UnsupportedMinorVersion,
            "unsupported archive manifest minor version",
        ));
    }
    let event_count = usize::try_from(manifest.event_count.get())
        .map_err(|_| archive_limit("manifest event count does not fit usize"))?;
    if event_count == 0 || event_count > MAX_EVENT_COUNT || manifest.events.len() != event_count {
        return Err(archive_manifest(
            "manifest events must exactly cover non-empty event_count within v1 limit",
        ));
    }
    if manifest.blobs.len() > MAX_BLOB_COUNT {
        return Err(archive_limit("manifest blob count exceeds v1 limit"));
    }

    for (index, entry) in manifest.events.iter().enumerate() {
        let expected = EventSequence::new(
            u64::try_from(index + 1)
                .map_err(|_| archive_limit("manifest event index does not fit u64"))?,
        )?;
        if entry.sequence != expected || entry.path != event_path(expected) {
            return Err(archive_manifest(
                "manifest event entries must be ordered and canonically named",
            ));
        }
        validate_archive_path(&entry.path)?;
        validate_sha256_content_digest(&entry.content_digest)?;
        if entry.byte_size.get() > MAX_EVENT_ENTRY_BYTES {
            return Err(archive_limit("manifest event entry exceeds size limit"));
        }
    }

    let mut prior_blob_path: Option<&str> = None;
    for entry in &manifest.blobs {
        validate_archive_path(&entry.path)?;
        validate_sha256_content_digest(&entry.content_digest)?;
        if entry.path != blob_path(&entry.content_digest) {
            return Err(archive_manifest(
                "manifest blob path is not content-addressed",
            ));
        }
        if prior_blob_path.is_some_and(|prior| prior >= entry.path.as_str()) {
            return Err(archive_manifest(
                "manifest blob entries must be strictly lexicographically ordered",
            ));
        }
        prior_blob_path = Some(&entry.path);
        if entry.byte_size.get() > MAX_SINGLE_BLOB_BYTES {
            return Err(archive_limit("manifest blob entry exceeds size limit"));
        }
    }
    Ok(())
}

fn canonical_file_options() -> SimpleFileOptions {
    SimpleFileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .last_modified_time(DateTime::DEFAULT)
        .system(System::Unix)
        .unix_permissions(0o600)
}

fn write_entry(
    writer: &mut ZipWriter<Cursor<Vec<u8>>>,
    path: &str,
    bytes: &[u8],
    options: SimpleFileOptions,
) -> Result<(), RunError> {
    validate_archive_path(path)?;
    writer.start_file(path, options).map_err(map_zip_error)?;
    writer.write_all(bytes).map_err(|error| {
        RunError::new(
            RunErrorCategory::Archive,
            RunErrorCode::StorageError,
            format!("write archive entry: {error}"),
        )
    })?;
    Ok(())
}

fn read_named_entry(
    archive: &mut ZipArchive<Cursor<&[u8]>>,
    path: &str,
    limit: u64,
) -> Result<Vec<u8>, RunError> {
    let mut file = archive.by_name(path).map_err(map_zip_error)?;
    if file.size() > limit {
        return Err(archive_limit("entry exceeds configured reader limit"));
    }
    let capacity =
        usize::try_from(file.size()).map_err(|_| archive_limit("entry size does not fit usize"))?;
    let mut bytes = Vec::with_capacity(capacity);
    file.read_to_end(&mut bytes).map_err(|error| {
        RunError::new(
            RunErrorCategory::Archive,
            RunErrorCode::ArchiveManifestInvalid,
            format!("read archive entry: {error}"),
        )
    })?;
    if u64::try_from(bytes.len()).map_err(|_| archive_limit("entry length does not fit u64"))?
        != file.size()
    {
        return Err(archive_manifest(
            "materialized entry size differs from ZIP metadata",
        ));
    }
    Ok(bytes)
}

fn validate_archive_path(path: &str) -> Result<(), RunError> {
    if path.is_empty()
        || path.len() > MAX_PATH_BYTES
        || path.starts_with('/')
        || path.ends_with('/')
        || path.contains('\\')
        || path.contains('\0')
    {
        return Err(archive_path("archive path violates the v1 path profile"));
    }
    let mut segments = path.split('/');
    if let Some(first) = segments.next() {
        let bytes = first.as_bytes();
        if bytes.len() == 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
            return Err(archive_path("Windows absolute drive paths are forbidden"));
        }
        if first.is_empty() || matches!(first, "." | "..") {
            return Err(archive_path("archive path contains a forbidden segment"));
        }
    }
    if segments.any(|segment| segment.is_empty() || matches!(segment, "." | "..")) {
        return Err(archive_path("archive path contains a forbidden segment"));
    }
    Ok(())
}

fn event_path(sequence: EventSequence) -> String {
    format!("events/{:016}.json", sequence.get())
}

fn blob_path(digest: &ContentDigest) -> String {
    format!("blobs/sha256/{}", digest.hex())
}

fn checked_entry_size(length: usize, limit: u64, kind: &str) -> Result<BudgetAmount, RunError> {
    let length = u64::try_from(length)
        .map_err(|_| archive_limit(format!("{kind} entry length does not fit u64")))?;
    if length > limit {
        return Err(archive_limit(format!("{kind} entry exceeds v1 size limit")));
    }
    BudgetAmount::new(length)
}

fn checked_total(current: u64, addition: u64) -> Result<u64, RunError> {
    let total = current
        .checked_add(addition)
        .ok_or_else(|| archive_limit("total uncompressed byte count overflow"))?;
    if total > MAX_TOTAL_UNCOMPRESSED_BYTES {
        return Err(archive_limit(
            "archive exceeds MAX_TOTAL_UNCOMPRESSED_BYTES",
        ));
    }
    Ok(total)
}

fn verify_declared_entry(
    bytes: &[u8],
    declared_size: BudgetAmount,
    declared_digest: &ContentDigest,
) -> Result<(), RunError> {
    let actual_size = u64::try_from(bytes.len())
        .map_err(|_| archive_limit("materialized entry length does not fit u64"))?;
    if actual_size != declared_size.get() {
        return Err(archive_digest(
            "manifest byte_size does not match entry bytes",
        ));
    }
    verify_content_digest(declared_digest, bytes)
}

fn sha256_content_digest(bytes: &[u8]) -> Result<ContentDigest, RunError> {
    ContentDigest::new("sha256", sha256_hex(bytes))
        .map_err(|error| archive_manifest(error.to_string()))
}

fn validate_sha256_content_digest(digest: &ContentDigest) -> Result<(), RunError> {
    if digest.algorithm() != "sha256"
        || digest.hex().len() != 64
        || !digest
            .hex()
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(archive_manifest(
            "archive ContentDigest must be canonical sha256 lowercase hex",
        ));
    }
    Ok(())
}

fn verify_content_digest(digest: &ContentDigest, bytes: &[u8]) -> Result<(), RunError> {
    validate_sha256_content_digest(digest)?;
    if sha256_hex(bytes) != digest.hex() {
        return Err(archive_digest("ContentDigest does not match entry bytes"));
    }
    Ok(())
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

fn map_zip_error(error: zip::result::ZipError) -> RunError {
    use zip::result::ZipError;
    match error {
        ZipError::CompressionMethodNotSupported(_) | ZipError::UnsupportedArchive(_) => {
            archive_feature(error.to_string())
        }
        ZipError::InvalidPassword => archive_feature("encrypted ZIP entries are unsupported"),
        ZipError::Io(_) | ZipError::InvalidArchive(_) | ZipError::FileNotFound => {
            archive_manifest(error.to_string())
        }
        _ => archive_manifest(error.to_string()),
    }
}

fn archive_path(message: impl Into<String>) -> RunError {
    RunError::new(
        RunErrorCategory::Archive,
        RunErrorCode::ArchivePathInvalid,
        message,
    )
}

fn archive_duplicate(message: impl Into<String>) -> RunError {
    RunError::new(
        RunErrorCategory::Archive,
        RunErrorCode::ArchiveDuplicateEntry,
        message,
    )
}

fn archive_feature(message: impl Into<String>) -> RunError {
    RunError::new(
        RunErrorCategory::Archive,
        RunErrorCode::ArchiveFeatureUnsupported,
        message,
    )
}

fn archive_limit(message: impl Into<String>) -> RunError {
    RunError::new(
        RunErrorCategory::Archive,
        RunErrorCode::ArchiveLimitExceeded,
        message,
    )
}

fn archive_manifest(message: impl Into<String>) -> RunError {
    RunError::new(
        RunErrorCategory::Archive,
        RunErrorCode::ArchiveManifestInvalid,
        message,
    )
}

fn archive_digest(message: impl Into<String>) -> RunError {
    RunError::new(
        RunErrorCategory::Archive,
        RunErrorCode::ArchiveDigestMismatch,
        message,
    )
}
