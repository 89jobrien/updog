use anyhow::Result;

use crate::phase::Phase;
use crate::run::RunConfig;

pub struct HumanFeedback;

impl Phase for HumanFeedback {
    fn id(&self) -> u8 {
        2
    }
    fn name(&self) -> &'static str {
        "Human+LLM Feedback"
    }

    fn run(&self, config: &RunConfig) -> Result<()> {
        let traces_path = config.working_dir.join("traces.json");
        let feedback_path = config.working_dir.join("feedback.json");

        println!("Phase 2 requires human + LLM review.\n");
        println!("Traces: {}", traces_path.display());
        println!("\nLLM prompt template:");
        println!("---");
        println!(
            r#"Given these agent trace samples, identify:
1. Patterns that should be blocked but aren't
2. Patterns that are blocked but shouldn't be (false positives)
3. Block messages that are unhelpful or wrong
4. Threshold values that fire too early or too late

Traces: <contents of {}>

Respond as JSON:
{{
  "clusters": [
    {{
      "id": "...",
      "type": "false-positive|missing-rule|wrong-message|threshold-tuning|new-behavior",
      "severity": "P1|P2|P3",
      "evidence_count": 0,
      "sample": "...",
      "diagnosis": "..."
    }}
  ]
}}"#,
            traces_path.display()
        );
        println!("---");
        println!("\nWrite output to: {}", feedback_path.display());
        println!(
            "Then resume with: ail run --agent {} --since {} --phase 3",
            config.agent, config.since
        );

        Ok(())
    }
}
