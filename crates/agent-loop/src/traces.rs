use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraceOutcome {
    Intercepted,
    Unhandled,
}

// TODO(test/unit): add serde roundtrip tests for TraceRecord — both Intercepted and Unhandled
// variants, with and without optional fields (est_tokens, rule_id)
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
