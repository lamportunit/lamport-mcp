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