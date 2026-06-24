use anyhow::Result;

use crate::run::RunConfig;

pub fn run(config: &RunConfig) -> Result<()> {
    println!("Phase 6 is optional — enable to remove the human from routine low-risk changes.\n");
    println!("Gate conditions before auto-triggering:");
    println!("  - change type: add-rule | update-message | add-exception (not new-behavior)");
    println!("  - HALO score >= 3.0");
    println!("  - all eval IDs in the handoff must have a baseline result\n");
    println!(
        "Heartbeat script: detect new HANDOFF.agent-improvement.*.md in .ctx/ and call Codex."
    );
    println!("\nWorking dir: {}", config.working_dir.display());

    Ok(())
}
