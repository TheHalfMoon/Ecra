#!/usr/bin/env bash
set -euo pipefail

# ECR-031 direct runtime dependencies are review-owned. Any change to this
# allowlist requires a matching research/donor-license/security disposition.
expected_direct="$({
  printf '%s\n' chacha20poly1305
  printf '%s\n' ecra-core
  printf '%s\n' ed25519-dalek
  printf '%s\n' getrandom
  printf '%s\n' hkdf
  printf '%s\n' serde
  printf '%s\n' serde_json
  printf '%s\n' sha2
  printf '%s\n' uuid
  printf '%s\n' zeroize

  host="$(rustc -vV | sed -n 's/^host: //p')"
  if [[ "$host" == *-apple-darwin ]]; then
    printf '%s\n' security-framework
  fi
} | sort -u)"

actual_direct="$(
  cargo tree -p ecra-identity --edges normal --depth 1 --prefix none \
    | awk 'NR > 1 {print $1}' \
    | sort -u
)"

if ! diff -u <(printf '%s\n' "$expected_direct") <(printf '%s\n' "$actual_direct"); then
  echo "ecra-identity direct runtime dependency allowlist changed; update ECR-031 T001/T010 evidence first" >&2
  exit 1
fi

tree="$(cargo tree -p ecra-identity --edges normal --prefix none)"

# These categories are owned by later slices or are broader than the frozen v1
# crypto/custody contract. Match package names at the start of cargo-tree lines.
forbidden='tokio|async-std|smol|reqwest|hyper|tonic|ureq|curl|surf|isahc|sqlx|diesel|sea-orm|mongodb|redis|playwright|chromiumoxide|fantoccini|headless_chrome|thirtyfour|async-openai|mistralrs|candle-core|cedar-policy|rmcp|opentelemetry|tracing-opentelemetry|sentry|duct|command-group|ring|openssl|sodiumoxide|rand|rand_chacha|wasm-bindgen|js-sys|secret-service|keyring|oo7'

if printf '%s\n' "$tree" | grep -Eiq "^(${forbidden})( |$)"; then
  echo "ecra-identity contains a prohibited ECR-031 runtime dependency category:" >&2
  printf '%s\n' "$tree" >&2
  exit 1
fi

host="$(rustc -vV | sed -n 's/^host: //p')"
if [[ "$host" == *-apple-darwin ]]; then
  if ! printf '%s\n' "$tree" | grep -Eq '^security-framework( |$)'; then
    echo "expected reviewed macOS security-framework boundary is missing" >&2
    exit 1
  fi
  if ! printf '%s\n' "$tree" | grep -Eq '^security-framework-sys( |$)'; then
    echo "expected reviewed macOS security-framework-sys native boundary is missing" >&2
    exit 1
  fi
else
  if printf '%s\n' "$tree" | grep -Eq '^security-framework(-sys)?( |$)'; then
    echo "macOS Security.framework dependency must not be active on non-macOS targets" >&2
    exit 1
  fi
fi

# Backend-specific Windows/Linux trust-store dependencies remain forbidden in
# Phase 1. Target-specific system crates pulled solely by getrandom are not a
# TrustBackend claim and are reviewed separately in the ledger.
if printf '%s\n' "$tree" | grep -Eiq '^(secret-service|keyring|oo7)( |$)'; then
  echo "Linux secret-store dependency appeared before a verified Linux backend exists" >&2
  exit 1
fi

printf '%s\n' "$tree"
