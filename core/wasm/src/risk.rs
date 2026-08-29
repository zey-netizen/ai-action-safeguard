use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RiskInput {
    pub access_scope: String,
    pub financial_impact: f64,
    pub irreversibility: f64,
    pub context_deviation: f64,
}

#[derive(Debug, Serialize)]
pub struct RiskResult {
    pub risk_score: f64,
    pub risk_level: String,
}

pub fn evaluate(input: RiskInput) -> RiskResult {
    let access_score = match input.access_scope.to_lowercase().as_str() {
        "read" => 10.0,
        "write" => 50.0,
        "delete" => 80.0,
        "admin" => 100.0,
        _ => 100.0,
    };

    let financial_score = (input.financial_impact / 1000.0 * 100.0)
        .clamp(0.0, 100.0);

    let irreversibility_score =
        input.irreversibility.clamp(0.0, 1.0) * 100.0;

    let deviation_score =
        input.context_deviation.clamp(0.0, 1.0) * 100.0;

    let risk_score =
        (access_score * 0.25)
        + (financial_score * 0.25)
        + (irreversibility_score * 0.25)
        + (deviation_score * 0.25);

    let risk_level = if risk_score < 25.0 {
        "low"
    } else if risk_score < 50.0 {
        "medium"
    } else if risk_score < 75.0 {
        "high"
    } else {
        "critical"
    };

    RiskResult {
        risk_score,
        risk_level: risk_level.to_string(),
    }
}
