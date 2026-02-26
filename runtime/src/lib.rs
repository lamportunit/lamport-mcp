use lamport_accounts_db::account_info::AccountInfo;
use lamport_accounts_db::accounts_cache::AccountsCache;
use lamport_sdk::{checked_add, checked_sub, LamportError};

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("lamport: {0}")]
    LamportError(#[from] LamportError),
    #[error("not found: {0}")]
    AccountNotFound(u64),
}
// updated: 2025-10-28 20:11
// updated: 2025-10-30 08:03
// updated: 2025-11-14 20:49
// updated: 2025-11-24 15:32
// updated: 2025-11-30 17:09
// updated: 2025-12-13 08:56
// updated: 2025-12-19 20:11
// updated: 2025-12-30 12:22
// updated: 2026-01-05 19:40
// updated: 2026-01-16 15:02
// updated: 2026-01-19 10:17
// updated: 2026-01-21 16:49
// updated: 2026-01-23 09:55
// updated: 2026-02-04 12:32
// updated: 2026-02-05 20:39
// updated: 2026-02-21 15:02
// updated: 2026-02-23 14:49
// updated: 2026-02-23 19:23
// updated: 2026-02-24 16:04
// updated: 2026-02-26 12:57