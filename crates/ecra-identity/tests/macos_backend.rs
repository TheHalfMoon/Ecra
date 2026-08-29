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

#[test]
fn t064_live_acceptance_requires_signed_provisioned_app_like_host() {
    let harness = include_str!("../../../scripts/run-ecr031-macos-live-acceptance.sh");
    let workflow = include_str!("../../../.github/workflows/ecr-031.yml");
    let readiness = include_str!("../../../.github/workflows/ecr-031-macos-host-readiness.yml");

    assert!(harness.contains("dev.ecra.identity.t064"));
    assert!(harness.contains("EcraT064Host.app"));
    assert!(harness.contains("Contents/embedded.provisionprofile"));
    assert!(harness.contains("com.apple.application-identifier"));
    assert!(harness.contains("com.apple.developer.team-identifier"));
    assert!(harness.contains("keychain-access-groups"));
    assert!(harness.contains("Apple Development:"));
    assert!(harness.contains("codesign --force --sign"));
    assert!(harness.contains("codesign --verify --strict"));
    assert!(harness.contains("--message-format=json-render-diagnostics"));
    assert!(harness.contains(
        "platform::macos::tests::data_protection_keychain_roundtrips_all_v1_secret_purposes"
    ));
    assert!(harness.contains(
        "platform::macos::tests::native_keychain_bootstrap_publishes_and_reopens_same_identity"
    ));
    assert!(!harness.contains("codesign --force --sign -"));
    assert!(!harness.contains("set_access_synchronized(Some(true))"));

    assert!(
        workflow.contains("run: bash scripts/run-ecr031-macos-live-acceptance.sh --readiness-only")
    );
    assert!(workflow.contains("run: bash scripts/run-ecr031-macos-live-acceptance.sh"));
    assert!(!workflow.contains(
        "platform::macos::tests::data_protection_keychain_roundtrips_all_v1_secret_purposes"
    ));
    assert!(!workflow.contains(
        "platform::macos::tests::native_keychain_bootstrap_publishes_and_reopens_same_identity"
    ));

    assert!(
        readiness
            .contains("run: bash scripts/run-ecr031-macos-live-acceptance.sh --readiness-only")
    );
    assert!(!readiness.contains("continue-on-error: true\n        shell: bash\n        run: |\n          set -euo pipefail\n          security find-identity"));
}
