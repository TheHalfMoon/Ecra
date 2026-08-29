# Ecra Platform Status

**Purpose:** compact operational lifecycle view for the platform roadmap.  
**Architecture/dependency authority:** `roadmap.md`.  
**Current execution detail:** `../../EXECUTION.md`.

## Canonically closed slices

| ID | Slice | Lifecycle | Notes |
|---|---|---|---|
| ECR-001 | Trusted Domain Kernel | `CLOSED_CANONICAL` | closure-ledger head `85e4bf65…`; CI `33099434232` passed |
| ECR-002 | Durable Run, Ledger & Budgets | `CLOSED_CANONICAL` | closure-convergence head `aadc19c9…`; ECR-002 CI `33155302100` and ECR-001 regression `33155302026` passed |
| ECR-004 | Verification & Reconciliation | `CLOSED_CANONICAL` | PR #7 feature head `990addb7…`; merge `2a95fbb4…`; closure-convergence head `c159c960…`; exact-head ECR-001/ECR-002/ECR-004 gates passed before T053 marker |

ECR-002 v1 durability and ECR-004 v1 evidence persistence remain synthetic/non-sensitive. Neither replaces ECR-031/ECR-003/ECR-025 protection, authority or privacy ownership.

## ECR-004 closure evidence

Final reviewed implementation candidate:

```text
HEAD 990addb79e6fe5a1ad2b16dae159c624959e2128
RUN  33255653083
JOB  99108796794
RESULT SUCCESS
```

Canonical merge and post-merge evidence:

```text
MERGE 2a95fbb4f20b1646505cb179f4822a758a546895
ECR-001  RUN 33255780673  JOB 99109106995  SUCCESS
ECR-002  RUN 33255780671  JOB 99109107144  SUCCESS
ECR-004  RUN 33255780663  JOB 99109107058  SUCCESS
```

Closure-convergence evidence on exact head `c159c96061a73ead9710985d07608e2b417fe275`:

```text
ECR-001  RUN 33256430974  JOB 99110882402  SUCCESS
ECR-002  RUN 33256430942  JOB 99110916386  SUCCESS
ECR-004  RUN 33256430965  JOB 99110882233  SUCCESS
```

The T053 marker must itself pass those three permanent workflows on the exact canonical `main` head before the external closure claim is made.

## Active trusted-substrate work

| ID | Slice | Lifecycle | Depends on | Live state |
|---|---|---|---|---|
| ECR-031 | Identity, Trust Root & Sensitive Storage | `BLOCKED_EXTERNAL_NATIVE_ACCEPTANCE` | ECR-001, ECR-002 | Draft PR #4; current blocking task T064; native Data Protection Keychain acceptance requires external Apple Development signing/provisioning/account/team state |
| ECR-003 | Authority, Information Flow, Policy & Secrets | `PLANNED_BLOCKED` | ECR-001, ECR-002, ECR-031 | implementation remains blocked until ECR-031 is `CLOSED_CANONICAL` |
| ECR-005 | Evaluation & Threat Harness | `PLANNED_BLOCKED_BY_DEPENDENCIES` | ECR-001, ECR-002, ECR-003, ECR-004, ECR-031 | ECR-004 is closed, but ECR-003 and ECR-031 remain open |

ECR-031 is the current dependency frontier. No later slice is implementation-eligible merely because ECR-031 is externally blocked.

## ECR-031 native blocker

The trusted self-hosted macOS runner has an interactive console user and required build/signing tools, but live evidence shows the runner user lacks the Apple signing/provisioning assets required by the Data Protection Keychain acceptance host:

- Apple Development code-signing identity: absent;
- suitable provisioning profile: absent;
- configured Xcode developer account registry: absent;
- usable Xcode development team: absent.

The exact unblock condition is to configure a valid Apple developer account/team in Xcode for the same runner user and allow Xcode to install/create an Apple Development signing identity plus a suitable provisioning profile, then rerun T064 acceptance on the exact feature head.

No legacy file-based Keychain, ad-hoc signing, plaintext/file/environment/memory fallback, `synchronizing=true`, weakened acceptance, or unsupported hardware/Secure Enclave assurance claim is authorized.

## ECR-004 frozen boundaries

- ECR-001 `VerificationReceipt` remains the only canonical independent verification record.
- `ActionReceipt` remains executor-observed evidence and cannot self-verify.
- reconciliation never fabricates `ActionReceipt`, mutates ECR-002 run-event truth, clears `unresolved_attempts`, changes `RunPhase`, or repairs/resumes the same run.
- `semantically_retryable*` remains advisory for a future new-attempt proposal only.
- ECR-004 persistence remains separate from ECR-002 run storage and synthetic/non-sensitive in v1.
- journal chaining is local integrity/corruption/substitution detection only, not hostile whole-store tamper resistance.
- no browser/network/model/provider/process/policy/authorization/identity/telemetry execution dependency entered `ecra-verify`.

## Planned critical path

```text
ECR-001 [CLOSED_CANONICAL]
  ↓
ECR-002 [CLOSED_CANONICAL]
  ├───────────────────────────────┐
  ↓                               ↓
ECR-031 [BLOCKED_EXTERNAL]      ECR-004 [CLOSED_CANONICAL]
  ↓
ECR-003 [BLOCKED]
  ↓
ECR-005 [BLOCKED]
  ↓
ECR-006 → ECR-007 → ECR-008
```

Other roadmap lanes remain governed by their explicit dependencies in `roadmap.md`; this status file does not grant implementation eligibility.

## Sensitive-state boundary

ECR-002 proved synthetic/non-sensitive local durability. ECR-004 proved synthetic/non-sensitive verification evidence persistence. ECR-031 owns protected identity/trust/storage foundations but is not canonically closed. Downstream real sensitive state remains gated by the appropriate ECR-031/ECR-003/ECR-025 owners.

## Update rule

When a slice lifecycle changes, update this file, `../../EXECUTION.md`, and the status field in `roadmap.md` in the same convergence/closure work. This file never overrides dependency semantics in `roadmap.md`.
