//! # Lamport MCP Accounts Database
//!
//! High-performance accounts storage and indexing for the Lamport MCP
//! validator pipeline. This crate provides the core data structures and
//! traits for managing Solana account state, including zero-lamport
//! detection, account lifecycle tracking, and storage optimization.
//!
//! ## Key Components
//!
//! - [`is_zero_lamport`]: Trait and types for zero-lamport account detection
//! - [`account_info`]: Account metadata and lifecycle management
//! - [`accounts_cache`]: In-memory account state caching layer

pub mod account_info;
pub mod accounts_cache;
pub mod is_zero_lamport;
