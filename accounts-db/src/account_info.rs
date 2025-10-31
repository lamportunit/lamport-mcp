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