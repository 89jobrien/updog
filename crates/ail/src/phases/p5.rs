use std::fs;
use std::path::Path;

use agent_loop::{Diagnosis, Handoff, HandoffChange};
use anyhow::{Context, Result};
use chrono::Utc;

use crate::run::RunConfig;

pub fn run(config: &RunConfig) -> Result<()> {
    let diagnosis_path = config.working_dir.join("diagnosis.json");

    if !diagnosis_path.exists() {
        println!("No diagnosis.json — complete phase 4 first.");
        return Ok(());
    }

    let diagnosis: Diagnosis = serde_json::from_str(
        &fs::read_to_string(&diagnosis_path).context("reading diagnosis.json")?,
    )
    .context("parsing diagnosis.json")?;

    let handoff_changes: Vec<HandoffChange> = diagnosis
        .changes
        .iter()
        .map(|ch| HandoffChange {
            rank: ch.rank,
            cluster_id: ch.cluster_id.clone(),
            target_file: ch.target_file.clone(),
            action: format!("{:?}", ch.action),
            evidence_count: ch.evidence_count,
            eval_gate: ch.eval_ids.first().cloned(),
            spec: String::from("# TODO: fill in the exact JSON/TOML block to add or modify"),
        })
        .collect();

    let handoff = Handoff {
        title: format!("agent improvement loop: {}", config.agent),
        agent: config.agent.clone(),
        date: Utc::now(),
        context: format!(
            "Agent: {}. Loop iteration covering last {} days of traces. Source: crs discover.",
            config.agent, config.since
        ),
        changes: handoff_changes,
    };

    let md = handoff.to_markdown();
    let handoff_path =
        Path::new(".ctx").join(format!("HANDOFF.agent-improvement.{}.md", config.agent));

    if config.dry_run {
        println!(
            "[dry-run] would write handoff to {}",
            handoff_path.display()
        );
        println!("\n{}", md);
        return Ok(());
    }

    fs::create_dir_all(".ctx")?;
    fs::write(&handoff_path, &md)?;
    println!("Wrote handoff → {}", handoff_path.display());

    Ok(())
}
