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

        let json = String::from_utf8_lossy(&out.stdout);
        parse_discover_output(&json)
    }
}

// TODO(test/fuzz): fuzz target requires separate `cargo +nightly fuzz` and dedicated `fuzz/` crate
pub(crate) fn parse_discover_output(json: &str) -> Result<Vec<TraceRecord>, TraceError> {
    let parsed: DiscoverOutput = serde_json::from_str(json)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_is_parse_error() {
        let err = parse_discover_output("").unwrap_err();
        assert!(matches!(err, TraceError::Parse(_)));
    }

    #[test]
    fn invalid_json_is_parse_error() {
        let err = parse_discover_output("not json").unwrap_err();
        assert!(matches!(err, TraceError::Parse(_)));
    }

    #[test]
    fn empty_output_returns_no_records() {
        let json = r#"{"intercepted":[],"unhandled":[],"scanned_commands":0,"scanned_sessions":0}"#;
        let records = parse_discover_output(json).unwrap();
        assert!(records.is_empty());
    }

    #[test]
    fn valid_output_maps_intercepted_and_unhandled() {
        let json = r#"{
            "intercepted": [
                {
                    "stem": "grep",
                    "example": "grep -r foo .",
                    "count": 3,
                    "est_tokens": 120,
                    "rule_id": "no-grep-use-rg"
                }
            ],
            "unhandled": [
                {
                    "stem": "jq",
                    "example": "jq '.' out.json",
                    "count": 2
                }
            ]
        }"#;
        let records = parse_discover_output(json).unwrap();
        assert_eq!(records.len(), 2);

        let intercepted = records.iter().find(|r| r.stem == "grep").unwrap();
        assert!(matches!(intercepted.outcome, TraceOutcome::Intercepted));
        assert_eq!(intercepted.rule_id.as_deref(), Some("no-grep-use-rg"));
        assert_eq!(intercepted.est_tokens, Some(120));
        assert_eq!(intercepted.count, 3);

        let unhandled = records.iter().find(|r| r.stem == "jq").unwrap();
        assert!(matches!(unhandled.outcome, TraceOutcome::Unhandled));
        assert!(unhandled.rule_id.is_none());
        assert!(unhandled.est_tokens.is_none());
        assert_eq!(unhandled.count, 2);
    }

    #[cfg(test)]
    mod proptest {
        use proptest::proptest;
        use super::super::*;

        proptest! {
            #[test]
            fn parse_never_panics_on_arbitrary_input(s in ".*") {
                match parse_discover_output(&s) {
                    Ok(_) => {}
                    Err(TraceError::Parse(_)) => {}
                    Err(TraceError::Unavailable(_)) => {
                        panic!("parse_discover_output must not return Unavailable");
                    }
                    Err(_) => {
                        panic!("parse_discover_output returned unknown error variant");
                    }
                }
            }
        }
    }
}
