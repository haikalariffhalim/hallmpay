#HALLMPAY CLI + MCP Server - Design Document

**Date:** 2026-02-25
**Status:** Approved

## Overview

Ads directdebit a CLI binary (`hallmpay`) and MCP server binary (`hallmpay-mcp`) to the existing HallmPay sdk crate. Both share the same library code. Designed for AI agent usage with JSON output.

## Decisions

| Aspect | Decision |
|--------|----------|
| CLI framework | clap (derive) |
| MCP framework | rmcp |
| MCP transport | stdio |
| Output format | JSON to stdout, errors to stderr |
| Config | ~/.hallmpay/config.toml + env vars + CLI flags |
| Config precedence | CLI flags > env vars > config file |
| Binary targets | `hallmpay-cli` (CLI), `hallmpay-mcp` (MCP server) |

## CLI Commands

```
hallmpay [--sandbox] [--api-version v1|v2]

  init                              # Create config interactively
  payment create [flags]            # Create payment intent
  payment get <id>                  # Get payment intent (v2)
  transaction get <id>              # Get transaction
  transaction list [filters]        # List transactions (v2)
  banks list                        # List FPX banks
  portal list                       # List portals
  portal channels <key>             # List portal channels
  checksum payment [flags]          # Generate payment checksum
  checksum enrollment [flags]       # Generate directdebit enrollment checksum
  checksum maintenance [flags]      # Generate directdebit maintenance checksum
  verify transaction <json|stdin>   # Verify callback checksums
  verify pre-transaction <json>
  verify return-url <json>
  verify directdebit-approval <json>
  verify directdebit-authorization <json>
  verify directdebit-transaction <json>
  mandate create <json>             # direct debit enrollment
  mandate update <id> <json>        # direct debit maintenance
  mandate terminate <id> <json>     # direct debit termination
  mandate get <id>                  # Get mandate
  mandate transaction <id>          # Get mandate transaction
```

## MCP Server Tools

13 tools exposed via stdio transport:
- create_payment_intent, get_payment_intent
- get_transaction, list_transactions
- list_banks, list_portals, get_channels
- generate_checksum, verify_callback
- create_mandate, update_mandate, terminate_mandate, get_mandate

## Config File

`~/.hallmpay/config.toml`:

```toml
token = "your_api_token"
secret_key = "your_secret_key"
sandbox = false
api_version = "v1"
```

Env vars: HALLMPAY_TOKEN, HALLMPAY_SECRET_KEY, HALLMPAY_SANDBOX, HALLMPAY_API_VERSION

## New Dependencies

- clap (derive feature) - CLI argument parsing
- rmcp (transport-stdio feature) - MCP server
- dirs - find home directory for config
- toml - parse config file
- serde (already present)
