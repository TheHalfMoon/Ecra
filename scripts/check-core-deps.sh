#!/usr/bin/env bash
set -euo pipefail

forbidden='tokio|async-std|reqwest|hyper|sqlx|rusqlite|diesel|sea-orm|playwright|chromiumoxide|fantoccini|cedar-policy|rmcp|opentelemetry|tracing-opentelemetry'

tree="$(cargo tree -p ecra-core --edges normal --prefix none)"

if printf '%s\n' "$tree" | grep -Eiq "^(${forbidden})( |$)"; then
  echo "ecra-core contains a prohibited runtime dependency category:" >&2
  printf '%s\n' "$tree" >&2
  exit 1
fi

printf '%s\n' "$tree"
