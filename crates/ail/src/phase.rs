use anyhow::Result;

use crate::run::RunConfig;

/// A single phase in the agent improvement loop.
///
/// Implement this trait to add a new phase without modifying the executor.
pub trait Phase {
    fn id(&self) -> u8;
    fn name(&self) -> &'static str;

    /// Optional phases are announced differently and may be skipped by default
    /// in future executor policies (e.g. `--skip-optional`).
    fn optional(&self) -> bool {
        false
    }

    fn run(&self, config: &RunConfig) -> Result<()>;
}
