//! Storage and value shapes for the `quest-engine` contract.
//!
//! ## Quest types
//!
//! | `QuestType` | Fund source        | Who approves  | Payout source         |
//! |-------------|--------------------|---------------|------------------------|
//! | `Build`     | `create_build_quest` (escrow in contract) | Employer | quest-engine -> learner with fee |
//! | `Explore`   | `reward-pool` (cross-called at verify)  | Admin     | `reward-pool`         |
//!
//! ## Submission lifecycle
//!
//! `Pending` → `Approved` or `Rejected` on [`crate::QuestEngineContract::review_submission`].
//! `Pending` → `Approved` on [`crate::QuestEngineContract::batch_review_submissions`].
use soroban_sdk::{contracttype, Address, BytesN};

/// Enum of supported quest types. The discriminator is part of the ABI —
/// adding a variant requires bumping the contract version.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuestType {
    /// Employer-funded quest with on-chain escrow and peer-reviewed approval.
    Build,
    /// Admin-verified off-chain action funded from `reward-pool`.
    Explore,
}

/// A single quest record.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Quest {
    /// Address that funded the quest (the employer for `Build`, the admin for `Explore`).
    pub employer: Address,
    /// Reward amount in the SAC token's base units.
    pub reward_amount: i128,
    /// Build vs Explore; constrains submission/approval flow.
    pub quest_type: QuestType,
    /// Pointer to quest content off-chain.
    pub metadata_hash: BytesN<32>,
    /// Whether the quest accepts new submissions / can be reviewed.
    pub active: bool,
}

/// Lifecycle state of a submission. Mirrors Soroban's `try_from_i32` deser.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubmissionStatus {
    /// Submitted; awaiting employer review.
    Pending,
    /// Approved and paid out (full or capped).
    Approved,
    /// Rejected by the employer; no payout.
    Rejected,
}

/// Per-(learner, quest) submission record.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Submission {
    /// Pointer to the learner's submission content off-chain.
    pub proof_hash: BytesN<32>,
    /// Lifecycle state — drives `review_submission`'s panic on non-pending.
    pub status: SubmissionStatus,
}

/// Storage keys underlying [`crate::QuestEngineContract`].
///
/// See the [module-level documentation](self) for the full layout.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Instance: registered admin `Address`.
    Admin,
    /// Persistent: per-id `Quest` record.
    Quest(u32),
    /// Persistent: per-(learner, quest) `Submission` record.
    Submission(Address, u32),
    /// Instance: configured USDC SAC token address.
    Token,
    /// Instance: monotonically incrementing quest ID counter.
    QuestCounter,
    /// Instance: configured `reward-pool` contract address.
    RewardPool,
    /// Instance: pause circuit-breaker (boolean).
    IsPaused,
    /// Instance: configured `stake-vault` contract address.
    StakeVault,
}
