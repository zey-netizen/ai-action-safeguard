use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPlan {
    pub goal: String,
    pub constraints: Vec<String>,
    pub allowed_actions: Vec<String>,
}

pub fn calculate_deviation(
    plan: &AgentPlan,
    action_type: &str,
) -> f64 {
    if plan
        .allowed_actions
        .iter()
        .any(|action| action == action_type)
    {
        0.0
    } else {
        1.0
    }
}
