use std::cmp::Ordering;
use std::fs;

use agent_loop::{ChangeAction, ChangeItem, Diagnosis, Feedback, HaloScore};
use anyhow::{Context, Result};

use crate::phase::Phase;
use crate::run::RunConfig;

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
            println!("No feedback.json — complete phases 1–2 first.");
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
                    action: ChangeAction::AddRule,
                    target_file: String::from("~/.config/coursers/course-correct-rules.json"),
                    evidence_count: cluster.evidence_count,
                    eval_ids: vec![],
                }
            })
            .collect();

        let diagnosis = Diagnosis { changes };
        let diagnosis_path = config.working_dir.join("diagnosis.json");

        if config.dry_run {
            println!("[dry-run] diagnosis ({} changes):", diagnosis.changes.len());
            println!("{}", serde_json::to_string_pretty(&diagnosis)?);
            return Ok(());
        }

        fs::write(&diagnosis_path, serde_json::to_string_pretty(&diagnosis)?)?;
        println!(
            "Wrote diagnosis ({} changes) → {}",
            diagnosis.changes.len(),
            diagnosis_path.display()
        );
        for ch in &diagnosis.changes {
            println!(
                "  #{} {} — halo={:.2}",
                ch.rank, ch.cluster_id, ch.halo_score
            );
        }

        Ok(())
    }
}
