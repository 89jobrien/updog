use std::path::PathBuf;

use anyhow::Result;
use chrono::Utc;

use crate::phases::{p1, p2, p3, p4, p5, p6, p7};

pub struct RunConfig {
    pub agent: String,
    pub since: u32,
    pub start_phase: u8,
    pub dry_run: bool,
    pub working_dir: PathBuf,
}

impl RunConfig {
    pub fn new(agent: String, since: u32, start_phase: u8, dry_run: bool) -> Result<Self> {
        let date = Utc::now().format("%Y-%m-%d").to_string();
        let working_dir = PathBuf::from(".ctx/ail").join(&agent).join(&date);
        if !dry_run {
            std::fs::create_dir_all(&working_dir)?;
        }
        Ok(Self {
            agent,
            since,
            start_phase,
            dry_run,
            working_dir,
        })
    }
}

pub fn execute(config: RunConfig) -> Result<()> {
    type PhaseFn = fn(&RunConfig) -> Result<()>;

    let phases: &[(u8, &str, PhaseFn)] = &[
        (1, "SDK Traces", p1::run),
        (2, "Human+LLM Feedback", p2::run),
        (3, "Promptfoo Evals", p3::run),
        (4, "HALO Diagnosis", p4::run),
        (5, "Codex Handoff", p5::run),
        (6, "Automation Heartbeat (optional)", p6::run),
        (7, "Harness Update", p7::run),
    ];

    for (id, name, phase_fn) in phases {
        if *id < config.start_phase {
            continue;
        }
        println!("\n--- Phase {id}: {name} ---");
        phase_fn(&config)?;
    }

    Ok(())
}
