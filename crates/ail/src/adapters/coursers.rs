use std::process::Command;

use agent_loop::{TraceError, TraceRecord, TraceSource};

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

        serde_json::from_slice(&out.stdout)
            .map_err(|e| TraceError::Parse(format!("crs discover output: {e}")))
    }
}
