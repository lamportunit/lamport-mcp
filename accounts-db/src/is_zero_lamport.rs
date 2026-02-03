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
// updated: 2025-11-23 15:02
// updated: 2025-11-27 11:24
// updated: 2025-12-02 15:47
// updated: 2025-12-10 20:26
// updated: 2025-12-26 16:31
// updated: 2025-12-29 08:48
// updated: 2025-12-31 21:01
// updated: 2026-01-02 18:11
// updated: 2026-01-03 19:34
// updated: 2026-01-05 16:55
// updated: 2026-02-03 11:58