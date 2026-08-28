from pathlib import Path

# Create implementation clarification.
clarification = """# ECR-031 Implementation Clarifications

## IC-001 — Phase 4 dependency-order correction

**Date:** 2026-08-28  
**Discovered at:** exact green Phase 3 closure head `7eaede3f9f10461c307c8900c021273a4dbffa03`  
**Classification:** MUST-level implementation dependency defect  
**Status:** REMEDIATED_IN_PLAN

### Finding

The original task graph placed T035–T042 before T043–T053 and T059–T068. Live implementation review showed that this order is not executable without weakening frozen requirements:

- local bootstrap requires the approved production randomness boundary owned by T044;
- protected secret handling required by bootstrap/rotation/backend operations requires T043;
- the authenticated trust-state store required by T041 depends on `ProtectedEnvelopeV1`, HKDF-SHA-256, ChaCha20-Poly1305 and authenticated open owned by T045–T050;
- rotation/bootstrap protected-secret operations require the Rust-owned `TrustBackend` and production/test selection boundary owned by T059–T060;
- crash/bootstrap tests in T040 depend on the protected store and bootstrap transaction, so they cannot precede those implementations;
- the original T035 combined pure schema work with generated/durable bootstrap completion, creating a cycle with T041.

Implementing placeholders, plaintext fallbacks, unsigned trust state, caller-supplied “generated” IDs, or a fake production backend would violate FR-021–FR-025 and C1/C2. Those shortcuts are prohibited.

### Remediation

Requirements, security claims and task ownership are not weakened. Task IDs already referenced by the planning package remain stable.

Execution dependencies are corrected as follows:

```text
Phase 3 exact closure gate
        ↓
T043 SensitiveBytes
        ↓
T044 SecureRandom
        ↓
T045 → T046 → T047 → T048 → T049 → T050
        ↓
T059 → T060
        ↓
T035 pure trust/enrollment/key schemas and invariants
        ↓
T036 unambiguous active-key selection
        ↓
T041 authenticated protected trust-state store
        ↓
T041A complete local bootstrap/enrollment transaction
        ↓
T038 retirement semantics
        ↓
T037 rotation using the protected store
        ↓
T039 revocation semantics
        ↓
T040 exhaustive bootstrap/lifecycle/crash tests
        ↓
T042 Phase 4 exact-head gate
```

T041A is a convergence subtask added because the original T035 mixed schema definition with a durable transaction that necessarily depends on T041. T041A owns the complete generated-ID bootstrap transaction: generate fresh opaque identifiers via `SecureRandom`, create/protect required backend secrets, publish authenticated protected state atomically, reopen/authenticate, then and only then return `EnrolledPrincipalHandle`. Partial initialization remains `incomplete_bootstrap` and never silently mints a second identity.

After T042, unfinished original phase families resume without renumbering: T051–T053, T054–T058, T061–T068, then T069 onward.

### Security effect

This correction strengthens implementation ordering. It does not broaden ECR-031 scope and does not change:

- Ecra-local identity non-claims;
- protected trust state as the sole lifecycle authority;
- no plaintext/in-memory/environment/file-key production fallback;
- no arbitrary caller-selected principal issuance;
- Ed25519 portable signing custody claims;
- rollback non-claim;
- ECR-003/ECR-004/ECR-016 and other slice boundaries.

### Gate

No semantic post-Phase-3 code may begin until this clarification plus synchronized `tasks.md`, `plan.md`, `STATUS.md`, `EXECUTION.md` and `analyze.md` is committed and the permanent ECR-031 workflow succeeds on that exact convergence head.
"""
Path("specs/031-identity-trust-root/implementation-clarifications.md").write_text(clarification)

