#[cfg(target_os = "macos")]
#[test]
fn macos_backend_is_explicitly_data_protection_and_local_only() {
    let source = include_str!("../src/platform/macos.rs");
    assert!(source.contains("use_protected_keychain()"));
    assert!(source.contains("set_access_synchronized(Some(false))"));
    assert!(source.contains("MacosDataProtectionKeychain"));
    assert!(source.contains("hardware_backed_private_operations"));
    assert!(source.contains("non_exportable_private_key"));
    assert!(!source.contains("SecureEnclave"));
    assert!(!source.contains("iCloud"));
}

#[cfg(not(target_os = "macos"))]
#[test]
fn macos_backend_source_remains_target_gated() {
    let platform = include_str!("../src/platform/mod.rs");
    assert!(platform.contains("#[cfg(target_os = \"macos\")]"));
    assert!(platform.contains("mod macos"));
}
