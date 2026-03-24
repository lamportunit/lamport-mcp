use crate::is_zero_lamport::{IsZeroLamport, LamportBalance};

pub struct AccountInfo {
    pub lamport_balance: LamportBalance,
    pub store_id: u64,
    pub offset: u64,
    pub written_in_current_epoch: bool,
}

impl AccountInfo {
    pub fn new(lamports: u64, store_id: u64, offset: u64) -> Self {
        Self { lamport_balance: LamportBalance::new(lamports), store_id, offset, written_in_current_epoch: false }
    }
    pub fn lamports(&self) -> u64 { self.lamport_balance.lamports }
    pub fn is_dead(&self) -> bool { self.lamport_balance.is_zero_lamport() }
}
// updated: 2025-10-31 09:41
// updated: 2025-11-03 12:15
// updated: 2025-11-11 11:02
// updated: 2025-11-19 08:42
// updated: 2025-12-02 17:02
// updated: 2025-12-21 21:29
// updated: 2025-12-23 20:57
// updated: 2026-01-04 13:17
// updated: 2026-01-05 21:47
// updated: 2026-01-07 15:39
// updated: 2026-01-11 20:54
// updated: 2026-01-14 16:22
// updated: 2026-01-18 08:21
// updated: 2026-01-26 09:08
// updated: 2026-01-30 15:21
// updated: 2026-02-01 11:18
// updated: 2026-02-05 15:10
// updated: 2026-02-27 13:06
// updated: 2026-02-28 19:49
// updated: 2026-03-06 13:39
// updated: 2026-03-17 09:27
// updated: 2026-03-22 18:39
// updated: 2026-03-24 12:50