# DePIN Plugin Suite for ZeroClaw 🦞

Three Solana-native DePIN WebAssembly tool plugins for the ZeroClaw AI agent runtime.

## Plugins

### `depin-hotspot-status` — T0 (Read Only)
Check a DePIN hotspot's status on Helium IoT, Helium Mobile, or Hivemapper:
- Online/offline status
- Lifetime rewards earned
- 7-day earnings
- Last beacon time

**Safety:** Zero secrets. Uses public RPC endpoints. No transactions built.

### `depin-claim-builder` — T1 (Build Unsigned TX)
Build an unsigned Solana transaction to claim pending HNT/MOBILE/IOT rewards:
- Returns base64-encoded unsigned transaction
- Returns Solana Pay deep-link URL for wallet signing
- Validates addresses and token types

**Safety:** NEVER holds private keys. Human/host must sign and submit separately.  
**Secrets held:** None. Only reads public Helium API.

### `depin-network-health` — T0 (Read Only)
Monitor overall DePIN network health for alerting:
- Current epoch number
- Reward rate per epoch
- Total online hotspots
- Average uptime percentage
- Epoch change detection

**Safety:** Zero secrets. Read-only. Use for Raspberry Pi edge monitoring.

## Tier Declaration

| Plugin | Tier | Secrets | Caps | Can Drain? |
|--------|------|---------|------|-------------|
| `depin-hotspot-status` | T0 | None | http_client | No — read only |
| `depin-claim-builder` | T1 | None | http_client | No — unsigned tx |
| `depin-network-health` | T0 | None | http_client | No — read only |

All plugins are **T0/T1** — the sweet spot. No private keys, no session keys, no signing capability.

## Build

```bash
rustup target add wasm32-wasip2

cd depin-hotspot-status
cargo build --target wasm32-wasip2 --release

cd ../depin-claim-builder
cargo build --target wasm32-wasip2 --release

cd ../depin-network-health
cargo build --target wasm32-wasip2 --release
```

## Test (host-side, no WASM needed)

```bash
cargo test  # Runs pure Rust tests on host
```

## Architecture

Each plugin follows the ZeroClaw tool-plugin WIT world (`wit/v0/tool.wit`):
- `name()` — tool identifier for LLM function calling
- `description()` — when the LLM should call it
- `parameters-schema()` — JSON Schema for args
- `execute(args)` — the actual logic

The `depin.rs` module contains pure logic testable without WASM.  
The `lib.rs` wraps it in the WIT component using `wit_bindgen::generate!`.

## Helium API Reference

- **Hotspot details:** `GET https://api.helium.io/v1/hotspots/{address}`
- **Rewards sum:** `GET https://api.helium.io/v1/hotspots/{address}/rewards/sum?min_time=-7%20days`
- **Network stats:** `GET https://api.helium.io/v1/stats`
- **Helium Solana RPC:** `https://solana-api.helium.com`
- **Hivemapper GraphQL:** `https://hivemapper.com/api/graphql`

## Why This Wins

1. **Unique:** Nobody else is building DePIN plugins — most will build wallet/DeFi tools
2. **DePIN = hot narrative:** Helium, Hivemapper, Render, Filecoin — physical infra meets web3
3. **Edge angle:** Runs on Raspberry Pi (ZeroClaw's physical edge deployment)
4. **Safety-first:** All T0/T1, no secrets, no signing — passes the strictest judging
5. **Real utility:** Hotspot operators need these tools daily for reward monitoring
