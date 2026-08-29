use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    pub id: String,
    pub provider: String,
    pub action_type: String,
    pub effect: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRuleset {
    pub rules: Vec<PolicyRule>,
}

#[derive(Debug, Serialize)]
pub struct PolicyResult {
    pub allowed: bool,
    pub violations: Vec<String>,
    pub reasons: Vec<String>,
}

pub fn evaluate(
    ruleset: &PolicyRuleset,
    action_type: &str,
) -> PolicyResult {
    let mut violations = Vec::new();
    let mut reasons = Vec::new();

    for rule in &ruleset.rules {
        if rule.action_type == action_type
            && rule.effect.to_lowercase() == "deny"
        {
            violations.push(rule.id.clone());
            reasons.push(rule.reason.clone());
        }
    }

    PolicyResult {
        allowed: violations.is_empty(),
        violations,
        reasons,
    }
}
