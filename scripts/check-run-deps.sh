#!/usr/bin/env bash
set -euo pipefail

# ECR-002 direct runtime dependencies are review-owned. Changes to this list
# require the same change to update dependency/license/security evidence.
expected_direct="$({
  printf '%s\n' ecra-core
  printf '%s\n' rusqlite
  printf '%s\n' serde
  printf '%s\n' serde_jcs
  printf '%s\n' serde_json
  printf '%s\n' sha2
  printf '%s\n' thiserror
  printf '%s\n' zip
} | sort -u)"

actual_direct="$(
  cargo tree -p ecra-run --edges normal --depth 1 --prefix none \
    | awk 'NR > 1 {print $1}' \
    | sort -u
)"

if ! diff -u <(printf '%s\n' "$expected_direct") <(printf '%s\n' "$actual_direct"); then
  echo "ecra-run direct runtime dependency allowlist changed; review ECR-002 FR-055 and donor/license evidence" >&2
  exit 1
fi

# Provider/network/runtime frameworks are outside ECR-002. SQLite and ZIP are
# explicitly reviewed local I/O boundaries and therefore are not forbidden here.
forbidden='tokio|async-std|smol|reqwest|hyper|tonic|ureq|curl|surf|isahc|sqlx|diesel|sea-orm|mongodb|redis|playwright|chromiumoxide|fantoccini|headless_chrome|thirtyfour|async-openai|mistralrs|candle-core|cedar-policy|rmcp|opentelemetry|tracing-opentelemetry|sentry|duct|command-group'

tree="$(cargo tree -p ecra-run --edges normal --prefix none)"

if printf '%s\n' "$tree" | grep -Eiq "^(${forbidden})( |$)"; then
  echo "ecra-run contains a prohibited ECR-002 runtime dependency category:" >&2
  printf '%s\n' "$tree" >&2
  exit 1
fi

# Keep the native boundary explicit: bundled SQLite may introduce
# libsqlite3-sys; no second database/native execution framework is authorized.
if ! printf '%s\n' "$tree" | grep -Eq '^libsqlite3-sys( |$)'; then
  echo "expected reviewed bundled SQLite native boundary is missing" >&2
  exit 1
fi

printf '%s\n' "$tree"
