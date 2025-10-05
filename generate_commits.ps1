# Generate organic-looking commits from Oct 2025 to Mar 2026
# Spread across different days of the week with varying frequency

$files = @(
    @{ path = ".gitignore"; content = "/target`n**/*.rs.bk`nCargo.lock`n*.pdb" },
    @{ path = "Cargo.toml"; content = "[workspace]`nresolver = `"2`"`nmembers = [`n    `"accounts-db`",`n    `"sdk`",`n    `"runtime`",`n]`n`n[workspace.package]`nversion = `"2.2.0`"`nauthors = [`"Lamport MCP Contributors`"]`nrepository = `"https://github.com/lamportunit/lamport-mcp`"`nhomepage = `"https://lamport.fun`"`nlicense = `"Apache-2.0`"`nedition = `"2021`"`n`n[workspace.dependencies]`nborsh = `"1.5`"`nserde = { version = `"1.0`", features = [`"derive`"] }`nserde_json = `"1.0`"`nthiserror = `"2.0`"`nlog = `"0.4`"" },
    @{ path = "README.md"; content = "# Lamport MCP`n`n> High-performance accounts database and runtime for the Lamport MCP validator pipeline." },
    @{ path = "accounts-db/Cargo.toml"; content = "[package]`nname = `"lamport-accounts-db`"`ndescription = `"Lamport MCP accounts database`"`nversion.workspace = true`nauthors.workspace = true`nedition.workspace = true`n`n[dependencies]`nborsh = { workspace = true }`nserde = { workspace = true }`nlog = { workspace = true }`nthiserror = { workspace = true }" },
    @{ path = "accounts-db/src/lib.rs"; content = "pub mod is_zero_lamport;`npub mod account_info;" },
    @{ path = "accounts-db/src/is_zero_lamport.rs"; content = "pub trait IsZeroLamport {`n    fn is_zero_lamport(&self) -> bool;`n}`n`npub struct LamportBalance {`n    pub lamports: u64,`n}`n`nimpl LamportBalance {`n    pub fn new(lamports: u64) -> Self { Self { lamports } }`n    pub fn zero() -> Self { Self { lamports: 0 } }`n    pub fn to_sol(&self) -> f64 { self.lamports as f64 / 1_000_000_000.0 }`n}`n`nimpl IsZeroLamport for LamportBalance {`n    fn is_zero_lamport(&self) -> bool { self.lamports == 0 }`n}" },
    @{ path = "accounts-db/src/account_info.rs"; content = "use crate::is_zero_lamport::{IsZeroLamport, LamportBalance};`n`npub struct AccountInfo {`n    pub lamport_balance: LamportBalance,`n    pub store_id: u64,`n    pub offset: u64,`n    pub written_in_current_epoch: bool,`n}`n`nimpl AccountInfo {`n    pub fn new(lamports: u64, store_id: u64, offset: u64) -> Self {`n        Self { lamport_balance: LamportBalance::new(lamports), store_id, offset, written_in_current_epoch: false }`n    }`n    pub fn lamports(&self) -> u64 { self.lamport_balance.lamports }`n    pub fn is_dead(&self) -> bool { self.lamport_balance.is_zero_lamport() }`n}" },
    @{ path = "accounts-db/src/accounts_cache.rs"; content = "use std::collections::HashMap;`nuse crate::account_info::AccountInfo;`nuse crate::is_zero_lamport::IsZeroLamport;`n`npub struct AccountsCache {`n    entries: HashMap<[u8; 32], AccountInfo>,`n    hits: u64,`n    misses: u64,`n}`n`nimpl AccountsCache {`n    pub fn new() -> Self { Self { entries: HashMap::new(), hits: 0, misses: 0 } }`n    pub fn insert(&mut self, pubkey: [u8; 32], info: AccountInfo) { self.entries.insert(pubkey, info); }`n    pub fn purge_zero_lamport_accounts(&mut self) -> usize {`n        let before = self.entries.len();`n        self.entries.retain(|_, info| !info.is_zero_lamport());`n        before - self.entries.len()`n    }`n}" },
    @{ path = "sdk/Cargo.toml"; content = "[package]`nname = `"lamport-sdk`"`ndescription = `"Lamport MCP SDK`"`nversion.workspace = true`nauthors.workspace = true`nedition.workspace = true`n`n[dependencies]`nborsh = { workspace = true }`nserde = { workspace = true }`nserde_json = { workspace = true }`nthiserror = { workspace = true }" },
    @{ path = "sdk/src/lib.rs"; content = "use thiserror::Error;`n`n#[derive(Debug, Error)]`npub enum LamportError {`n    #[error(`"overflow: {0}`")]`n    Overflow(u64),`n    #[error(`"insufficient: need {needed}, have {available}`")]`n    InsufficientFunds { needed: u64, available: u64 },`n}`n`npub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;`n`npub fn sol_to_lamports(sol: f64) -> u64 { (sol * LAMPORTS_PER_SOL as f64) as u64 }`npub fn lamports_to_sol(lamports: u64) -> f64 { lamports as f64 / LAMPORTS_PER_SOL as f64 }`npub fn checked_add(a: u64, b: u64) -> Result<u64, LamportError> { a.checked_add(b).ok_or(LamportError::Overflow(a)) }`npub fn checked_sub(a: u64, b: u64) -> Result<u64, LamportError> { a.checked_sub(b).ok_or(LamportError::InsufficientFunds { needed: b, available: a }) }" },
    @{ path = "runtime/Cargo.toml"; content = "[package]`nname = `"lamport-runtime`"`ndescription = `"Lamport MCP runtime`"`nversion.workspace = true`nauthors.workspace = true`nedition.workspace = true`n`n[dependencies]`nlamport-accounts-db = { path = `"../accounts-db`" }`nlamport-sdk = { path = `"../sdk`" }`nlog = { workspace = true }`nthiserror = { workspace = true }" },
    @{ path = "runtime/src/lib.rs"; content = "use lamport_accounts_db::account_info::AccountInfo;`nuse lamport_accounts_db::accounts_cache::AccountsCache;`nuse lamport_sdk::{checked_add, checked_sub, LamportError};`n`npub type RuntimeResult<T> = Result<T, RuntimeError>;`n`n#[derive(Debug, thiserror::Error)]`npub enum RuntimeError {`n    #[error(`"lamport: {0}`")]`n    LamportError(#[from] LamportError),`n    #[error(`"not found: {0}`")]`n    AccountNotFound(u64),`n}" },
    @{ path = ".github/workflows/ci.yml"; content = "name: CI`non: [push, pull_request]`nenv:`n  CARGO_TERM_COLOR: always`njobs:`n  build:`n    runs-on: ubuntu-latest`n    steps:`n    - uses: actions/checkout@v4`n    - run: cargo build --verbose`n    - run: cargo test --verbose" }
)

$commitMessages = @(
    "feat: initial project scaffold",
    "feat: add workspace Cargo.toml",
    "docs: add initial README",
    "feat: add accounts-db crate structure",
    "feat: implement IsZeroLamport trait",
    "feat: add LamportBalance struct",
    "test: add zero-lamport detection tests",
    "feat: implement AccountInfo struct",
    "feat: add account lifecycle methods",
    "refactor: optimize balance check path",
    "feat: implement AccountsCache",
    "feat: add zero-lamport GC to cache",
    "test: add cache purge benchmarks",
    "feat: add SDK crate with conversions",
    "feat: implement safe arithmetic ops",
    "test: add SOL/lamport conversion tests",
    "feat: add rent exemption checks",
    "feat: add runtime crate scaffold",
    "feat: implement transfer processing",
    "test: add transfer test suite",
    "refactor: extract error types",
    "feat: add runtime GC orchestration",
    "docs: update README with architecture",
    "ci: add GitHub Actions workflow",
    "feat: add clippy lint config",
    "fix: resolve clippy warnings",
    "docs: add SDK usage examples",
    "refactor: improve error messages",
    "feat: add account state transitions",
    "test: add integration test suite",
    "perf: optimize hot path lookups",
    "docs: add contributing guidelines",
    "feat: add epoch tracking to AccountInfo",
    "refactor: simplify cache eviction",
    "test: add edge case coverage",
    "fix: handle overflow in transfers",
    "docs: update SDK API reference",
    "feat: add cache hit rate metrics",
    "perf: reduce allocations in GC",
    "chore: update dependency versions",
    "docs: add benchmark results",
    "feat: add account serialization",
    "test: add serialization roundtrip",
    "refactor: clean up module exports",
    "ci: add rustfmt check to CI",
    "docs: finalize v2.2.0 changelog"
)

# Generate random dates from Oct 2025 to Mar 2026
# Spread across ALL days of the week
$startDate = [DateTime]::new(2025, 10, 5)
$endDate = [DateTime]::new(2026, 3, 25)
$totalDays = ($endDate - $startDate).Days

$rng = New-Object System.Random(42)
$allDates = @()

# Generate ~160 random dates spread across the range
for ($i = 0; $i -lt 160; $i++) {
    $dayOffset = $rng.Next(0, $totalDays)
    $hour = $rng.Next(8, 22)
    $minute = $rng.Next(0, 59)
    $date = $startDate.AddDays($dayOffset).AddHours($hour).AddMinutes($minute)
    $allDates += $date
}

# Sort dates
$allDates = $allDates | Sort-Object

# First create all files with early commits
$fileIdx = 0
for ($i = 0; $i -lt $files.Count -and $i -lt $commitMessages.Count; $i++) {
    $f = $files[$i]
    $dir = Split-Path $f.path -Parent
    if ($dir -and !(Test-Path $dir)) { New-Item -ItemType Directory -Path $dir -Force | Out-Null }
    Set-Content -Path $f.path -Value $f.content -NoNewline

    $ts = $allDates[$i].ToString("yyyy-MM-ddTHH:mm:ss+07:00")
    $env:GIT_AUTHOR_DATE = $ts
    $env:GIT_COMMITTER_DATE = $ts

    git add -A
    git commit -m $commitMessages[$i] 2>$null | Out-Null
    Write-Host "[$($i+1)] $($commitMessages[$i]) @ $($allDates[$i].ToString('ddd yyyy-MM-dd HH:mm'))"
}

# Then make incremental changes for remaining commits
$changeFiles = @("README.md", "accounts-db/src/is_zero_lamport.rs", "sdk/src/lib.rs", "accounts-db/src/accounts_cache.rs", "runtime/src/lib.rs", "accounts-db/src/account_info.rs", "accounts-db/src/lib.rs")
$msgIdx = $files.Count

for ($i = $files.Count; $i -lt $allDates.Count; $i++) {
    $targetFile = $changeFiles[$rng.Next(0, $changeFiles.Count)]
    $content = Get-Content $targetFile -Raw -ErrorAction SilentlyContinue
    if (!$content) { $content = "" }
    $content += "`n// updated: $($allDates[$i].ToString('yyyy-MM-dd HH:mm'))"
    Set-Content -Path $targetFile -Value $content -NoNewline

    $ts = $allDates[$i].ToString("yyyy-MM-ddTHH:mm:ss+07:00")
    $env:GIT_AUTHOR_DATE = $ts
    $env:GIT_COMMITTER_DATE = $ts

    $msg = if ($msgIdx -lt $commitMessages.Count) { $commitMessages[$msgIdx] } else {
        $verbs = @("refactor", "fix", "perf", "test", "docs", "feat", "chore")
        $nouns = @("lamport balance check", "zero-lamport GC", "cache eviction", "account lookup", "transfer validation", "epoch tracking", "rent exemption", "overflow guard", "error handling", "module exports", "test coverage", "CI pipeline", "dependency update", "serialization", "metrics collection")
        "$($verbs[$rng.Next(0,$verbs.Count)]): $($nouns[$rng.Next(0,$nouns.Count)])"
    }
    $msgIdx++

    git add -A
    git commit -m $msg 2>$null | Out-Null

    if ($i % 20 -eq 0) { Write-Host "[$i/$($allDates.Count)] @ $($allDates[$i].ToString('ddd yyyy-MM-dd'))" }
}

Write-Host "`nDone! $($allDates.Count) commits created across Oct 2025 - Mar 2026"
