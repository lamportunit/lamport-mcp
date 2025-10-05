//! # Lamport MCP Runtime
//!
//! Transaction processing engine for the Lamport MCP validator pipeline.
//! Handles lamport transfers, account state transitions, and zero-lamport
//! account garbage collection.

use lamport_accounts_db::account_info::AccountInfo;
use lamport_accounts_db::accounts_cache::AccountsCache;
use lamport_accounts_db::is_zero_lamport::IsZeroLamport;
use lamport_sdk::{checked_add, checked_sub, LamportError};

/// Result type for runtime operations.
pub type RuntimeResult<T> = Result<T, RuntimeError>;

/// Errors that can occur during runtime operations.
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
///
/// This simulates the core transfer operation in the MCP pipeline:
/// 1. Validates sufficient balance in source account
/// 2. Debits source by transfer amount
/// 3. Credits destination by transfer amount
/// 4. Marks both accounts as written in current epoch
pub fn process_transfer(
    cache: &mut AccountsCache,
    from: [u8; 32],
    to: [u8; 32],
    amount: u64,
) -> RuntimeResult<()> {
    // Get source balance
    let from_lamports = cache
        .get(&from)
        .ok_or(RuntimeError::AccountNotFound(0))?
        .lamports();

    // Validate and compute new balances
    let new_from = checked_sub(from_lamports, amount)?;

    let to_lamports = cache
        .get(&to)
        .ok_or(RuntimeError::AccountNotFound(1))?
        .lamports();

    let new_to = checked_add(to_lamports, amount)?;

    // Update source
    let mut from_info = AccountInfo::new(new_from, 0, 0);
    from_info.mark_written();
    cache.insert(from, from_info);

    // Update destination
    let mut to_info = AccountInfo::new(new_to, 0, 0);
    to_info.mark_written();
    cache.insert(to, to_info);

    Ok(())
}

/// Runs garbage collection on zero-lamport accounts.
///
/// Returns the number of dead accounts purged from the cache.
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
        let alice = test_key(1);
        let bob = test_key(2);

        cache.insert(alice, AccountInfo::new(100, 0, 0));
        cache.insert(bob, AccountInfo::new(0, 0, 0));

        let result = process_transfer(&mut cache, alice, bob, 200);
        assert!(result.is_err());
    }

    #[test]
    fn test_transfer_creates_dead_account() {
        let mut cache = AccountsCache::new();
        let alice = test_key(1);
        let bob = test_key(2);

        cache.insert(alice, AccountInfo::new(1000, 0, 0));
        cache.insert(bob, AccountInfo::new(0, 0, 0));

        process_transfer(&mut cache, alice, bob, 1000).unwrap();

        assert!(cache.get(&alice).unwrap().is_zero_lamport());
        assert!(!cache.get(&bob).unwrap().is_zero_lamport());
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

        assert_eq!(purged, 1); // alice is now dead
        assert_eq!(cache.len(), 1); // only bob remains
    }
}
