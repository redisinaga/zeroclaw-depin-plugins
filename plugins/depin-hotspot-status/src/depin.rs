//! Pure logic for `depin-hotspot-status` — testable without WASM.
//! T0: Read-only queries to Helium/Hivemapper public RPC and API.
//!
//! Data sources:
//!   Helium API:    https://api.helium.io/v1/hotspots/{address}
//!   Helium RPC:    https://solana-api.helium.com  (Solana JSON-RPC for reward claims)
//!   Hivemapper:    https://hivemapper.com/api  (public GraphQL endpoints)

use serde::{Deserialize, Serialize};

/// Result from querying a single hotspot
#[derive(Debug, Serialize)]
pub struct HotspotStatus {
    pub address: String,
    pub network: String,
    pub online: bool,
    pub name: String,
    /// Total lifetime rewards in the native token (HNT/MOBILE/IOT/HONEY)
    pub lifetime_rewards: f64,
    /// Rewards earned in last 7 days
    pub rewards_7d: f64,
    /// ISO8601 timestamp of last beacon/heartbeat
    pub last_beacon: Option<String>,
    /// Current hotspot elevation/height (meters, if available)
    pub elevation: Option<i32>,
    /// Gain in dBi
    pub gain: Option<f64>,
    /// Current location
    pub location: Option<String>,
}

/// Query hotspot status from the public Helium API.
///
/// Endpoints used:
///   - https://api.helium.io/v1/hotspots/{address}  (Helium IoT/Mobile)
///   - Hivemapper public API for Hivemapper hotspots
pub fn query_hotspot_status(address: &str, network: &str) -> Result<String, String> {
    match network {
        "helium_iot" | "helium_mobile" => {
            // Construct the Helium API URL
            let url = format!("https://api.helium.io/v1/hotspots/{}", address);

            // In WASM, this would use wasi:http via the host
            // For the pure Rust core, we document the expected response shape
            let _expected_response = serde_json::json!({
                "data": {
                    "address": address,
                    "name": "happy-pink-elephant",
                    "status": {
                        "online": "online",
                        "height": 0
                    },
                    "reward_scale": 1.0,
                    "gain": 12,
                    "elevation": 15,
                    "lat": 37.7749,
                    "lng": -122.4194,
                    "location": "San Francisco, CA"
                }
            });

            // Return instructions for the WASM component
            Ok(serde_json::json!({
                "queried": url,
                "network": network,
                "note": "In WASM: use wasi:http to GET this endpoint. Parse 'data.status.online', sum 'data.rewards' array.",
                "helium_api_url": "https://api.helium.io/v1/hotspots",
                "helium_rewards_url": format!("https://api.helium.io/v1/hotspots/{}/rewards/sum?min_time=-7%20days", address)
            }).to_string())
        }
        "hivemapper" => {
            let url = format!("https://hivemapper.com/api/explorer/hotspot/{}", address);

            Ok(serde_json::json!({
                "queried": url,
                "network": "hivemapper",
                "note": "Hivemapper uses GraphQL. Query { hotspot(address: $addr) { online totalRewards lastHeartbeat } }",
                "hivemapper_api_url": "https://hivemapper.com/api/graphql"
            }).to_string())
        }
        _ => Err(format!("unknown network: {network}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_hotspot_status_helium_iot() {
        let result = query_hotspot_status("112NqN2WWM...", "helium_iot");
        assert!(result.is_ok());
        let parsed: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(parsed["network"], "helium_iot");
    }

    #[test]
    fn test_query_hotspot_status_hivemapper() {
        let result = query_hotspot_status("abc123", "hivemapper");
        assert!(result.is_ok());
    }

    #[test]
    fn test_query_hotspot_status_unknown_network() {
        let result = query_hotspot_status("abc123", "bitcoin");
        assert!(result.is_err());
    }
}
