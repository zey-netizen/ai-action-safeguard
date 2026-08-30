import http from "node:http";

const PORT = process.env.PORT || 3000;

const PAYMENT_REQUIREMENTS = {
  x402Version: 2,
  scheme: "exact",
  network: "eip155:84532",
  amount: "500000",
  asset: "USDC",
  description: "AI Action Safeguard verification"
};

const server = http.createServer((req, res) => {
  if (req.url !== "/verify" || req.method !== "POST") {
    res.writeHead(404, {
      "Content-Type": "application/json"
    });

    res.end(JSON.stringify({
      error: "Not found"
    }));

    return;
  }

  const paymentSignature =
    req.headers["payment-signature"];

  if (!paymentSignature) {
    res.writeHead(402, {
      "Content-Type": "application/json"
    });

    res.end(JSON.stringify({
      error: "Payment Required",
      payment_requirements: PAYMENT_REQUIREMENTS
    }));

    return;
  }

  res.writeHead(200, {
    "Content-Type": "application/json"
  });

  res.end(JSON.stringify({
    valid: false,
    status: "verification_not_implemented",
    message: "Real x402 verification will be connected in the next payment stage."
  }));
});

server.listen(PORT, () => {
  console.log(
    `AI Action Safeguard Payment Verifier running on port ${PORT}`
  );
});
