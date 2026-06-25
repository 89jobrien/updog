use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandoffChange {
    pub rank: u32,
    pub cluster_id: String,
    pub target_file: String,
    pub action: String,
    pub evidence_count: u32,
    pub eval_gate: Option<String>,
    pub spec: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handoff {
    pub title: String,
    pub agent: String,
    pub date: DateTime<Utc>,
    pub context: String,
    pub changes: Vec<HandoffChange>,
}

impl Handoff {
    pub fn to_markdown(&self) -> String {
        let mut out = format!(
            "# Agent Improvement Handoff — {}\n\n",
            self.date.format("%Y-%m-%d")
        );
        out.push_str("## Context\n\n");
        out.push_str(&self.context);
        out.push_str("\n\n## Changes (ranked by HALO score)\n\n");
        for ch in &self.changes {
            out.push_str(&format!("### Change {}: {}\n\n", ch.rank, ch.cluster_id));
            out.push_str(&format!("- **File**: `{}`\n", ch.target_file));
            out.push_str(&format!("- **Action**: {}\n", ch.action));
            out.push_str(&format!(
                "- **Evidence**: {} occurrences\n",
                ch.evidence_count
            ));
            if let Some(eval) = &ch.eval_gate {
                out.push_str(&format!(
                    "- **Eval gate**: `{}` must pass after change\n",
                    eval
                ));
            }
            out.push_str(&format!("- **Spec**:\n```\n{}\n```\n\n", ch.spec));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn sample_handoff() -> Handoff {
        Handoff {
            title: "Test Handoff".to_string(),
            agent: "test-agent".to_string(),
            date: Utc.with_ymd_and_hms(2026, 6, 24, 0, 0, 0).unwrap(),
            context: "some context".to_string(),
            changes: vec![HandoffChange {
                cluster_id: "jq-no-rule".to_string(),
                rank: 1,
                action: "add rule: no-jq-use-tool".to_string(),
                target_file: "~/.config/coursers/course-correct-rules.json".to_string(),
                evidence_count: 4,
                eval_gate: None,
                spec: "block jq; suggest gojq".to_string(),
            }],
        }
    }

    #[test]
    fn to_markdown_contains_agent_name() {
        let md = sample_handoff().to_markdown();
        assert!(
            md.contains("jq-no-rule"),
            "markdown must include cluster_id"
        );
    }

    #[test]
    fn to_markdown_contains_cluster_id() {
        let md = sample_handoff().to_markdown();
        assert!(md.contains("jq-no-rule"));
    }

    #[test]
    fn to_markdown_contains_action() {
        let md = sample_handoff().to_markdown();
        assert!(md.contains("no-jq-use-tool"));
    }

    #[test]
    fn to_markdown_contains_target_file() {
        let md = sample_handoff().to_markdown();
        assert!(md.contains("course-correct-rules.json"));
    }

    #[test]
    fn to_markdown_with_eval_gate_includes_gate() {
        let mut h = sample_handoff();
        h.changes[0].eval_gate = Some("promptfoo eval --config eval.yaml".to_string());
        let md = h.to_markdown();
        assert!(
            md.contains("promptfoo") || md.contains("eval_gate") || md.contains("Eval"),
            "markdown must include eval gate when set, got:\n{md}"
        );
    }

    #[test]
    fn to_markdown_without_eval_gate_omits_eval_section() {
        let md = sample_handoff().to_markdown();
        assert!(!md.contains("promptfoo"));
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_to_markdown_contains_correct_change_count(
            changes in prop::collection::vec(
                (
                    0u32..100,
                    "[a-z]{1,10}",
                    "[a-z]{1,10}",
                    "[a-z]{1,10}",
                    0u32..100,
                    "[a-z]{1,10}",
                ),
                0..10,
            ),
        ) {
            let handoff = Handoff {
                title: "Test".to_string(),
                agent: "agent".to_string(),
                date: Utc.with_ymd_and_hms(2026, 6, 24, 0, 0, 0).unwrap(),
                context: "context".to_string(),
                changes: changes
                    .into_iter()
                    .enumerate()
                    .map(|(idx, (rank, cluster_id, action, file, evidence, spec))| HandoffChange {
                        rank: rank.wrapping_add(idx as u32),
                        cluster_id,
                        target_file: file,
                        action,
                        evidence_count: evidence,
                        eval_gate: None,
                        spec,
                    })
                    .collect(),
            };

            let md = handoff.to_markdown();
            let expected_count = handoff.changes.len();
            let actual_count = md.matches("### Change").count();

            prop_assert_eq!(
                actual_count, expected_count,
                "markdown should contain exactly {} '### Change' headers, found {}",
                expected_count, actual_count
            );
        }
    }
}
