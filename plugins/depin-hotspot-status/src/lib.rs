//! ZeroClaw WIT tool plugin: `depin_hotspot_status` — T0 (read-only)
//!
//! Queries Helium/Hivemapper hotspot status via public RPC endpoints.
//! Returns: online status, total HNT/MOBILE/IOT rewards, 7-day earnings,
//! last beacon timestamp. No secrets held, no transactions built.
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

    struct DepinHotspotStatus;

    const PLUGIN_NAME: &str = "depin-hotspot-status";
    const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
    const TOOL_NAME: &str = "depin_hotspot_status";

    #[derive(Deserialize)]
    struct ExecuteArgs {
        hotspot_address: String,
        network: String,
        #[serde(rename = "__config", default)]
        config: std::collections::HashMap<String, String>,
    }

    impl PluginInfo for DepinHotspotStatus {
        fn plugin_name() -> String { PLUGIN_NAME.to_string() }
        fn plugin_version() -> String { PLUGIN_VERSION.to_string() }
    }

    impl Tool for DepinHotspotStatus {
        fn name() -> String { TOOL_NAME.to_string() }

        fn description() -> String {
            "Check a DePIN hotspot's status on Helium IoT, Helium Mobile, or Hivemapper. \
             Returns online status, total lifetime rewards, 7-day earnings, and last beacon time. \
             Uses public Helium RPC — no API key required. T0: read-only, no transactions.".to_string()
        }

        fn parameters_schema() -> String {
            serde_json::json!({
                "type": "object",
                "properties": {
                    "hotspot_address": {
                        "type": "string",
                        "description": "The hotspot address (base58 Solana format)"
                    },
                    "network": {
                        "type": "string",
                        "enum": ["helium_iot", "helium_mobile", "hivemapper"],
                        "description": "Which DePIN network to query"
                    }
                },
                "required": ["hotspot_address", "network"]
            }).to_string()
        }

        fn execute(args: String) -> Result<ToolResult, String> {
            let parsed: ExecuteArgs = serde_json::from_str(&args)
                .map_err(|e| format!("invalid arguments: {e}"))?;

            let result = crate::depin::query_hotspot_status(&parsed.hotspot_address, &parsed.network)
                .map_err(|e| format!("query failed: {e}"))?;

            Ok(ToolResult {
                success: true,
                output: result,
                error: None,
            })
        }
    }

    export!(DepinHotspotStatus);
}
