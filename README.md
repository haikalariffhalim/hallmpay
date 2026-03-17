# HallmPay

HallmPay- Malaysian Payment Gateway API Provider Build with Rust 

[![Crates.io](https://img.shields.io/crates/v/hallmpay)](https://crates.io/crates/hallmpay)
[![Docs.rs](https://docs.rs/hallmpay/badge.svg)](https://docs.rs/hallmpay)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Installation

```toml
[dependencies]
hallmpay = "0.1"
tokio = { version = "1", features = ["full"] }
```

Or with `cargo add`:

```sh
cargo add hallmpay
```

## Quick Start

```rust
use hallmpay::{ HallmPayAPI, ApiVersion, PaymentChannel, PaymentIntentRequest};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HallmPayAPI::builder("your_api_token_here")
        .sandbox(true)
        .api_version(ApiVersion::V3)
        .timeout(Duration::from_secs(60))
        .build()?;

    let request = PaymentIntentRequest {
        payment_channel: PaymentChannel::Fpx as u8,
        order_number: "ORDER-001".to_string(),
        amount: 100.00,
        payer_name: "Ali Yahoo".to_string(),
        payer_email: "ali@yahoo.com".to_string(),
        payer_telephone_number: Some("+60123456789".to_string()),
        currency: Some("MYR".to_string()),
        callback_url: None,
        return_url: None,
        metadata: None,
        checksum: None,
    };

    let intent = client.create_payment_intent(&request).await?;
    println!("Redirect URL: {}", intent.url);

    Ok(())
}
```

## Payment Channels

| ID | Variant | Description |
|----|---------|-------------|
| 1 | `PaymentChannel::Fpx` | FPX Online Banking |
| 2 | `PaymentChannel::ManualTransfer` | Manual Bank Transfer |
| 3 | `PaymentChannel::FpxDirectDebit` | FPX Direct Debit |
| 4 | `PaymentChannel::FpxLineOfCredit` | FPX Line of Credit |
| 5 | `PaymentChannel::DuitnowDobw` | DuitNow Online Banking/Wallets |
| 6 | `PaymentChannel::DuitnowQr` | DuitNow QR |
| 7 | `PaymentChannel::Spaylater` | SPayLater |
| 8 | `PaymentChannel::BoostPayflex` | Boost PayFlex |
| 9 | `PaymentChannel::Qrisob` | QRIS Online Banking |
| 10 | `PaymentChannel::Qriswallet` | QRIS Wallet |
| 11 | `PaymentChannel::Nets` | NETS |
| 12 | `PaymentChannel::CreditCard` | Credit Card |
| 13 | `PaymentChannel::Alipay` | Alipay |
| 14 | `PaymentChannel::Wechatpay` | WeChat Pay |
| 15 | `PaymentChannel::Promptpay` | PromptPay |

## Features

- **API v1** - full support for both API versions with compile-time version guard
- **Sandbox and production** - switch environments with `.sandbox(true)`
- **Checksum generation** - HMAC-SHA256 checksums for payment intents and direct debit
- **Callback verification** - verify incoming webhook payloads from HallmPay
- **Typed errors** - `HallmPayError` variants for 404, 422, 429, timeouts, and version mismatches
- **Async/await** - fully async with `tokio` and `reqwest`

## API Overview

### Client

```rust
// Default client (production, v1, 30s timeout)
let client = HallmPayAPI::new("token")?;

// Custom configuration
let client = HallmPayBuilder::builder("token")
    .sandbox(true)
    .api_version(ApiVersion::V1)
    .timeout(Duration::from_secs(60))
    .build()?;
```

### Payment Intents 

```rust
client.create_payment_intent(&request).await?;
client.get_payment_intent("api_id").await?;   // v3 only
```

### Transactions

```rust
client.get_transaction("tx_id").await?;
client.get_all_transactions(&params).await?;
client.get_transactions_by_order("ORDER-001").await?;
client.get_transactions_by_email("ali@yahoo.com").await?;
client.get_transactions_by_status("paid").await?;
client.get_transactions_by_channel(PaymentChannel::Fpx).await?;
client.get_transaction_by_reference("REF-001").await?;
```

### FPX Banks

```rust
let banks = client.fpx_banks_list().await?;
```

### Portals

```rust
let portals = client.get_portals().await?;
let channels = client.get_channels("portal_key").await?;
```

### FPX Direct Debit

```rust
client.create_fpx_direct_debit_enrollment(&data).await?;
client.create_fpx_direct_debit_maintenance("mandate_id", &data).await?;
client.create_fpx_direct_debit_termination("mandate_id", &data).await?;
client.get_fpx_direct_debit("mandate_id").await?;
client.get_fpx_direct_debit_transaction("tx_id").await?;
```

### Manual Bank Transfer

```rust
client.create_manual_bank_transfer(&data, false).await?;
client.update_manual_bank_transfer_status("ref_no", "paid", "100.00").await?;
```

### Checksum Generation

```rust
use hallmpay::checksum;

let cs = checksum::payment_intent(
    "secret_key",
    PaymentChannel::Fpx as u8,
    "ORDER-001",
    100.00,
    "Ali Yahoo",
    "ali@yahoo.com",
);
```

### Callback Verification

```rust
use hallmpay::verification;

let is_valid = verification::verify_callback("secret_key", &callback_data);
```

For full API documentation, see [docs.rs/hallmpay](https://docs.rs/hallmpay-api).

## CLI Usage

Install the CLI and MCP server:

```sh
cargo install hallmpay
```

This installs two binaries: `hallmpay` (CLI) and `hallmpay-mcp` (MCP server).

### Configuration

```sh
# Create config file
hallmpay init

# Or set environment variables
export HALLM_TOKEN="your_api_token"
export HALLM_SECRET_KEY="your_secret_key"
export HALLM_SANDBOX=true
export HALLM_API_VERSION=v1
```

Config precedence: CLI flags > environment variables > `~/.hallmpay/config.toml`

### Examples

```sh
# List FPX banks
hallmpay banks list

# Create a payment intent
hallmpay payment create --channel 1 --order ORD-001 --amount 100.00 --name "Ali" --email "ali@yahoo.com"

# Get a transaction
hallmpay transaction get tx_123

# Generate checksum
hallmpay checksum payment --secret sk_test --channel 1 --order ORD-001 --amount 100 --name "Ali" --email "ali@yahoo.com"

# Verify callback (JSON as argument)
hallmpay verify transaction '{"id":"1","checksum":"abc..."}' --secret sk_test

# Use sandbox environment
hallmpay --sandbox banks list

# Use experimental API v2 
hallmpay --experimental --api v2 transaction list --status success
```

All commands output JSON to stdout. Errors go to stderr.

## MCP Server (for AI Agents)

The `hallmpay-mcp` binary exposes HallmPay operations as MCP tools via stdio transport.

### Claude Desktop Configuration

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "hallmpay-mcp": {
      "command": "hallmpay-mcp",
      "env": {
        "HALLM_TOKEN": "your_token",
        "HALLM_SECRET_KEY": "your_secret"
      }
    }
  }
}
```

### Available Tools

| Tool | Description |
|------|-------------|
| `create_payment` | Create a payment intent |
| `get_payment` | Get payment intent by ID (v2) |
| `get_transaction` | Get transaction by ID |
| `list_transactions` | List transactions with filters (v2) |
| `list_banks` | List FPX banks |
| `list_portals` | List portals |
| `get_channels` | Get portal payment channels |
| `generate_checksum` | Generate HMAC-SHA256 checksum |
| `verify_callback` | Verify callback signature |
| `create_mandate` | Create DD enrollment |
| `update_mandate` | Update DD mandate |
| `get_mandate` | Get mandate details |
| `get_mandate_transaction` | Get mandate transaction |

## License

MIT - see [LICENSE](LICENSE).
