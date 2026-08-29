#!/usr/bin/env bash
set -euo pipefail

readonly BUNDLE_ID="dev.ecra.identity.t064"
readonly EXECUTABLE_NAME="EcraT064Host"
readonly PROFILE_DIR_XCODE="$HOME/Library/Developer/Xcode/UserData/Provisioning Profiles"
readonly PROFILE_DIR_MOBILEDEVICE="$HOME/Library/MobileDevice/Provisioning Profiles"

fail() {
  printf 'ECR-031 T064 readiness failure: %s\n' "$1" >&2
  exit 1
}

require_macos_host() {
  [[ "$(uname -s)" == "Darwin" ]] || fail "trusted macOS host required"

  local console_user runner_user
  console_user="$(stat -f '%Su' /dev/console 2>/dev/null || printf 'unknown')"
  runner_user="$(id -un 2>/dev/null || printf 'unknown')"

  [[ "$console_user" != "root" ]] || fail "interactive console user required"
  [[ "$console_user" != "loginwindow" ]] || fail "interactive console user required"
  [[ "$console_user" != "unknown" ]] || fail "interactive console user required"
  [[ "$runner_user" == "$console_user" ]] || fail "runner user must own the interactive console session"

  command -v security >/dev/null 2>&1 || fail "security tool unavailable"
  command -v codesign >/dev/null 2>&1 || fail "codesign tool unavailable"
  command -v xcodebuild >/dev/null 2>&1 || fail "xcodebuild tool unavailable"
  command -v python3 >/dev/null 2>&1 || fail "python3 unavailable"
}

profile_authorization() {
  local profile_path="$1"
  local decoded_path="$2"

  security cms -D -i "$profile_path" >"$decoded_path" 2>/dev/null || return 1

  python3 - "$decoded_path" "$BUNDLE_ID" <<'PY'
from __future__ import annotations

from datetime import datetime, timezone
from pathlib import Path
import plistlib
import sys

path = Path(sys.argv[1])
bundle_id = sys.argv[2]

try:
    with path.open("rb") as handle:
        profile = plistlib.load(handle)
except Exception:
    raise SystemExit(1)

platforms = profile.get("Platform") or []
if "OSX" not in platforms and "macOS" not in platforms:
    raise SystemExit(1)

expires = profile.get("ExpirationDate")
if not isinstance(expires, datetime):
    raise SystemExit(1)
if expires.tzinfo is None:
    expires = expires.replace(tzinfo=timezone.utc)
if expires <= datetime.now(timezone.utc):
    raise SystemExit(1)

entitlements = profile.get("Entitlements")
if not isinstance(entitlements, dict):
    raise SystemExit(1)

prefixes = profile.get("ApplicationIdentifierPrefix") or []
teams = profile.get("TeamIdentifier") or []
if not prefixes or not teams:
    raise SystemExit(1)
prefix = prefixes[0]
team = teams[0]
if not isinstance(prefix, str) or not isinstance(team, str):
    raise SystemExit(1)
if not prefix or not team or "\n" in prefix or "\n" in team:
    raise SystemExit(1)

profile_team = entitlements.get("com.apple.developer.team-identifier")
if profile_team is not None and profile_team != team:
    raise SystemExit(1)

separator = "" if prefix.endswith(".") else "."
application_identifier = f"{prefix}{separator}{bundle_id}"

profile_application_identifier = (
    entitlements.get("com.apple.application-identifier")
    or entitlements.get("application-identifier")
)
if not isinstance(profile_application_identifier, str):
    raise SystemExit(1)

def authorizes(pattern: str, value: str) -> bool:
    if pattern == value:
        return True
    if pattern.endswith("*"):
        return value.startswith(pattern[:-1])
    return False

if not authorizes(profile_application_identifier, application_identifier):
    raise SystemExit(1)

keychain_groups = entitlements.get("keychain-access-groups")
if not isinstance(keychain_groups, list) or not keychain_groups:
    raise SystemExit(1)
if not any(
    isinstance(group, str) and authorizes(group, application_identifier)
    for group in keychain_groups
):
    raise SystemExit(1)

print(team)
print(application_identifier)
PY
}

