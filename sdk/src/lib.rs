use thiserror::Error;

#[derive(Debug, Error)]
pub enum LamportError {
    #[error("overflow: {0}")]
    Overflow(u64),
    #[error("insufficient: need {needed}, have {available}")]
    InsufficientFunds { needed: u64, available: u64 },
}

pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

pub fn sol_to_lamports(sol: f64) -> u64 { (sol * LAMPORTS_PER_SOL as f64) as u64 }
pub fn lamports_to_sol(lamports: u64) -> f64 { lamports as f64 / LAMPORTS_PER_SOL as f64 }
pub fn checked_add(a: u64, b: u64) -> Result<u64, LamportError> { a.checked_add(b).ok_or(LamportError::Overflow(a)) }
pub fn checked_sub(a: u64, b: u64) -> Result<u64, LamportError> { a.checked_sub(b).ok_or(LamportError::InsufficientFunds { needed: b, available: a }) }
// updated: 2025-10-23 08:38
// updated: 2025-10-26 20:56
// updated: 2025-10-31 16:12
// updated: 2025-11-05 08:13