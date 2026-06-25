use std::path::PathBuf;

use agent_loop::TraceSource;
use anyhow::Result;
use chrono::Utc;

use crate::phases;

/// Configuration for a single improvement loop run.
///
/// Construct with [`RunConfig::new`] (working dir derived from agent name + today's date)
/// or [`RunConfig::new_with_dir`] (explicit working directory, useful in tests).
///
/// # Examples
///
/// ```rust,no_run
/// use ail::run::{RunConfig, execute};
/// use agent_loop::{TraceSource, TraceError, TraceRecord};
///
/// struct NoopSource;
/// impl TraceSource for NoopSource {
///     fn collect(&self, _: u32) -> Result<Vec<TraceRecord>, TraceError> { Ok(vec![]) }
/// }
///
/// let config = RunConfig::new(
///     "current".to_string(),
///     30,
///     1,
///     true, // dry_run
///     Box::new(NoopSource),
/// );
/// execute(config).unwrap();
/// ```
pub struct RunConfig {
    pub agent: String,
    pub since: u32,
    pub start_phase: u8,
    pub dry_run: bool,
    pub working_dir: PathBuf,
    pub(crate) source: Box<dyn TraceSource>,
}

impl RunConfig {
    /// Standard constructor. Does not perform IO; call [`execute`] to run the loop.
    pub fn new(
        agent: String,
        since: u32,
        start_phase: u8,
        dry_run: bool,
        source: Box<dyn TraceSource>,
    ) -> Self {
        let date = Utc::now().format("%Y-%m-%d").to_string();
        let working_dir = PathBuf::from(".ctx/ail").join(&agent).join(&date);
        Self {
            agent,
            since,
            start_phase,
            dry_run,
            working_dir,
            source,
        }
    }

    /// Constructor with an explicit working directory and trace source.
    /// Intended for tests that need to control the output location.
    pub fn new_with_dir(
        agent: String,
        since: u32,
        start_phase: u8,
        dry_run: bool,
        working_dir: PathBuf,
        source: Box<dyn TraceSource>,
    ) -> Self {
        Self {
            agent,
            since,
            start_phase,
            dry_run,
            working_dir,
            source,
        }
    }
}

pub fn execute(config: RunConfig) -> Result<()> {
    if !config.dry_run {
        std::fs::create_dir_all(&config.working_dir)?;
    }

    let phases = phases::all_phases();

    for phase in &phases {
        if phase.id() < config.start_phase {
            continue;
        }
        crate::ui::phase_header(phase.id(), phase.name(), phase.optional());
        phase.run(&config)?;
    }

    Ok(())
}
