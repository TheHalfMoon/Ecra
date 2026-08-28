from pathlib import Path

TASKS = Path("specs/031-identity-trust-root/tasks.md")
STATUS = Path("specs/031-identity-trust-root/STATUS.md")
EXECUTION = Path("EXECUTION.md")

tasks = TASKS.read_text()
for task in ("T043", "T044"):
    old = f"- [ ] **{task}**"
    new = f"- [x] **{task}**"
    if tasks.count(old) != 1:
        raise SystemExit(f"expected exactly one open {task}, found {tasks.count(old)}")
    tasks = tasks.replace(old, new, 1)
TASKS.write_text(tasks)

status = STATUS.read_text().replace("\x0b", "").replace("ECS-001/ECR-002", "ECR-001/ECR-002")
start_marker = "## Current execution frontier\n"
end_marker = "## Phase 1 verified closure evidence"
start = status.find(start_marker)
end = status.find(end_marker, start)
if start < 0 or end < 0:
    raise SystemExit("STATUS frontier markers not found")
frontier = """## Current execution frontier

1. IC-001 dependency convergence is exact-head verified on `21bce89f2e77bc2a54e74c37d349e9b53aa7631b` by permanent ECR-031 CI run `33168062289`, job `98838136692`, result `SUCCESS`.
2. T043 `SensitiveBytes` is exact-head verified on `62048d9061dc1b74a9b5e0fed7376fe0ae08f2c3` by permanent ECR-031 CI run `33168253618`, job `98838768800`, result `SUCCESS`; formatting is redacted and owned bytes are zeroized on drop without process/OS-wide memory-secrecy claims.
3. T044 `SecureRandom` is exact-head verified on `0f84b2215529442cf7efbd1d3fa2892f224e6e6e` by permanent ECR-031 CI run `33168674153`, job `98840158147`, result `SUCCESS`; production uses the locked system CSPRNG boundary and deterministic randomness remains `cfg(test)` only.
4. This record-only convergence marks T043–T044 complete. Require permanent ECR-031 CI `completed/success` on the exact ledger-convergence head; then begin T045 strict `ProtectedEnvelopeV1`/AAD work and continue the corrected dependency graph.
"""
status = status[:start] + frontier + "\n" + status[end:]

evidence_heading = "## Corrected prerequisite wave evidence"
if evidence_heading in status:
    raise SystemExit("corrected prerequisite evidence already present")
evidence = """## Corrected prerequisite wave evidence

```text
IC001_VERIFIED_HEAD       21bce89f2e77bc2a54e74c37d349e9b53aa7631b
IC001_ECR031_CI_RUN       33168062289
IC001_ECR031_CI_JOB       98838136692
IC001_ECR031_CI_RESULT    SUCCESS
T043_VERIFIED_HEAD        62048d9061dc1b74a9b5e0fed7376fe0ae08f2c3
T043_ECR031_CI_RUN        33168253618
T043_ECR031_CI_JOB        98838768800
T043_ECR031_CI_RESULT     SUCCESS
T044_VERIFIED_HEAD        0f84b2215529442cf7efbd1d3fa2892f224e6e6e
T044_ECR031_CI_RUN        33168674153
T044_ECR031_CI_JOB        98840158147
T044_ECR031_CI_RESULT     SUCCESS
NEXT                      T045
```

The initial T044 semantic head `522ea0f824dca4f60582f0d365c0a9a0919484f9` failed workspace tests only because its source-architecture assertion matched the assertion's own string literal. The production `SystemSecureRandom` implementation, build, format and strict Clippy were already green. Commit `0f84b2215529442cf7efbd1d3fa2892f224e6e6e` fixed the test forward-only by inspecting only the production source prefix before `#[cfg(test)]`; the complete permanent gate then passed.
"""
status = status.replace(end_marker, evidence + "\n\n" + end_marker, 1)
STATUS.write_text(status)

execution = EXECUTION.read_text()
phase_prefix = "Current phase: "
frontier_prefix = "Current task frontier: "
phase_lines = [line for line in execution.splitlines() if line.startswith(phase_prefix)]
frontier_lines = [line for line in execution.splitlines() if line.startswith(frontier_prefix)]
if len(phase_lines) != 1 or len(frontier_lines) != 1:
    raise SystemExit("EXECUTION phase/frontier lines not unique")
execution = execution.replace(
    phase_lines[0],
    "Current phase: Corrected prerequisite wave — IC-001, T043 and T044 exact-head verified; record-only ledger convergence gates T045",
    1,
)
execution = execution.replace(
    frontier_lines[0],
    "Current task frontier: exact-head CI on the T043–T044 ledger convergence, then T045",
    1,
)
anchor = "Implementation clarification: IC-001 — Phase 4 dependency-order correction\n"
if execution.count(anchor) != 1:
    raise SystemExit("EXECUTION IC-001 anchor not unique")
evidence_lines = """IC-001 verified convergence head: 21bce89f2e77bc2a54e74c37d349e9b53aa7631b
IC-001 ECR-031 CI: 33168062289 / job 98838136692 — SUCCESS
T043 verified head: 62048d9061dc1b74a9b5e0fed7376fe0ae08f2c3
T043 ECR-031 CI: 33168253618 / job 98838768800 — SUCCESS
T044 verified head: 0f84b2215529442cf7efbd1d3fa2892f224e6e6e
T044 ECR-031 CI: 33168674153 / job 98840158147 — SUCCESS
"""
execution = execution.replace(anchor, anchor + evidence_lines, 1)
EXECUTION.write_text(execution)
