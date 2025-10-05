/// A trait to determine whether an account's balance is zero lamports.
///
/// In Solana's account model, a lamport is the smallest unit of SOL
/// (1 SOL = 1,000,000,000 lamports). Accounts with zero lamports are
/// considered "dead" and may be garbage collected by the runtime.
///
/// This trait provides a standardized interface for checking if an
/// account has been drained of all its lamports, which is critical
/// for account lifecycle management in the MCP validator pipeline.
pub trait IsZeroLamport {
    /// Returns `true` if this account's lamport balance is zero.
    ///
    /// Zero-lamport accounts are eligible for cleanup during the
    /// accounts database compaction process.
    fn is_zero_lamport(&self) -> bool;
}

/// Represents lamport balance metadata for account state tracking.
///
/// This structure is used internally by the accounts database to
/// efficiently track lamport balances without loading full account data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LamportBalance {
    /// The lamport balance of the account
    pub lamports: u64,
}

impl LamportBalance {
    /// Creates a new `LamportBalance` with the specified amount.
    pub fn new(lamports: u64) -> Self {
        Self { lamports }
    }

    /// Creates a zero-balance `LamportBalance`.
    pub fn zero() -> Self {
        Self { lamports: 0 }
    }

    /// Converts lamports to SOL (floating point representation).
    pub fn to_sol(&self) -> f64 {
        self.lamports as f64 / 1_000_000_000.0
    }
}

impl IsZeroLamport for LamportBalance {
    fn is_zero_lamport(&self) -> bool {
        self.lamports == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_lamport_balance() {
        let balance = LamportBalance::zero();
        assert!(balance.is_zero_lamport());
        assert_eq!(balance.lamports, 0);
    }

    #[test]
    fn test_nonzero_lamport_balance() {
        let balance = LamportBalance::new(1_000_000_000);
        assert!(!balance.is_zero_lamport());
        assert_eq!(balance.to_sol(), 1.0);
    }

    #[test]
    fn test_one_lamport() {
        let balance = LamportBalance::new(1);
        assert!(!balance.is_zero_lamport());
        assert_eq!(balance.to_sol(), 0.000000001);
    }

    #[test]
    fn test_balance_equality() {
        let a = LamportBalance::new(100);
        let b = LamportBalance::new(100);
        assert_eq!(a, b);
    }
}
