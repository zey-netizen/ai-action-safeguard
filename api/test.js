import test from "node:test";
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { fileURLToPath } from "node:url";
import path from "node:path";

const API_DIR = path.dirname(fileURLToPath(import.meta.url));
const SERVER_URL = "http://127.0.0.1:8080";

let server;

function waitForServer(url, timeout = 15000) {
  return new Promise((resolve, reject) => {
    const start = Date.now();

    const check = async () => {
      try {
        const response = await fetch(url);

        if (response.ok) {
          resolve();
          return;
        }
      } catch {}

      if (Date.now() - start >= timeout) {
        reject(new Error("API server did not start within timeout"));
        return;
      }

      setTimeout(check, 250);
    };

    check();
  });
}

test.before(async () => {
  server = spawn(
    process.execPath,
    ["server.js"],
    {
      cwd: API_DIR,
      env: {
        ...process.env,
        PORT: "8080",
        AIS_API_KEY: "test-secret-key"
      },
      stdio: "inherit"
    }
  );

  await waitForServer(`${SERVER_URL}/health`);
});

test.after(() => {
  if (server && !server.killed) {
    server.kill("SIGTERM");
  }
});

test("health endpoint works", async () => {
  const response = await fetch(
    `${SERVER_URL}/health`
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
  `${SERVER_URL}/v1/evaluate`,
  {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "Authorization": "Bearer test-secret-key"
    },
      body: JSON.stringify(input)
    }
  );

  assert.equal(response.status, 200);

  const body = await response.json();

  assert.equal(body.allowed, false);
  assert.equal(body.policy_violations.length, 1);
});

assert.equal(
  typeof body.request_id,
  "string"
);

assert.ok(
  body.request_id.length > 0
);
