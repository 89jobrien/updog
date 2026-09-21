//! Interface implemented by Agent Improvement Loop phases.

use anyhow::Result;

use crate::run::RunConfig;

/// A single phase in the agent improvement loop.
///
/// Implement this trait to add a new phase without modifying the executor.
pub trait Phase {
    /// Returns the phase's stable execution-order identifier.
    fn id(&self) -> u8;
    /// Returns the phase name shown in terminal output.
    fn name(&self) -> &'static str;

    /// Optional phases are announced differently and may be skipped by default
    /// in future executor policies (e.g. `--skip-optional`).
    fn optional(&self) -> bool {
        false
    }

    /// Executes the phase using the shared run configuration.
    fn run(&self, config: &RunConfig) -> Result<()>;
}
