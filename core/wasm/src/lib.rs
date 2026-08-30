use wasm_bindgen::prelude::*;
use serde_json;

mod risk;
mod memory;
mod policy;

#[wasm_bindgen]
pub fn version() -> String {
    "AI Action Safeguard Core v0.1.0".to_string()
}

#[wasm_bindgen]
pub fn evaluate_risk(input_json: String) -> String {
    let input: risk::RiskInput =
        match serde_json::from_str(&input_json) {
            Ok(value) => value,
            Err(error) => {
                return serde_json::json!({
                    "error": error.to_string()
                }).to_string();
            }
        };

    let result = risk::evaluate(input);

    serde_json::to_string(&result)
        .unwrap_or_else(|error| {
            serde_json::json!({
                "error": error.to_string()
            }).to_string()
        })
}

#[wasm_bindgen]
pub fn calculate_context_deviation(
    plan_json: String,
    action_type: String,
) -> String {
    let plan: memory::AgentPlan =
        match serde_json::from_str(&plan_json) {
            Ok(value) => value,
            Err(error) => {
                return serde_json::json!({
                    "error": error.to_string()
                }).to_string();
            }
        };

    let deviation =
        memory::calculate_deviation(&plan, &action_type);

    serde_json::json!({
        "context_deviation": deviation
    }).to_string()
}

#[wasm_bindgen]
pub fn evaluate_policy(
    ruleset_json: String,
    action_type: String,
) -> String {
    let ruleset: policy::PolicyRuleset =
        match serde_json::from_str(&ruleset_json) {
            Ok(value) => value,
            Err(error) => {
                return serde_json::json!({
                    "error": error.to_string()
                }).to_string();
            }
        };

    let result =
        policy::evaluate(&ruleset, &action_type);

    serde_json::to_string(&result)
        .unwrap_or_else(|error| {
            serde_json::json!({
                "error": error.to_string()
            }).to_string()
        })
}

#[wasm_bindgen]
pub fn evaluate_action(input_json: String) -> String {
    let input: serde_json::Value =
        match serde_json::from_str(&input_json) {
            Ok(value) => value,
            Err(error) => {
                return serde_json::json!({
                    "allowed": false,
                    "error": error.to_string()
                }).to_string();
            }
        };

    let action_type = input["action"]["type"]
        .as_str()
        .unwrap_or("");

    let plan = &input["original_plan"];

    let deviation =
        memory::calculate_deviation_from_json(
            plan,
            action_type
        );

    let risk_input: risk::RiskInput =
        match serde_json::from_value(
            input["risk_parameters"].clone()
        ) {
            Ok(value) => value,
            Err(error) => {
                return serde_json::json!({
                    "allowed": false,
                    "error": error.to_string()
                }).to_string();
            }
        };

    let mut risk_result = risk::evaluate(risk_input);

    let policy_ruleset: policy::PolicyRuleset =
        match serde_json::from_value(
            input["policy_ruleset"].clone()
        ) {
            Ok(value) => value,
            Err(error) => {
                return serde_json::json!({
                    "allowed": false,
                    "error": error.to_string()
                }).to_string();
            }
        };

    let policy_result =
        policy::evaluate(
            &policy_ruleset,
            action_type
        );

    let deviation_score = deviation * 100.0;

    risk_result.risk_score =
        ((risk_result.risk_score * 0.75)
        + (deviation_score * 0.25))
        .clamp(0.0, 100.0);

    risk_result.risk_level =
        if risk_result.risk_score < 25.0 {
            "low".to_string()
        } else if risk_result.risk_score < 50.0 {
            "medium".to_string()
        } else if risk_result.risk_score < 75.0 {
            "high".to_string()
        } else {
            "critical".to_string()
        };

    let allowed =
        policy_result.allowed
        && risk_result.risk_level != "critical";

    let requires_confirmation =
        risk_result.risk_level == "high";

    let mut reasons = risk_result
        .risk_level
        .as_str()
        .to_string();

    if !policy_result.reasons.is_empty() {
        reasons.push_str(": policy violation");
    }

    serde_json::json!({
        "allowed": allowed,
        "risk_score": risk_result.risk_score,
        "risk_level": risk_result.risk_level,
        "context_deviation": deviation,
        "policy_violations": policy_result.violations,
        "reasons": [reasons],
        "requires_confirmation": requires_confirmation
    }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_action_should_be_allowed() {
        let input = serde_json::json!({
            "original_plan": {
                "goal": "Search flights",
                "constraints": [
                    "Do not purchase without confirmation"
                ],
                "allowed_actions": [
                    "search_flights"
                ]
            },
            "action": {
                "type": "search_flights"
            },
            "risk_parameters": {
                "access_scope": "read",
                "financial_impact": 0,
                "irreversibility": 0.0,
                "context_deviation": 0.0
            },
            "policy_ruleset": {
                "rules": []
            }
        });

        let result = evaluate_action(input.to_string());
        let result: serde_json::Value =
            serde_json::from_str(&result).unwrap();

        assert_eq!(result["allowed"], true);
        assert_eq!(result["context_deviation"], 0.0);
    }

    #[test]
    fn unexpected_action_should_have_deviation() {
        let input = serde_json::json!({
            "original_plan": {
                "goal": "Search flights",
                "constraints": [],
                "allowed_actions": [
                    "search_flights"
                ]
            },
            "action": {
                "type": "financial.transfer"
            },
            "risk_parameters": {
                "access_scope": "write",
                "financial_impact": 500,
                "irreversibility": 1.0,
                "context_deviation": 0.0
            },
            "policy_ruleset": {
                "rules": []
            }
        });

        let result = evaluate_action(input.to_string());
        let result: serde_json::Value =
            serde_json::from_str(&result).unwrap();

        assert_eq!(result["context_deviation"], 1.0);
        assert_ne!(result["risk_level"], "low");
    }
}
