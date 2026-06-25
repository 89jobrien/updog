/// End-to-end demo: runs phases 1, 4, and 5 with a FakeTraceSource and
/// asserts that the expected output files are produced.
use std::fs;

use agent_loop::{
    ClusterType, Feedback, FeedbackCluster, Severity, TraceError, TraceOutcome, TraceRecord,
    TraceSource,
};
use ail::run::{RunConfig, execute};
use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Test double
// ---------------------------------------------------------------------------

struct FakeTraceSource(Vec<TraceRecord>);

impl TraceSource for FakeTraceSource {
    fn collect(&self, _since_days: u32) -> Result<Vec<TraceRecord>, TraceError> {
        Ok(self.0.clone())
    }
}

fn fake_traces() -> Vec<TraceRecord> {
    vec![
        TraceRecord {
            command: String::from("grep -r TODO ."),
            stem: String::from("grep"),
            count: 13,
            est_tokens: Some(9),
            rule_id: Some(String::from("no-grep-use-tool")),
            outcome: TraceOutcome::Intercepted,
        },
        TraceRecord {
            command: String::from("ls -la"),
            stem: String::from("ls"),
            count: 7,
            est_tokens: Some(6),
            rule_id: Some(String::from("no-ls-use-glob")),
            outcome: TraceOutcome::Intercepted,
        },
        TraceRecord {
            command: String::from("jq '.data[]' output.json"),
            stem: String::from("jq"),
            count: 4,
            est_tokens: None,
            rule_id: None,
            outcome: TraceOutcome::Unhandled,
        },
    ]
}

fn fake_feedback() -> Feedback {
    Feedback {
        clusters: vec![
            FeedbackCluster {
                id: String::from("jq-no-rule"),
                cluster_type: ClusterType::MissingRule,
                severity: Severity::P2,
                evidence_count: 4,
                sample: String::from("jq '.data[]' output.json"),
                diagnosis: String::from("jq is used frequently but has no block rule"),
            },
            FeedbackCluster {
                id: String::from("grep-false-positive"),
                cluster_type: ClusterType::FalsePositive,
                severity: Severity::P1,
                evidence_count: 13,
                sample: String::from("grep -r TODO ."),
                diagnosis: String::from("no-grep-use-tool fires on rg aliases too"),
            },
        ],
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn config_in(dir: &TempDir, start_phase: u8) -> RunConfig {
    RunConfig::new_with_dir(
        String::from("test"),
        7,
        start_phase,
        false,
        dir.path().to_path_buf(),
        Box::new(FakeTraceSource(fake_traces())),
    )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn phase1_writes_traces_json() {
    let dir = TempDir::new().unwrap();
    let config = config_in(&dir, 1);

    // Only run phase 1 (phase 2 onwards needs feedback.json)
    execute(config).unwrap();

    let traces_path = dir.path().join("traces.json");
    assert!(traces_path.exists(), "traces.json should be written");

    let raw = fs::read_to_string(&traces_path).unwrap();
    let records: Vec<TraceRecord> = serde_json::from_str(&raw).unwrap();
    assert_eq!(records.len(), 3);

    let intercepted: Vec<_> = records
        .iter()
        .filter(|r| matches!(r.outcome, TraceOutcome::Intercepted))
        .collect();
    let unhandled: Vec<_> = records
        .iter()
        .filter(|r| matches!(r.outcome, TraceOutcome::Unhandled))
        .collect();
    assert_eq!(intercepted.len(), 2);
    assert_eq!(unhandled.len(), 1);
    assert_eq!(unhandled[0].stem, "jq");
}

#[test]
fn phase4_writes_diagnosis_ranked_by_halo() {
    let dir = TempDir::new().unwrap();

    // Plant traces.json and feedback.json so phases 1–3 can be skipped
    fs::write(
        dir.path().join("traces.json"),
        serde_json::to_string(&fake_traces()).unwrap(),
    )
    .unwrap();
    fs::write(
        dir.path().join("feedback.json"),
        serde_json::to_string(&fake_feedback()).unwrap(),
    )
    .unwrap();

    let config = config_in(&dir, 4);
    execute(config).unwrap();

    let raw = fs::read_to_string(dir.path().join("diagnosis.json")).unwrap();
    let diag: agent_loop::Diagnosis = serde_json::from_str(&raw).unwrap();

    assert_eq!(diag.changes.len(), 2);
    // P1 cluster (grep-false-positive, evidence=13) should rank above P2 (jq, evidence=4)
    assert_eq!(diag.changes[0].cluster_id, "grep-false-positive");
    assert_eq!(diag.changes[1].cluster_id, "jq-no-rule");
    // Ranks are 1-indexed and ascending
    assert_eq!(diag.changes[0].rank, 1);
    assert!(diag.changes[0].halo_score > diag.changes[1].halo_score);
}

#[test]
fn phase5_writes_handoff_markdown() {
    let dir = TempDir::new().unwrap();

    fs::write(
        dir.path().join("traces.json"),
        serde_json::to_string(&fake_traces()).unwrap(),
    )
    .unwrap();
    fs::write(
        dir.path().join("feedback.json"),
        serde_json::to_string(&fake_feedback()).unwrap(),
    )
    .unwrap();

    // Run phases 4+5 together
    let config = config_in(&dir, 4);
    execute(config).unwrap();

    // Phase 5 writes to .ctx/ relative to cwd — use dry_run to avoid cwd side effects
    let dir2 = TempDir::new().unwrap();
    fs::write(
        dir2.path().join("diagnosis.json"),
        fs::read_to_string(dir.path().join("diagnosis.json")).unwrap(),
    )
    .unwrap();
    let config2 = RunConfig::new_with_dir(
        String::from("test"),
        7,
        5,
        true, // dry-run: prints but doesn't write to cwd
        dir2.path().to_path_buf(),
        Box::new(FakeTraceSource(vec![])),
    );
    // Should not error even in dry-run
    execute(config2).unwrap();
}

#[test]
fn phase5_writes_handoff_inside_working_dir() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("traces.json"),
        serde_json::to_string(&fake_traces()).unwrap(),
    )
    .unwrap();
    fs::write(
        dir.path().join("feedback.json"),
        serde_json::to_string(&fake_feedback()).unwrap(),
    )
    .unwrap();
    // Run phases 4+5 (non-dry-run)
    let config = config_in(&dir, 4);
    execute(config).unwrap();
    // .ctx must be created inside working_dir, not process cwd
    let ctx_dir = dir.path().join(".ctx");
    assert!(
        ctx_dir.exists(),
        ".ctx must be created inside working_dir, not process cwd"
    );
}

#[test]
fn phase4_returns_error_on_malformed_feedback_json() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("traces.json"),
        serde_json::to_string(&fake_traces()).unwrap(),
    )
    .unwrap();
    fs::write(dir.path().join("feedback.json"), b"not valid json").unwrap();
    let config = config_in(&dir, 4);
    assert!(
        execute(config).is_err(),
        "must error on malformed feedback.json"
    );
}

#[test]
fn phase5_returns_error_on_malformed_diagnosis_json() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("traces.json"),
        serde_json::to_string(&fake_traces()).unwrap(),
    )
    .unwrap();
    fs::write(
        dir.path().join("feedback.json"),
        serde_json::to_string(&fake_feedback()).unwrap(),
    )
    .unwrap();
    fs::write(dir.path().join("diagnosis.json"), b"{ broken").unwrap();
    let config = config_in(&dir, 5);
    assert!(
        execute(config).is_err(),
        "must error on malformed diagnosis.json"
    );
}
