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
    // TODO(test/unit): add unit tests for weight() covering all three variants (P1=3.0, P2=2.0,
    // P3=1.0) — currently only tested indirectly via halo.rs
    pub fn weight(self) -> f64 {
        match self {
            Severity::P1 => 3.0,
            Severity::P2 => 2.0,
            Severity::P3 => 1.0,
        }
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
