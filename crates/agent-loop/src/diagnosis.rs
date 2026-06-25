use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ChangeAction {
    AddRule,
    UpdateThreshold,
    FixMessage,
    AddException,
    NewBehavior,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeItem {
    pub rank: u32,
    pub cluster_id: String,
    pub cluster_type: String,
    pub halo_score: f64,
    pub action: ChangeAction,
    pub target_file: String,
    pub evidence_count: u32,
    pub eval_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnosis {
    pub changes: Vec<ChangeItem>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_item_serde_roundtrip() {
        let original = ChangeItem {
            rank: 1,
            cluster_id: "test-cluster".to_string(),
            cluster_type: "missing-rule".to_string(),
            halo_score: 8.5,
            action: ChangeAction::AddRule,
            target_file: "/path/to/rules.json".to_string(),
            evidence_count: 5,
            eval_ids: vec!["eval-1".to_string(), "eval-2".to_string()],
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ChangeItem = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.rank, original.rank);
        assert_eq!(deserialized.cluster_id, original.cluster_id);
        assert_eq!(deserialized.cluster_type, original.cluster_type);
        assert_eq!(deserialized.halo_score, original.halo_score);
        assert_eq!(deserialized.action, original.action);
        assert_eq!(deserialized.target_file, original.target_file);
        assert_eq!(deserialized.evidence_count, original.evidence_count);
        assert_eq!(deserialized.eval_ids, original.eval_ids);
    }

    #[test]
    fn diagnosis_serde_roundtrip() {
        let original = Diagnosis {
            changes: vec![
                ChangeItem {
                    rank: 1,
                    cluster_id: "cluster-1".to_string(),
                    cluster_type: "add-rule".to_string(),
                    halo_score: 9.2,
                    action: ChangeAction::AddRule,
                    target_file: "/rules.json".to_string(),
                    evidence_count: 3,
                    eval_ids: vec!["eval-1".to_string()],
                },
                ChangeItem {
                    rank: 2,
                    cluster_id: "cluster-2".to_string(),
                    cluster_type: "fix-message".to_string(),
                    halo_score: 7.1,
                    action: ChangeAction::FixMessage,
                    target_file: "/messages.json".to_string(),
                    evidence_count: 2,
                    eval_ids: vec![],
                },
            ],
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Diagnosis = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.changes.len(), original.changes.len());
        for (i, original_change) in original.changes.iter().enumerate() {
            let des_change = &deserialized.changes[i];
            assert_eq!(des_change.rank, original_change.rank);
            assert_eq!(des_change.cluster_id, original_change.cluster_id);
            assert_eq!(des_change.halo_score, original_change.halo_score);
        }
    }
}
