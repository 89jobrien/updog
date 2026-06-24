use anyhow::Result;

use crate::phase::Phase;
use crate::run::RunConfig;

pub struct AutomationHeartbeat;

impl Phase for AutomationHeartbeat {
    fn id(&self) -> u8 {
        6
    }
    fn name(&self) -> &'static str {
        "Automation Heartbeat"
    }
    fn optional(&self) -> bool {
        true
    }

    fn run(&self, config: &RunConfig) -> Result<()> {
        println!("Enable to remove the human from routine low-risk changes.\n");
        println!("Gate conditions before auto-triggering:");
        println!("  - change type: add-rule | update-message | add-exception (not new-behavior)");
        println!("  - HALO score >= 3.0");
        println!("  - all eval IDs in the handoff must have a baseline result\n");
        println!("Heartbeat: detect new HANDOFF.agent-improvement.*.md in .ctx/ and call Codex.");
        println!("\nWorking dir: {}", config.working_dir.display());

        Ok(())
    }
}
