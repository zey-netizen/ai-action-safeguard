import http from "node:http";
import { evaluate_action } from "./wasm/ai_action_safeguard_core.js";

const PORT = Number(process.env.PORT || 8080);

function sendJson(res, status, body) {
  res.writeHead(status, {
    "Content-Type": "application/json",
    "Cache-Control": "no-store"
  });

  res.end(JSON.stringify(body));
}

function readBody(req) {
  return new Promise((resolve, reject) => {
    let body = "";

    req.on("data", chunk => {
      body += chunk;

      if (body.length > 1024 * 1024) {
        reject(new Error("request_too_large"));
        req.destroy();
      }
    });

    req.on("end", () => resolve(body));
    req.on("error", reject);
  });
}

const server = http.createServer(async (req, res) => {
  if (req.method === "GET" && req.url === "/health") {
    return sendJson(res, 200, {
      status: "ok",
      service: "ai-action-safeguard",
      version: "1.0.0"
    });
  }

  if (req.method === "GET" && req.url === "/") {
    return sendJson(res, 200, {
      name: "AI Action Safeguard",
      version: "1.0.0",
      endpoints: [
        "GET /health",
        "POST /v1/evaluate"
      ]
    });
  }

  if (req.method !== "POST" || req.url !== "/v1/evaluate") {
    return sendJson(res, 404, {
      error: "not_found"
    });
  }

  try {
    const body = await readBody(req);

    let input;

    try {
      input = JSON.parse(body);
    } catch {
      return sendJson(res, 400, {
        allowed: false,
        error: "invalid_json"
      });
    }

    if (
      !input ||
      typeof input !== "object" ||
      !input.action ||
      !input.original_plan ||
      !input.risk_parameters ||
      !input.policy_ruleset
    ) {
      return sendJson(res, 400, {
        allowed: false,
        error: "invalid_evaluation_input"
      });
    }

    const result = evaluate_action(
      JSON.stringify(input)
    );

    let output;

    try {
      output = JSON.parse(result);
    } catch {
      return sendJson(res, 500, {
        allowed: false,
        error: "invalid_core_response"
      });
    }

    return sendJson(res, 200, output);
  } catch (error) {
    return sendJson(res, 500, {
      allowed: false,
      error: error.message || "internal_error"
    });
  }
});

server.listen(PORT, () => {
  console.log(
    `AI Action Safeguard API running on port ${PORT}`
  );
});
