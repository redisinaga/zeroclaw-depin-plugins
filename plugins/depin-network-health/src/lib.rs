//! ZeroClaw WIT tool plugin: `depin_network_health` — T0 (read-only)
//!
//! Monitors DePIN network health metrics: current epoch, reward rate,
//! online hotspot count, average uptime, epoch change alerts.
//! Supports Helium IoT, Helium Mobile, and Hivemapper.
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

    struct DepinNetworkHealth;

    const PLUGIN_NAME: &str = "depin-network-health";
    const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
    const TOOL_NAME: &str = "depin_network_health";

    #[derive(Deserialize)]
    struct ExecuteArgs {
        network: String,
        #[serde(rename = "__config", default)]
        config: std::collections::HashMap<String, String>,
    }

    impl PluginInfo for DepinNetworkHealth {
        fn plugin_name() -> String { PLUGIN_NAME.to_string() }
        fn plugin_version() -> String { PLUGIN_VERSION.to_string() }
    }

    impl Tool for DepinNetworkHealth {
        fn name() -> String { TOOL_NAME.to_string() }

        fn description() -> String {
            "Monitor DePIN network health and metrics. Returns current epoch number, \
             reward rate per epoch, total online hotspots, average uptime percentage, \
             and recent epoch change timestamps. Supports Helium IoT, Helium Mobile, \
             and Hivemapper. T0: read-only, no transactions. Use for alerting on \
             epoch changes, reward rate drops, or mass offline events.".to_string()
        }

        fn parameters_schema() -> String {
            serde_json::json!({
                "type": "object",
                "properties": {
                    "network": {
                        "type": "string",
                        "enum": ["helium_iot", "helium_mobile", "hivemapper"],
                        "description": "Which DePIN network to monitor"
                    }
                },
                "required": ["network"]
            }).to_string()
        }

        fn execute(args: String) -> Result<ToolResult, String> {
            let parsed: ExecuteArgs = serde_json::from_str(&args)
                .map_err(|e| format!("invalid arguments: {e}"))?;

            let result = crate::depin::query_network_health(&parsed.network)
                .map_err(|e| format!("query failed: {e}"))?;

            Ok(ToolResult {
                success: true,
                output: result,
                error: None,
            })
        }
    }

    export!(DepinNetworkHealth);
}
