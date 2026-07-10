//! Storage and value shapes for the `course-registry` contract.
//!
//! ## Lifecycle of a `Course`
//!
//! 1. Admin calls [`crate::CourseRegistry::create_course`] → record is
//!    stored under [`DataKey::Course`] keyed by an incrementing ID.
//! 2. A learner enrolls → their progress is stored under
//!    [`DataKey::Progress`].
//! 3. Verifier calls [`crate::CourseRegistry::complete_module`] per module.
//! 4. On `progress == total_modules`, the contract cross-calls the configured
//!    badge NFT and reward pool addresses (see [`DataKey::BadgeNftAddress`]
//!    and [`DataKey::RewardPoolAddress`]).
use soroban_sdk::{contracttype, Address, BytesN};

/// A single course as stored on-chain.
///
/// `metadata_hash` typically resolves to an IPFS CID or an equivalent content
/// pointer; the bytes are 32-wide because `BytesN<32>` mirrors Soroban's
/// wasm-hash surface area.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Course {
    /// Original instructor address; transferable via
    /// [`crate::CourseRegistry::transfer_ownership`].
    pub instructor: Address,
    /// Number of modules required to complete the course.
    pub total_modules: u32,
    /// Pointer to course content off-chain.
    pub metadata_hash: BytesN<32>,
    /// Soft-delete flag. Falset blocks `enroll` but does NOT erase the record.
    pub active: bool,
}

/// Storage keys underlying [`crate::CourseRegistry`].
///
/// See the [module-level documentation](self) for the full layout.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Persistent: per-id `Course` record.
    Course(u32),
    /// Persistent: per-(learner, course) progress counter.
    Progress(Address, u32),
    /// Instance: monotonically incrementing course ID counter.
    CourseCount,
    /// Instance: registered admin `Address`.
    Admin,
    /// Instance: configured `badge-nft` contract address (optional).
    BadgeNftAddress,
    /// Instance: configured `reward-pool` contract address (optional).
    RewardPoolAddress,
}
