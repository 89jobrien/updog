use std::process::Command;

use anyhow::Result;

use crate::phase::Phase;
use crate::run::RunConfig;

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
            println!("[dry-run] crs validate");
            println!("[dry-run] cargo nextest run --workspace");
            return Ok(());
        }

        println!("Running crs validate...");
        match Command::new("crs").arg("validate").status() {
            Ok(s) if s.success() => println!("crs validate passed"),
            Ok(s) => eprintln!("crs validate exited {s}"),
            Err(e) => eprintln!("crs validate not available: {e}"),
        }

        println!("\nNext steps:");
        println!(
            "  1. promptfoo eval --config {}",
            config.working_dir.join("evals.yaml").display()
        );
        println!("  2. cargo nextest run --workspace  (in your project)");
        println!("  3. git add <changed files>");
        println!("  4. git commit -m 'fix(agent): <cluster_id> — <one-line diagnosis>'");
        println!("  5. Archive: rename .ctx/HANDOFF.agent-improvement.*.md → *.completed");

        Ok(())
    }
}
