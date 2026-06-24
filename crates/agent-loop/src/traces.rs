use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single agent action record from a trace source (crs discover, SDK logs, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceRecord {
    pub command: String,
    pub exit_code: i32,
    pub timestamp: DateTime<Utc>,
    pub session_id: Option<String>,
    pub rule_fired: Option<String>,
}
