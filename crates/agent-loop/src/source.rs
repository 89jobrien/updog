use crate::traces::TraceRecord;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum TraceError {
    #[error("trace source unavailable: {0}")]
    #[diagnostic(
        code(updog::trace::unavailable),
        help("Install crs: `cargo install --path ~/dev/coursers/crates/crs`")
    )]
    Unavailable(String),

    #[error("failed to parse trace output: {0}")]
    #[diagnostic(
        code(updog::trace::parse),
        help("Check that `crs discover --format json` produces valid JSON")
    )]
    Parse(String),
}

/// Port for collecting agent trace records.
///
/// Implement this trait to plug in any trace source — crs/coursers (default),
/// LangSmith runs, OpenAI SDK traces, custom JSONL files, etc.
pub trait TraceSource {
    fn collect(&self, since_days: u32) -> Result<Vec<TraceRecord>, TraceError>;
}
