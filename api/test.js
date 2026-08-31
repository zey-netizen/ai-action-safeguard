import test from "node:test";
import assert from "node:assert/strict";
import { spawn } from "node:child_process";

let server;

test.before(async () => {
  server = spawn(
    process.execPath,
    ["server.js"],
    {
      cwd: new URL(".", import.meta.url).pathname,
      stdio: "ignore"
    }
  );

  await new Promise(resolve => setTimeout(resolve, 1000));
});

test.after(() => {
  if (server) {
    server.kill();
  }
});

test("health endpoint works", async () => {
  const response = await fetch(
    "http://127.0.0.1:8080/health"
  );

  assert.equal(response.status, 200);

  const body = await response.json();

  assert.equal(body.status, "ok");
});

test("evaluate endpoint blocks denied policy action", async () => {
  const input = {
    original_plan: {
      goal: "Search flights",
      constraints: [
        "Do not purchase anything"
      ],
      allowed_actions: [
        "search_flights"
      ]
    },

    action: {
      type: "financial.transfer"
    },

    risk_parameters: {
      access_scope: "write",
      financial_impact: 500,
      irreversibility: 1.0,
      context_deviation: 0.0
    },

    policy_ruleset: {
      provider: "test",
      version: "1.0.0",
      updated_at: "2026-08-30",
      source: "test",
      rules: [
        {
          id: "TEST-FINANCIAL-001",
          provider: "test",
          action_type: "financial.transfer",
          effect: "deny",
          severity: "critical",
          reason: "Financial transfer is prohibited.",
          conditions: [],
          source_reference: "Test policy"
        }
      ]
    }
  };

  const response = await fetch(
    "http://127.0.0.1:8080/v1/evaluate",
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(input)
    }
  );

  assert.equal(response.status, 200);

  const body = await response.json();

  assert.equal(body.allowed, false);
  assert.equal(body.policy_violations.length, 1);
});