tasks_path = Path("specs/031-identity-trust-root/tasks.md")
tasks = tasks_path.read_text()
old_phase4 = """## Phase 4 — Local bootstrap, protected trust state and key lifecycle

- [ ] **T035** Implement strict `TrustRootRecord`, `KeyRecord`, `ProtectedTrustStateV1` and local enrollment invariants with no serialized private/root/symmetric key material; local principal IDs are generated, never derived from OS/user labels. **Paths:** `crates/ecra-identity/src/bootstrap.rs`, `crates/ecra-identity/src/key.rs`. **FR-015–FR-018, FR-021, FR-024**
- [ ] **T036** Implement one-active-key-per-trust-root/purpose selection inside authenticated protected trust state and reject ambiguous/duplicate active generations. **Path:** `crates/ecra-identity/src/key.rs`. **FR-018, FR-021**
- [ ] **T037** Implement rotate transition: create/protect next generation, activate it and atomically publish new protected trust state; prior active becomes retired according to purpose. **Paths:** `crates/ecra-identity/src/key.rs`, `crates/ecra-identity/src/store.rs`. **FR-019, FR-021**
- [ ] **T038** Implement retirement semantics blocking new signing/protection while permitting only contract-authorized historical verification/decryption. **Path:** `crates/ecra-identity/src/key.rs`. **FR-017–FR-020**
- [ ] **T039** Implement revocation semantics in protected trust state blocking new use/current assertion validation; distinguish revocation from unavailable/destroyed key and reject ordinary metadata attempts to unrevoke/reactivate. **Paths:** `crates/ecra-identity/src/key.rs`, `crates/ecra-identity/src/validation.rs`, `crates/ecra-identity/src/store.rs`. **FR-020, FR-021**
- [ ] **T040** Add exhaustive lifecycle/bootstrap tests including first enrollment, crash before/after backend secret creation and before/after protected-state publish, incomplete-bootstrap recovery, invalid second bootstrap, generation collision, stale-key issuance, revoked-key validation, stale/unsigned metadata and explicit no-monotonic-rollback-overclaim fixture. **Paths:** `crates/ecra-identity/tests/bootstrap.rs`, `crates/ecra-identity/tests/key_lifecycle.rs`. **SC-004, SC-008**
- [ ] **T041** Implement the ECR-031-owned versioned protected trust-state store using authenticated envelope + crash-safe atomic replacement; ordinary metadata is rebuildable/non-authoritative. Add migration/corruption fixtures; do not reuse ECR-002 tables as identity authority. **Paths:** `crates/ecra-identity/src/store.rs`, `contracts/ecra-identity-v1/migrations/`, `crates/ecra-identity/tests/migration.rs`. **FR-021, FR-054**
- [ ] **T042** Exact-head Phase 4 CI and status ledger update. **Path:** `specs/031-identity-trust-root/STATUS.md`. **SC-004, SC-008, SC-013**
"""
new_phase4 = """## Phase 4 — Local bootstrap, protected trust state and key lifecycle

> **Implementation dependency correction IC-001:** before T035, execute the already-owned prerequisite tasks T043 → T044 → T045 → T046 → T047 → T048 → T049 → T050 → T059 → T060. These stable task IDs remain defined in their original phase sections below; executing them early is the canonical dependency order, not a scope bypass. See `implementation-clarifications.md`.

- [ ] **T035** Implement strict `TrustRootRecord`, `KeyRecord`, `ProtectedTrustStateV1` and pure local enrollment/trust-state schema invariants with no serialized private/root/symmetric key material. IDs remain opaque and cannot be derived from OS/user labels; the complete generated/durable bootstrap transaction is owned by T041A after the protected store exists. **Paths:** `crates/ecra-identity/src/bootstrap.rs`, `crates/ecra-identity/src/key.rs`. **FR-015–FR-018, FR-021, FR-024**
- [ ] **T036** Implement one-active-key-per-trust-root/purpose selection inside authenticated protected trust state and reject ambiguous/duplicate active generations. **Path:** `crates/ecra-identity/src/key.rs`. **FR-018, FR-021**
- [ ] **T041** Implement the ECR-031-owned versioned protected trust-state store using authenticated envelope + crash-safe atomic replacement; ordinary metadata is rebuildable/non-authoritative. Add migration/corruption fixtures; do not reuse ECR-002 tables as identity authority. **Paths:** `crates/ecra-identity/src/store.rs`, `contracts/ecra-identity-v1/migrations/`, `crates/ecra-identity/tests/migration.rs`. **FR-021, FR-054**
- [ ] **T041A** Implement the complete local bootstrap/enrollment transaction using `SecureRandom`, the production/test-isolated `TrustBackend` boundary and the authenticated protected trust-state store: generate fresh opaque principal/root/enrollment/key IDs, protect required secrets, atomically publish state, reopen/authenticate, then return `EnrolledPrincipalHandle`; partial state is `incomplete_bootstrap` and never silently remints identity. **Paths:** `crates/ecra-identity/src/bootstrap.rs`, `crates/ecra-identity/src/backend.rs`, `crates/ecra-identity/src/store.rs`. **FR-003, FR-013, FR-015, FR-021–FR-025, FR-056; C1/C2; SC-008**
- [ ] **T038** Implement retirement semantics blocking new signing/protection while permitting only contract-authorized historical verification/decryption. **Path:** `crates/ecra-identity/src/key.rs`. **FR-017–FR-020**
- [ ] **T037** Implement rotate transition: create/protect next generation, activate it and atomically publish new protected trust state; prior active becomes retired according to purpose. **Paths:** `crates/ecra-identity/src/key.rs`, `crates/ecra-identity/src/store.rs`. **FR-019, FR-021**
- [ ] **T039** Implement revocation semantics in protected trust state blocking new use/current assertion validation; distinguish revocation from unavailable/destroyed key and reject ordinary metadata attempts to unrevoke/reactivate. **Paths:** `crates/ecra-identity/src/key.rs`, `crates/ecra-identity/src/validation.rs`, `crates/ecra-identity/src/store.rs`. **FR-020, FR-021**
- [ ] **T040** Add exhaustive lifecycle/bootstrap tests including first enrollment, crash before/after backend secret creation and before/after protected-state publish, incomplete-bootstrap recovery, invalid second bootstrap, generation collision, stale-key issuance, revoked-key validation, stale/unsigned metadata and explicit no-monotonic-rollback-overclaim fixture. **Paths:** `crates/ecra-identity/tests/bootstrap.rs`, `crates/ecra-identity/tests/key_lifecycle.rs`. **SC-004, SC-008**
- [ ] **T042** Exact-head Phase 4 CI and status ledger update after T035–T041A and the corrected prerequisite wave are complete. **Path:** `specs/031-identity-trust-root/STATUS.md`. **SC-004, SC-008, SC-013**
"""
if tasks.count(old_phase4) != 1:
    raise SystemExit("Phase 4 task block did not match exactly")