find_profile_and_identity() {
  local scratch_dir="$1"
  local selected_profile=""
  local selected_team=""
  local selected_application_identifier=""
  local decoded="$scratch_dir/profile.plist"

  while IFS= read -r -d '' profile; do
    local authorization
    if authorization="$(profile_authorization "$profile" "$decoded")"; then
      selected_team="$(printf '%s\n' "$authorization" | sed -n '1p')"
      selected_application_identifier="$(printf '%s\n' "$authorization" | sed -n '2p')"
      if [[ -n "$selected_team" && -n "$selected_application_identifier" ]]; then
        selected_profile="$profile"
        break
      fi
    fi
  done < <(
    for directory in "$PROFILE_DIR_XCODE" "$PROFILE_DIR_MOBILEDEVICE"; do
      if [[ -d "$directory" ]]; then
        find "$directory" -maxdepth 1 -type f \
          \( -name '*.mobileprovision' -o -name '*.provisionprofile' \) -print0
      fi
    done
  )

  [[ -n "$selected_profile" ]] || fail "no unexpired macOS provisioning profile authorizes ${BUNDLE_ID} and its keychain access group"

  local identity_output identity_hash
  identity_output="$(security find-identity -v -p codesigning 2>/dev/null || true)"
  identity_hash="$(printf '%s\n' "$identity_output" \
    | awk -v team="$selected_team" '
        $0 ~ /^[[:space:]]*[0-9]+\)/ &&
        $0 ~ /Apple Development:/ &&
        index($0, "(" team ")") {
          gsub(/^[[:space:]]*[0-9]+\)[[:space:]]*/, "", $0)
          print $1
          exit
        }
      ')"

  [[ "$identity_hash" =~ ^[0-9A-Fa-f]{40}$ ]] || fail "no Apple Development signing identity matches the provisioning-profile team"

  printf '%s\n' "$selected_profile"
  printf '%s\n' "$selected_team"
  printf '%s\n' "$selected_application_identifier"
  printf '%s\n' "$identity_hash"
}

write_plists() {
  local info_path="$1"
  local entitlements_path="$2"
  local team="$3"
  local application_identifier="$4"

  python3 - "$info_path" "$entitlements_path" "$team" "$application_identifier" "$BUNDLE_ID" "$EXECUTABLE_NAME" <<'PY'
from pathlib import Path
import plistlib
import sys

info_path = Path(sys.argv[1])
entitlements_path = Path(sys.argv[2])
team = sys.argv[3]
application_identifier = sys.argv[4]
bundle_id = sys.argv[5]
executable_name = sys.argv[6]

info = {
    "CFBundleDevelopmentRegion": "en",
    "CFBundleExecutable": executable_name,
    "CFBundleIdentifier": bundle_id,
    "CFBundleInfoDictionaryVersion": "6.0",
    "CFBundleName": "Ecra T064 Host",
    "CFBundlePackageType": "APPL",
    "CFBundleShortVersionString": "1.0",
    "CFBundleVersion": "1",
}

entitlements = {
    "com.apple.application-identifier": application_identifier,
    "com.apple.developer.team-identifier": team,
    "keychain-access-groups": [application_identifier],
}

with info_path.open("wb") as handle:
    plistlib.dump(info, handle, fmt=plistlib.FMT_XML, sort_keys=True)
with entitlements_path.open("wb") as handle:
    plistlib.dump(entitlements, handle, fmt=plistlib.FMT_XML, sort_keys=True)
PY
}

build_test_executable() {
  local scratch_dir="$1"
  local artifacts="$scratch_dir/cargo-artifacts.json"

  cargo test -p ecra-identity --lib --locked --no-run \
    --message-format=json-render-diagnostics >"$artifacts"

  python3 - "$artifacts" <<'PY'
from pathlib import Path
import json
import sys

artifacts = Path(sys.argv[1])
candidates: list[str] = []
for line in artifacts.read_text(encoding="utf-8").splitlines():
    try:
        message = json.loads(line)
    except json.JSONDecodeError:
        continue
    if message.get("reason") != "compiler-artifact":
        continue
    target = message.get("target") or {}
    profile = message.get("profile") or {}
    executable = message.get("executable")
    if (
        target.get("name") == "ecra_identity"
        and "lib" in (target.get("kind") or [])
        and profile.get("test") is True
        and isinstance(executable, str)
    ):
        candidates.append(executable)

if len(candidates) != 1:
    raise SystemExit(f"expected exactly one ecra-identity lib test executable, found {len(candidates)}")
print(candidates[0])
PY
}

