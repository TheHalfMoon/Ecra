# ECR-031 Planning Analyze

**Pass:** 3 — implementation dependency revalidation
**Date:** 2026-08-28
**Result:** `IMPLEMENTATION_DEPENDENCY_DRIFT_REMEDIATED`
**Implementation:** Phase 3 exact closure is green; post-Phase-3 semantic work resumes only after IC-001 convergence is exact-green, starting at corrected prerequisite T043.

## 1. Inputs reviewed

- `.specify/memory/constitution.md` v1.1.0
- `AGENTS.md`
- `EXECUTION.md`
- `specs/000-ecra-platform/{roadmap,STATUS,architecture,threat-model,gap-audit,risk-register,decision-log}.md`
- ECR-001 closed identity/domain types
- ECR-002 closed durability/run semantics
- `specs/031-identity-trust-root/{STATUS,spec,research,data-model,threat-model,plan,quickstart,tasks}.md`
- `specs/031-identity-trust-root/contracts/identity-trust-v1.md`
- requirements checklist Pass 2
- primary NIST/Apple/Microsoft/Freedesktop/RFC references recorded in research

## 2. Coverage snapshot

```text
FR-001–FR-058: owned in spec/contract/plan/tasks
SC-001–SC-016: owned in spec/quickstart/tasks
Tasks T001–T082: executable candidate with exact paths
G1–G15: PASS / explicit PASS-N/A
Checklist: PASS_FOR_ANALYZE_PASS_2
PASS_1_BLOCKERS_FOUND=4
PASS_1_BLOCKERS_REMEDIATED=4
```

## 3. Pass-1 remediation verification

### C1 — Local principal bootstrap/enrollment — REMEDIATED

Primary ownership:
- `spec.md`: Ecra-local principal bootstrap/non-claim requirements and success criteria;
- `data-model.md`: `LocalPrincipalEnrollmentV1`, `EnrolledPrincipalHandle`, `IssuerSession`;
- `contracts/identity-trust-v1.md`: bootstrap/enrollment wire and incomplete-bootstrap rules;
- `threat-model.md`: T19/T20;
- `plan.md`: Workstream B;
- `quickstart.md`: bootstrap crash/non-claim target;
- `tasks.md`: T021/T035/T040/T064/T073.

Result:
- PrincipalId is generated opaque local identity, not username/email/Actor label;
- bootstrap does not claim external/legal/NIST identity proofing;
- bootstrap completes only after protected-state durable publish + authenticated reopen;
- orphan backend material cannot silently become enrollment or trigger a second mint.

### C2 — Authoritative protected trust snapshot / rollback boundary — REMEDIATED

Primary ownership:
- `data-model.md`: `ProtectedTrustStateV1` and `VerifiedTrustSnapshot`;
- contract: authoritative protected-state and verified-snapshot construction;
- plan: Workstream C and lifecycle workstream;
- threat model: T21;
- quickstart: trust-state/lifecycle/migration tests;
- tasks: T025/T031/T035–T041/T069.

Result:
- ordinary unsigned metadata is not lifecycle/revocation authority;
- validation/issuance consume only `VerifiedTrustSnapshot` produced after protected-state authentication/invariant validation;
- lifecycle mutations republish authenticated state;
- stale ordinary metadata cannot reactivate/unrevoke a key;
- v1 explicitly does not claim universal monotonic rollback resistance against restoration of older valid protected state plus equivalent authorized keystore state.

The exact ordinary projection encoding may still be chosen during implementation, but that choice cannot change the protected-state authority rule and therefore is not a security ambiguity.

### C3 — Non-ambient assertion issuance — REMEDIATED

Primary ownership:
- spec: no arbitrary-principal mint and no ECR-031 issuance service;
- data model: opaque `EnrolledPrincipalHandle` / `IssuerSession`;
- contract: issuer-session creation and subject immutability;
- plan: Workstream D issuance;
- threat model: T22;
- quickstart: issuance misuse target;
- tasks: T021/T026/T028/T031/T033/T071.

Result:
- no production `issue(arbitrary_principal_id, ...)` contract exists;
- caller cannot substitute session subject principal;
- v1 on-behalf-of issuance is bounded to the enrolled local principal;
- `IssuerSession` is identity issuance context, not CapabilityGrant/authorization;
- no network/IPC minting service is introduced.

### C4 — Frozen v1 signing custody — REMEDIATED

Primary ownership:
- research R8;
- data model signature/custody section;
- contract v1 signing custody;
- plan Workstream E;
- threat model T13/T23;
- quickstart portable-signing/backend tests;
- tasks T014/T026/T043/T055/T061/T063/T064/T067.

Result:
- canonical v1 assertion/protected-anchor algorithm is Ed25519;
- private signing material is a software key protected at rest by native backend and materialized only for bounded redacted/zeroizing signing use;
- portable path does not claim Secure Enclave/hardware-backed/non-exportable signing;
- future native non-exportable signing requires a versioned suite and separate evidence.

## 4. Requirement / task consistency

No functional requirement or success criterion is left without a concrete implementation/test/convergence task family.

Key mappings:

```text
identity/assertion FR-001–FR-014 -> T011–T034
trust/key lifecycle FR-015–FR-025 -> T011/T014/T021/T025/T026/T035–T044/T059–T068
protected storage FR-026–FR-035 -> T043–T053/T059–T074
protected anchor FR-036–FR-040 -> T018/T019/T023/T054–T058
platform/backends FR-041–FR-048 -> T002–T010/T059–T074
errors/versioning/provenance FR-049–FR-058 -> T001–T019/T031/T043/T052/T069–T082
SC-001–SC-016 -> explicit CI/fixture/closure targets across T007/T017/T019/T031–T034/T040/T049–T053/T056–T058/T064–T082
```

## 5. Constitution gates

```text
G1  PASS — ECR-001 Actor/Principal/IdentityAssertion types reused
G2  PASS — no authority output; issuance is enrolled/session-bound
G3  PASS — enrollment/issuer/root/key/digest provenance explicit
G4  PASS — bootstrap/key/state mutations have crash/atomicity tests
G5  PASS — crypto authentication != ECR-004 outcome verification
G6  PASS — authoritative protected state, incomplete bootstrap and rollback boundary explicit
G7  PASS — no plaintext fallback; bounded/redacted secret materialization
G8  PASS — local-first, no cloud identity requirement
G9  PASS — native/protocol systems remain adapters
G10 PASS_FOR_PLANNING — exact dependency/license/advisory lock is mandatory T001 before adoption
G11 PASS-N/A — no browser patch/bridge
G12 PASS — reproducible scoped security acceptance, no superiority claim
G13 PASS — no remote egress; later disclosure remains ECR-003
G14 PASS — bootstrap/principal/on-behalf-of/issuer-session semantics have explicit owner
G15 PASS — parser/state bounds and no recursive execution loops
```

No failed constitutional gate remains.

## 6. Risk review

Relevant platform risks remain owned without implicit acceptance:

```text
R-018 protocol identity/audience mapping      -> ECR-031 + ECR-016; no token passthrough here
R-036 Actor mistaken for Principal            -> C1 + validation/type boundaries
R-052 hash-chain authenticity overclaim       -> ProtectedAnchor remains distinct
R-053 sensitive state before protection       -> protected trust state/backend/no fallback
R-054 protocol confused deputy                -> future ECR-016; no protocol service in ECR-031
```

Additional discovered planning risks are now explicit in the slice threat model:
- bootstrap identity overclaim;
- incomplete bootstrap;
- unsigned lifecycle metadata rollback/unrevocation;
- ambient assertion mint;
- software-signing hardware-assurance overclaim.

## 7. Non-blocking implementation gates

These are deliberately deferred to ordered tasks, not unresolved planning ambiguity:

- T001 must re-verify current dependency versions/licenses/advisories/MSRV before any dependency adoption;
- Windows/Linux remain unsupported/unverified unless native evidence is added;
- ordinary protected-state projection encoding may be finalized during implementation so long as `ProtectedTrustStateV1` remains the authenticated authority;
- single-use assertion nonce persistence is required only if a single-use assertion class is actually enabled; the pure replay contract remains explicit either way.

If T001 or implementation evidence invalidates a frozen MUST-level assumption, implementation stops and creates an implementation clarification/convergence task rather than weakening the requirement.

## 8. Pass-2 conclusion

```text
UNOWNED_FR=0
UNOWNED_SC=0
MUST_LEVEL_PLANNING_GAPS=0
FAILED_CONSTITUTION_GATES=0
IMPLICIT_CRITICAL_RISK_ACCEPTANCE=0
PASS_1_BLOCKERS_FOUND=4
PASS_1_BLOCKERS_REMEDIATED=4
RESULT=ZERO_BLOCKING_PLANNING_DRIFT_FOUND
NEXT=CONVERGE_LIFECYCLE_TO_TASKS_READY_AND_REQUIRE_EXACT_GREEN_PLANNING_HEAD
```

## 9. Pass-3 implementation dependency revalidation

After Phase 3 closed on exact head `7eaede3f9f10461c307c8900c021273a4dbffa03`, implementation-time dependency review found one MUST-level ordering defect that Pass 2 did not expose:

```text
T035/T040 need SecureRandom                       -> T044
T037/T040 bootstrap/rotation need secret custody -> T043 + T059/T060
T041 authenticated store needs envelope          -> T045–T050
T040 crash tests need the store/bootstrap        -> T041/T041A
```

The original linear phase order would therefore require either implementing later prerequisites implicitly or weakening fail-closed security contracts. Both are prohibited.

IC-001 remediates the dependency graph while preserving stable existing task IDs and all FR/SC/C1–C4 semantics. T035 is narrowed only as a task unit to pure schema/invariant work; the displaced complete bootstrap transaction is explicitly owned by new convergence subtask T041A, so no requirement is dropped.

```text
UNOWNED_FR=0
UNOWNED_SC=0
MUST_LEVEL_REQUIREMENT_GAPS=0
MUST_LEVEL_DEPENDENCY_DEFECTS_FOUND=1
MUST_LEVEL_DEPENDENCY_DEFECTS_REMEDIATED=1
FAILED_CONSTITUTION_GATES=0
RESULT=IMPLEMENTATION_DEPENDENCY_DRIFT_REMEDIATED
NEXT=REQUIRE_EXACT_GREEN_IC001_CONVERGENCE_HEAD_THEN_T043
```
