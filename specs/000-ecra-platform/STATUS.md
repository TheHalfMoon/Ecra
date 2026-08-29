# Ecra Platform Status

**Purpose:** compact operational lifecycle view for the platform roadmap.  
**Architecture/dependency authority:** `roadmap.md`.  
**Current execution detail:** `../../EXECUTION.md`.

## Canonically closed slices

| ID | Slice | Lifecycle | Notes |
|---|---|---|---|
| ECR-001 | Trusted Domain Kernel | `CLOSED_CANONICAL` | closure-ledger head `85e4bf65…`; CI `33099434232` passed |
| ECR-002 | Durable Run, Ledger & Budgets | `CLOSED_CANONICAL` | closure-convergence head `aadc19c9…`; ECR-002 CI `33155302100` and ECR-001 regression `33155302026` passed |

ECR-002 is sealed as a dependency. Its v1 durability authorization remains synthetic/non-sensitive and does not replace ECR-031/ECR-003/ECR-025 protection/policy/privacy ownership.

## Active trusted-substrate work

| ID | Slice | Lifecycle | Depends on | Live state |
|---|---|---|---|---|
| ECR-031 | Identity, Trust Root & Sensitive Storage | `BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE` | ECR-001, ECR-002 | implementation PR #4; non-native work advanced, native Data Protection Keychain acceptance blocked by missing Apple Development identity/profile/team |
| ECR-004 | Verification & Reconciliation | `IMPLEMENTING_FINAL_CONVERGENCE` | ECR-001, ECR-002 | implementation PR #6; T045 Phase 7 exact-head gate passed, T046–T049 closure convergence in progress |
| ECR-003 | Authority, Information Flow, Policy & Secrets | `PLANNED_BLOCKED` | ECR-001, ECR-002, ECR-031 | implementation remains blocked until ECR-031 is `CLOSED_CANONICAL` |

ECR-031 and ECR-004 are independent lanes. ECR-004 does not depend on ECR-031 and may reach closure while ECR-031 remains externally blocked, but ECR-004 cannot absorb identity/trust-root/sensitive-storage scope.

## ECR-004 implementation evidence

Canonical implementation base:

```text
4fb61f8b41267983fc460c666fddd7781d91653c
ECR-001 exact-base CI 33237289643 SUCCESS
ECR-002 exact-base CI 33237289693 SUCCESS
```

Verified branch checkpoints include:

```text
Phase 1  e223ba5fbf8c375c580e7a93f524be3fd4c311fa  run 33237728338  SUCCESS
Phase 2  40c18b4bcf1e6c124587cdfbc0e423822eb5b138  run 33245650032  SUCCESS
T011A    75cac2aed9099d7ba82295c442b37764b284302c  run 33245970650  SUCCESS
Phase 3  f5181ca4f903f2d039463b03b3e328b1fa9c30dd  run 33246658250  SUCCESS
Phase 4  412de3f481d84154c5c2a85f11c6a6da0c89e35a  run 33247226826  SUCCESS
Phase 5  fb3fdf1ce113a55d3d7276f54681a7f55dc542b3  run 33247815573  SUCCESS
Phase 6  18ad19ae4b4f4d5f48270485af666e7204b95a0e  run 33249643366  SUCCESS
T040     815b95ed0f95513e583aa077f04e863998d0d425  run 33250068524  SUCCESS
T041     2a86dd909abfcb9d8658eab589787eb376a73004  run 33250250973  SUCCESS
T043     67207e1bc91434555bfe31997f4af9f641324a76  run 33250358128  SUCCESS
T045     90ed1bbeafea72ee655bc58a96e94696096f360e  run 33251037913  SUCCESS
```

T046/T047 traceability and constitution recheck own FR-001–FR-046, SC-001–SC-013 and G1–G15 with zero unowned MUST requirement and zero constitutional blocker. T048 found only bounded documentation convergence, owned by T049. PR #6 remains Draft until T050 exact-head final gate succeeds.

## ECR-004 frozen boundaries

- reuse ECR-001 `VerificationReceipt`; no parallel verification truth record;
- `ActionReceipt` remains executor-observed evidence and cannot self-verify;
- aggregate states expose conflict rather than last-write-wins;
- reconciliation preserves UNKNOWN unless explicit independent evidence confirms effect/no-effect;
- no synthetic `ActionReceipt` is created by reconciliation;
- retry disposition is safety advisory for a future new-attempt proposal, not authorization or same-run scheduling;
- every reconciliation outcome leaves ECR-002 prepared/unreceipted/unresolved state and `RunPhase` unchanged;
- ECR-002 `RunEvent` v1 remains unchanged and no ECR-004 sidecar projection represents run resolution;
- ECR-004 uses a separate append-only verification journal with rebuildable projections;
- the journal digest chain is integrity/corruption/substitution detection only, not hostile complete-store tamper resistance;
- v1 acceptance stores synthetic/non-sensitive evidence metadata/references/digests only;
- no browser/network/model/provider/process/policy/authorization/identity/telemetry execution dependency is admitted.

