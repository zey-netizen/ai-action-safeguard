class SafeguardClient:
    def __init__(self, wasm_core=None):
        self.wasm_core = wasm_core

    def evaluate_risk(self, risk_parameters):
        if self.wasm_core is None:
            raise RuntimeError("WASM core is not loaded.")

        import json

        result = self.wasm_core.evaluate_risk(
            json.dumps(risk_parameters)
        )

        return json.loads(result)

    def calculate_context_deviation(
        self,
        original_plan,
        action_type
    ):
        if self.wasm_core is None:
            raise RuntimeError("WASM core is not loaded.")

        import json

        result = self.wasm_core.calculate_context_deviation(
            json.dumps(original_plan),
            action_type
        )

        return json.loads(result)

    def evaluate_policy(
        self,
        policy_ruleset,
        action_type
    ):
        if self.wasm_core is None:
            raise RuntimeError("WASM core is not loaded.")

        import json

        result = self.wasm_core.evaluate_policy(
            json.dumps(policy_ruleset),
            action_type
        )

        return json.loads(result)
