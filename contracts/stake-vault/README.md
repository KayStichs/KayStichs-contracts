# Stake Vault Contract

> Time-locked staking → multi-tier quest-multiplier for the KayStichs Protocol.
>
> Symbol on dashboards: **SV** — booster for `quest-engine` payouts.

## Overview

`stake-vault` accepts a single **SAC staking token** (e.g. KAYSTICHS on
testnet) and rewards stakers with a tier-based multiplier that
`quest-engine.review_submission` reads via `get_multiplier(learner)`. The
intent is alignment: medium-horizon stakers get bigger rewards on quest
approvals, but cannot unstake for **604 800 seconds (~7 days)** after
their last stake-write — disincentivizing multiplier gaming.

## Public API

| Function                                | Caller  | Notes                                                            |
|-----------------------------------------|---------|------------------------------------------------------------------|
| `initialize(admin, token)`              | deployer| One-shot.                                                        |
| `stake(user, amount)`                   | user    | Pulls `amount` tokens; resets the lock timestamp; amounts add.   |
| `unstake(user)`                         | user    | Returns the full stake *if* the lock window has elapsed.         |
| `get_multiplier(user)`                  | any     | Returns `100 / 120 / 200` based on stake amount (read-only).     |
| `upgrade_contract(admin, new_wasm_hash)`| admin   | Standard wasm-upgrade flow.                                       |

### Multiplier tiers

```text
amount >= 500  →  200   // 2.0x boost
amount >= 100  →  120   // 1.2x boost
amount <  100  →  100   // 1.0x (no boost)
```

Tiers are evaluated *at query time* — a learner who unstakes drops back
to `100` immediately. This is intentional so that unstaking doesn't
leave dangling multiplier expectations.

### Lock period

```text
lock_period = 604800 seconds  // ~7 days
unstakable if: env.ledger().timestamp() >= stake.lock_timestamp + lock_period
```

The lock is **reset every time you stake** — adding more tokens does not
extend the existing lock, it *restarts* it. Make this loud in your UI.

## Storage

```text
DataKey::Admin             -> Address       (instance)
DataKey::Token             -> Address       (instance)
DataKey::UserStake(addr)   -> StakeInfo     (persistent, one per staker)
```

`StakeInfo` is `{ amount: i128, lock_timestamp: u64 }`. Removal is the
unstake path (`remove(&DataKey::UserStake(user))`).

## Events

| Event                   | Topics              | Data                              |
|-------------------------|---------------------|-----------------------------------|
| `StakeVaultInitialized` | `admin`, `token`    | —                                 |
| `Staked`                | `user`              | `amount`, `total_staked`, `lock_timestamp` |
| `Unstaked`              | `user`              | `amount`                          |
| `ContractUpgraded`      | `admin`             | `new_wasm_hash: BytesN<32>`       |

`Staked` carries total + lock so an indexer can rebuild a timeline
without further calls.

## Safety guarantees

- **Lock period is enforced at the contract.** Stakers cannot unstake
  early, even via direct admin path — there is no bypass function.
- **Tokens are escrowed.** Staked tokens live inside the contract
  address, not in user wallets, until unstaked.
- **Pausable via token contract.** If the staking SAC is itself
  pausable, you should pause token transfers from `stake-vault` off-chain.
- **No re-staking race.** Sequential stake calls are fine; concurrent
  calls from the same address will serialize through the Soroban VM.

## Building & testing

```bash
cargo build -p stake-vault --release --target wasm32v1-none
cargo test  -p stake-vault --lib
```

## Acceptance criteria

- ✅ Lock of 604 800 s enforced on `unstake`.
- ✅ Tier thresholds match the table above.
- ✅ Multi-cumulative stake updates `amount` and resets lock timestamp.
- ✅ Unstake clears the storage entry atomically.

## Future enhancements

- Sub-tier breakdown (101-200, 201-499) for more granular boosts.
- Per-course cooldowns (you must wait 1 day *after* a quest approval
  to unlock the bonus on the next quest).
- Yield-bearing stake (split interest with treasury)
