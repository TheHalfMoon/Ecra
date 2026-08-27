#!/usr/bin/env bash
set -euo pipefail

crate_root="crates/ecra-run"
lib_rs="$crate_root/src/lib.rs"

if ! grep -Fqx '#![forbid(unsafe_code)]' "$lib_rs"; then
  echo "ecra-run must retain #![forbid(unsafe_code)] at crate scope" >&2
  exit 1
fi

if grep -RInE '#!?\[(allow|expect)\(unsafe_code\)\]' "$crate_root/src" --include='*.rs'; then
  echo "ecra-run must not weaken the unsafe_code lint" >&2
  exit 1
fi

unsafe_hits="$(
  grep -RInE '(^|[^[:alnum:]_])unsafe([^[:alnum:]_]|$)' "$crate_root/src" --include='*.rs' \
    | grep -vF '#![forbid(unsafe_code)]' \
    || true
)"

if [[ -n "$unsafe_hits" ]]; then
  echo "Ecra-owned ecra-run production source contains an unsafe token:" >&2
  printf '%s\n' "$unsafe_hits" >&2
  exit 1
fi

# ECR-002 library persistence/archive code is local-only. Provider/network,
# telemetry and process execution are not authorized production surfaces.
for pattern in \
  'std::net' \
  'TcpStream' \
  'UdpSocket' \
  'std::process' \
  'Command::new' \
  'reqwest' \
  'hyper::' \
  'ureq::' \
  'opentelemetry' \
  'sentry::'
do
  if grep -RInF "$pattern" "$crate_root/src" --include='*.rs'; then
    echo "ecra-run production source contains prohibited network/provider/process surface: $pattern" >&2
    exit 1
  fi
done

echo "ecra-run unsafe/local-I/O boundary: PASS"
