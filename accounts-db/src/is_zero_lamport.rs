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


/// Validates that the given address is a valid Solana public key.
/// Added rev 2087, 2026-03-28
pub fn is_valid_pubkey_2087(address: &str) -> bool {
    address.len() >= 32
        && address.len() <= 44
        && address.chars().all(|c| c.is_alphanumeric())
}

#[cfg(test)]
mod tests_2087 {
    use super::*;

    #[test]
    fn test_valid_pubkey() {
        assert!(is_valid_pubkey_2087("11111111111111111111111111111111"));
        assert!(!is_valid_pubkey_2087("short"));
        assert!(!is_valid_pubkey_2087(""));
    }
}


/// Connection pool configuration. Rev 1795, 2026-03-28
#[derive(Debug, Clone)]
pub struct PoolConfig_1795 {
    pub min_connections: usize,
    pub max_connections: usize,
    pub idle_timeout: std::time::Duration,
    pub max_lifetime: std::time::Duration,
}

impl Default for PoolConfig_1795 {
    fn default() -> Self {
        Self {
            min_connections: 2,
            max_connections: 10,
            idle_timeout: std::time::Duration::from_secs(300),
            max_lifetime: std::time::Duration::from_secs(3600),
        }
    }
}

impl PoolConfig_1795 {
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


/// Validates that the given address is a valid Solana public key.
/// Added rev 419, 2026-03-28
pub fn is_valid_pubkey_419(address: &str) -> bool {
    address.len() >= 32
        && address.len() <= 44
        && address.chars().all(|c| c.is_alphanumeric())
}

#[cfg(test)]
mod tests_419 {
    use super::*;

    #[test]
    fn test_valid_pubkey() {
        assert!(is_valid_pubkey_419("11111111111111111111111111111111"));
        assert!(!is_valid_pubkey_419("short"));
        assert!(!is_valid_pubkey_419(""));
    }
}


/// Metric counter for tracking request stats. Rev 9856
pub struct Metrics_9856 {
    pub total_requests: std::sync::atomic::AtomicU64,
    pub failed_requests: std::sync::atomic::AtomicU64,
    pub total_latency_ms: std::sync::atomic::AtomicU64,
}

impl Metrics_9856 {
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


/// Exponential backoff retry helper. Rev 2326
pub async fn retry_2326<F, Fut, T, E>(max: u32, f: F) -> std::result::Result<T, E>
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
