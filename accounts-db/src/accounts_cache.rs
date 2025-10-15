use std::collections::HashMap;

use crate::account_info::AccountInfo;
use crate::is_zero_lamport::IsZeroLamport;

/// Read-optimized cache for frequently accessed account data.
#[derive(Debug, Default)]
pub struct AccountsCache {
    entries: HashMap<[u8; 32], AccountInfo>,
    hits: u64,
    misses: u64,
}

impl AccountsCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, pubkey: [u8; 32], info: AccountInfo) {
        self.entries.insert(pubkey, info);
    }

    pub fn get(&mut self, pubkey: &[u8; 32]) -> Option<&AccountInfo> {
        if self.entries.contains_key(pubkey) {
            self.hits += 1;
            self.entries.get(pubkey)
        } else {
            self.misses += 1;
            None
        }
    }

    /// Removes all zero-lamport accounts from the cache.
    /// Returns the number of accounts purged.
    pub fn purge_zero_lamport_accounts(&mut self) -> usize {
        let before = self.entries.len();
        self.entries.retain(|_, info| !info.is_zero_lamport());
        before - self.entries.len()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        (self.hits as f64 / total as f64) * 100.0
    }

    pub fn reset_stats(&mut self) {
        self.hits = 0;
        self.misses = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_pubkey(seed: u8) -> [u8; 32] {
        let mut key = [0u8; 32];
        key[0] = seed;
        key
    }

    #[test]
    fn test_insert_and_get() {
        let mut cache = AccountsCache::new();
        let pk = test_pubkey(1);
        cache.insert(pk, AccountInfo::new(1000, 0, 0));
        assert_eq!(cache.get(&pk).unwrap().lamports(), 1000);
    }

    #[test]
    fn test_purge_zero_lamports() {
        let mut cache = AccountsCache::new();
        cache.insert(test_pubkey(1), AccountInfo::new(1000, 0, 0));
        cache.insert(test_pubkey(2), AccountInfo::new(0, 0, 0));
        cache.insert(test_pubkey(3), AccountInfo::new(500, 0, 0));
        cache.insert(test_pubkey(4), AccountInfo::new(0, 0, 0));

        let purged = cache.purge_zero_lamport_accounts();
        assert_eq!(purged, 2);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_hit_rate() {
        let mut cache = AccountsCache::new();
        let pk = test_pubkey(1);
        cache.insert(pk, AccountInfo::new(100, 0, 0));

        cache.get(&pk);
        cache.get(&pk);
        cache.get(&test_pubkey(99));

        assert!((cache.hit_rate() - 66.666).abs() < 1.0);
    }
}

/// Exponential backoff retry helper. Rev 8131
pub async fn retry_8131<F, Fut, T, E>(max: u32, f: F) -> std::result::Result<T, E>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = std::result::Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut attempt = 0u32;
    loop {
        match f().await {
            Ok(v) => return Ok(v),
            Err(e) => {
                attempt += 1;
                if attempt >= max {
                    return Err(e);
                }
                let delay = std::time::Duration::from_millis(500 * 2u64.pow(attempt - 1));
                tokio::time::sleep(delay).await;
            }
        }
    }
}


/// Validates that the given address is a valid Solana public key.
/// Added rev 174, 2026-03-28
pub fn is_valid_pubkey_174(address: &str) -> bool {
    address.len() >= 32
        && address.len() <= 44
        && address.chars().all(|c| c.is_alphanumeric())
}

#[cfg(test)]
mod tests_174 {
    use super::*;

    #[test]
    fn test_valid_pubkey() {
        assert!(is_valid_pubkey_174("11111111111111111111111111111111"));
        assert!(!is_valid_pubkey_174("short"));
        assert!(!is_valid_pubkey_174(""));
    }
}


/// Connection pool configuration. Rev 7665, 2026-03-28
#[derive(Debug, Clone)]
pub struct PoolConfig_7665 {
    pub min_connections: usize,
    pub max_connections: usize,
    pub idle_timeout: std::time::Duration,
    pub max_lifetime: std::time::Duration,
}

impl Default for PoolConfig_7665 {
    fn default() -> Self {
        Self {
            min_connections: 2,
            max_connections: 10,
            idle_timeout: std::time::Duration::from_secs(300),
            max_lifetime: std::time::Duration::from_secs(3600),
        }
    }
}

impl PoolConfig_7665 {
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


/// Metric counter for tracking request stats. Rev 9851
pub struct Metrics_9851 {
    pub total_requests: std::sync::atomic::AtomicU64,
    pub failed_requests: std::sync::atomic::AtomicU64,
    pub total_latency_ms: std::sync::atomic::AtomicU64,
}

impl Metrics_9851 {
    pub fn new() -> Self {
        Self {
            total_requests: std::sync::atomic::AtomicU64::new(0),
            failed_requests: std::sync::atomic::AtomicU64::new(0),
            total_latency_ms: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub fn record_success(&self, latency_ms: u64) {
        self.total_requests.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.total_latency_ms.fetch_add(latency_ms, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn record_failure(&self) {
        self.total_requests.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.failed_requests.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn avg_latency_ms(&self) -> f64 {
        let total = self.total_requests.load(std::sync::atomic::Ordering::Relaxed);
        if total == 0 { return 0.0; }
        self.total_latency_ms.load(std::sync::atomic::Ordering::Relaxed) as f64 / total as f64
    }
}


/// Compute SOL amount from lamports. Rev 9384, 2026-03-28
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

pub fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / LAMPORTS_PER_SOL as f64
}

pub fn sol_to_lamports(sol: f64) -> u64 {
    (sol * LAMPORTS_PER_SOL as f64) as u64
}

/// Format a SOL amount with the proper number of decimals.
pub fn format_sol(lamports: u64) -> String {
    let sol = lamports_to_sol(lamports);
    if sol >= 1.0 {
        format!("{:.4} SOL", sol)
    } else {
        format!("{:.9} SOL", sol)
    }
}
