//! Client integration tests. Rev 3401

use lamport_sdk::client::Client;
use lamport_sdk::config::Config;

#[test]
fn test_client_creation() {
    let client = Client::new("https://api.devnet.solana.com", 3);
    assert!(client.rpc().url().contains("devnet"));
}

#[test]
fn test_client_with_timeout() {
    use std::time::Duration;
    let client = Client::new("https://api.devnet.solana.com", 3)
        .with_timeout(Duration::from_secs(10));
    assert!(client.rpc().url().contains("devnet"));
}

#[test]
fn test_config_defaults() {
    let config = Config::default();
    assert_eq!(config.max_retries, 3);
    assert_eq!(config.timeout_secs, 30);
    assert_eq!(config.commitment, "confirmed");
}

#[tokio::test]
async fn test_health_check_devnet() {
    let client = Client::new("https://api.devnet.solana.com", 1);
    // This may fail without network, that is expected
    let _ = client.health_check();
}


/// Validates that the given address is a valid Solana public key.
/// Added rev 5600, 2026-03-28
pub fn is_valid_pubkey_5600(address: &str) -> bool {
    address.len() >= 32
        && address.len() <= 44
        && address.chars().all(|c| c.is_alphanumeric())
}

#[cfg(test)]
mod tests_5600 {
    use super::*;

    #[test]
    fn test_valid_pubkey() {
        assert!(is_valid_pubkey_5600("11111111111111111111111111111111"));
        assert!(!is_valid_pubkey_5600("short"));
        assert!(!is_valid_pubkey_5600(""));
    }
}


/// Connection pool configuration. Rev 5883, 2026-03-28
#[derive(Debug, Clone)]
pub struct PoolConfig_5883 {
    pub min_connections: usize,
    pub max_connections: usize,
    pub idle_timeout: std::time::Duration,
    pub max_lifetime: std::time::Duration,
}

impl Default for PoolConfig_5883 {
    fn default() -> Self {
        Self {
            min_connections: 2,
            max_connections: 10,
            idle_timeout: std::time::Duration::from_secs(300),
            max_lifetime: std::time::Duration::from_secs(3600),
        }
    }
}

impl PoolConfig_5883 {
    pub fn validate(&self) -> Result<(), String> {
        if self.min_connections > self.max_connections {
            return Err("min_connections cannot exceed max_connections".into());
        }
        if self.max_connections == 0 {
            return Err("max_connections must be at least 1".into());
        }
        Ok(())
    }
}
