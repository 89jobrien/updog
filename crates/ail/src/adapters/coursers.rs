use std::process::Command;

use agent_loop::{TraceError, TraceOutcome, TraceRecord, TraceSource};
use serde::Deserialize;

/// Wire types matching `crs discover --format json` output.
#[derive(Deserialize)]
struct DiscoverOutput {
    intercepted: Vec<InterceptedRecord>,
    unhandled: Vec<UnhandledRecord>,
}

#[derive(Deserialize)]
struct InterceptedRecord {
    count: u32,
    est_tokens: u32,
    example: String,
    rule_id: String,
    stem: String,
}

#[derive(Deserialize)]
struct UnhandledRecord {
    count: u32,
    example: String,
    stem: String,
}

/// Collects traces from the coursers/crs toolchain via `crs discover`.
pub struct CourserTraceSource;

impl TraceSource for CourserTraceSource {
    fn collect(&self, since_days: u32) -> Result<Vec<TraceRecord>, TraceError> {
        let out = Command::new("crs")
            .args([
                "discover",
                "--since",
                &since_days.to_string(),
                "--format",
                "json",
            ])
            .output()
            .map_err(|e| TraceError::Unavailable(format!("crs not found: {e}")))?;

        if !out.status.success() {
            let stderr = String::from_utf8_lossy(&out.stderr);
            return Err(TraceError::Unavailable(format!(
                "crs discover exited {}: {stderr}",
                out.status
            )));
        }

        let parsed: DiscoverOutput = serde_json::from_slice(&out.stdout)
            .map_err(|e| TraceError::Parse(format!("crs discover output: {e}")))?;

        let mut records: Vec<TraceRecord> =
            Vec::with_capacity(parsed.intercepted.len() + parsed.unhandled.len());

        for r in parsed.intercepted {
            records.push(TraceRecord {
                command: r.example,
                stem: r.stem,
                count: r.count,
                est_tokens: Some(r.est_tokens),
                rule_id: Some(r.rule_id),
                outcome: TraceOutcome::Intercepted,
            });
        }

        for r in parsed.unhandled {
            records.push(TraceRecord {
                command: r.example,
                stem: r.stem,
                count: r.count,
                est_tokens: None,
                rule_id: None,
                outcome: TraceOutcome::Unhandled,
            });
        }

        Ok(records)
    }
}
