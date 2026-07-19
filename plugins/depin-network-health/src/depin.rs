//! Pure logic for `depin-network-health` — testable without WASM.
//! T0: Monitors DePIN network health via public RPC/API endpoints.
//!
//! Metrics tracked:
//!   - Current epoch number
//!   - Reward rate per epoch (tokens distributed)
//!   - Total online hotspots
//!   - Average uptime percentage
//!   - Epoch change timestamps (for alerting)
//!
//! Use case: An AI agent running on a Raspberry Pi watches these metrics
//! and alerts when reward rates drop or hotspots go offline en masse.

use serde::Serialize;

/// Network health snapshot
#[derive(Debug, Serialize)]
pub struct NetworkHealth {
    pub network: String,
    pub current_epoch: u64,
    pub total_online_hotspots: u64,
    pub total_hotspots: u64,
    pub avg_uptime_pct: f64,
    pub reward_rate_per_epoch: f64,
    pub reward_token: String,
    pub last_epoch_change: Option<String>,
    pub next_epoch_estimate: Option<String>,
    pub alerts: Vec<String>,
}

/// Query network health from Helium public stats API.
///
/// Endpoints:
///   - https://api.helium.io/v1/stats          (global network stats)
///   - https://api.helium.io/v1/hotspots/count (total hotspots)
pub fn query_network_health(network: &str) -> Result<String, String> {
    let (api_url, reward_token) = match network {
        "helium_iot" => ("https://api.helium.io/v1/stats", "IOT"),
        "helium_mobile" => ("https://api.helium.io/v1/stats", "MOBILE"),
        "hivemapper" => ("https://hivemapper.com/api/explorer/stats", "HONEY"),
        _ => return Err(format!("unknown network: {network}")),
    };

    Ok(serde_json::json!({
        "network": network,
        "current_epoch": "PLACEHOLDER",
        "total_online_hotspots": "PLACEHOLDER",
        "total_hotspots": "PLACEHOLDER",
        "avg_uptime_pct": "PLACEHOLDER",
        "reward_rate_per_epoch": "PLACEHOLDER",
        "reward_token": reward_token,
        "last_epoch_change": "PLACEHOLDER",
        "next_epoch_estimate": "PLACEHOLDER",
        "alerts": [],
        "note": "In WASM: use wasi:http to GET the stats endpoint. Parse hot spot count, epoch info, and recent reward distributions. Compare against thresholds to generate alerts.",
        "helium_stats_url": api_url,
        "helium_hotspot_count_url": format!("https://api.helium.io/v1/hotspots/count")
    }).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_network_health_iot() {
        let result = query_network_health("helium_iot");
        assert!(result.is_ok());
        let parsed: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(parsed["network"], "helium_iot");
        assert_eq!(parsed["reward_token"], "IOT");
    }

    #[test]
    fn test_query_network_health_mobile() {
        let result = query_network_health("helium_mobile");
        assert!(result.is_ok());
        let parsed: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(parsed["reward_token"], "MOBILE");
    }

    #[test]
    fn test_query_network_health_hivemapper() {
        let result = query_network_health("hivemapper");
        assert!(result.is_ok());
    }

    #[test]
    fn test_query_network_health_invalid() {
        assert!(query_network_health("ethereum").is_err());
    }
}
