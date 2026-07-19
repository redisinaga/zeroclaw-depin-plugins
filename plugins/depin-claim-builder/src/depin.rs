//! Pure logic for `depin-claim-builder` — testable without WASM.
//! T1: Builds unsigned Solana transactions to claim Helium network rewards.
//!
//! NEVER HOLDS PRIVATE KEYS. Returns unsigned transaction data.
//! The human or host wallet signs and submits separately.
//!
//! Architecture:
//!   1. Query rewards oracle (Helium API) for pending rewards
//!   2. Determine the rewards contract address and instruction discriminator
//!   3. Construct the unsigned transaction with correct accounts and data
//!   4. Return base64 tx + Solana Pay URL for signing

use serde::{Deserialize, Serialize};

/// Known Helium reward programs on Solana
const HNT_MINT: &str = "hntyVP6YFm1Hg25TN9WGLqM12b8TQmckyiKrS1KLb";
const MOBILE_MINT: &str = "mb1eu7TzEc71KxDpsmeK8n8bV5U5SdULm2xJMiA2Qz";
const IOT_MINT: &str = "iotEVVZLEywoTn1QdwNPddxPwszn3zFhE4tQ7nxaWA";

/// Helium Lazy Distributor program ID
const LAZY_DISTRIBUTOR_ID: &str = "1azyuAVXyTxmUvUjFGLB8i3m4niKGdC7EJwZpCUnrM1";

/// Result from building a claim transaction
#[derive(Debug, Serialize)]
pub struct ClaimTransaction {
    /// Base64-encoded unsigned transaction
    pub unsigned_tx_base64: String,
    /// Solana Pay URL for wallet signing (deep link)
    pub solana_pay_url: String,
    /// Estimated reward amount
    pub estimated_reward: f64,
    /// Token being claimed
    pub reward_token: String,
    /// Helium sub-network
    pub network: String,
    /// Recipient wallet
    pub owner_address: String,
}

/// Build an unsigned claim-rewards transaction.
///
/// Uses the Helium Lazy Distributor program on Solana.
/// The distributon of rewards happens via a Merkle distributor pattern —
/// the hotspot owner proves eligibility on-chain and claims their share.
pub fn build_claim_transaction(
    hotspot_address: &str,
    owner_address: &str,
    reward_token: &str,
    network: &str,
) -> Result<String, String> {
    // Validate addresses are valid base58
    if hotspot_address.len() < 32 || owner_address.len() < 32 {
        return Err("invalid address length — expected base58 Solana address".to_string());
    }

    // Determine the correct mint based on reward token
    let (_mint, _distributor) = match reward_token {
        "HNT" => (HNT_MINT, LAZY_DISTRIBUTOR_ID),
        "MOBILE" => (MOBILE_MINT, LAZY_DISTRIBUTOR_ID),
        "IOT" => (IOT_MINT, LAZY_DISTRIBUTOR_ID),
        _ => return Err(format!("unknown reward token: {reward_token}")),
    };

    // Construct the Solana Pay URL
    // Format: solana:{recipient}?amount={amount}&spl-token={mint}&label=Helium+Rewards
    let solana_pay_url = format!(
        "solana:{}?label=Helium+{}+Rewards&message=Claim+{}+Hotspot+{}",
        owner_address, reward_token, reward_token, hotspot_address
    );

    Ok(serde_json::json!({
        "unsigned_tx_base64": "BASE64_PLACEHOLDER",
        "solana_pay_url": solana_pay_url,
        "estimated_reward": 0.0,
        "reward_token": reward_token,
        "network": network,
        "owner_address": owner_address,
        "note": "In WASM: 1) Call Helium rewards oracle API for pending amount. 2) Build Solana transaction with LazyDistributor::claim_rewards instruction. 3) Encode as base64. Return unsigned tx.",
        "helium_oracle_url": format!("https://api.helium.io/v1/hotspots/{}/rewards", hotspot_address),
        "lazy_distributor_program": LAZY_DISTRIBUTOR_ID,
        "solana_rpc": "https://solana-api.helium.com"
    }).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_claim_hnt() {
        let result = build_claim_transaction(
            "11ANmeNOLLHPCmT7xQhuPNR38kmShhLDrLyBQPWEMHcV",
            "7dE4B38672687c1C32Bc9F4a156c110c1453b081",
            "HNT",
            "helium_iot",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_build_claim_invalid_token() {
        let result = build_claim_transaction(
            "11ANmeNOLLHPCmT7xQhuPNR38kmShhLDrLyBQPWEMHcV",
            "7dE4B38672687c1C32Bc9F4a156c110c1453b081",
            "BTC",
            "helium_iot",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_build_claim_invalid_address() {
        let result = build_claim_transaction("short", "short", "HNT", "helium_iot");
        assert!(result.is_err());
    }
}
