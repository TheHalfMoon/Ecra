use std::{fs, path::Path};

use ecra_identity::envelope::SensitiveBytes;

const SYNTHETIC_SECRET: &str = "ECR031_SYNTHETIC_SIGNING_SECRET_DO_NOT_LOG";

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
