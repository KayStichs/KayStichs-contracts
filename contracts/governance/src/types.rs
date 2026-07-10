// Storage and value shapes for the `governance` contract.
//
// ## Voting semantics
//
// Vote weight is the learner's **badge count**. Storage of a vote record is
// strictly binary (one vote per `(voter, proposal_id)`); the field-level vote
// totals (`votes_for`, `votes_against`) on the proposal record are *derived*
// and may be tower-hacked by reverting from additive math — see the
// `Governance` impl in `lib.rs` for the arithmetic checks.

#![allow(clippy::doc_markdown)]

use soroban_sdk::{contracttype, Address, BytesN};

/// A badge-weighted DAO proposal.
///
/// `metadata_hash` typically resolves to an off-chain text payload (a PDF,
/// audit doc, rewrite proposal etc.). The `end_time` is a hard close —
/// `execute_proposal` panics before it.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    /// Proposal ID (key into [`DataKey::Proposal`]).
    pub id: u32,
    /// Address that opened the proposal.
    pub proposer: Address,
    /// Pointer to proposal content off-chain.
    pub metadata_hash: BytesN<32>,
    /// Sum of `badge_count` votes with `support = true`.
    pub votes_for: u32,
    /// Sum of `badge_count` votes with `support = false`.
    pub votes_against: u32,
    /// Ledger timestamp at which voting closes.
    pub end_time: u64,
    /// Whether this proposal has been executed or cancelled.
    pub executed: bool,
}

/// Storage keys underlying [`crate::Governance`].
///
/// See the [module-level documentation](self) for the full layout.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Persistent: per-id `Proposal` record.
    Proposal(u32),
    /// Persistent: per-(voter, proposal) cast-vote ledger (prevents double-vote).
    UserVote(Address, u32),
    /// Instance: registered admin `Address`.
    Admin,
}
