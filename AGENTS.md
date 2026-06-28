# Updog — Agent Coding Guide

Agent Improvement Loop (AIL) workspace. Systematic behavior improvement from
observed shell traces: phases for collection, feedback, eval generation, HALO
scoring, handoff, automation, and harness updates.

## Build, Lint, and Test

### Quick Reference

```bash
# Full CI (format + lint + test)
just ci

# Individual gates
cargo xtask fmt-check    # Format check
cargo xtask clippy       # Linting
cargo xtask test         # Tests
just rail-ci             # Cargo rail (change-aware build + test)

# Isolated Linux VM (requires smolvm on PATH)
just ci-vm

# Crux pipeline
just ci-crux
```

### Running Tests

```bash
# All tests
cargo xtask test
cargo nextest run --workspace

# Specific test
cargo nextest run -E 'test(test_name)'

# In a single crate
cd crates/ail && cargo test
cd crates/agent-loop && cargo test
```

## Code Style Guidelines

### Rust Version & Toolchain

- **Rust Version**: 1.87.0 (pinned in workspace)
- **Edition**: 2024
- **Components**: rustfmt, clippy

### Formatting (rustfmt)

```toml
max_width = 100
edition = "2024"
```

- Line width: 100 characters maximum
- Consistency over personal preference

### Linting (clippy)

- `cargo clippy --workspace -- -D warnings`
- No disallowed methods configured; prefer standard patterns
- Favor errors over unwrap/expect in production code

### Naming Conventions

- **Structs/Enums**: `PascalCase` (`TraceRecord`, `FeedbackCluster`)
- **Functions/Methods**: `snake_case` (`collect_traces`, `score_halo`)
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case` (`adapters`, `phases`)
- **Files**: `snake_case.rs`

### Imports & Dependencies

```rust
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::env;
```

### Error Handling

- Primary: `anyhow::Result<T>` for application errors
- Propagate with `?` operator
- Log with `eprintln!` (user-facing) or `tracing` (internal)

### Architecture Pattern

```rust
// agent-loop owns serializable domain types
pub struct TraceRecord { /* ... */ }
pub struct FeedbackCluster { /* ... */ }
pub struct HALOScore { /* ... */ }

// ail owns phase orchestration + shell-out logic
// Implements TraceSource adapters (e.g., CourserTraceSource)
```

## Project Structure

```
updog/
├── crates/
│   ├── agent-loop/              # Domain types library
│   │   └── src/
│   │       ├── lib.rs
│   │       └── errors.rs        # TraceError, miette display
│   ├── ail/                     # CLI binary + phase impl
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── main.rs          # clap CLI
│   │       └── adapters/
│   │           └── coursers.rs  # TraceSource adapter
│   └── tests/                   # Integration tests
└── xtask/                       # Build task runner
    └── src/main.rs
```

## Extension Points

### Adding a Custom Trace Source

Implement `agent_loop::TraceSource`:

```rust
pub trait TraceSource {
    fn collect(&self, since_days: u32)
        -> Result<Vec<TraceRecord>, TraceError>;
}
```

Wire in `ail::adapters`:

```rust
pub struct MySource;

impl TraceSource for MySource {
    fn collect(&self, since_days: u32) -> Result<Vec<TraceRecord>> {
        // Fetch from your system
        Ok(vec![])
    }
}
```

## Phase Reference

| Phase | Name                 | Input          | Output          |
|-------|----------------------|----------------|-----------------|
| 1     | SDK Traces           | crs discover   | traces.json     |
| 2     | Human+LLM Feedback   | traces.json    | feedback.json   |
| 3     | Promptfoo Evals      | feedback.json  | evals.yaml      |
| 4     | HALO Diagnosis       | feedback.json  | diagnosis.json  |
| 5     | Codex Handoff        | diagnosis.json | HANDOFF.\*.md   |
| 6     | Automation Heartbeat | HANDOFF.\*.md  | (auto-trigger)  |
| 7     | Harness Update       | evals.yaml     | commit + archive|

## CLI Usage

```bash
# Install
cargo install --path crates/ail

# Full loop (dry-run)
ail run --agent current --dry-run

# Start from phase 4
ail run --agent current --phase 4

# Debug logging
RUST_LOG=debug ail run --agent current --dry-run
```

## Dependencies

| Crate     | Purpose                                          |
|-----------|--------------------------------------------------|
| anyhow    | Error handling                                   |
| chrono    | Time utilities                                   |
| clap      | CLI parsing (derive)                             |
| console   | Terminal output                                  |
| miette    | Diagnostic display + error formatting            |
| serde     | Serialization (derive)                           |
| serde_json| JSON I/O                                         |
| thiserror | Error types (derive)                             |
| tracing   | Structured logging                               |

## Testing

- **Unit tests**: In same file (`mod tests {}`)
- **Integration tests**: In `tests/` directory
- **Test isolation**: Avoid side effects; use `tempfile` for scratch

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_collection() {
        // Test implementation
    }
}
```

## Commit Guidelines

### Pre-commit Checks

```bash
just ci    # Format check + lint + test
```

### Message Style

Follow [Conventional Commits](https://www.conventionalcommits.org/).

Format: `<type>(<scope>): <description>`

Examples:

- `feat(ail): add custom trace source support`
- `fix(agent-loop): correct HALO score calculation`
- `docs: update phase reference`

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`

## Development Workflow

1. **Setup**: Clone and build locally
2. **Development**: Make changes, run `just ci` frequently
3. **Testing**: Write tests in-file or in `tests/`
4. **Linting**: `cargo clippy --workspace -- -D warnings`
5. **Commit**: `just ci` passing, then commit with conventional message
