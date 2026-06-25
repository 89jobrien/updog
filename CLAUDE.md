# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this
repository.

Agent Improvement Loop workspace. Two crates:

- `agent-loop` — data model library (traces, feedback clusters, HALO scoring, diagnosis, handoff)
- `ail` — binary CLI (`ail run [--agent] [--since] [--phase] [--dry-run]`)
- `xtask` — build task runner (`cargo xtask ci|fmt|fmt-check|clippy|test`)

## Commands

```sh
cargo build
cargo build --release
cargo install --path crates/ail

cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test --workspace

# Run a single test
cargo test --workspace <test_name>
cargo nextest run --workspace -E 'test(name_substr)'

# Full CI gate (fmt-check + clippy + nextest)
cargo xtask ci

# Debug logging
RUST_LOG=debug ail run --agent current --dry-run
```

## Architecture

`agent-loop` owns all serializable types. `ail` owns phase orchestration and shell-out logic.
Phases call installed binaries (`crs`, `coursers`, `promptfoo`) — no compile-time coupling to
the coursers workspace.

### Extension point

To add a new trace source, implement `agent_loop::TraceSource`:

```rust
pub trait TraceSource {
    fn collect(&self, since_days: u32) -> Result<Vec<TraceRecord>, TraceError>;
}
```

The default adapter is `ail::adapters::coursers::CourserTraceSource` (shells out to
`crs discover --format json`). Tests use `FakeTraceSource` from `crates/ail/tests/demo.rs`.

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
