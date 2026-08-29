# ECR-004 Traceability and Constitution Closure

**Slice:** ECR-004 — Verification & Reconciliation  
**Implementation branch:** `004-verification-receipts-impl`  
**Canonical implementation base:** `4fb61f8b41267983fc460c666fddd7781d91653c`  
**Closure review scope:** T046–T047  
**Constitution:** v1.1.0

This document maps every ECR-004 functional requirement and success criterion to implementation/tests/contracts and re-checks the binding G1–G15 gates and named verification risks. It records implementation truth; it grants no authority and does not replace exact-head CI evidence.

## Functional requirement traceability

| Requirement | Implementation | Acceptance evidence |
|---|---|---|
| FR-001 | `crates/ecra-verify/src/request.rs`, `src/lib.rs` reuse `ecra_core::VerificationReceipt` | `tests/request_contract.rs`, `tests/boundaries.rs` |
| FR-002 | `request.rs`, `aggregate.rs`, `checkpoint.rs`, `reconcile.rs` use canonical `VerificationTarget` | request/aggregate/checkpoint/reconcile contract tests |
| FR-003 | `evidence.rs` self-attestation checks; no ActionReceipt verification adapter | `tests/evidence.rs`, `tests/boundaries.rs`, `crates/ecra-core/tests/non_authoritative_metadata.rs` |
| FR-004 | no verified flag or parallel truth object | boundary/source tests; ECR-001 non-authoritative metadata regression |
| FR-005 | strict `VerificationRequestV1` exact target/verifier fields | `tests/request_contract.rs`, valid/invalid request fixtures |
| FR-006 | method is descriptive input; evidence rule determines decision grade | `src/evidence.rs`, `tests/evidence.rs` |
| FR-007 | request validation requires evidence for non-NotEvaluated | `request.rs`, invalid fixtures, request contract tests |
| FR-008 | optional canonical principal reference only; no identity validation | `request.rs`, dependency/boundary scripts |
| FR-009 | canonical request/receipt processing and portability determinism | request tests, `tests/portability.rs`, aggregate 1,000-repeat test |
| FR-010 | strict versions/unknown fields/duplicates/bindings with typed errors | `error.rs`, `request.rs`, invalid fixtures, hostile-input tests |
| FR-011 | journal persists references/typed records, not arbitrary raw payloads | `journal.rs`, `store.rs`, sentinel tests in `boundaries.rs`/`sqlite_store.rs` |
| FR-012 | immutable artifact/content-digest decision-grade binding | `evidence.rs`, `tests/evidence.rs` |
| FR-013 | explicit supplied evaluation/as-of freshness rules | `evidence.rs`, freshness tests |
| FR-014 | same receipt or executor network receipt cannot prove its own conclusive claim | `evidence.rs`, receipt/action-attempt self-attestation tests |
| FR-015 | independent model judgment requires independent non-model evidence | `evidence.rs`, model-judgment test |
| FR-016 | no provenance/freshness/dispute mutation | fact-targeted `tests/evidence.rs`, ECR-001 regressions |
| FR-017 | bounded notes and synthetic secret sentinel coverage | `request.rs`, `reconcile.rs`, `tests/boundaries.rs`, `tests/sqlite_store.rs` |
| FR-018 — deterministic aggregate over exact target | `aggregate.rs` | `tests/aggregate.rs`, aggregate fixtures |
| FR-019 — Absent/Verified/Rejected/Inconclusive/Conflicted closed states | `aggregate.rs` | aggregate fixture matrix |
| FR-020 — Verified + Rejected always Conflicted | `aggregate.rs` | conflict/permutation tests |
| FR-021 — NotEvaluated never satisfies checkpoint | `aggregate.rs`, `checkpoint.rs` | aggregate/checkpoint fixture tests |
| FR-022 — bounded strict checkpoint requirements | `checkpoint.rs` | `tests/checkpoint.rs`, valid/invalid checkpoint fixtures, bounded sequence deserialization |
| FR-023 — deterministic satisfied/unsatisfied/conflicted target sets | `checkpoint.rs` | checkpoint tests |
| FR-024 — no capability/approval/policy/declassification/executor surface | `checkpoint.rs` | `tests/checkpoint_boundaries.rs`, `tests/boundaries.rs` |
| FR-025 — checkpoint view derived only; no ECR-002 phase mutation | `checkpoint.rs` read-only design | checkpoint boundaries + ECR-002 regressions |
| FR-026 — exact RunId/attempt/action binding | `reconcile.rs` | reconciliation binding tests |
| FR-027 — exact three reconciliation outcomes | `reconcile.rs` | reconciliation tests |
| FR-028 — conclusive effect evidence -> effect_confirmed -> duplicate block | `reconcile.rs` | retry disposition matrix with canonical evidence revalidation |
| FR-029 — conclusive explicit no-effect evidence required; absence insufficient | `reconcile.rs`, IC-003 | `tests/reconcile.rs`, normative contract |
| FR-030 — still_unknown preserves unresolved effect and ECR-002 blocker | `reconcile.rs` read-only state input | ECR-002 compatibility tests |
| FR-031 — no synthetic `ActionReceipt` construction | `reconcile.rs` API/source boundary | `tests/boundaries.rs`, reconciliation state tests |
| FR-032 — reconciliation retains canonical verification IDs | `reconcile.rs` | support resolution/order/query-bound tests |
| FR-033 — `RetryDispositionV1` derives only advisory future-new-attempt semantics | `reconcile.rs`, crate README | retry matrix, evidence-revalidation test, boundary tests |
| FR-034 — external-reconciliation/never-blind paths remain fail-closed | `reconcile.rs` | retry matrix, ECR-002 blind-retry regressions |
| FR-035 — reconciliation records immutable/append-only and disagreement retained | `reconcile.rs`, `journal.rs`, `store.rs` | reconciliation/store tests |
| FR-036 — typed IDs and append-only journal records | `ids.rs`, `journal.rs`, `store.rs` | journal/store tests |
| FR-037 — separate ECR-004 SQLite store; no ECR-002 event/schema mutation | `store.rs` | boundaries + ECR-002 regressions |
| FR-038 — canonical IDs plus previous/content digest substitution detection | `journal.rs`, `store.rs` | journal goldens, corruption tests |
| FR-039 — synthetic/non-sensitive persistence boundary | README, journal/store design | sentinel tests |
| FR-040 — reopen/replay reproduces derived views | `store.rs` | `tests/sqlite_store.rs` reopen/replay |
| FR-041 — no browser/network/model/provider/process runtime dependency | crate Cargo manifest + `scripts/check-verify-deps.sh` | locked exact-head dependency gate |
| FR-042 — `#![forbid(unsafe_code)]`; no Ecra-authored unsafe | `src/lib.rs`, `scripts/check-verify-unsafe.sh` | exact-head boundary gate |
| FR-043 — exact count/byte/query ceilings | request/checkpoint/reconcile/journal/store constants | complete-input, sequence, source-query and journal-materialization limits plus T040 hostile/max-bound tests |
| FR-044 — typed error categories/codes for malformed/binding/evidence/conflict/store/unresolved cases | `error.rs` | contract, corruption and hostile-input tests |
| FR-045 — no ambient fetch/clock/provider; offline fixture operation | pure verification modules + local store | offline CI gate |
| FR-046 — reconciliation cannot mutate supplied ECR-002 state or make same run retryable/resumable | `reconcile.rs` takes `&RunState`; no mutation bridge | dedicated unresolved-state compatibility acceptance + ECR-002 regressions |

