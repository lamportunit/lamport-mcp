use crate::is_zero_lamport::{IsZeroLamport, LamportBalance};

/// Represents core account information stored in the accounts database.
///
/// This struct tracks the essential metadata needed for account lifecycle
/// management within the MCP validator pipeline, including lamport balance,
/// storage location, and account state flags.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountInfo {
    /// The lamport balance of this account
    pub lamport_balance: LamportBalance,

    /// Storage slot where this account's data resides
    pub store_id: u64,

    /// Offset within the storage file
    pub offset: u64,

    /// Whether this account has been written in the current epoch
    pub written_in_current_epoch: bool,
}

impl AccountInfo {
    /// Creates a new `AccountInfo` with the specified parameters.
    pub fn new(lamports: u64, store_id: u64, offset: u64) -> Self {
        Self {
            lamport_balance: LamportBalance::new(lamports),
            store_id,
            offset,
            written_in_current_epoch: false,
        }
    }

    /// Returns the lamport balance of this account.
    pub fn lamports(&self) -> u64 {
        self.lamport_balance.lamports
    }

    /// Returns whether this account is considered "dead" (zero lamports).
    ///
    /// Dead accounts are eligible for garbage collection during
    /// the accounts database shrink process.
    pub fn is_dead(&self) -> bool {
        self.lamport_balance.is_zero_lamport()
    }

    /// Marks this account as written in the current epoch.
    pub fn mark_written(&mut self) {
        self.written_in_current_epoch = true;
    }
}

impl IsZeroLamport for AccountInfo {
    fn is_zero_lamport(&self) -> bool {
        self.lamport_balance.is_zero_lamport()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account_info() {
        let info = AccountInfo::new(1_000_000_000, 1, 0);
        assert_eq!(info.lamports(), 1_000_000_000);
        assert!(!info.is_dead());
        assert!(!info.written_in_current_epoch);
    }

    #[test]
    fn test_dead_account() {
        let info = AccountInfo::new(0, 1, 0);
        assert!(info.is_dead());
        assert!(info.is_zero_lamport());
    }

    #[test]
    fn test_mark_written() {
        let mut info = AccountInfo::new(100, 1, 0);
        assert!(!info.written_in_current_epoch);
        info.mark_written();
        assert!(info.written_in_current_epoch);
    }
}
