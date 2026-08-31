import test from "node:test";
import assert from "node:assert/strict";
import { Buffer } from "node:buffer";

const BASE_URL = "http://localhost:3000";

test("payment verifier returns 402 without payment", async () => {
  const response = await fetch(`${BASE_URL}/verify`, {
    method: "POST"
  });

  assert.equal(response.status, 402);

  const body = await response.json();

  assert.equal(body.x402Version, 2);
  assert.ok(Array.isArray(body.accepts));
});

test("payment verifier rejects malformed payment", async () => {
  const signature = Buffer.from(
    JSON.stringify({
      x402Version: 1
    })
  ).toString("base64");

  const response = await fetch(`${BASE_URL}/verify`, {
    method: "POST",
    headers: {
      "payment-signature": signature
    }
  });

  assert.equal(response.status, 200);

  const body = await response.json();

  assert.equal(body.valid, false);
  assert.equal(body.status, "invalid_payment");
});

test("payment verifier accepts structurally valid x402 v2 payload", async () => {
  const payment = {
    x402Version: 2,
    accepted: {
      scheme: "exact",
      network: "eip155:84532",
      amount: "500000",
      asset: "",
      payTo: "",
      maxTimeoutSeconds: 60,
      extra: {
        name: "USDC",
        version: "2"
      }
    },
    payload: {
      signature: "0xTEST_SIGNATURE",
      authorization: {
        from: "0x1111111111111111111111111111111111111111",
        to: "0x2222222222222222222222222222222222222222",
        value: "500000",
        validAfter: "0",
        validBefore: "9999999999",
        nonce: "0x1234"
      }
    }
  };

  const signature = Buffer.from(
    JSON.stringify(payment)
  ).toString("base64");

  const response = await fetch(`${BASE_URL}/verify`, {
    method: "POST",
    headers: {
      "payment-signature": signature
    }
  });

  assert.equal(response.status, 200);

  const body = await response.json();

  assert.equal(body.valid, true);
  assert.equal(body.status, "verified");
  assert.equal(
    body.payer,
    "0x1111111111111111111111111111111111111111"
  );
});
