#!/usr/bin/env bash
set -euo pipefail

crate_root="crates/ecra-identity"
lib_rs="$crate_root/src/lib.rs"

if ! grep -Fqx '#![forbid(unsafe_code)]' "$lib_rs"; then
  echo "ecra-identity must retain #![forbid(unsafe_code)] at crate scope" >&2
  exit 1
fi

if grep -RInE '#!?\[(allow|expect)\(unsafe_code\)\]' "$crate_root/src" --include='*.rs'; then
  echo "ecra-identity must not weaken the unsafe_code lint" >&2
  exit 1
fi

unsafe_hits="$(
  grep -RInE '(^|[^[:alnum:]_])unsafe([^[:alnum:]_]|$)' "$crate_root/src" --include='*.rs' \
    | grep -vF '#![forbid(unsafe_code)]' \
    || true
)"

if [[ -n "$unsafe_hits" ]]; then
  echo "Ecra-owned ecra-identity production source contains an unsafe token:" >&2
  printf '%s\n' "$unsafe_hits" >&2
  exit 1
fi

# ECR-031 is a local trust substrate. Network/provider/protocol/process execution
# and ambient environment-secret access are not authorized production surfaces.
for pattern in \
  'std::net' \
  'TcpStream' \
  'UdpSocket' \
  'std::process' \
  'Command::new' \
  'std::env::var' \
  'env::var' \
  'reqwest' \
  'hyper::' \
  'ureq::' \
  'curl::' \
  'opentelemetry' \
  'sentry::' \
  'rmcp::' \
  'cedar_policy'
do
  if grep -RInF "$pattern" "$crate_root/src" --include='*.rs'; then
    echo "ecra-identity production source contains prohibited execution/ambient surface: $pattern" >&2
    exit 1
  fi
done

# Reviewed dependency-owned low-level/native boundaries are intentionally not
# described as unsafe-free: security-framework/security-framework-sys wrap
# Apple's Security.framework/CoreFoundation C APIs, and getrandom reaches OS
# entropy APIs. Those boundaries remain outside Ecra-authored Rust and are
# separately constrained by check-identity-deps.sh and the donor/license ledger.
echo "ecra-identity authored-unsafe/local-trust boundary: PASS"
