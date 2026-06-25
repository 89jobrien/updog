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
    /// Compute a HALO score for a feedback cluster.
    ///
    /// # Panics
    ///
    /// Panics if `effort` is 0 (would produce division by zero).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use agent_loop::{FeedbackCluster, ClusterType, Severity, HaloScore};
    ///
    /// let cluster = FeedbackCluster {
    ///     id: "grep-fp".to_string(),
    ///     cluster_type: ClusterType::FalsePositive,
    ///     severity: Severity::P1,
    ///     evidence_count: 10,
    ///     sample: String::new(),
    ///     diagnosis: String::new(),
    /// };
    /// let score = HaloScore::from_cluster(&cluster, 0.8, 2);
    /// assert!((score.score - 12.0).abs() < f64::EPSILON); // (10 * 3.0 * 0.8) / 2
    /// ```
    pub fn from_cluster(cluster: &FeedbackCluster, confidence: f64, effort: u8) -> Self {
        assert!(effort > 0, "effort must be > 0 to avoid division by zero");
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

#[cfg(test)]
mod tests {
    use crate::feedback::{ClusterType, FeedbackCluster, Severity};
    use proptest::prelude::*;

    use super::*;

    fn cluster(id: &str, severity: Severity, evidence_count: u32) -> FeedbackCluster {
        FeedbackCluster {
            id: id.to_string(),
            cluster_type: ClusterType::MissingRule,
            severity,
            evidence_count,
            sample: String::new(),
            diagnosis: String::new(),
        }
    }

    #[test]
    fn score_formula_p1() {
        // impact = 10 * 3.0 = 30, score = (30 * 0.8) / 2 = 12.0
        let c = cluster("x", Severity::P1, 10);
        let s = HaloScore::from_cluster(&c, 0.8, 2);
        assert!((s.score - 12.0).abs() < f64::EPSILON);
        assert!((s.impact - 30.0).abs() < f64::EPSILON);
    }

    #[test]
    fn score_formula_p3() {
        // impact = 10 * 1.0 = 10, score = (10 * 1.0) / 1 = 10.0
        let c = cluster("x", Severity::P3, 10);
        let s = HaloScore::from_cluster(&c, 1.0, 1);
        assert!((s.score - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn higher_severity_scores_higher_than_lower_with_equal_evidence() {
        let p1 = HaloScore::from_cluster(&cluster("a", Severity::P1, 5), 0.8, 2);
        let p3 = HaloScore::from_cluster(&cluster("b", Severity::P3, 5), 0.8, 2);
        assert!(p1.score > p3.score);
    }

    #[test]
    fn higher_evidence_scores_higher_at_same_severity() {
        let more = HaloScore::from_cluster(&cluster("a", Severity::P2, 20), 0.8, 2);
        let less = HaloScore::from_cluster(&cluster("b", Severity::P2, 5), 0.8, 2);
        assert!(more.score > less.score);
    }

    #[test]
    fn zero_evidence_gives_zero_score() {
        let c = cluster("x", Severity::P1, 0);
        let s = HaloScore::from_cluster(&c, 0.8, 2);
        assert_eq!(s.score, 0.0);
    }

    #[test]
    #[should_panic(expected = "effort must be > 0 to avoid division by zero")]
    fn halo_score_effort_zero_panics() {
        let c = cluster("x", Severity::P1, 10);
        let _ = HaloScore::from_cluster(&c, 0.8, 0);
    }

    #[test]
    fn score_formula_p2() {
        // impact = 10 * 2.0 = 20, score = (20 * 1.0) / 1 = 20.0
        let c = cluster("x", Severity::P2, 10);
        let s = HaloScore::from_cluster(&c, 1.0, 1);
        assert!((s.score - 20.0).abs() < f64::EPSILON);
        assert!((s.impact - 20.0).abs() < f64::EPSILON);
    }

    proptest! {
        #[test]
        fn halo_score_always_finite_for_nonzero_effort(effort in 1u8..=255u8) {
            let c = cluster("x", Severity::P1, 10);
            let s = HaloScore::from_cluster(&c, 0.8, effort);
            prop_assert!(s.score.is_finite());
        }
    }
}