tasks = tasks.replace(old_phase4, new_phase4, 1)

old_graph = """T021–T034 assertion/bootstrap interfaces/issuance/validation
        ↓
T035–T042 protected bootstrap/trust state/key lifecycle
        ↓
T043–T053 protected envelope + secret custody
        ↓
T054–T058 protected anchor
        ↓
T059–T068 native backend/macOS acceptance
        ↓
T069–T074 cross-cutting gates
"""
new_graph = """T021–T034 assertion/bootstrap interfaces/issuance/validation
        ↓
IC-001 prerequisite wave:
T043 → T044 → T045 → T046 → T047 → T048 → T049 → T050 → T059 → T060
        ↓
T035 → T036 → T041 → T041A → T038 → T037 → T039 → T040 → T042
        ↓
T051–T053 remaining protected-envelope/redaction closure
        ↓
T054–T058 protected anchor
        ↓
T061–T068 native backend/macOS acceptance
        ↓
T069–T074 cross-cutting gates
"""
if tasks.count(old_graph) != 1:
    raise SystemExit("Dependency graph block did not match exactly")
tasks = tasks.replace(old_graph, new_graph, 1)
tasks_path.write_text(tasks)

plan_path = Path("specs/031-identity-trust-root/plan.md")
plan = plan_path.read_text()
marker = """## 4. Workstream A — primitives and strict contracts
"""
insert = """## 3.1 Implementation dependency correction IC-001

Live Phase 4 implementation review found that the original phase order was not executable without violating frozen requirements. Bootstrap and lifecycle persistence depend on `SensitiveBytes`, `SecureRandom`, authenticated-envelope primitives and the Rust-owned `TrustBackend`, while crash tests depend on the store they verify.

The canonical execution order is therefore dependency-driven while stable task IDs are preserved:

```text
T043 → T044 → T045 → T046 → T047 → T048 → T049 → T050 → T059 → T060
  → T035 → T036 → T041 → T041A → T038 → T037 → T039 → T040 → T042
```

T035 now owns the strict pure schemas/invariants. T041A owns the complete generated/durable bootstrap transaction after the authenticated store exists. This is an ordering correction only: no security requirement, backend fail-closed rule, identity non-claim or slice boundary is weakened. See `implementation-clarifications.md`.

"""
if plan.count(marker) != 1:
    raise SystemExit("Plan insertion marker did not match exactly")
