# Quickstart / Verification: ECR-002 Durable Run, Ledger & Budgets

**Feature:** ECR-002  
**Purpose:** exact reproducible verification surface for implementation, convergence and canonical closure

## 1. Toolchain

Use repository-pinned Rust from `rust-toolchain.toml`.

```bash
rustc --version
cargo --version
```

Expected toolchain contract: Rust 1.98.x / Edition 2024. The Phase 8 dependency evidence recorded Rust/Cargo 1.98.0.

## 2. Full workspace gate

```bash
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --workspace --locked
cargo test --workspace --locked --offline
```

## 3. ECR-001 regression boundaries

```bash
bash scripts/check-core-unsafe.sh
bash scripts/check-core-deps.sh
cargo test -p ecra-core --locked
cargo tree -p ecra-core
```

ECR-002 must not weaken the closed ECR-001 zero-I/O/dependency/unsafe contract.

## 4. ECR-002 dedicated gates

Run all dedicated targets:

```bash
cargo test -p ecra-run --test event_contract --locked
cargo test -p ecra-run --test reducer --locked
cargo test -p ecra-run --test attempts --locked
cargo test -p ecra-run --test budgets --locked
cargo test -p ecra-run --test sqlite_store --locked
cargo test -p ecra-run --test migration --locked
cargo test -p ecra-run --test crash_recovery --locked
cargo test -p ecra-run --test archive --locked
cargo test -p ecra-run --test portability --locked
cargo test -p ecra-run --test boundaries --locked
cargo test -p ecra-run --locked
bash scripts/check-run-unsafe.sh
bash scripts/check-run-deps.sh
cargo tree -p ecra-run
```

The trusted `.github/workflows/ecr-002.yml` invokes the same full workspace, regression, ECR-002 contract, rustdoc, offline, unsafe/dependency and locked-dependency evidence surface on the feature branch and `main`.

## 5. Required behavioral evidence

### Deterministic reducer

The reducer tests prove the same accepted event history reduced at least 1,000 times yields identical canonical RunState bytes/digest.

### Attempt crash matrix

```text
A crash before durable preparation
  -> no attempt exists

B crash after durable preparation, before provider call marker
  -> prepared/no receipt, unresolved after recovery boundary

C crash after simulated external effect, before receipt commit
  -> prepared/no receipt, UNKNOWN/reconciliation required, no blind retry

D crash after receipt commit
  -> exact receipt recovered; effect is not repeated
```

### Concurrent append

Two independent connections start from the same expected head; exactly one commits the next sequence. The loser receives a typed busy/head conflict and must re-read.

### Projection rebuild

Delete `run_heads`, rebuild from `run_events`, and require byte-equivalent derived state for the same history.

### SQLite configuration

Tests read back:

```text
journal_mode = wal
synchronous  = FULL-equivalent numeric value
foreign_keys = 1
trusted_schema = 0
```

Do not claim stronger physical durability than SQLite/VFS/storage assumptions support.

### Budget boundaries

Cover every v1 dimension for:

```text
0
soft-1
soft
hard-1
hard
MAX_SAFE_INTEGER
overflow attempt
```

No wraparound. Hard exhaustion creates durable suspension evidence and blocks further governed work.

### `.ecra` deterministic export

Repeated export of identical logical history/blobs must produce byte-identical bytes. Validate exact archive profile:

```text
Stored entries only
fixed 1980-01-01 timestamp
fixed 0600 file permission
no comments/encryption/symlinks/directories
manifest first
events ascending
blobs lexicographic
JCS JSON
```

### Malicious archive corpus

Reject before trusted materialization:

```text
absolute path
../ traversal
. segment
backslash
NUL
path > 512 bytes
duplicate entry
symlink/directory
non-Stored compression
encryption
>16,384 entries
>10,000 events
>6,000 blobs
manifest >8 MiB
event >4 MiB
blob >64 MiB
total uncompressed >512 MiB
unexpected/unmanifested entry
manifest/content/ledger digest mismatch
unsupported version
```

### Formatting portability

LF, CRLF and compact JSON representations of the same accepted envelope must parse to the same typed value and produce identical reducer state and deterministic archive bytes.

## 6. Migration gate

For each database schema migration fixture:

```text
open old fixture
verify exact old version
migrate transactionally
validate authoritative event bytes/meaning
rebuild projection
verify expected new schema/state
```

A forced migration failure must roll back with the old store still readable under its old supported path.

Newer unsupported schema fails closed.

## 7. Sensitive-state and egress audit

Repository acceptance fixtures contain no real:
- credentials/API tokens;
- browser cookies/session secrets;
- private documents/PHI/financial records;
- production identity assertions/approvals.

`crates/ecra-run/tests/boundaries.rs` also enforces high-confidence secret-marker checks and production source scans proving no network/telemetry/model/browser/provider/process execution call surface.

## 8. Dependency/license evidence

Current exact verified boundary:

```text
rusqlite            0.40.2, default-features=false, bundled
libsqlite3-sys      0.38.2
bundled SQLite      3.53.2
zip                 8.6.0, default-features=false
tempfile            3.27.0, dev-only
proptest            1.11.0, dev-only/workspace-aligned
Cargo.lock SHA-256  b720472bf40a554ab61afb74eae95dd625bc6b2604e47a632991faea630e42c6
```

License/provenance/native-boundary evidence is recorded in `research/donor-license-ledger.md`. No source-copying is hidden as dependency usage.

## 9. Exact-head rule

A PASS applies only to the SHA actually checked out by CI. Any later code, test, contract, workflow, task, status, or convergence mutation moves the head and requires another exact-head gate before Ready/merge.

## 10. Closure rule

ECR-002 is not `CLOSED_CANONICAL` until:

```text
all T001–T073 complete
+ final analyze/convergence has zero blocking drift
+ exact final feature head full gate PASS
+ clean review state
+ PR merged with expected head
+ canonical main ECR-002 gate PASS
+ roadmap/platform/active status/EXECUTION closure ledger converged
```
