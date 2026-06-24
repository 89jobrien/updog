use std::process::Command;

use crate::phase::Phase;
use crate::run::RunConfig;
use anyhow::Result;

pub struct SdkTraces;

impl Phase for SdkTraces {
    fn id(&self) -> u8 {
        1
    }
    fn name(&self) -> &'static str {
        "SDK Traces"
    }

    fn run(&self, config: &RunConfig) -> Result<()> {
        if config.dry_run {
            println!(
                "[dry-run] collect traces via configured source (since {} days)",
                config.since
            );
            return Ok(());
        }

        let traces = config.source.collect(config.since).unwrap_or_else(|e| {
            eprintln!("trace source error: {e}; continuing with empty set");
            vec![]
        });

        let out_path = config.working_dir.join("traces.json");
        std::fs::write(&out_path, serde_json::to_string_pretty(&traces)?)?;
        println!("{} trace records → {}", traces.len(), out_path.display());

        // crs stats: display-only frequency summary, best-effort
        if let Ok(stats) = Command::new("crs").arg("stats").output()
            && stats.status.success()
        {
            println!("\n{}", String::from_utf8_lossy(&stats.stdout));
        }

        Ok(())
    }
}
