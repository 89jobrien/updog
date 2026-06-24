use std::process::Command;

use agent_loop::TraceRecord;
use anyhow::{Context, Result};

use crate::run::RunConfig;

pub fn run(config: &RunConfig) -> Result<()> {
    if config.dry_run {
        println!(
            "[dry-run] crs discover --since {} --format json",
            config.since
        );
        println!("[dry-run] crs insights --format json");
        println!("[dry-run] crs stats");
        return Ok(());
    }

    let discover = Command::new("crs")
        .args([
            "discover",
            "--since",
            &config.since.to_string(),
            "--format",
            "json",
        ])
        .output()
        .context("crs discover failed — is crs installed?")?;

    let traces: Vec<TraceRecord> = if discover.status.success() {
        serde_json::from_slice(&discover.stdout).unwrap_or_default()
    } else {
        eprintln!(
            "crs discover exited non-zero; continuing with empty trace set\n{}",
            String::from_utf8_lossy(&discover.stderr)
        );
        vec![]
    };

    let out_path = config.working_dir.join("traces.json");
    std::fs::write(&out_path, serde_json::to_string_pretty(&traces)?)?;
    println!("{} trace records → {}", traces.len(), out_path.display());

    // crs stats for a quick frequency summary
    if let Ok(stats) = Command::new("crs").arg("stats").output() {
        if stats.status.success() {
            println!("\n{}", String::from_utf8_lossy(&stats.stdout));
        }
    }

    Ok(())
}
