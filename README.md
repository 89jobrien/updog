# updog

Agent Improvement Loop — systematic agent behavior improvement from observed traces.

## What it does

`ail` runs a 7-phase loop that:

1. **SDK Traces** — collects shell command traces via `crs discover`
2. **Human+LLM Feedback** — prompts for human + LLM review; produces `feedback.json`
3. **Promptfoo Evals** — generates an eval template (`evals.yaml`) from feedback clusters
4. **HALO Diagnosis** — scores each cluster (High-impact, Actionable, Low-effort, Observable); writes ranked `diagnosis.json`
5. **Codex Handoff** — converts diagnosis into a `HANDOFF.agent-improvement.<agent>.md`
6. **Automation Heartbeat** — optional; auto-triggers low-risk changes when gate conditions pass
7. **Harness Update** — runs `crs validate` and walks through the commit + archive checklist

## Crates

| Crate        | Type         | Purpose                                                                                           |
| ------------ | ------------ | ------------------------------------------------------------------------------------------------- |
| `agent-loop` | library      | Domain types: traces, feedback, HALO scoring, diagnosis, handoff. Defines the `TraceSource` port. |
| `ail`        | binary + lib | CLI (`ail run`), phase implementations, `CourserTraceSource` adapter                              |
| `xtask`      | binary       | Build task runner (`cargo xtask ci\|fmt\|clippy\|test`)                                           |
| `fuzz`       | fuzz (excluded from workspace) | libFuzzer target for `parse_discover_output`; run with `cargo +nightly fuzz` |

## Quick start

```bash
# Install
cargo install --path crates/ail

# Run the full loop (dry-run)
ail run --agent current --dry-run

# Run from a specific phase
ail run --agent current --phase 4

# Debug logging
RUST_LOG=debug ail run --agent current --dry-run
```

## Development

```bash
# Full CI gate (fmt-check + clippy + nextest)
cargo xtask ci        # or: just ci

# Individual gates
cargo xtask fmt-check
cargo xtask clippy
cargo xtask test

# Isolated Linux VM (requires smolvm)
just ci-vm

# Crux pipeline
just ci-crux
```

## Extending

Implement `agent_loop::TraceSource` to plug in a custom trace source:

```rust
use agent_loop::{TraceSource, TraceError, TraceRecord};

struct MySource;

impl TraceSource for MySource {
    fn collect(&self, since_days: u32) -> Result<Vec<TraceRecord>, TraceError> {
        // fetch traces from your system
        Ok(vec![])
    }
}
```

The default adapter is `CourserTraceSource` in `crates/ail/src/adapters/coursers.rs`,
which shells out to `crs discover --format json`.

## License

MIT OR Apache-2.0
