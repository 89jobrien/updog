use serde::{Deserialize, Serialize};

use crate::feedback::FeedbackCluster;

/// HALO score for a feedback cluster.
/// HALO = High-impact, Actionable, Low-effort, Observable.
///
/// score = (impact × confidence) / effort
/// impact = evidence_count × severity_weight  (P1=3, P2=2, P3=1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HaloScore {
    pub cluster_id: String,
    pub impact: f64,
    pub confidence: f64,
    pub effort: u8,
    pub score: f64,
}

impl HaloScore {
    pub fn from_cluster(cluster: &FeedbackCluster, confidence: f64, effort: u8) -> Self {
        let impact = cluster.evidence_count as f64 * cluster.severity.weight();
        let score = (impact * confidence) / effort as f64;
        Self {
            cluster_id: cluster.id.clone(),
            impact,
            confidence,
            effort,
            score,
        }
    }
}
