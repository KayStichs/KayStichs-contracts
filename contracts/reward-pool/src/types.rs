//! Storage key namespace for the `reward-pool` contract.
//!
//! ## Layout
//!
//! | Variant         | Backing tier  | Notes                                |
//! |-----------------|---------------|--------------------------------------|
//! | `Admin`         | instance      | Stored once at `initialize`.         |
//! | `Token`         | instance      | The SAC token wired for payouts.     |
//! | `IsPaused`      | instance      | Boolean emergency-circuit-breaker.   |
//! | `Spender(addr)` | persistent    | One entry per approved spender.       |
//!
//! ## Why toggle via boolean (not existential)
//!
//! Spenders are stored as `bool` rather than as `Option<()>` style keys
//! so the same key can be flipped back to `true` cheaply, and so the
//! `get(key)` cost is constant rather than scoped to lookup-result.
//!
//! Adding/removing spenders is *additive* in v1.0; a `remove_approved_spender`
//! path is tracked in [`ARCHITECTURE.md`](../../../ARCHITECTURE.md#error-reference) §Roadmap.
use soroban_sdk::{contracttype, Address};

/// Storage keys underlying [`crate::contract_impl::RewardPool`].
///
/// See the [module-level documentation](self) for the full layout.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Instance: admin `Address` registered at `initialize`.
    Admin,
    /// Instance: SAC token wired at `initialize`.
    Token,
    /// Instance: pause circuit-breaker (boolean).
    IsPaused,
    /// Persistent: whitelist entry per approved spender address.
    Spender(Address),
}
