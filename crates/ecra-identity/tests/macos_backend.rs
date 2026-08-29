#[test]
fn macos_backend_contract_is_target_gated_and_fail_closed() {
    let platform = include_str!("../src/platform/mod.rs");
    let backend = include_str!("../src/platform/macos.rs");

    assert!(platform.contains("#[cfg(target_os = \"macos\")]"));
    assert!(platform.contains("mod macos"));
    assert!(backend.contains("use_protected_keychain()"));
    assert!(backend.contains("set_access_synchronized(Some(false))"));
    assert!(backend.contains("BackendInvariantViolation"));
    assert!(backend.contains("TrustRootLocked"));
    assert!(backend.contains("TrustRootUnavailable"));
    assert!(backend.contains("KeyNotFound"));
    assert!(!backend.contains("with_hardware_backed_private_operations(true)"));
    assert!(!backend.contains("with_non_exportable_private_key(true)"));
}
