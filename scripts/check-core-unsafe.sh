#!/usr/bin/env bash
set -euo pipefail

crate_root="crates/ecra-core"
lib_rs="$crate_root/src/lib.rs"

if ! grep -Fqx '#![forbid(unsafe_code)]' "$lib_rs"; then
  echo "ecra-core must retain #![forbid(unsafe_code)] at crate scope" >&2
  exit 1
fi

if grep -RInE '#!?\[(allow|expect)\(unsafe_code\)\]' "$crate_root/src" --include='*.rs'; then
  echo "ecra-core must not weaken the unsafe_code lint" >&2
  exit 1
fi

unsafe_hits="$(
  grep -RInE '(^|[^[:alnum:]_])unsafe([^[:alnum:]_]|$)' "$crate_root/src" --include='*.rs' \
    | grep -vF '#![forbid(unsafe_code)]' \
    || true
)"

if [[ -n "$unsafe_hits" ]]; then
  echo "ecra-core contains an unsafe token; ECR-001 authorizes no exception:" >&2
  printf '%s\n' "$unsafe_hits" >&2
  exit 1
fi

echo "ecra-core unsafe boundary: PASS"
