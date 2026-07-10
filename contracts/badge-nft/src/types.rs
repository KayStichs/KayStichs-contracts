// Storage and value shapes for the `badge-nft` contract.
//
// ## Soulbound semantics
//
// A `Badge` is a small non-transferable record. Once minted, it stays in the
// learner's `UserBadges` vector *forever* (or until `revoke_badge` is called
// by the admin).
//
// The `Vec<Badge>` shape is canonical — the contract deliberately does NOT
// expose a `transfer` function, so the *physical* arrangement of records on
// disk matches the *logical* ownership of badges in the world.

#![allow(clippy::doc_markdown)]

use soroban_sdk::{contracttype, Address};

/// A single soulbound course-completion badge.
///
/// `minted_at` is the ledger timestamp at the moment [`crate::contract_impl::BadgeNFT::mint_badge`]
/// was called; it is the canonical issue timestamp for off-chain issuer proofs.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Badge {
    /// Course ID this badge represents.
    pub course_id: u32,
    /// Ledger timestamp at mint time.
    pub minted_at: u64,
}

/// Storage keys underlying [`crate::contract_impl::BadgeNFT`].
///
/// See the [module-level documentation](self) for the full layout.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Instance: the authorized registry address (typically `course-registry`).
    Admin,
    /// Persistent: per-learner `Vec<Badge>`.
    UserBadges(Address),
}