plan = plan.replace(marker, insert + marker, 1)
plan_path.write_text(plan)

status_path = Path("specs/031-identity-trust-root/STATUS.md")
status = status_path.read_text()
old_frontier = """## Current execution frontier

1. Phase 3 semantic implementation is verified on exact head `35df7cab41c85cf9f0c9e6f6b7d20c0a57b18d15` by permanent ECR-031 CI run `33165443131`, job `98829634574`, result `SUCCESS`;
2. this record-only ledger convergence marks T021–T034 complete and synchronizes `tasks.md`, `STATUS.md`, and `EXECUTION.md`;
3. require permanent ECR-031 CI to reach terminal `completed/success` on the exact ledger-convergence head before semantic Phase 4 work begins;
4. after that exact-head gate succeeds, begin T035 and continue Phase 4 strictly in dependency order; any dependency/feature/native-backend change requires fresh reviewed disposition and exact-head evidence.
"""
new_frontier = """## Current execution frontier

1. Phase 3 closure head `7eaede3f9f10461c307c8900c021273a4dbffa03` passed permanent ECR-031 CI run `33165941748`, job `98831297208`, result `SUCCESS`; Phase 3 is verified on-branch and the next implementation wave is eligible.
2. Implementation review discovered MUST-level dependency-order defect IC-001: original T035–T042 cannot be completed honestly before T043–T050 and T059–T060 foundations.
3. This convergence head synchronizes `implementation-clarifications.md`, `tasks.md`, `plan.md`, `analyze.md`, `STATUS.md`, and `EXECUTION.md` without weakening requirements.
4. Require permanent ECR-031 CI `completed/success` on the exact IC-001 convergence head; then begin corrected prerequisite task T043, followed by the dependency graph in `tasks.md`.
"""
if status.count(old_frontier) != 1:
    raise SystemExit("Status current frontier did not match exactly")
status = status.replace(old_frontier, new_frontier, 1)

old_p4 = """## Phase 4 execution frontier

Semantic Phase 4 T035–T042 is eligible only after the record-only Phase 3 ledger-convergence head itself passes permanent ECR-031 CI. Once that exact closure head is green, re-read the live Phase 4 requirements/contracts/data model and implement T035 first. The authenticated-envelope dependency in T041 must be reconciled with the ordered Phase 5 envelope implementation before any store behavior is claimed complete; no task-order bypass is permitted.
"""
new_p4 = """## Phase 4 dependency convergence

The record-only Phase 3 closure head `7eaede3f9f10461c307c8900c021273a4dbffa03` passed permanent ECR-031 CI run `33165941748` / job `98831297208` with result `SUCCESS`.

Live implementation review then proved the previously noted T041 envelope tension is broader and MUST-level: complete bootstrap/rotation/store semantics also require T043 `SensitiveBytes`, T044 `SecureRandom`, T045–T050 authenticated-envelope implementation/evidence, and T059–T060 backend boundary/production-selection guarantees. T040 crash tests also depend on T041 rather than preceding it.

IC-001 remediates the graph without weakening frozen requirements. After this documentation convergence head is exact-green, execution starts at T043 and follows the corrected dependency graph. No plaintext fallback, unsigned lifecycle authority, fake generated IDs or test backend in production is permitted.
"""
if status.count(old_p4) != 1:
    raise SystemExit("Status Phase4 frontier did not match exactly")
