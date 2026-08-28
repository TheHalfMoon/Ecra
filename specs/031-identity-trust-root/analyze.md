# ECR-031 Planning Analyze

**Pass:** 1  
**Date:** 2026-08-28  
**Result:** `PLANNING_REWORK_REQUIRED`  
**Implementation:** FORBIDDEN until blocking findings are remediated and this analysis is rerun.

## 1. Inputs reviewed

- `.specify/memory/constitution.md` v1.1.0
- `AGENTS.md`
- `EXECUTION.md`
- `specs/000-ecra-platform/{roadmap,STATUS,architecture,threat-model,gap-audit,risk-register,decision-log}.md`
- ECR-001 closed identity/domain types
- ECR-002 closed durability/run semantics
- `specs/031-identity-trust-root/{STATUS,spec,research,data-model,threat-model,plan,quickstart,tasks}.md`
- `specs/031-identity-trust-root/contracts/identity-trust-v1.md`
- requirements checklist
- primary NIST/Apple/Microsoft/Freedesktop/RFC references recorded in research

## 2. Coverage snapshot

```text
FR-001–FR-058: named in spec
SC-001–SC-016: named in spec
Tasks T001–T082: executable candidate
G1–G15: addressed in plan
Checklist: PASS_FOR_ANALYZE
```

The package is structurally strong, but four MUST-level planning gaps prevent implementation authorization.

## 3. Blocking findings

### A1 — BLOCKING — local principal/trust-root bootstrap is underspecified

**Conflict:** The package validates a `PrincipalId` and local trust root but does not define how the first local principal/trust root becomes enrolled or what identity claim that bootstrap is allowed to make.

**Risk:** Implementers may equate the OS username, device account, Actor label or mere possession of a newly generated key with a proofed real-world human identity. That violates G14/D-035 and overclaims NIST-style identity proofing.

**Required remediation:**
- define a local installation principal bootstrap/enrollment record;
- state that v1 establishes an **Ecra-local principal under the current protected local installation/user context**, not a legally/externally proofed human identity;
- prohibit importing OS username/email/display name as canonical `PrincipalId`;
- define trust-root creation/first-key activation atomicity and crash behavior;
- add bootstrap fixtures/tests/tasks and explicit non-claim.

### A2 — BLOCKING — authoritative lifecycle state / rollback boundary is unclear

**Conflict:** Data model says key lifecycle may be persisted, but does not freeze whether ordinary app metadata or native protected state is authoritative for current generation/revocation.

**Risk:** A filesystem attacker could restore stale ordinary metadata and make a retired/revoked key appear active if the validator trusts that metadata. This directly affects R-053 and assertion validity.

**Required remediation:**
- freeze authoritative trust snapshot ownership: security-critical current-generation/revocation state must be protected/authenticated under the trust backend/root, not trusted from ordinary unsigned metadata;
- ordinary DB/files may be rebuildable/audit projections only unless wrapped/authenticated by the protected state contract;
- validation must consume a verified `TrustSnapshot` produced only after protected-state authentication;
- explicitly state v1 rollback guarantee boundaries: no universal monotonic rollback resistance is claimed without a backend monotonic/external anchor; restoring the entire authorized OS trust store is outside the filesystem-only adversary guarantee;
- add stale-snapshot/rollback negative tests.

### A3 — BLOCKING — assertion issuance authority boundary is underspecified

**Conflict:** Validation is carefully separated from authorization, but the package has no normative rule for who may invoke assertion issuance/signing. A generic public `issue_on_behalf_of(principal, actor)` API would let any caller manufacture identity evidence even before ECR-003 exists.

**Risk:** ECR-031 could become an ambient identity mint, counterfeiting the exact authority boundary it is meant to protect.

**Required remediation:**
- ECR-031 v1 library must not expose arbitrary principal/on-behalf-of minting from caller-provided IDs;
- issuance must require an opaque `IssuerSession`/`EnrolledPrincipalHandle` obtained from the authenticated/protected local bootstrap path or an already validated parent identity context according to a frozen bounded rule;
- caller may request actor binding but cannot select another principal merely by ID;
- no IPC/network issuance service is part of ECR-031;
- future broader delegation authorization remains ECR-003.

### A4 — BLOCKING — signing key custody vs algorithm portability needs a frozen v1 path

**Conflict:** Research correctly notes Secure Enclave may not support Ed25519, while contracts use Ed25519 examples and the plan leaves software-wrapped vs native signing unresolved.

**Risk:** Implementation could either extract a key that was claimed non-exportable/hardware-backed or introduce platform-specific assertion wire algorithms without a stable acceptance baseline.

**Required remediation:** freeze a v1 signing strategy before implementation. Recommended bounded strategy:
1. canonical assertion/protected-anchor v1 portable software signing algorithm is Ed25519;
2. Ed25519 private signing key is generated from CSPRNG and stored only as an ECR-031 protected secret wrapped/protected by the native trust backend; it is zeroized after bounded use;
3. this path MUST NOT claim non-exportable or Secure Enclave-backed signing;
4. Secure Enclave/native non-exportable signing is a future algorithm-suite extension only after a versioned contract adds/accepts the native algorithm and evidence;
5. macOS v1 trust-root acceptance therefore proves Keychain protection of the wrapped signing/master secret, not universal hardware signing.

This keeps one portable v1 wire while preserving honest platform claims.

## 4. Non-blocking observations

### N1 — Windows/Linux product support is honestly bounded
The package does not claim native verification without evidence. Keep this limitation explicit through closure.

### N2 — Secret Service draft status is correctly stated
No change required; retain exact upstream draft caveat.

### N3 — Protected anchor distinction is strong
Types/contracts preserve `ProtectedAnchor != LedgerDigest != VerificationReceipt`.

### N4 — ECR-003/ECR-004 boundaries are strong
No general authorization or independent outcome verification is pulled into ECR-031.

## 5. Constitution gate effect

```text
G1  PASS
G2  FAIL_PENDING_A3
G3  PASS
G4  PASS
G5  PASS
G6  FAIL_PENDING_A1_A2
G7  PASS
G8  PASS
G9  PASS
G10 PASS_FOR_PLANNING / implementation dependency adoption still gated
G11 PASS-N/A
G12 PASS
G13 PASS
G14 FAIL_PENDING_A1_A3
G15 PASS
```

Because G2/G6/G14 have unresolved MUST-level planning gaps, the package MUST NOT be marked `TASKS_READY` yet.

## 6. Required convergence work

Append/fold these planning fixes before Pass 2:

- **C1** Local principal bootstrap/enrollment semantics + non-claim.
- **C2** Protected authoritative `TrustSnapshot` lifecycle/revocation ownership + rollback boundary.
- **C3** Non-ambient assertion issuance through `EnrolledPrincipalHandle`/`IssuerSession`; no caller-selected arbitrary principal mint.
- **C4** Freeze v1 Ed25519 signing key as native-backend-protected wrapped software secret; no Secure Enclave signing claim in v1.
- **C5** Add FR/SC/tasks/fixtures for bootstrap, stale trust snapshot and issuance misuse.
- **C6** Re-run checklist and analyze; zero failed constitution gates required.

## 7. Pass-1 conclusion

```text
UNOWNED_EXISTING_FR=0
UNOWNED_EXISTING_SC=0
MUST_LEVEL_PLANNING_GAPS=4
FAILED_CONSTITUTION_GATES=3
IMPLICIT_CRITICAL_RISK_ACCEPTANCE=0
RESULT=PLANNING_REWORK_REQUIRED
```

Do not start code or create the ECR-031 implementation branch until Pass 2 is clean.
