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