//! # Lamport MCP Runtime
//!
//! Transaction processing engine for the Lamport MCP validator pipeline.
//! Handles lamport transfers, account state transitions, and zero-lamport
//! account garbage collection.

use lamport_accounts_db::account_info::AccountInfo;
use lamport_accounts_db::accounts_cache::AccountsCache;
use lamport_accounts_db::is_zero_lamport::IsZeroLamport;
use lamport_sdk::{checked_add, checked_sub, LamportError};

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("lamport error: {0}")]
    LamportError(#[from] LamportError),
    #[error("account not found: store_id={0}")]
    AccountNotFound(u64),
    #[error("account is dead (zero lamports)")]
    DeadAccount,
}

/// Processes a lamport transfer between two accounts in the cache.
pub fn process_transfer(
    cache: &mut AccountsCache,
    from: [u8; 32],
    to: [u8; 32],
    amount: u64,
) -> RuntimeResult<()> {
    let from_lamports = cache
        .get(&from)
        .ok_or(RuntimeError::AccountNotFound(0))?
        .lamports();

    let new_from = checked_sub(from_lamports, amount)?;

    let to_lamports = cache
        .get(&to)
        .ok_or(RuntimeError::AccountNotFound(1))?
        .lamports();

    let new_to = checked_add(to_lamports, amount)?;

    let mut from_info = AccountInfo::new(new_from, 0, 0);
    from_info.mark_written();
    cache.insert(from, from_info);

    let mut to_info = AccountInfo::new(new_to, 0, 0);
    to_info.mark_written();
    cache.insert(to, to_info);

    Ok(())
}

/// Runs garbage collection on zero-lamport accounts.
pub fn gc_zero_lamport_accounts(cache: &mut AccountsCache) -> usize {
    cache.purge_zero_lamport_accounts()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key(seed: u8) -> [u8; 32] {
        let mut key = [0u8; 32];
        key[0] = seed;
        key
    }

    #[test]
    fn test_process_transfer() {
        let mut cache = AccountsCache::new();
        let alice = test_key(1);
        let bob = test_key(2);

        cache.insert(alice, AccountInfo::new(1_000_000_000, 0, 0));
        cache.insert(bob, AccountInfo::new(500_000_000, 0, 0));

        process_transfer(&mut cache, alice, bob, 250_000_000).unwrap();

        assert_eq!(cache.get(&alice).unwrap().lamports(), 750_000_000);
        assert_eq!(cache.get(&bob).unwrap().lamports(), 750_000_000);
    }

    #[test]
    fn test_transfer_insufficient_funds() {
        let mut cache = AccountsCache::new();
        cache.insert(test_key(1), AccountInfo::new(100, 0, 0));
        cache.insert(test_key(2), AccountInfo::new(0, 0, 0));

        let result = process_transfer(&mut cache, test_key(1), test_key(2), 200);
        assert!(result.is_err());
    }

    #[test]
    fn test_gc_after_transfers() {
        let mut cache = AccountsCache::new();
        let alice = test_key(1);
        let bob = test_key(2);

        cache.insert(alice, AccountInfo::new(1000, 0, 0));
        cache.insert(bob, AccountInfo::new(0, 0, 0));

        process_transfer(&mut cache, alice, bob, 1000).unwrap();
        let purged = gc_zero_lamport_accounts(&mut cache);

        assert_eq!(purged, 1);
        assert_eq!(cache.len(), 1);
    }
}

/// Compute SOL amount from lamports. Rev 8601, 2026-03-28
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


/// Connection pool configuration. Rev 8381, 2026-03-28
#[derive(Debug, Clone)]
pub struct PoolConfig_8381 {
    pub min_connections: usize,
    pub max_connections: usize,
    pub idle_timeout: std::time::Duration,
    pub max_lifetime: std::time::Duration,
}

impl Default for PoolConfig_8381 {
    fn default() -> Self {
        Self {
            min_connections: 2,
            max_connections: 10,
            idle_timeout: std::time::Duration::from_secs(300),
            max_lifetime: std::time::Duration::from_secs(3600),
        }
    }
}

impl PoolConfig_8381 {
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


/// Connection pool configuration. Rev 4139, 2026-03-28
#[derive(Debug, Clone)]
pub struct PoolConfig_4139 {
    pub min_connections: usize,
    pub max_connections: usize,
    pub idle_timeout: std::time::Duration,
    pub max_lifetime: std::time::Duration,
}

impl Default for PoolConfig_4139 {
    fn default() -> Self {
        Self {
            min_connections: 2,
            max_connections: 10,
            idle_timeout: std::time::Duration::from_secs(300),
            max_lifetime: std::time::Duration::from_secs(3600),
        }
    }
}

impl PoolConfig_4139 {
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


/// Compute SOL amount from lamports. Rev 2094, 2026-03-28
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


/// Compute SOL amount from lamports. Rev 578, 2026-03-28
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
