use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    pub id: String,
    pub provider: String,
    pub action_type: String,
    pub effect: String,
    pub severity: String,
    pub reason: String,
    pub conditions: Vec<String>,
    pub source_reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRuleset {
    pub provider: String,
    pub version: String,
    pub updated_at: String,
    pub source: String,
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
        if rule.action_type != action_type {
            continue;
        }

        let effect = rule.effect.to_lowercase();

        match effect.as_str() {
            "deny" => {
                violations.push(rule.id.clone());
                reasons.push(rule.reason.clone());
            }

            "review" => {
                violations.push(format!("REVIEW:{}", rule.id));
                reasons.push(rule.reason.clone());
            }

            "allow" => {}

            _ => {
                violations.push(format!("INVALID:{}", rule.id));
                reasons.push(format!(
                    "Unknown policy effect: {}",
                    rule.effect
                ));
            }
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
            provider: "test".to_string(),
            version: "1.0.0".to_string(),
            updated_at: "2026-08-30".to_string(),
            source: "test".to_string(),
            rules: vec![
                PolicyRule {
                    id: "TEST-DENY-001".to_string(),
                    provider: "test".to_string(),
                    action_type: "financial.transfer".to_string(),
                    effect: "deny".to_string(),
                    severity: "critical".to_string(),
                    reason: "Financial transfer is denied.".to_string(),
                    conditions: vec![],
                    source_reference: "Test policy".to_string(),
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
    fn unrelated_action_should_pass() {
        let ruleset = PolicyRuleset {
            provider: "test".to_string(),
            version: "1.0.0".to_string(),
            updated_at: "2026-08-30".to_string(),
            source: "test".to_string(),
            rules: vec![
                PolicyRule {
                    id: "TEST-DENY-001".to_string(),
                    provider: "test".to_string(),
                    action_type: "financial.transfer".to_string(),
                    effect: "deny".to_string(),
                    severity: "critical".to_string(),
                    reason: "Financial transfer is denied.".to_string(),
                    conditions: vec![],
                    source_reference: "Test policy".to_string(),
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
