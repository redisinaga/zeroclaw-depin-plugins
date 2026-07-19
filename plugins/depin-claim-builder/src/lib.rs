//! ZeroClaw WIT tool plugin: `depin_claim_builder` — T1 (build unsigned tx)
//!
//! Builds an unsigned Solana transaction to claim pending Helium rewards
//! (HNT/MOBILE/IOT). Returns base64-encoded unsigned transaction + Solana
//! Pay URL. NEVER holds private keys. Human or host signs separately.
//!
//! Build: rustup target add wasm32-wasip2
//!        cargo build --target wasm32-wasip2 --release

pub mod depin;

#[cfg(target_family = "wasm")]
mod component {
    wit_bindgen::generate!({
        path: "../../wit/v0",
        world: "tool-plugin",
        features: ["plugins-wit-v0"],
    });

    use serde::Deserialize;
    use exports::zeroclaw::plugin::plugin_info::Guest as PluginInfo;
    use exports::zeroclaw::plugin::tool::{Guest as Tool, ToolResult};

    struct DepinClaimBuilder;

    const PLUGIN_NAME: &str = "depin-claim-builder";
    const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
    const TOOL_NAME: &str = "depin_claim_builder";

    #[derive(Deserialize)]
    struct ExecuteArgs {
        hotspot_address: String,
        owner_address: String,
        reward_token: String,
        network: String,
        #[serde(rename = "__config", default)]
        config: std::collections::HashMap<String, String>,
    }

    impl PluginInfo for DepinClaimBuilder {
        fn plugin_name() -> String { PLUGIN_NAME.to_string() }
        fn plugin_version() -> String { PLUGIN_VERSION.to_string() }
    }

    impl Tool for DepinClaimBuilder {
        fn name() -> String { TOOL_NAME.to_string() }

        fn description() -> String {
            "Build an unsigned Solana transaction to claim pending HNT/MOBILE/IOT rewards \
             from a Helium hotspot. Returns a base64-encoded unsigned transaction and a \
             Solana Pay URL for the owner to sign. T1: builds tx only — NEVER holds keys. \
             Owner must sign and submit separately.".to_string()
        }

        fn parameters_schema() -> String {
            serde_json::json!({
                "type": "object",
                "properties": {
                    "hotspot_address": {
                        "type": "string",
                        "description": "Helium hotspot address (base58 Solana)"
                    },
                    "owner_address": {
                        "type": "string",
                        "description": "Hotspot owner's Solana wallet (rewards destination)"
                    },
                    "reward_token": {
                        "type": "string",
                        "enum": ["HNT", "MOBILE", "IOT"],
                        "description": "Which reward token to claim"
                    },
                    "network": {
                        "type": "string",
                        "enum": ["helium_iot", "helium_mobile"],
                        "description": "Helium sub-network"
                    }
                },
                "required": ["hotspot_address", "owner_address", "reward_token", "network"]
            }).to_string()
        }

        fn execute(args: String) -> Result<ToolResult, String> {
            let parsed: ExecuteArgs = serde_json::from_str(&args)
                .map_err(|e| format!("invalid arguments: {e}"))?;

            let result = crate::depin::build_claim_transaction(
                &parsed.hotspot_address,
                &parsed.owner_address,
                &parsed.reward_token,
                &parsed.network,
            ).map_err(|e| format!("build failed: {e}"))?;

            Ok(ToolResult {
                success: true,
                output: result,
                error: None,
            })
        }
    }

    export!(DepinClaimBuilder);
}
