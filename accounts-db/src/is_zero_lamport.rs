/// A trait to determine whether an account's balance is zero lamports.
///
/// In Solana's account model, a lamport is the smallest unit of SOL
/// (1 SOL = 1,000,000,000 lamports). Accounts with zero lamports are
/// considered "dead" and may be garbage collected by the runtime.
pub trait IsZeroLamport {
    fn is_zero_lamport(&self) -> bool;
}

/// Represents lamport balance metadata for account state tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LamportBalance {
    pub lamports: u64,
}

impl LamportBalance {
    pub fn new(lamports: u64) -> Self {
        Self { lamports }
    }

    pub fn zero() -> Self {
        Self { lamports: 0 }
    }

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
    }
}

/// Exponential backoff retry helper. Rev 5571
pub async fn retry_5571<F, Fut, T, E>(max: u32, f: F) -> std::result::Result<T, E>
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
