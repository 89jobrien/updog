use std::path::PathBuf;

use agent_loop::TraceSource;
use anyhow::Result;
use chrono::Utc;

use crate::phase::Phase;
use crate::phases::{p1, p2, p3, p4, p5, p6, p7};

pub struct RunConfig {
    pub agent: String,
    pub since: u32,
    pub start_phase: u8,
    pub dry_run: bool,
    pub working_dir: PathBuf,
    pub source: Box<dyn TraceSource>,
}

impl RunConfig {
    pub fn new(
        agent: String,
        since: u32,
        start_phase: u8,
        dry_run: bool,
        source: Box<dyn TraceSource>,
    ) -> Result<Self> {
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
            source,
        })
    }
}

pub fn execute(config: RunConfig) -> Result<()> {
    let phases: Vec<Box<dyn Phase>> = vec![
        Box::new(p1::SdkTraces),
        Box::new(p2::HumanFeedback),
        Box::new(p3::PromptfooEvals),
        Box::new(p4::HaloDiagnosis),
        Box::new(p5::CodexHandoff),
        Box::new(p6::AutomationHeartbeat),
        Box::new(p7::HarnessUpdate),
    ];

    for phase in &phases {
        if phase.id() < config.start_phase {
            continue;
        }
        crate::ui::phase_header(phase.id(), phase.name(), phase.optional());
        phase.run(&config)?;
    }

    Ok(())
}
