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
