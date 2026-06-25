use std::cmp::Ordering;
use std::fs;

use agent_loop::{ChangeAction, ChangeItem, Diagnosis, Feedback, HaloScore};
use anyhow::{Context, Result};
use console::style;
use tracing::debug;

use crate::phase::Phase;
use crate::run::RunConfig;
use crate::ui;

/// Policy constants for the diagnosis phase.
///
/// Extracted from hardcoded literals so the defaults are substitutable without
/// touching the scoring logic.
pub struct DiagnosisPolicy {
    pub default_action: ChangeAction,
    pub default_target_file: String,
}

impl Default for DiagnosisPolicy {
    fn default() -> Self {
        Self {
            default_action: ChangeAction::AddRule,
            default_target_file: "~/.config/coursers/course-correct-rules.json".to_string(),
        }
    }
}

pub struct HaloDiagnosis;

impl Phase for HaloDiagnosis {
    fn id(&self) -> u8 {
        4
    }
    fn name(&self) -> &'static str {
        "HALO Diagnosis"
    }

    fn run(&self, config: &RunConfig) -> Result<()> {
        let feedback_path = config.working_dir.join("feedback.json");

        if !feedback_path.exists() {
            ui::warn(format!(
                "No feedback.json at {} — complete phases 1–2 first.",
                ui::path_str(&feedback_path)
            ));
            return Ok(());
        }

        let feedback: Feedback = serde_json::from_str(
            &fs::read_to_string(&feedback_path).context("reading feedback.json")?,
        )
        .context("parsing feedback.json")?;

        // Default scoring: confidence=0.8, effort=2 (5–20 lines).
        // Edit diagnosis.json and re-run phase 5 with tuned values.
        let mut scored: Vec<HaloScore> = feedback
            .clusters
            .iter()
            .map(|c| HaloScore::from_cluster(c, 0.8, 2))
            .collect();

        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal));

        debug!(count = scored.len(), "HALO scores computed");

        let policy = DiagnosisPolicy::default();

        let changes: Vec<ChangeItem> = scored
            .iter()
            .enumerate()
            .map(|(i, score)| {
                let cluster = feedback
                    .clusters
                    .iter()
                    .find(|c| c.id == score.cluster_id)
                    .expect("cluster must exist");
                ChangeItem {
                    rank: (i + 1) as u32,
                    cluster_id: cluster.id.clone(),
                    cluster_type: format!("{:?}", cluster.cluster_type),
                    halo_score: score.score,
                    action: policy.default_action.clone(),
                    target_file: policy.default_target_file.clone(),
                    evidence_count: cluster.evidence_count,
                    eval_ids: vec![],
                }
            })
            .collect();

        let diagnosis = Diagnosis { changes };
        let diagnosis_path = config.working_dir.join("diagnosis.json");

        if config.dry_run {
            ui::dry_run(format!(
                "diagnosis ({} changes) → {}",
                diagnosis.changes.len(),
                ui::path_str(&diagnosis_path)
            ));
            println!("{}", serde_json::to_string_pretty(&diagnosis)?);
            return Ok(());
        }

        fs::write(&diagnosis_path, serde_json::to_string_pretty(&diagnosis)?)?;
        ui::success(format!(
            "diagnosis ({} changes) → {}",
            diagnosis.changes.len(),
            ui::path_str(&diagnosis_path)
        ));
        for ch in &diagnosis.changes {
            println!(
                "  {} {} {}",
                style(format!("#{}", ch.rank)).dim(),
                ch.cluster_id,
                style(format!("halo={:.2}", ch.halo_score)).cyan()
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnosis_policy_default_values() {
        let policy = DiagnosisPolicy::default();
        assert_eq!(policy.default_action, ChangeAction::AddRule);
        assert!(!policy.default_target_file.is_empty());
    }
}