**FR result:** FR-001–FR-046 have implementation and acceptance owners. Zero unowned MUST functional requirement remains.

## Success criterion traceability

| Criterion | Evidence |
|---|---|
| SC-001 | request fixtures/boundaries reject target/evidence/version mismatch and preserve ActionReceipt != VerificationReceipt |
| SC-002 | aggregate permutation and 1,000-repeat byte-equivalence tests; portability tests |
| SC-003 | Verified+Rejected conflict fixtures retain both receipt IDs and produce Conflicted |
| SC-004 | reconciliation/retry matrix plus ECR-002 blind-retry guard regressions |
| SC-005 | effect-confirmed duplicate block; no-effect bounded advisory; no authorization/same-run permission |
| SC-006 | source/state tests prove no ActionReceipt fabrication and no ECR-002 state/history mutation |
| SC-007 | SQLite restart/reopen/replay and projection rebuild equivalence tests |
| SC-008 | mutable evidence without immutable binding cannot produce decision-grade conclusive verification; executor network receipt remains self-attesting even when immutably bound |
| SC-009 | exact maxima/max+1 typed failures, bounded complete request/checkpoint JSON inputs, bounded reconciliation source query, 4,096-entry materialization ceiling |
| SC-010 | permanent exact-head ECR-004 workflow executes locked build/fmt/Clippy/tests/rustdoc/offline and ECR-001/ECR-002 regressions |
| SC-011 | unsafe/dependency scripts and locked cargo-tree evidence reject forbidden runtime categories |
| SC-012 | this document maps every FR/SC and the post-implementation analyze owns remaining convergence |
| SC-013 | dedicated exact-head unresolved-state compatibility acceptance verifies byte/semantic state equality, unresolved membership and same-run guard preservation |

