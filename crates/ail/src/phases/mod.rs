pub mod p1;
pub mod p2;
pub mod p3;
pub mod p4;
pub mod p5;
pub mod p6;
pub mod p7;

use crate::phase::Phase;

pub fn all_phases() -> Vec<Box<dyn Phase>> {
    vec![
        Box::new(p1::SdkTraces),
        Box::new(p2::HumanFeedback),
        Box::new(p3::PromptfooEvals),
        Box::new(p4::HaloDiagnosis),
        Box::new(p5::CodexHandoff),
        Box::new(p6::AutomationHeartbeat),
        Box::new(p7::HarnessUpdate),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_phases_returns_seven_phases_in_order() {
        let phases = all_phases();
        assert_eq!(phases.len(), 7);
        assert_eq!(phases[0].id(), 1);
        assert_eq!(phases[6].id(), 7);
    }
}
