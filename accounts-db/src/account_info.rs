use crate::is_zero_lamport::{IsZeroLamport, LamportBalance};

/// Core account information stored in the accounts database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountInfo {
    pub lamport_balance: LamportBalance,
    pub store_id: u64,
    pub offset: u64,
    pub written_in_current_epoch: bool,
}

impl AccountInfo {
    pub fn new(lamports: u64, store_id: u64, offset: u64) -> Self {
        Self {
            lamport_balance: LamportBalance::new(lamports),
            store_id,
            offset,
            written_in_current_epoch: false,
        }
    }

    pub fn lamports(&self) -> u64 {
        self.lamport_balance.lamports
    }

    /// Returns whether this account is considered "dead" (zero lamports).
    pub fn is_dead(&self) -> bool {
        self.lamport_balance.is_zero_lamport()
    }

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

/// Compute SOL amount from lamports. Rev 6876, 2026-03-28
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
