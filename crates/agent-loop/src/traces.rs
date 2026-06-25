use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraceOutcome {
    Intercepted,
    Unhandled,
}

/// Normalized trace record — populated by any `TraceSource` implementation.
///
/// Adapters map their native format to this type; callers of `TraceSource::collect`
/// see only this schema regardless of the underlying source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceRecord {
    /// Representative command string (e.g. `crs discover`'s `example` field).
    pub command: String,
    /// Command stem (first token or logical group).
    pub stem: String,
    /// Number of times this pattern appeared.
    pub count: u32,
    /// Estimated token cost (available for intercepted records from coursers).
    pub est_tokens: Option<u32>,
    /// Rule that fired, if any.
    pub rule_id: Option<String>,
    pub outcome: TraceOutcome,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_record_intercepted_roundtrip() {
        let original = TraceRecord {
            command: "crs discover".to_string(),
            stem: "crs".to_string(),
            count: 5,
            est_tokens: Some(128),
            rule_id: Some("rule-001".to_string()),
            outcome: TraceOutcome::Intercepted,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TraceRecord = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.command, original.command);
        assert_eq!(deserialized.stem, original.stem);
        assert_eq!(deserialized.count, original.count);
        assert_eq!(deserialized.est_tokens, original.est_tokens);
        assert_eq!(deserialized.rule_id, original.rule_id);
    }

    #[test]
    fn trace_record_unhandled_roundtrip() {
        let original = TraceRecord {
            command: "unknown-cmd arg".to_string(),
            stem: "unknown".to_string(),
            count: 2,
            est_tokens: None,
            rule_id: None,
            outcome: TraceOutcome::Unhandled,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TraceRecord = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.command, original.command);
        assert_eq!(deserialized.stem, original.stem);
        assert_eq!(deserialized.count, original.count);
        assert_eq!(deserialized.est_tokens, original.est_tokens);
        assert_eq!(deserialized.rule_id, original.rule_id);
    }
}
