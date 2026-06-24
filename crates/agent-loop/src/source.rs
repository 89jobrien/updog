use crate::traces::TraceRecord;

#[derive(Debug, thiserror::Error)]
pub enum TraceError {
    #[error("trace source unavailable: {0}")]
    Unavailable(String),
    #[error("failed to parse trace output: {0}")]
    Parse(String),
}

/// Port for collecting agent trace records.
///
/// Implement this trait to plug in any trace source — crs/coursers (default),
/// LangSmith runs, OpenAI SDK traces, custom JSONL files, etc.
pub trait TraceSource {
    fn collect(&self, since_days: u32) -> Result<Vec<TraceRecord>, TraceError>;
}