**SC result:** SC-001–SC-013 have acceptance owners. Zero unowned MUST success criterion remains.

## Constitution G1–G15 re-check

- **G1 Domain coherence — PASS:** canonical ECR-001 VerificationReceipt/Target/Evidence and ECR-002 run/attempt types are reused; no competing verification or execution truth model exists.
- **G2 Authority — PASS:** verification/checkpoint/reconciliation/retry outputs grant no capability, approval, declassification, execution or ambient authority.
- **G3 Provenance — PASS:** verification consumes EvidenceRef and does not rewrite ECR-001 provenance/freshness/dispute truth.
- **G4 Side effects — PASS:** ECR-004 executes no provider side effect; exact attempt identity, UNKNOWN, idempotency/retry semantics are preserved; only bounded local journal append occurs.
- **G5 Verification — PASS:** executor self-report is not verification; canonical VerificationReceipt is the only authoritative verification record.
- **G6 Durability — PASS:** append-only journal, transactional initialization/migration, expected-head append, reopen/replay and projection rebuild are tested; ECR-002 remains execution truth.
- **G7 Privacy/secrets — PASS:** v1 acceptance is synthetic/non-sensitive references/digests/metadata; sentinel tests cover persisted rows, fixtures, errors and debug/display paths.
- **G8 Local-first — PASS:** all v1 acceptance works offline with no cloud dependency.
- **G9 Interoperability — PASS/N-A:** no external protocol is introduced as trusted domain truth.
- **G10 Donor/license — PASS:** T044 reconciles exact direct dependencies/licenses/native boundary; no donor source reuse entered the slice.
- **G11 Upstream/browser maintenance — PASS/N-A:** no browser engine, privileged patch or browser permission surface exists in ECR-004.
- **G12 Benchmarks/claims — PASS:** claims are limited to deterministic contract/resource/integrity behavior; no verifier-accuracy, hostile-tamper, authenticity or exactly-once superlative is made.
- **G13 Information flow/egress — PASS:** ECR-004 performs no remote acquisition or egress; external references are data only and raw sensitive payload persistence is excluded.
- **G14 Identity/principal binding — PASS:** Actor/verifier identity is not treated as authenticated Principal; optional principal reference is evidence only and no ECR-031 trust-root assertion is minted/validated.
- **G15 Bounded execution — PASS:** record/evidence/checkpoint/support/notes/journal/query work is explicitly bounded; no recursive model/tool/process/provider execution exists.

No constitution gate requires amendment or exception for ECR-004 v1.

## Verification risk / threat closure

| Risk | Closure evidence |
|---|---|
| executor self-verification | decision-grade self-attestation rejection + ActionReceipt/VerificationReceipt boundary tests |
| wrong-target/cross-run/cross-attempt substitution | exact typed target/run/attempt/action binding and typed mismatch rejection |
| UNKNOWN coerced into retry/success | still_unknown fail-closed outcome + ECR-002 unresolved-state compatibility gate |
| duplicate external effects | effect_confirmed -> duplicate retry block; blind retry remains blocked in original run |
| absence of provider receipt used as no-effect proof | IC-003 and reconciliation tests require explicit evidence for no_effect_confirmed |
| mutable evidence presented as durable decision grade | artifact/content-digest binding and freshness rules |
| malicious evidence text becomes authority | no policy/approval parser/model execution; notes bounded/non-authoritative |
| verifier conflict/capture | append-only receipts, deterministic Conflicted view, all receipt IDs retained; no infallibility claim |
| journal corruption/substitution | sequence/previous/content digest validation, immutable canonical rows, corruption tests |
| hostile full-store rewrite overclaim | explicitly excluded; no protected trust anchor claim |
| projection poisoning | canonical journal replay authoritative; projection rebuild/poison tests |
| secret/raw-private leakage | references/digests only acceptance + sentinel scans; real sensitive evidence remains out of scope |
| ECR-004 authority bypass | no grants/approval/declassification/schedule/execute API; dependency/source scans |
| same-run unresolved-state bypass | `&RunState` read-only boundary; no RunEvent/ActionReceipt bridge; explicit RunResumed/ExecutionCompleted/blind-retry regressions |

## Closure conclusion

T046 and T047 find **zero unowned MUST requirement and zero unresolved constitutional blocker**. The remaining required lifecycle work is post-implementation analyze/convergence, final exact-head gate, review, merge and post-merge canonical evidence. This conclusion does not itself claim `CLOSED_CANONICAL`.