status = status.replace(old_p4, new_p4, 1)
status_path.write_text(status)

execution_path = Path("EXECUTION.md")
execution = execution_path.read_text()
old_exec = """Current phase: Phase 3 closure convergence — T021–T034 semantically complete; Phase 4 blocked on the final record-only exact-head gate
Phase 1 verified head: 0289596bb7cdbb81d5f03c445fd324e985294143
Phase 1 ECR-031 CI: 33161529028 / job 98816955646 — SUCCESS
Phase 1 Cargo.lock SHA-256: 5bd1b14d1643ff59492bafb7c6195b270cfc1424832788ad8078e62f22d907bc
Phase 2 verified head: 4ddb6da267ebc90647e27fde382385a9d2529452
Phase 2 ECR-031 CI: 33163366128 / job 98822931741 — SUCCESS
Phase 3 verified semantic head: 35df7cab41c85cf9f0c9e6f6b7d20c0a57b18d15
Phase 3 ECR-031 CI: 33165443131 / job 98829634574 — SUCCESS
Current task frontier: exact-head CI on the record-only Phase 3 closure ledger, then T035
"""
new_exec = """Current phase: Phase 4 dependency convergence — Phase 3 exact closure is green; IC-001 corrects an implementation-blocking task dependency cycle before new semantic code
Phase 1 verified head: 0289596bb7cdbb81d5f03c445fd324e985294143
Phase 1 ECR-031 CI: 33161529028 / job 98816955646 — SUCCESS
Phase 1 Cargo.lock SHA-256: 5bd1b14d1643ff59492bafb7c6195b270cfc1424832788ad8078e62f22d907bc
Phase 2 verified head: 4ddb6da267ebc90647e27fde382385a9d2529452
Phase 2 ECR-031 CI: 33163366128 / job 98822931741 — SUCCESS
Phase 3 verified semantic head: 35df7cab41c85cf9f0c9e6f6b7d20c0a57b18d15
Phase 3 semantic ECR-031 CI: 33165443131 / job 98829634574 — SUCCESS
Phase 3 verified closure head: 7eaede3f9f10461c307c8900c021273a4dbffa03
Phase 3 closure ECR-031 CI: 33165941748 / job 98831297208 — SUCCESS
Implementation clarification: IC-001 — Phase 4 dependency-order correction
Current task frontier: exact-head CI on IC-001 convergence, then T043
"""
if execution.count(old_exec) != 1:
    raise SystemExit("EXECUTION current block did not match exactly")
execution_path.write_text(execution.replace(old_exec, new_exec, 1))

analyze_path = Path("specs/031-identity-trust-root/analyze.md")
analyze = analyze_path.read_text()
header_old = """**Pass:** 2  
**Date:** 2026-08-28  
**Result:** `ZERO_BLOCKING_PLANNING_DRIFT_FOUND`  
**Implementation:** eligible only after lifecycle/status convergence to `TASKS_READY` on an exact green planning head.
"""
header_new = """**Pass:** 3 — implementation dependency revalidation  
**Date:** 2026-08-28  
**Result:** `IMPLEMENTATION_DEPENDENCY_DRIFT_REMEDIATED`  
**Implementation:** Phase 3 exact closure is green; post-Phase-3 semantic work resumes only after IC-001 convergence is exact-green, starting at corrected prerequisite T043.
"""
if analyze.count(header_old) != 1:
    raise SystemExit("Analyze header did not match exactly")
analyze = analyze.replace(header_old, header_new, 1)
append = """

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
"""
if "## 9. Pass-3 implementation dependency revalidation" in analyze:
    raise SystemExit("Analyze Pass 3 already present")
analyze_path.write_text(analyze.rstrip() + append + "\n")
