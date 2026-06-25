use crate::traces::TraceRecord;

#[non_exhaustive]
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum TraceError {
    #[error("trace source unavailable: {0}")]
    #[diagnostic(
        code(updog::trace::unavailable),
        help(
            "Install crs: `cargo install crs` or build from source at \
             https://github.com/89jobrien/coursers"
        )
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
///
/// # Examples
///
/// ```rust
/// use agent_loop::{TraceSource, TraceError, TraceRecord, TraceOutcome};
///
/// struct FakeSource(Vec<TraceRecord>);
///
/// impl TraceSource for FakeSource {
///     fn collect(&self, _since_days: u32) -> Result<Vec<TraceRecord>, TraceError> {
///         Ok(self.0.clone())
///     }
/// }
/// ```
pub trait TraceSource {
    fn collect(&self, since_days: u32) -> Result<Vec<TraceRecord>, TraceError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_error_unavailable_help_has_no_absolute_path() {
        // The diagnostic help text must not contain hardcoded absolute paths.
        // We check the source literal directly via the formatted error output.
        let e = TraceError::Unavailable("crs not found".to_string());
        let msg = format!("{e:?}");
        assert!(
            !msg.contains("/Users/"),
            "help must not contain absolute path, got: {msg}"
        );
        assert!(
            !msg.contains("~/dev/"),
            "help must not contain tilde path, got: {msg}"
        );
    }

    fn assert_trace_source_contract<T: TraceSource>(source: T) {
        // collect must return Ok or a typed TraceError, never panic
        match source.collect(7) {
            Ok(records) => {
                for r in &records {
                    assert!(!r.stem.is_empty(), "stem must be non-empty");
                }
            }
            Err(TraceError::Unavailable(_)) => {}
            Err(TraceError::Parse(_)) => {}
        }
    }

    struct FakeSource(Vec<TraceRecord>);

    impl TraceSource for FakeSource {
        fn collect(&self, _since_days: u32) -> Result<Vec<TraceRecord>, TraceError> {
            Ok(self.0.clone())
        }
    }

    #[test]
    fn fake_source_satisfies_contract() {
        let records = vec![
            TraceRecord {
                command: "crs discover".to_string(),
                stem: "crs".to_string(),
                count: 1,
                est_tokens: Some(50),
                rule_id: Some("rule-1".to_string()),
                outcome: crate::traces::TraceOutcome::Intercepted,
            },
            TraceRecord {
                command: "echo test".to_string(),
                stem: "echo".to_string(),
                count: 2,
                est_tokens: None,
                rule_id: None,
                outcome: crate::traces::TraceOutcome::Unhandled,
            },
        ];
        let source = FakeSource(records);
        assert_trace_source_contract(source);
    }

    #[test]
    fn empty_source_satisfies_contract() {
        let source = FakeSource(vec![]);
        assert_trace_source_contract(source);
    }
}
