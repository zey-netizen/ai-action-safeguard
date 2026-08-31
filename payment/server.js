import http from "node:http";

const PORT = Number(process.env.PORT || 3000);

const PAYMENT_REQUIREMENTS = {
  scheme: "exact",
  network: process.env.X402_NETWORK || "eip155:84532",
  amount: process.env.X402_AMOUNT || "500000",
  asset: process.env.X402_ASSET || "",
  payTo: process.env.X402_PAY_TO || "",
  maxTimeoutSeconds: Number(
    process.env.X402_MAX_TIMEOUT_SECONDS || 60
  ),
  extra: {
    name: "USDC",
    version: "2"
  }
};

function json(res, status, body) {
  res.writeHead(status, {
    "Content-Type": "application/json",
    "Cache-Control": "no-store"
  });

  res.end(JSON.stringify(body));
}

function parsePaymentSignature(header) {
  if (!header || typeof header !== "string") {
    return null;
  }

  try {
    const decoded = Buffer.from(header, "base64").toString("utf8");
    return JSON.parse(decoded);
  } catch {
    return null;
  }
}

function validatePaymentPayload(payment) {
  const errors = [];

  if (!payment || typeof payment !== "object") {
    return {
      valid: false,
      errors: ["invalid_payment_payload"]
    };
  }

  if (payment.x402Version !== 2) {
    errors.push("invalid_x402_version");
  }

  if (!payment.accepted) {
    errors.push("missing_accepted");
  } else {
    if (payment.accepted.scheme !== PAYMENT_REQUIREMENTS.scheme) {
      errors.push("scheme_mismatch");
    }

    if (payment.accepted.network !== PAYMENT_REQUIREMENTS.network) {
      errors.push("network_mismatch");
    }

    if (
      String(payment.accepted.amount) !==
      String(PAYMENT_REQUIREMENTS.amount)
    ) {
      errors.push("amount_mismatch");
    }

    if (
      PAYMENT_REQUIREMENTS.asset &&
      payment.accepted.asset !== PAYMENT_REQUIREMENTS.asset
    ) {
      errors.push("asset_mismatch");
    }

    if (
      PAYMENT_REQUIREMENTS.payTo &&
      payment.accepted.payTo !== PAYMENT_REQUIREMENTS.payTo
    ) {
      errors.push("payTo_mismatch");
    }
  }

  if (!payment.payload) {
    errors.push("missing_payload");
  } else {
    if (
      typeof payment.payload.signature !== "string" ||
      payment.payload.signature.length === 0
    ) {
      errors.push("missing_signature");
    }

    const authorization = payment.payload.authorization;

    if (!authorization) {
      errors.push("missing_authorization");
    } else {
      if (!authorization.from) {
        errors.push("missing_authorization_from");
      }

      if (!authorization.to) {
        errors.push("missing_authorization_to");
      }

      if (authorization.value === undefined) {
        errors.push("missing_authorization_value");
      }

      if (authorization.validAfter === undefined) {
        errors.push("missing_validAfter");
      }

      if (authorization.validBefore === undefined) {
        errors.push("missing_validBefore");
      }

      if (!authorization.nonce) {
        errors.push("missing_nonce");
      }
    }
  }

  return {
    valid: errors.length === 0,
    errors
  };
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
  if (req.url !== "/verify" || req.method !== "POST") {
    return json(res, 404, {
      error: "not_found"
    });
  }

  const paymentSignature =
    req.headers["payment-signature"];

  if (!paymentSignature) {
    return json(res, 402, {
      error: "payment_required",
      x402Version: 2,
      accepts: [
        PAYMENT_REQUIREMENTS
      ]
    });
  }

  const payment = parsePaymentSignature(paymentSignature);

  if (!payment) {
    return json(res, 400, {
      valid: false,
      status: "invalid_payment_signature",
      message: "PAYMENT-SIGNATURE is not valid base64-encoded JSON."
    });
  }

  const result = validatePaymentPayload(payment);

  if (!result.valid) {
    return json(res, 200, {
      valid: false,
      status: "invalid_payment",
      errors: result.errors
    });
  }

  return json(res, 200, {
    valid: true,
    status: "verified",
    x402Version: 2,
    payer: payment.payload.authorization.from,
    network: payment.accepted.network,
    amount: payment.accepted.amount,
    asset: payment.accepted.asset || null,
    payTo: payment.accepted.payTo || null
  });
});

server.listen(PORT, () => {
  console.log(
    `AI Action Safeguard Payment Verifier running on port ${PORT}`
  );
});