## ECR-031 boundary

ECR-031 owns local principal/trust-root/key lifecycle/protected storage semantics. The live implementation remains externally blocked on native macOS Data Protection Keychain acceptance because a valid Apple Development code-signing identity, suitable provisioning profile and usable developer account/team are absent on the trusted runner user. No legacy/plaintext/ad-hoc fallback is authorized.

## Planned critical path

| ID | Slice | Lifecycle | Depends on |
|---|---|---|---|
| ECR-031 | Identity, Trust Root & Sensitive Storage | `BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE` | ECR-001, ECR-002 |
| ECR-004 | Verification & Reconciliation | `IMPLEMENTING_FINAL_CONVERGENCE` | ECR-001, ECR-002 |
| ECR-003 | Authority, Information Flow, Policy & Secrets | `PLANNED_BLOCKED` | ECR-001, ECR-002, ECR-031 |
| ECR-005 | Evaluation & Threat Harness | `PLANNED_BLOCKED_BY_DEPENDENCIES` | ECR-001, ECR-002, ECR-003, ECR-004, ECR-031 |
| ECR-006 | Stock Firefox / WebDriver BiDi Prototype | `PLANNED` | ECR-001–ECR-005, ECR-031 |
| ECR-007 | Browser Foundation & Upstream Strategy | `PLANNED` | ECR-006 |
| ECR-008 | Ecra Browser Wedge | `PLANNED` | ECR-003, ECR-004, ECR-006, ECR-007, ECR-031 |
| ECR-009 | Search Evidence Fabric | `PLANNED` | ECR-001, ECR-002, ECR-003, ECR-004 |
| ECR-010 | Workspace & Memory | `PLANNED` | ECR-001–ECR-004, ECR-009, ECR-031 |
| ECR-011 | Browser-Native Semantic Capabilities | `PLANNED` | ECR-003, ECR-004, ECR-006, ECR-009 |
| ECR-012 | Skill IR | `PLANNED` | ECR-001–ECR-004 |
| ECR-013 | Skill Compiler | `PLANNED` | ECR-005, ECR-010, ECR-011, ECR-012 |
| ECR-014 | Deterministic Replay | `PLANNED` | ECR-012, ECR-013 |
| ECR-015 | Divergence & Repair | `PLANNED` | ECR-014 |
| ECR-016 | Protocol Gateway | `PLANNED` | ECR-001–ECR-004, ECR-009, ECR-010, ECR-012, ECR-031 |
| ECR-017 | Plugin & Sandbox Runtime | `PLANNED` | ECR-003, ECR-004, ECR-005, ECR-016 |
| ECR-018 | Terminal Execution | `PLANNED` | ECR-002–ECR-005, ECR-017, ECR-031 |
| ECR-019 | Developer Workspace | `PLANNED` | ECR-009, ECR-010, ECR-016, ECR-018 |
| ECR-020 | Data & Analytics | `PLANNED` | ECR-004, ECR-009, ECR-010, ECR-017, ECR-031 |
| ECR-021 | Local Model Gateway | `PLANNED` | ECR-009–ECR-017, ECR-024 |

## Wave view

```text
A. Trusted substrate
   ECR-001 [CLOSED] -> ECR-002 [CLOSED]
        -> {ECR-031 [BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE], ECR-004 [FINAL_CONVERGENCE]}
        -> ECR-003 -> ECR-005

B. Browser wedge
   ECR-006 -> ECR-007 -> ECR-008

C. Trusted knowledge/context
   ECR-009 -> ECR-010 -> ECR-011

D. Skills
   ECR-012 -> ECR-013 -> ECR-014 -> ECR-015

E. Ecosystem/work surfaces
   ECR-016/ECR-017 -> ECR-018/ECR-019/ECR-020/ECR-021
```

## Sensitive-state boundary

ECR-002 proved synthetic/non-sensitive local durability. ECR-004 likewise remains synthetic/non-sensitive for persisted evidence. ECR-031 owns protected identity/trust/storage foundations but is not yet canonically closed. Downstream real sensitive state remains gated by the appropriate ECR-031/ECR-003/ECR-025 owners.

## Update rule

When a slice lifecycle changes, update this file, `../../EXECUTION.md`, and the status field in `roadmap.md` in the same convergence/closure work. This file never overrides dependency semantics in `roadmap.md`.