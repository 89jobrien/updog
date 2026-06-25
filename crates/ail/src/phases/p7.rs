use std::process::Command;

use anyhow::Result;
use tracing::warn;

use crate::phase::Phase;
use crate::run::RunConfig;
use crate::ui;

// TODO(test/integration): add integration test for phase 7 dry-run — verify dry_run=true
// returns Ok and does not invoke crs validate or write any files
pub struct HarnessUpdate;

impl Phase for HarnessUpdate {
    fn id(&self) -> u8 {
        7
    }
    fn name(&self) -> &'static str {
        "Harness Update"
    }

    fn run(&self, config: &RunConfig) -> Result<()> {
        if config.dry_run {
            ui::dry_run("crs validate");
            ui::dry_run("cargo nextest run --workspace");
            return Ok(());
        }

        ui::info("Running crs validate…");
        match Command::new("crs").arg("validate").status() {
            Ok(s) if s.success() => ui::success("crs validate passed"),
            Ok(s) => warn!(exit = %s, "crs validate failed"),
            Err(e) => warn!(error = %e, "crs validate not available"),
        }

        println!();
        ui::info("Next steps:");
        ui::info(format!(
            "  1. {}",
            ui::code(format!(
                "promptfoo eval --config {}",
                config.working_dir.join("evals.yaml").display()
            ))
        ));
        ui::info(format!(
            "  2. {}",
            ui::code("cargo nextest run --workspace")
        ));
        ui::info(format!("  3. {}", ui::code("git add <changed files>")));
        ui::info(format!(
            "  4. {}",
            ui::code("git commit -m 'fix(agent): <cluster_id> — <diagnosis>'")
        ));
        ui::info(format!(
            "  5. Archive: rename {} → {}",
            ui::code("HANDOFF.agent-improvement.*.md"),
            ui::code("*.completed")
        ));

        Ok(())
    }
}
