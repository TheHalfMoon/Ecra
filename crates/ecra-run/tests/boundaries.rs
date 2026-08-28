use std::fs;
use std::path::Path;

fn production_sources() -> [(&'static str, &'static str); 11] {
    [
        ("archive.rs", include_str!("../src/archive.rs")),
        ("budget.rs", include_str!("../src/budget.rs")),
        ("digest.rs", include_str!("../src/digest.rs")),
        ("error.rs", include_str!("../src/error.rs")),
        ("event.rs", include_str!("../src/event.rs")),
        ("lib.rs", include_str!("../src/lib.rs")),
        ("migration.rs", include_str!("../src/migration.rs")),
        ("recovery.rs", include_str!("../src/recovery.rs")),
        ("sqlite.rs", include_str!("../src/sqlite.rs")),
        ("state.rs", include_str!("../src/state.rs")),
        ("store.rs", include_str!("../src/store.rs")),
    ]
}

fn scan_fixture_tree(path: &Path, checked: &mut usize) {
    for entry in fs::read_dir(path).expect("fixture directory") {
        let entry = entry.expect("fixture entry");
        let path = entry.path();
        if path.is_dir() {
            scan_fixture_tree(&path, checked);
            continue;
        }
        let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
            continue;
        };
        if !matches!(extension, "json" | "jcs" | "sha256" | "md") {
            continue;
        }
        let bytes = fs::read(&path).expect("fixture bytes");
        let Ok(text) = std::str::from_utf8(&bytes) else {
            continue;
        };
        *checked += 1;
        for forbidden in [
            "-----BEGIN PRIVATE KEY-----",
            "-----BEGIN OPENSSH PRIVATE KEY-----",
            "Authorization: Bearer ",
            "\"access_token\"",
            "\"refresh_token\"",
            "AKIA",
            "ghp_",
            "github_pat_",
            "sk-proj-",
        ] {
            assert!(
                !text.contains(forbidden),
                "committed ECR-002 fixture contains a credential/secret marker {forbidden:?}: {}",
                path.display()
            );
        }
    }
}

#[test]
fn archive_module_has_no_raw_sqlite_export_dependency() {
    let source = include_str!("../src/archive.rs");
    for forbidden in ["rusqlite", "Connection", "-wal", "-shm", "SQLite format 3"] {
        assert!(
            !source.contains(forbidden),
            "archive production source must remain logical-content-only: {forbidden}"
        );
    }
}

#[test]
fn deterministic_reducer_and_archive_sources_have_no_ambient_nondeterminism() {
    for (name, source) in [
        ("state.rs", include_str!("../src/state.rs")),
        ("archive.rs", include_str!("../src/archive.rs")),
    ] {
        for forbidden in [
            "std::time",
            "SystemTime",
            "Instant::now",
            "std::env",
            "env::var",
            "std::process",
            "Command::new",
            "std::net",
            "TcpStream",
            "UdpSocket",
            "rand::",
            "thread_rng",
            "OsRng",
            "getrandom",
        ] {
            assert!(
                !source.contains(forbidden),
                "deterministic production source {name} contains ambient dependency token {forbidden:?}"
            );
        }
    }
}

#[test]
fn library_has_no_network_provider_or_process_call_surface() {
    for (name, source) in production_sources() {
        for forbidden in [
            "std::net",
            "TcpStream",
            "UdpSocket",
            "std::process",
            "Command::new",
            "reqwest::",
            "hyper::",
            "ureq::",
            "tonic::",
            "tokio::net",
            "async_openai",
            "mistralrs",
            "candle_core",
            "rmcp::",
            "opentelemetry::",
            "sentry::",
        ] {
            assert!(
                !source.contains(forbidden),
                "ecra-run production source {name} contains prohibited provider/network/process token {forbidden:?}"
            );
        }
    }
}

#[test]
fn committed_ecr002_text_fixtures_have_no_secret_markers() {
    let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../contracts/ecra-run-v1");
    let mut checked = 0_usize;
    scan_fixture_tree(&fixture_root, &mut checked);
    assert!(
        checked > 0,
        "fixture audit must inspect committed text fixtures"
    );
}

#[test]
fn run_docs_keep_security_and_authority_nonclaims_explicit() {
    let readme = include_str!("../README.md");
    let threat_model = include_str!("../../../specs/002-durable-run-ledger/threat-model.md");

    for required in [
        "Actor attribution is not Principal authentication",
        "A receipt is not verification",
        "Missing receipt remains UNKNOWN",
        "Projection is not history",
        "LedgerDigest is not hostile-tamper protection",
        "A budget is not authority",
        "`.ecra` is not a secret container",
    ] {
        assert!(
            readme.contains(required),
            "README must preserve misuse warning: {required}"
        );
    }

    for required in [
        "without allowing persistence, replay, archives, budgets or attempt bookkeeping to fabricate authority, verification, identity, or side-effect certainty",
        "Real secrets/sensitive user payloads are not authorized ECR-002 acceptance assets",
        "Ecra does not claim immunity to lying hardware, broken filesystems, malicious whole-store rewrite or all forms of physical corruption",
        "ECR-002 v1 fixtures/import-export are synthetic/non-sensitive",
    ] {
        assert!(
            threat_model.contains(required) || readme.contains(required),
            "security documentation must preserve non-claim: {required}"
        );
    }

    for forbidden in [
        "tamper-proof ledger",
        "receipt proves verification",
        "budget grants authority",
        "authenticated ActorId",
        "encrypted .ecra container",
    ] {
        assert!(
            !readme.contains(forbidden) && !threat_model.contains(forbidden),
            "security documentation contains prohibited overclaim: {forbidden}"
        );
    }
}
