use anyhow::Result;

use crate::phase::Phase;
use crate::run::RunConfig;
use crate::ui;

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
        ui::info("Enable to remove the human from routine low-risk changes.\n");
        ui::info("Gate conditions before auto-triggering:");
        ui::info("  - change type: add-rule | update-message | add-exception (not new-behavior)");
        ui::info("  - HALO score >= 3.0");
        ui::info("  - all eval IDs in the handoff must have a baseline result\n");
        ui::info("Heartbeat: detect new HANDOFF.agent-improvement.*.md in .ctx/ and call Codex.");
        ui::info(format!(
            "Working dir: {}",
            ui::path_str(&config.working_dir)
        ));

        Ok(())
    }
}
