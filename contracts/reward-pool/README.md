# Reward Pool Contract

> Whitelisted payout vault for credentialed rewards in the KayStichs Protocol.
>
> Symbol on dashboards: **RP** — paired symbol: `usdc`.

## Overview

`reward-pool` holds USDC (or any SAC token wired at init) and dispenses it to
learners when called by an authorized spender. Spenders are vetted contracts
(`course-registry`, `quest-engine`) that have been registered via
`add_approved_spender(admin, spender)`.

The pool ships an **emergency circuit breaker** (`set_pause`) and a one-shot
**sweep** primitive (`emergency_sweep`) so admins can drain funds into a
recovery wallet if a vulnerability is found in the wild.

## Public API

| Function                              | Caller              | Notes                                                              |
|---------------------------------------|---------------------|--------------------------------------------------------------------|
| `initialize(admin, token)`            | deployer / admin    | One-shot; panics with `"Already initialized"` on retry.           |
| `add_approved_spender(admin, spender)`| admin               | Toggle-on; calling again with the same address is idempotent.      |
| `set_pause(admin, status)`            | admin               | `true` freezes `distribute_reward` and `batch_review`-style flows. |
| `distribute_reward(caller, learner, amount)` | whitelisted spender | Caller must be registered; amount > 0.                  |
| `fund_pool(donor, amount)`            | any signed donor    | Pulls `amount` tokens from the donor into the pool.               |
| `emergency_sweep(admin, recovery_wallet)` | admin           | Drains the entire token balance to `recovery_wallet`.             |
| `upgrade_contract(admin, new_wasm_hash)` | admin             | Standard wasm-upgrade flow; emits `ContractUpgraded`.              |

## Storage

```text
DataKey::Admin              -> Address          (instance)
DataKey::Token              -> Address          (instance)
DataKey::IsPaused           -> bool             (instance)
DataKey::Spender(Address)   -> bool             (persistent, one entry per spender)
```

The whitelist uses **persistent** storage because the set is unbounded by
design (every integrator gets its own key).

## Events

| Event                | Topics                           | Data             |
|----------------------|----------------------------------|------------------|
| `PoolInitialized`    | `admin`, `token`                 | —                |
| `SpenderAdded`       | `spender`                        | —                |
| `RewardDistributed`  | `caller`, `learner`              | `amount: i128`   |
| `PoolFunded`         | `donor`                          | `amount: i128`   |
| `EmergencySweep`     | `admin`, `recovery_wallet`       | `amount: i128`   |
| `ContractUpgraded`   | `admin`                          | `new_wasm_hash: BytesN<32>` |

Indexers MUST subscribe by `(contract_id, event_name)` and treat the topic
list as stable — additional topics are additive but existing ones are never
removed.

## Safety guarantees

1. **Pause-aware distributions.** `distribute_reward` checks the
   `IsPaused` flag *before* touching any other state, so a paused pool can
   never leak funds even via an in-flight spender call.
2. **Whitelist enforced at call time.** Each call must re-read the
   `Spender(caller)` entry; removal of spenders is intentionally out of
   scope for v1.0 but will land in v1.1 (see [`ROADMAP.md`](../../ROADMAP.md)).
3. **Sweep drains whole balance.** `emergency_sweep` transfers
   `balance(contract)` in one call rather than tracking amounts — this is
   **by design**, since during incident response the goal is to move
   *whatever is there* as fast as possible.
4. **No self-distribution.** Front-ends SHOULD ensure they never wire
   the `reward-pool` contract as an approved spender of itself.

## Building

```bash
cargo build -p reward-pool --release --target wasm32v1-none
```

## Testing

```bash
cargo test -p reward-pool --lib
```

The test suite covers:

- Initialize happy path + idempotency panic.
- Spender authorization (positive + negative).
- Distribute reward pause / paused-state / unauthorized-spender panics.
- Fund pool from multiple donors.
- Emergency sweep happy path, unauthorized-caller panic, zero-balance case.

## Acceptance criteria

- ✅ Only authorized spenders can call `distribute_reward`.
- ✅ Pause flag freezes distributions before any state mutation.
- ✅ Emergency sweep drains entire token balance atomically.
- ✅ All state changes emit a Soroban event with stable topics.
- ✅ Username-readable panic messages — see [`ERRORS.md`](../../ARCHITECTURE.md#error-reference).

## Future enhancements

- `remove_approved_spender(admin, spender)` for v1.1.
- Per-spender spend caps (`Spender(spender)` value as `i128` rather than bool).
- Rate-limit per learner (`reward_per_epoch(env, learner, since_ts) -> i128`).
