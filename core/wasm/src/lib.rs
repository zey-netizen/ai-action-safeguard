use wasm_bindgen::prelude::*;
use serde_json;

mod risk;

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
                })
                .to_string();
            }
        };

    let result = risk::evaluate(input);

    serde_json::to_string(&result)
        .unwrap_or_else(|error| {
            serde_json::json!({
                "error": error.to_string()
            })
            .to_string()
        })
}
