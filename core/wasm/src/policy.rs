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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deny_rule_should_block_action() {
        let ruleset = PolicyRuleset {
            rules: vec![
                PolicyRule {
                    id: "TEST-DENY-001".to_string(),
                    provider: "test".to_string(),
                    action_type: "financial.transfer".to_string(),
                    effect: "deny".to_string(),
                    reason: "Financial transfer is denied by policy.".to_string(),
                }
            ],
        };

        let result = evaluate(
            &ruleset,
            "financial.transfer"
        );

        assert!(!result.allowed);
        assert_eq!(
            result.violations,
            vec!["TEST-DENY-001"]
        );
    }

    #[test]
    fn allowed_action_should_pass() {
        let ruleset = PolicyRuleset {
            rules: vec![
                PolicyRule {
                    id: "TEST-DENY-001".to_string(),
                    provider: "test".to_string(),
                    action_type: "financial.transfer".to_string(),
                    effect: "deny".to_string(),
                    reason: "Financial transfer is denied by policy.".to_string(),
                }
            ],
        };

        let result = evaluate(
            &ruleset,
            "search_flights"
        );

        assert!(result.allowed);
        assert!(result.violations.is_empty());
    }
}
