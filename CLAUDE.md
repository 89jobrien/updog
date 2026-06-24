# CLAUDE.md — updog

Agent Improvement Loop workspace. Two crates:

- `agent-loop` — data model library (traces, feedback clusters, HALO scoring, diagnosis, handoff)
- `ail` — binary CLI (`ail run [--agent] [--since] [--phase] [--dry-run]`)

## Commands

```sh
cargo build
cargo build --release
cargo install --path crates/ail

cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

## Architecture

`agent-loop` owns all serializable types. `ail` owns phase orchestration and shell-out logic.
Phases call installed binaries (`crs`, `coursers`, `promptfoo`) — no compile-time coupling to
the coursers workspace.

Working files written to `.ctx/ail/<agent>/<date>/`:

- `traces.json` — phase 1 output
- `feedback.json` — phase 2 input (human-filled)
- `evals.yaml` — phase 3 template
- `diagnosis.json` — phase 4 output
- `HANDOFF.agent-improvement.<agent>.md` (in `.ctx/`) — phase 5 output

## Phase reference

| Phase | Name                 | Input          | Output           |
| ----- | -------------------- | -------------- | ---------------- |
| 1     | SDK Traces           | crs discover   | traces.json      |
| 2     | Human+LLM Feedback   | traces.json    | feedback.json    |
| 3     | Promptfoo Evals      | feedback.json  | evals.yaml       |
| 4     | HALO Diagnosis       | feedback.json  | diagnosis.json   |
| 5     | Codex Handoff        | diagnosis.json | HANDOFF.\*.md    |
| 6     | Automation Heartbeat | HANDOFF.\*.md  | (auto-trigger)   |
| 7     | Harness Update       | evals.yaml     | commit + archive |
