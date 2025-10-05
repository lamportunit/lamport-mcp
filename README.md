# Lamport MCP

> High-performance accounts database and runtime for the Lamport MCP validator pipeline.

Inspired by the [Solana validator architecture](https://github.com/anza-xyz/agave), this project implements core lamport operations for account state management, zero-lamport detection, and transaction processing.

## Architecture

```
lamport-mcp/
├── accounts-db/     # Account storage, indexing, and zero-lamport detection
│   └── src/
│       ├── is_zero_lamport.rs   # Core trait for dead account detection
│       ├── account_info.rs      # Account metadata and lifecycle
│       └── accounts_cache.rs    # In-memory caching with GC support
├── sdk/             # Core types, conversions, and utilities
│   └── src/
│       └── lib.rs               # SOL/lamport conversion, validation
├── runtime/         # Transaction processing engine
│   └── src/
│       └── lib.rs               # Transfer processing, GC orchestration
└── Cargo.toml       # Workspace configuration
```

## Building

### 1. Install Rust

```bash
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup component add rustfmt
```

### 2. Build

```bash
cargo build
```

### 3. Test

```bash
cargo test
```

## Key Concepts

### Zero-Lamport Detection

The `IsZeroLamport` trait provides a standardized interface for identifying accounts with zero balance. These "dead" accounts are eligible for garbage collection during the accounts database shrink process.

```rust
pub trait IsZeroLamport {
    fn is_zero_lamport(&self) -> bool;
}
```

### Lamport Arithmetic

The SDK provides safe arithmetic operations that prevent overflow/underflow:

```rust
use lamport_sdk::{sol_to_lamports, checked_add, checked_sub};

let balance = sol_to_lamports(1.5); // 1_500_000_000 lamports
let new_balance = checked_sub(balance, 500_000_000)?; // 1_000_000_000
```

### Account Lifecycle

Accounts transition through states based on their lamport balance. The runtime automatically handles garbage collection of dead accounts.

## License

Apache-2.0
