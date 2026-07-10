# KayStichs Deployment Guide

> **Audience:** operators deploying KayStichs contracts to Stellar Testnet (or beyond).
> **All commands assume** you've installed the Stellar CLI and the WASM targets (see [`README.md` §Quickstart](./README.md#quickstart)).

## Table of Contents

1. [Pre-flight Checklist](#pre-flight-checklist)
2. [Network Selection](#network-selection)
3. [Deployment Sequence](#deployment-sequence)
4. [Initialization Parameters](#initialization-parameters)
5. [Verification Step](#verification-step)
6. [Smoke Tests](#smoke-tests)
7. [Recovery Plan](#recovery-plan)
8. [Troubleshooting](#troubleshooting)

---

## Pre-flight Checklist

- [ ] `stellar --version >= 23.0.1`
- [ ] `rustup target list --installed | grep wasm32`
- [ ] Testnet account funded with `XLM` and the `USDC` SAC trustline.
- [ ] Admin keypair backed up offline.
- [ ] GCC / clang installed (soroban-cli uses clang for WASM optimization).

## Network Selection

```bash
# Testnet
stellar network use testnet

# Futurenet (catches Soroban bleeding-edge features)
stellar network use futurenet

# Mainnet (after audit sign-off, do not deploy from a developer laptop)
stellar network use mainnet
```

Pin which network you'll deploy to via `NETWORK=testnet` env variable or `--network` flag on every invocation.

## Deployment Sequence

```bash
# 0. Pin admin identity
NETWORK=testnet
ADMIN=GBPK7... # your funded admin G-address
MY_TOKEN=CA... # USDC SAC contract

# 1. Build wasm
./scripts/build.sh

# 2. Deploy, in dependency order
REWARD_POOL_ID=$(./scripts/deploy.sh reward-pool.wasm --admin $ADMIN --token $MY_TOKEN)

# 3. Deploy the rest
STAKE_VAULT_ID=$(./scripts/deploy.sh stake-vault.wasm --admin $ADMIN --token $MY_TOKEN)
BADGE_NFT_ID=$(./scripts/deploy.sh badge-nft.wasm     --admin $ADMIN)
COURSE_REGISTRY_ID=$(./scripts/deploy.sh course-registry.wasm --admin $ADMIN)
QUEST_ENGINE_ID=$(./scripts/deploy.sh quest-engine.wasm --admin $ADMIN --token $MY_TOKEN \
                  --reward-pool $REWARD_POOL_ID --stake-vault $STAKE_VAULT_ID)
GOVERNANCE_ID=$(./scripts/deploy.sh governance.wasm --admin $ADMIN --badge-contract $BADGE_NFT_ID)

# 4. Wire cross-contract addresses
./scripts/deploy.sh course-registry.set_reward_pool_address --admin $ADMIN --reward-pool $REWARD_POOL_ID
./scripts/deploy.sh course-registry.set_badge_nft_address   --admin $ADMIN --badge-nft  $BADGE_NFT_ID
./scripts/deploy.sh reward-pool.add_approved_spender       --admin $ADMIN --spender   $COURSE_REGISTRY_ID
./scripts/deploy.sh reward-pool.add_approved_spender       --admin $ADMIN --spender   $QUEST_ENGINE_ID

# 5. Fund the pool
./scripts/deploy.sh reward-pool.fund_pool --donor $ADMIN --amount 10000000000
```

> **Why this order?** Booting a contract that depends on an unwired neighbor fails. Booting in this order lets us wire dependencies AFTER all addresses exist.

## Initialization Parameters

| Contract         | `initialize(... )` parameters                                              |
|------------------|---------------------------------------------------------------------------|
| `reward-pool`    | `admin`, `token`                                                          |
| `badge-nft`      | `admin`                                                                   |
| `course-registry`| `admin`                                                                   |
| `quest-engine`   | `admin`, `token`, `reward_pool`, `stake_vault`                            |
| `governance`     | `admin`, `badge_contract_address`                                         |
| `stake-vault`    | `admin`, `token`                                                          |

Each `initialize` is **idempotent** at the panic level: calling it twice will revert with `"Already initialized"`.

## Verification Step

Run after each deployment:

```bash
./scripts/verify-wasm.sh     # rebuilds wasm + runs cargo test
./scripts/verify-rebrand.sh  # checks all "KayStichs" tokens are intact
```

## Smoke Tests

End-to-end happy-path check:

```bash
./scripts/smoke.sh     # not yet implemented — see ROADMAP
```

A canonical smoke test does:

1. `initialize` all six contracts on a fresh Testnet address.
2. Wire cross-contract addresses (per *Address Wiring Cheat-sheet* in [`INTEGRATION.md`](./INTEGRATION.md)).
3. Run a single course: `create_course → enroll → complete_module`.
4. Run a single build quest: `create_build_quest → submit_proof → review_submission`.
5. Confirm a single stake: `stake → assert get_multiplier == 200` (or tier chosen).
6. Cleanup: `emergency_sweep` and remove test artifacts.

> ⏱️ The reference smoke-script lives in [`scripts/`](./scripts).

## Post-deploy Monitoring

After a real Testnet / Futurenet deploy, schedule the following checks into
an off-chain runner (cron / Kubernetes CronJob / Foundry):

| Check                            | Cadence | What to look for                                   |
|----------------------------------|---------|----------------------------------------------------|
| `ContractUpgraded` events        | **alert on any** | Wasm drift is a security incident unless coordinated. |
| `EmergencySweep` events          | alert on any | Sweeps should be rehearsed, not silent.        |
| `RewardPool.distribute_reward` failure rate | 5 min | Failure spike == a downstream contract is mis-wired. |
| `StakeVault.unstake` failure rate| 1 hour       | Failure spike == lock-period regression.            |
| `ProposalExecuted` events       | daily summary | Completed proposals should map to off-chain `upgrade_contract` calls. |

A minimal stack:

- **Indexer**: a Soroban RPC `getEvents` poller indexed by `(contract_id, event_name)`.
- **Storage**: flat files in `data/kaystichs-events/` for offline analysis.
- **Notifier**: PagerDuty / Slack webhook for the alert rows above.

## Recovery Plan

If the protocol is compromised:

1. `reward-pool.set_pause(admin, true)` — freeze payouts.
2. `quest-engine.set_pause(admin, true)` — freeze reviews.
3. `governance.execute_proposal(<upgrade_proposal_id>)` — approve the patched wasm.
4. Per-contract `upgrade_contract(admin, new_wasm_hash)`.
5. `reward-pool.emergency_sweep(admin, recovery_wallet)` — drain remaining tokens.
6. `course-registry.set_course_status(admin, id, false)` — disable compromised courses.

> Always pause BEFORE sweeping. See [SECURITY.md §Emergency Sweep](./SECURITY.md#emergency-sweep).

## Troubleshooting

| Symptom                          | Likely cause                                        |
|----------------------------------|-----------------------------------------------------|
| `Already initialized` panic      | Calling `initialize` twice. Don't.                  |
| `Contract not initialized` panic | Storage pointer missing. `initialize` first.        |
| `Unauthorized` on admin op       | Caller's `Address` ≠ stored admin. Check signer.    |
| `Not authorized spender` from RP | Add caller via `reward-pool.add_approved_spender`.  |
| `Lock period active` from SV     | Lock = 604 800 seconds (~7 days). Just wait.        |

---

*If something in this guide is broken on Testnet, open a `docs` PR — the protocol's reliability depends on this file being trustworthy.*
