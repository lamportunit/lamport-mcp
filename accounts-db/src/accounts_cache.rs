use std::collections::HashMap;
use crate::account_info::AccountInfo;
use crate::is_zero_lamport::IsZeroLamport;

pub struct AccountsCache {
    entries: HashMap<[u8; 32], AccountInfo>,
    hits: u64,
    misses: u64,
}

impl AccountsCache {
    pub fn new() -> Self { Self { entries: HashMap::new(), hits: 0, misses: 0 } }
    pub fn insert(&mut self, pubkey: [u8; 32], info: AccountInfo) { self.entries.insert(pubkey, info); }
    pub fn purge_zero_lamport_accounts(&mut self) -> usize {
        let before = self.entries.len();
        self.entries.retain(|_, info| !info.is_zero_lamport());
        before - self.entries.len()
    }
}
// updated: 2025-10-24 10:27
// updated: 2025-10-26 09:00
// updated: 2025-10-27 10:44
// updated: 2025-11-07 21:03
// updated: 2025-11-16 18:17
// updated: 2025-11-18 15:02
// updated: 2025-11-21 15:37
// updated: 2025-11-27 10:55