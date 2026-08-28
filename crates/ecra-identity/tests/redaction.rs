use ecra_identity::envelope::SensitiveBytes;

const SYNTHETIC_SECRET: &str = "ECR031_SYNTHETIC_SIGNING_SECRET_DO_NOT_LOG";

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
