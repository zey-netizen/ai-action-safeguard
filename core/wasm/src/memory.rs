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

pub fn calculate_deviation_from_json(
    plan: &serde_json::Value,
    action_type: &str,
) -> f64 {
    let allowed_actions =
        match plan["allowed_actions"].as_array() {
            Some(actions) => actions,
            None => return 1.0,
        };

    if allowed_actions.iter().any(|action| {
        action.as_str() == Some(action_type)
    }) {
        0.0
    } else {
        1.0
    }
}
