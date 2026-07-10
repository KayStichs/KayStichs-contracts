# Badge NFT Contract

> Soulbound course-completion badges for the KayStichs Protocol.
>
> Symbol on dashboards: **BN** — the protocol's only NFT-like primitive and the keystone of `governance` voting weight.

## Overview

`badge-nft` mints a small **non-transferable** record every time the
`course-registry` confirms a learner's final-module completion. Badges live
as `Vec<Badge>` per learner in **persistent** storage; once minted, the
record cannot be transferred, sold, or re-minted for the same course.

This contract has no token transfer surface — it is *soulbound* on principle.
The only way a creditor-side check works is if `governance` asks
`badge-nft.get_badges(voter)` and trusts that count as voting weight.

## Public API

| Function                              | Caller              | Notes                                                            |
|---------------------------------------|---------------------|------------------------------------------------------------------|
| `initialize(admin)`                   | deployer            | One-shot; admin is the authorized registry address.              |
| `mint_badge(caller, learner, course_id)` | authorized registry | Panics on duplicate `(learner, course_id)`.                      |
| `revoke_badge(admin, learner, course_id)`| admin              | Reverses a forgery; emits `BadgeRevoked` only if badge existed.  |
| `get_badges(learner)`                 | any                 | Returns `Vec<Badge>`. Empty for never-minted learners.           |
| `get_badge_count(learner)`            | any                 | Convenience wrapper around `get_badges(...).len()`.              |
| `has_badge(learner, course_id)`       | any                 | O(N) over badges — fine for the small expected N per learner.    |
| `upgrade_contract(admin, new_wasm_hash)`| admin              | Standard wasm-upgrade flow.                                      |

### Authorization model

`mint_badge` and `revoke_badge` both expect `caller.require_auth()` and
check that `caller == stored_admin`. In production, the deploying entity
sets `stored_admin` to the **course-registry contract address** — the only
caller expected to mint or revoke in normal flow.

## Storage

```text
DataKey::Admin              -> Address          (instance)
DataKey::UserBadges(addr)   -> Vec<Badge>       (persistent, one entry per learner)
```

A learner with K completions owns K elements in their `UserBadges` vector.
Total cost grows linearly with completions — keep this in mind for the
eventual TTL/bump schedule for testnet archival.

## Events

| Event            | Topics                  | Data             |
|------------------|-------------------------|------------------|
| `BadgeMinted`    | `learner`, `course_id`  | `minted_at: u64` |
| `BadgeRevoked`   | `learner`, `course_id`  | —                |
| `ContractUpgraded`| `admin`                | `new_wasm_hash: BytesN<32>` |

`minted_at` is the ledger timestamp at mint time, useful for off-chain
issuer proofs and time-based UI sorting.

## Soulbound enforcement

Two-sided check:

1. **No transfer function exists.** The contract deliberately does **not**
   expose any `transfer(...)` function. There is no path that would let a
   learner hand a badge to someone else.
2. **Duplicate mint panics.** A learner who already has `(course_id, X)`
   in their vector cannot mint a second one for the same course — the
   function iterates the existing vector and panics on hit.

> If a future version *does* add transfer primitives (don't), treat that
> as a breaking event and re-audit `governance.cast_vote` weighting.

## Building & testing

```bash
cargo build -p badge-nft --release --target wasm32v1-none
cargo test  -p badge-nft --lib
```

## Acceptance criteria

- ✅ Duplicate `(learner, course_id)` mint panics.
- ✅ `mint_badge` only callable by `stored_admin` (the registry contract).
- ✅ `revoke_badge` removes the correct entry and emits `BadgeRevoked`.
- ✅ All read-only helpers (`get_badges`, `get_badge_count`, `has_badge`)
  return deterministic results even for never-minted learners.

## Future enhancements

- TTL bumps helper: `bump_badge_storage_ttl(learner, course_id, ttl)`.
- Off-chain revocation registry (multi-sig recovery flow).
- Optional `Badge.uri: String` field (manifest pointer for richer UIs).