main() {
  require_macos_host

  local scratch_dir
  scratch_dir="$(mktemp -d "${TMPDIR:-/tmp}/ecra-t064.XXXXXX")"
  trap '[[ -z "${scratch_dir:-}" ]] || rm -rf -- "$scratch_dir"' EXIT

  local selection profile team application_identifier identity_hash
  selection="$(find_profile_and_identity "$scratch_dir")"
  profile="$(printf '%s\n' "$selection" | sed -n '1p')"
  team="$(printf '%s\n' "$selection" | sed -n '2p')"
  application_identifier="$(printf '%s\n' "$selection" | sed -n '3p')"
  identity_hash="$(printf '%s\n' "$selection" | sed -n '4p')"

  printf 't064_host_readiness=ready\n'
  printf 't064_bundle_id=%s\n' "$BUNDLE_ID"
  printf 't064_profile_authorization=matched\n'
  printf 't064_apple_development_identity=matched\n'

  if [[ "${1:-}" == "--readiness-only" ]]; then
    return 0
  fi
  [[ $# -eq 0 ]] || fail "unsupported argument"

  local test_executable
  test_executable="$(build_test_executable "$scratch_dir")"
  [[ -x "$test_executable" ]] || fail "compiled ecra-identity test executable unavailable"

  local app_dir contents_dir macos_dir info_path entitlements_path wrapped_executable
  app_dir="$scratch_dir/EcraT064Host.app"
  contents_dir="$app_dir/Contents"
  macos_dir="$contents_dir/MacOS"
  info_path="$contents_dir/Info.plist"
  entitlements_path="$scratch_dir/Entitlements.plist"
  wrapped_executable="$macos_dir/$EXECUTABLE_NAME"

  mkdir -p "$macos_dir"
  cp "$test_executable" "$wrapped_executable"
  chmod 0755 "$wrapped_executable"
  cp "$profile" "$contents_dir/embedded.provisionprofile"
  write_plists "$info_path" "$entitlements_path" "$team" "$application_identifier"

  codesign --force --sign "$identity_hash" --entitlements "$entitlements_path" \
    --timestamp=none "$app_dir" >/dev/null
  codesign --verify --strict --verbose=2 "$app_dir"

  local signed_entitlements="$scratch_dir/SignedEntitlements.plist"
  codesign --display --entitlements - --xml "$app_dir" >"$signed_entitlements" 2>/dev/null \
    || fail "unable to inspect signed entitlements"

  python3 - "$signed_entitlements" "$team" "$application_identifier" <<'PY'
from pathlib import Path
import plistlib
import sys

path = Path(sys.argv[1])
team = sys.argv[2]
application_identifier = sys.argv[3]
try:
    with path.open("rb") as handle:
        entitlements = plistlib.load(handle)
except Exception:
    raise SystemExit("signed entitlement payload is unreadable")

if entitlements.get("com.apple.application-identifier") != application_identifier:
    raise SystemExit("signed application identifier mismatch")
if entitlements.get("com.apple.developer.team-identifier") != team:
    raise SystemExit("signed team identifier mismatch")
if entitlements.get("keychain-access-groups") != [application_identifier]:
    raise SystemExit("signed keychain access group mismatch")
PY

  printf 't064_signed_app_like_host=verified\n'

  "$wrapped_executable" \
    platform::macos::tests::data_protection_keychain_roundtrips_all_v1_secret_purposes \
    --exact --ignored
  "$wrapped_executable" \
    platform::macos::tests::native_keychain_bootstrap_publishes_and_reopens_same_identity \
    --exact --ignored

  printf 't064_data_protection_keychain_acceptance=passed\n'
}

main "$@"
