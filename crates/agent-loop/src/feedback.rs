use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ClusterType {
    FalsePositive,
    MissingRule,
    WrongMessage,
    ThresholdTuning,
    NewBehavior,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    P1,
    P2,
    P3,
}

impl Severity {
    pub fn weight(self) -> f64 {
        match self {
            Severity::P1 => 3.0,
            Severity::P2 => 2.0,
            Severity::P3 => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_p1_weight_is_3() {
        assert_eq!(Severity::P1.weight(), 3.0);
    }

    #[test]
    fn severity_p2_weight_is_2() {
        assert_eq!(Severity::P2.weight(), 2.0);
    }

    #[test]
    fn severity_p3_weight_is_1() {
        assert_eq!(Severity::P3.weight(), 1.0);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackCluster {
    pub id: String,
    #[serde(rename = "type")]
    pub cluster_type: ClusterType,
    pub severity: Severity,
    pub evidence_count: u32,
    pub sample: String,
    pub diagnosis: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback {
    pub clusters: Vec<FeedbackCluster>,
}
