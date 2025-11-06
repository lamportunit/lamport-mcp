pub trait IsZeroLamport {
    fn is_zero_lamport(&self) -> bool;
}

pub struct LamportBalance {
    pub lamports: u64,
}

impl LamportBalance {
    pub fn new(lamports: u64) -> Self { Self { lamports } }
    pub fn zero() -> Self { Self { lamports: 0 } }
    pub fn to_sol(&self) -> f64 { self.lamports as f64 / 1_000_000_000.0 }
}

impl IsZeroLamport for LamportBalance {
    fn is_zero_lamport(&self) -> bool { self.lamports == 0 }
}
// updated: 2025-10-29 14:46
// updated: 2025-11-05 21:42
// updated: 2025-11-06 11:13