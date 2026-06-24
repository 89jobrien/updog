use std::process::Command;

use anyhow::Result;
use tracing::debug;

use crate::phase::Phase;
use crate::run::RunConfig;
use crate::ui;

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
            ui::dry_run(format!(
                "collect traces via configured source (since {} days)",
                config.since
            ));
            return Ok(());
        }

        let traces = config.source.collect(config.since).unwrap_or_else(|e| {
            ui::warn(format!(
                "trace source error: {e} — continuing with empty set"
            ));
            vec![]
        });

        debug!(count = traces.len(), "trace records collected");

        let out_path = config.working_dir.join("traces.json");
        std::fs::write(&out_path, serde_json::to_string_pretty(&traces)?)?;
        ui::success(format!(
            "{} trace records → {}",
            traces.len(),
            ui::path_str(&out_path)
        ));

        // crs stats: frequency summary, best-effort display
        if let Ok(stats) = Command::new("crs").arg("stats").output()
            && stats.status.success()
        {
            println!("\n{}", String::from_utf8_lossy(&stats.stdout));
        }

        Ok(())
    }
}
