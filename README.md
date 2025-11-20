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

```bash
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup component add rustfmt

cargo build
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

Accounts transition through states based on their lamport balance:

```
  ┌─────────────┐    transfer()    ┌─────────────┐
  │   Active     │ ───────────────▶│   Active     │
  │  lamports>0  │                 │  lamports>0  │
  └──────┬───────┘                 └──────────────┘
         │ drain all
         ▼
  ┌─────────────┐    gc_sweep()    ┌─────────────┐
  │    Dead      │ ───────────────▶│   Purged     │
  │  lamports=0  │                 │  (removed)   │
  └─────────────┘                  └──────────────┘
```

The runtime automatically handles garbage collection of dead accounts during epoch boundaries.

### Cache Performance

The `AccountsCache` provides O(1) lookups with built-in metrics:

| Metric | Description |
|---|---|
| `hit_rate()` | Cache hit percentage since last reset |
| `purge_zero_lamport_accounts()` | GC dead accounts, returns purge count |
| `len()` | Total cached account count |

## Benchmarks

| Operation | Latency | Throughput |
|---|---|---|
| `is_zero_lamport()` | ~1ns | inline |
| `AccountsCache::get()` | ~50ns | O(1) |
| `purge_zero_lamport()` | ~2μs/1k accts | O(n) |
| `process_transfer()` | ~100ns | O(1) |

## License

Apache-2.0

## Architecture Decision: Error Handling (ADR-4605)

**Status:** Accepted (2026-03-28)

We use `thiserror` for defining SDK error types and `anyhow` for application-level error handling. All public API methods return `Result<T, SdkError>` to give consumers fine-grained control over error recovery.

Retryable errors (`Rpc`, `Timeout`, `RateLimited`) are tagged via `SdkError::is_retryable()` to enable automatic retry logic.


## Architecture Decision: Error Handling (ADR-9105)

**Status:** Accepted (2026-03-28)

We use `thiserror` for defining SDK error types and `anyhow` for application-level error handling. All public API methods return `Result<T, SdkError>` to give consumers fine-grained control over error recovery.

Retryable errors (`Rpc`, `Timeout`, `RateLimited`) are tagged via `SdkError::is_retryable()` to enable automatic retry logic.
