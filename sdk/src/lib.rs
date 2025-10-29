//! # Lamport MCP SDK
//!
//! Core types and utilities for working with lamport values in the
//! MCP validator pipeline.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LamportError {
    #[error("lamport amount overflow: {0}")]
    Overflow(u64),
    #[error("insufficient lamports: need {needed}, have {available}")]
    InsufficientFunds { needed: u64, available: u64 },
    #[error("invalid lamport amount")]
    InvalidAmount,
}

pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;
pub const MIN_RENT_EXEMPT_BALANCE: u64 = 890_880;

pub fn sol_to_lamports(sol: f64) -> u64 {
    (sol * LAMPORTS_PER_SOL as f64) as u64
}

pub fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / LAMPORTS_PER_SOL as f64
}

pub fn validate_transfer(from_balance: u64, transfer_amount: u64) -> Result<(), LamportError> {
    if transfer_amount > from_balance {
        return Err(LamportError::InsufficientFunds {
            needed: transfer_amount,
            available: from_balance,
        });
    }
    Ok(())
}

pub fn checked_add(a: u64, b: u64) -> Result<u64, LamportError> {
    a.checked_add(b).ok_or(LamportError::Overflow(a))
}

pub fn checked_sub(a: u64, b: u64) -> Result<u64, LamportError> {
    a.checked_sub(b).ok_or(LamportError::InsufficientFunds {
        needed: b,
        available: a,
    })
}

pub fn is_rent_exempt(lamports: u64) -> bool {
    lamports >= MIN_RENT_EXEMPT_BALANCE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol_lamport_conversion() {
        assert_eq!(sol_to_lamports(1.0), LAMPORTS_PER_SOL);
        assert_eq!(lamports_to_sol(LAMPORTS_PER_SOL), 1.0);
    }

    #[test]
    fn test_checked_arithmetic() {
        assert_eq!(checked_add(100, 200).unwrap(), 300);
        assert!(checked_add(u64::MAX, 1).is_err());
        assert_eq!(checked_sub(300, 100).unwrap(), 200);
        assert!(checked_sub(100, 200).is_err());
    }

    #[test]
    fn test_rent_exemption() {
        assert!(!is_rent_exempt(0));
        assert!(is_rent_exempt(890_880));
        assert!(is_rent_exempt(1_000_000_000));
    }
}