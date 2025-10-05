use std::collections::HashMap;

use crate::account_info::AccountInfo;
use crate::is_zero_lamport::IsZeroLamport;

/// A read-optimized cache for frequently accessed account data.
///
/// The `AccountsCache` provides O(1) lookups for hot accounts while
/// supporting efficient eviction of zero-lamport (dead) accounts
/// during periodic cleanup cycles.
#[derive(Debug, Default)]
pub struct AccountsCache {
    /// Map of account public key (as bytes) to account info
    entries: HashMap<[u8; 32], AccountInfo>,
    /// Number of cache hits since last reset
    hits: u64,
    /// Number of cache misses since last reset
    misses: u64,
}

impl AccountsCache {
    /// Creates a new empty `AccountsCache`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts an account into the cache.
    pub fn insert(&mut self, pubkey: [u8; 32], info: AccountInfo) {
        self.entries.insert(pubkey, info);
    }

    /// Looks up an account in the cache.
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
    ///
    /// Returns the number of accounts purged.
    pub fn purge_zero_lamport_accounts(&mut self) -> usize {
        let before = self.entries.len();
        self.entries.retain(|_, info| !info.is_zero_lamport());
        before - self.entries.len()
    }

    /// Returns the total number of cached accounts.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the cache hit rate as a percentage.
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        (self.hits as f64 / total as f64) * 100.0
    }

    /// Resets cache statistics.
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
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_purge_zero_lamports() {
        let mut cache = AccountsCache::new();
        cache.insert(test_pubkey(1), AccountInfo::new(1000, 0, 0));
        cache.insert(test_pubkey(2), AccountInfo::new(0, 0, 0)); // dead
        cache.insert(test_pubkey(3), AccountInfo::new(500, 0, 0));
        cache.insert(test_pubkey(4), AccountInfo::new(0, 0, 0)); // dead

        let purged = cache.purge_zero_lamport_accounts();
        assert_eq!(purged, 2);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_hit_rate() {
        let mut cache = AccountsCache::new();
        let pk = test_pubkey(1);
        cache.insert(pk, AccountInfo::new(100, 0, 0));

        cache.get(&pk); // hit
        cache.get(&pk); // hit
        cache.get(&test_pubkey(99)); // miss

        assert!((cache.hit_rate() - 66.666).abs() < 1.0);
    }

    #[test]
    fn test_empty_cache() {
        let cache = AccountsCache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.hit_rate(), 0.0);
    }
}
