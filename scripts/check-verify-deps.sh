#!/usr/bin/env bash
set -euo pipefail

expected_direct="$({
  printf '%s\n' ecra-core
  printf '%s\n' ecra-run
  printf '%s\n' rusqlite
  printf '%s\n' serde
  printf '%s\n' serde_jcs
  printf '%s\n' serde_json
  printf '%s\n' sha2
  printf '%s\n' thiserror
  printf '%s\n' uuid
} | sort -u)"

actual_direct="$(
  cargo tree -p ecra-verify --edges normal --depth 1 --prefix none \
    | awk 'NR > 1 {print $1}' \
    | sort -u
)"

if ! diff -u <(printf '%s\n' "$expected_direct") <(printf '%s\n' "$actual_direct"); then
  echo "ecra-verify direct runtime dependency allowlist changed; review ECR-004 T001/FR-041 and donor/license evidence" >&2
  exit 1
fi

forbidden='tokio|async-std|smol|reqwest|hyper|tonic|ureq|curl|surf|isahc|sqlx|diesel|sea-orm|mongodb|redis|playwright|chromiumoxide|fantoccini|headless_chrome|thirtyfour|async-openai|mistralrs|candle-core|cedar-policy|rmcp|opentelemetry|tracing-opentelemetry|sentry|duct|command-group'

tree="$(cargo tree -p ecra-verify --edges normal --prefix none)"

if printf '%s\n' "$tree" | grep -Eiq "^(${forbidden})( |$)"; then
  echo "ecra-verify contains a prohibited ECR-004 runtime dependency category:" >&2
  printf '%s\n' "$tree" >&2
  exit 1
fi

# ECR-004 does not own ZIP/URL behavior directly even though those packages may
# remain transitive through the closed ecra-run/ecra-core dependencies.
if grep -Eq '^zip[[:space:]]*=' crates/ecra-verify/Cargo.toml; then
  echo "ecra-verify must not add zip as a direct dependency" >&2
  exit 1
fi
if grep -Eq '^url[[:space:]]*=' crates/ecra-verify/Cargo.toml; then
  echo "ecra-verify must not add url as a direct dependency" >&2
  exit 1
fi

# The only reviewed native boundary is inherited bundled SQLite.
if ! printf '%s\n' "$tree" | grep -Eq '^libsqlite3-sys( |$)'; then
  echo "expected reviewed bundled SQLite native boundary is missing" >&2
  exit 1
fi

printf '%s\n' "$tree"
