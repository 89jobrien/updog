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
