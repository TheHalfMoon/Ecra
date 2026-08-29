#!/usr/bin/env bash
set -euo pipefail

crate_root="crates/ecra-verify"
lib_rs="$crate_root/src/lib.rs"

if ! grep -Fqx '#![forbid(unsafe_code)]' "$lib_rs"; then
  echo "ecra-verify must retain #![forbid(unsafe_code)] at crate scope" >&2
  exit 1
fi

if grep -RInE '#!?\[(allow|expect)\(unsafe_code\)\]' "$crate_root/src" --include='*.rs'; then
  echo "ecra-verify must not weaken the unsafe_code lint" >&2
  exit 1
fi

unsafe_hits="$(
  grep -RInE '(^|[^[:alnum:]_])unsafe([^[:alnum:]_]|$)' "$crate_root/src" --include='*.rs' \
    | grep -vF '#![forbid(unsafe_code)]' \
    || true
)"

if [[ -n "$unsafe_hits" ]]; then
  echo "Ecra-owned ecra-verify production source contains an unsafe token:" >&2
  printf '%s\n' "$unsafe_hits" >&2
  exit 1
fi

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
  'sentry::' \
  'RunReducer' \
  'RunEvent::' \
  'ReceiptRecorded'
do
  if grep -RInF "$pattern" "$crate_root/src" --include='*.rs'; then
    echo "ecra-verify production source contains prohibited execution/provider/run-mutation surface: $pattern" >&2
    exit 1
  fi
done

echo "ecra-verify unsafe/read-only-run boundary: PASS"
