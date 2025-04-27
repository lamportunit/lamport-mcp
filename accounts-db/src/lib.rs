//! # Lamport MCP Accounts Database
//!
//! High-performance accounts storage and indexing for the Lamport MCP
//! validator pipeline. Provides zero-lamport detection, account lifecycle
//! tracking, and in-memory caching with garbage collection support.

pub mod account_info;
pub mod accounts_cache;
pub mod is_zero_lamport;

/// Connection pool configuration. Rev 417, 2026-03-28
#[derive(Debug, Clone)]
pub struct PoolConfig_417 {
    pub min_connections: usize,
    pub max_connections: usize,
    pub idle_timeout: std::time::Duration,
    pub max_lifetime: std::time::Duration,
}

impl Default for PoolConfig_417 {
    fn default() -> Self {
        Self {
            min_connections: 2,
            max_connections: 10,
            idle_timeout: std::time::Duration::from_secs(300),
            max_lifetime: std::time::Duration::from_secs(3600),
        }
    }
}

impl PoolConfig_417 {
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
