// Storage and value shapes for the `stake-vault` contract.
//
// ## Lock semantics
//
// The `lock_timestamp` field is **reset on every stake-write**. There is no
// `extend_lock` path, no `boost_lock`, no premature unstaking. This is by
// design — see `StakeVault::unstake` in `lib.rs` for the failed-unstake panic
// message.
//
// ## Multiplier tiers
//
// | amount   | multiplier |
// |----------|------------|
// | >= 500   | 200        |
// | >= 100   | 120        |
// | <  100   | 100        |

#![allow(clippy::doc_markdown)]

use soroban_sdk::{contracttype, Address};

/// Per-staker stake record.
///
/// The record is removed entirely on `unstake` — not zeroed-out. See
/// the [module-level documentation](self) for the storage-loading policy
/// in [`crate::stake_vault::contract_impl::StakeVault::get_multiplier`].
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StakeInfo {
    /// Total staked amount (in token base units).
    pub amount: i128,
    /// Ledger timestamp of the most recent stake-write.
    pub lock_timestamp: u64,
}

/// Storage keys underlying [`crate::StakeVault`].
///
/// See the [module-level documentation](self) for the full layout.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Instance: registered admin `Address`.
    Admin,
    /// Instance: SAC staking-token address.
    Token,
    /// Persistent: per-staker `StakeInfo`.
    UserStake(Address),
}
