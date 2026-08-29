use std::{fs, path::Path};

use ecra_identity::{ProtectedEnvelopeV1, envelope::SensitiveBytes};

const SYNTHETIC_SECRET: &str = "ECR031_SYNTHETIC_SIGNING_SECRET_DO_NOT_LOG";
const SENSITIVE_SENTINELS: &[&str] = &[
    SYNTHETIC_SECRET,
    "ECR031_SYNTHETIC_MASTER_SECRET_DO_NOT_LOG",
    "ECR031_SYNTHETIC_PRIVATE_KEY_DO_NOT_LOG",
    "ECR031_SYNTHETIC_SECRET_VALUE_DO_NOT_LOG",
];

fn block<'a>(source: &'a str, start: &str, end: &str) -> &'a str {
    let start_index = source.find(start).expect("start marker");
    let tail = &source[start_index..];
    let end_index = tail.find(end).expect("end marker");
    &tail[..end_index]
}

fn assert_tree_excludes_sentinel(root: &Path, sentinel: &[u8]) {
    let mut stack = vec![root.to_path_buf()];
    let mut scanned_files = 0usize;

    while let Some(path) = stack.pop() {
        let metadata = fs::metadata(&path).unwrap();
        if metadata.is_dir() {
            for entry in fs::read_dir(&path).unwrap() {
                stack.push(entry.unwrap().path());
            }
            continue;
        }

        let bytes = fs::read(&path).unwrap();
        scanned_files += 1;
        assert!(
            !bytes
                .windows(sentinel.len())
                .any(|window| window == sentinel),
            "plaintext synthetic secret persisted in {}",
            path.display()
        );
    }

    assert!(
        scanned_files > 0,
        "sentinel scan must cover persisted fixtures"
    );
}

#[test]
fn sensitive_bytes_debug_and_display_are_redacted() {
    let secret = SensitiveBytes::new(SYNTHETIC_SECRET.as_bytes().to_vec());

    assert_eq!(secret.len(), SYNTHETIC_SECRET.len());
    assert!(!secret.is_empty());

    let debug = format!("{secret:?}");
    let display = format!("{secret}");

    for rendered in [&debug, &display] {
        assert!(rendered.contains("REDACTED"));
        assert!(!rendered.contains(SYNTHETIC_SECRET));
        assert!(!rendered.contains("SIGNING_SECRET"));
    }
}

#[test]
fn sensitive_bytes_storage_is_zeroizing_and_has_no_memory_secrecy_overclaim() {
    let source = include_str!("../src/envelope.rs");

    assert!(source.contains("pub struct SensitiveBytes(Zeroizing<Vec<u8>>);"));
    assert!(source.contains("Self(Zeroizing::new(bytes))"));
    assert!(source.contains("does not claim process-wide"));
    assert!(source.contains("OS memory secrecy"));
}

#[test]
fn persisted_ecr031_fixtures_exclude_plaintext_synthetic_secret() {
    let persisted_fixtures =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../contracts/ecra-identity-v1");

    assert_tree_excludes_sentinel(&persisted_fixtures, SYNTHETIC_SECRET.as_bytes());
}

#[test]
fn parser_errors_and_log_rendering_never_echo_secret_input() {
    for sentinel in SENSITIVE_SENTINELS {
        let malformed = format!(r#"{{"secret":"{sentinel}"}}"#);
        let error = ProtectedEnvelopeV1::from_json_slice(malformed.as_bytes()).unwrap_err();
        let rendered = [
            format!("{error}"),
            format!("{error:?}"),
            format!("identity operation failed: {error}"),
        ];

        for output in rendered {
            assert!(!output.contains(sentinel));
            assert!(!output.contains("SYNTHETIC_"));
        }
    }
}

#[test]
fn backend_capability_surface_carries_no_secret_material() {
    let source = include_str!("../src/backend.rs");
    let capabilities = block(
        source,
        "pub struct TrustBackendCapabilities {",
        "impl TrustBackendCapabilities {",
    );

    for forbidden in [
        "SensitiveBytes",
        "Vec<u8>",
        "[u8",
        "secret_ref",
        "secret:",
        "private_material",
    ] {
        assert!(
            !capabilities.contains(forbidden),
            "backend capability surface must not carry secret material: {forbidden}"
        );
    }
    assert!(capabilities.contains("backend_kind: TrustBackendKind"));
}

#[test]
fn persisted_envelope_metadata_excludes_all_secret_sentinels() {
    let persisted =
        include_bytes!("../../../contracts/ecra-identity-v1/expected/protected-envelope-v1.json");

    for sentinel in SENSITIVE_SENTINELS {
        assert!(
            !persisted
                .windows(sentinel.len())
                .any(|window| window == sentinel.as_bytes())
        );
    }
}
