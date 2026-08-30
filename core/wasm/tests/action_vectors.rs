use ai_action_safeguard_core::evaluate_action;
use serde_json::Value;
use std::fs;

#[test]
fn run_all_action_test_vectors() {
    let path = format!(
        "{}/test_vectors/action_tests.json",
        env!("CARGO_MANIFEST_DIR")
    );

    let content = fs::read_to_string(&path)
        .expect("Failed to read action_tests.json");

    let suite: Value = serde_json::from_str(&content)
        .expect("action_tests.json contains invalid JSON");

    let tests = suite["tests"]
        .as_array()
        .expect("action_tests.json must contain a tests array");

    assert!(
        !tests.is_empty(),
        "At least one test vector is required"
    );

    for test in tests {
        let name = test["name"]
            .as_str()
            .unwrap_or("unnamed_test");

        let input = &test["input"];
        let expected = &test["expected"];

        let input_string = serde_json::to_string(input)
            .expect("Failed to serialize test input");

        let output_string = evaluate_action(input_string);

        let output: Value = serde_json::from_str(&output_string)
            .unwrap_or_else(|_| {
                panic!(
                    "Test '{}' returned invalid JSON:\n{}",
                    name,
                    output_string
                )
            });

        assert!(
            output.get("error").is_none(),
            "Test '{}' returned an error: {}",
            name,
            output_string
        );

        if let Some(expected_allowed) = expected["allowed"].as_bool() {
            let actual_allowed = output["allowed"]
                .as_bool()
                .unwrap_or(false);

            assert_eq!(
                actual_allowed,
                expected_allowed,
                "Test '{}' failed: allowed mismatch\nOutput: {}",
                name,
                output_string
            );
        }

        if let Some(expected_level) = expected["risk_level"].as_str() {
            let actual_level = output["risk_level"]
                .as_str()
                .unwrap_or("");

            assert_eq!(
                actual_level,
                expected_level,
                "Test '{}' failed: risk_level mismatch\nOutput: {}",
                name,
                output_string
            );
        }

        if let Some(expected_deviation) =
            expected["context_deviation"].as_f64()
        {
            let actual_deviation =
                output["context_deviation"]
                    .as_f64()
                    .unwrap_or(-1.0);

            assert!(
                (actual_deviation - expected_deviation).abs() < 0.000001,
                "Test '{}' failed: context_deviation mismatch. Expected {}, got {}",
                name,
                expected_deviation,
                actual_deviation
            );
        }

        println!(
            "PASS: {} -> allowed={}, risk_level={}",
            name,
            output["allowed"],
            output["risk_level"]
        );
    }
}
