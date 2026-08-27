#!/usr/bin/env bash
set -euo pipefail

# ECR-001 runtime dependencies are deliberately tiny and review-owned. Any new
# direct production dependency must update this allowlist in the same reviewed
# change that records its license/security rationale.
expected_direct="$({
  printf '%s\n' serde
  printf '%s\n' serde_json
  printf '%s\n' serde_jcs
  printf '%s\n' sha2
  printf '%s\n' thiserror
  printf '%s\n' url
  printf '%s\n' uuid
} | sort -u)"

actual_direct="$(
  cargo tree -p ecra-core --edges normal --depth 1 --prefix none \
    | awk 'NR > 1 {print $1}' \
    | sort -u
)"

if ! diff -u <(printf '%s\n' "$expected_direct") <(printf '%s\n' "$actual_direct"); then
  echo "ecra-core direct runtime dependency allowlist changed; review FR-050 and donor/license evidence" >&2
  exit 1
fi

# Defense in depth: reject known crates from prohibited runtime categories even
# if they arrive transitively. The direct allowlist above remains the primary
# fail-closed boundary for novel dependencies.
forbidden='tokio|async-std|smol|reqwest|hyper|tonic|ureq|curl|surf|isahc|sqlx|rusqlite|diesel|sea-orm|mongodb|redis|playwright|chromiumoxide|fantoccini|headless_chrome|thirtyfour|async-openai|mistralrs|candle-core|cedar-policy|rmcp|a2a|duct|command-group|opentelemetry|tracing-opentelemetry|sentry'

tree="$(cargo tree -p ecra-core --edges normal --prefix none)"

if printf '%s\n' "$tree" | grep -Eiq "^(${forbidden})( |$)"; then
  echo "ecra-core contains a prohibited FR-050 runtime dependency category:" >&2
  printf '%s\n' "$tree" >&2
  exit 1
fi

printf '%s\n' "$tree"
