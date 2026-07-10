# KayStichs Architecture

> **Audience:** protocol integrators, smart-contract auditors, on-chain analysts.
> **Goal:** explain *how* the six KayStichs contracts compose — and *why* each is bounded the way it is.

## Table of Contents

1. [High-Level Diagram](#high-level-diagram)
2. [Contract Responsibilities](#contract-responsibilities)
3. [Cross-Contract Data Flow](#cross-contract-data-flow)
4. [Authorization Model](#authorization-model)
5. [Storage Model](#storage-model)
6. [Event Taxonomy](#event-taxonomy)
7. [Failure Modes & Defensive Calls](#failure-modes--defensive-calls)

---

## High-Level Diagram

```text
            ┌────────────────────────────────────────────────────┐
            │                Learner (off-chain)                │
            └───────────────────┬────────────────────────────────┘
                                │  enroll / submit_proof / stake
                                ▼
┌──────────────┐  module done  ┌─────────────────┐  distribute_reward  ┌──────────────┐
│ course-registry│ ───────────▶│  reward-pool   │ ◀────────────────── │ quest-engine │
└──────┬───────┘               └─────────────────┘                     └──────┬───────┘
       │ mint_badge                                                       │ get_multiplier
       ▼                                                                   ▼
┌──────────────┐   get_badges  ┌─────────────┐                          ┌──────────────┐
│  badge-nft   │ ───────────▶ │ governance  │                          │ stake-vault  │
└──────────────┘              └─────────────┘                          └──────────────┘
```

## Contract Responsibilities

| Contract         | Owns                                                       | Does NOT Own                                |
|------------------|------------------------------------------------------------|---------------------------------------------|
| `course-registry`| Course metadata, learner progress, completion trigger      | Direct payouts, badge state ownership       |
| `quest-engine`   | Quest lifecycle, employer escrows, multiplier rewards     | Treasury, governance                        |
| `reward-pool`    | Whitelisted payout vault with pause + emergency sweep      | Quest definitions, learner credentials      |
| `badge-nft`      | Soulbound badge minting / revocation                       | Course lifecycle, payout math               |
| `governance`     | Badge-weighted DAO proposals                               | Course / quest business logic               |
| `stake-vault`    | Time-locked stake → multi-tier multiplier                  | Payouts, badges                             |

The cardinal rule: **no contract owns what another contract owns**. This keeps blast radius small if any single contract is compromised.

## Cross-Contract Data Flow

1. **Course completion → Badge mint → Reward payout**
   1. Verifier calls `course_registry.complete_module(learner, course_id)`.
   2. On `new_progress == total_modules`:
      - `course_registry` cross-calls `badge_nft.mint_badge(itself, learner, course_id)`.
      - `course_registry` cross-calls `reward_pool.distribute_reward(itself, learner, 10 USDC)`.
2. **Build-Quest approval with staking multiplier**
   1. Employer calls `quest_engine.review_submission(employer, learner, quest_id, true)`.
   2. Quest engine fetches `stake_vault.get_multiplier(learner)`.
   3. `(reward - 15%-fee) × multiplier/100` is paid to learner; `15%` is routed to `reward_pool`.
3. **Explore-Quest verification**
   1. Admin calls `quest_engine.verify_explore_quest(admin, learner, quest_id)`.
   2. Quest engine cross-calls `reward_pool.distribute_reward(itself, learner, amount)`.
4. **Governance voting weight**
   1. Voter calls `governance.cast_vote(voter, proposal_id, support)`.
   2. Governance cross-calls `badge_nft.get_badges(voter)` and uses the badge count as vote weight.

## Authorization Model

Every state-changing call in the workspace follows the **same 3-step pattern**:

```rust
caller.require_auth();                                             // 1. crypto signature
let stored_admin: Address = env.storage().instance().get(&Admin)?;  // 2. fetch canonical admin
assert!(caller == stored_admin, "Unauthorized");                    // 3. role check
```

Cross-contract callers authenticate their OWN `Address` (`env.current_contract_address()`); the callee treats the caller as an authorized contract if it sits in an appropriate whitelist (e.g. `reward_pool.Spender(...)`).

**Every contract exposes `upgrade_contract(admin, wasm_hash)`** which is gated by the same 3-step pattern. This is the only mutation that changes code; all other mutations change state.

## Storage Model

All contracts use the **two-tier** Soroban storage model:

- **Instance storage** — admin address, token address, mint allowance, pause flag, *root pointers to other contracts*.
- **Persistent storage** — per-record data (courses, quests, votes, stakes, etc.).

Why split?

- Instance storage is cheap to read but expensive to expand indefinitely.
- Persistent storage is the canonical place for unbounded growth (each `Learn`er's badges, each quest, etc.).

Quota is not a concern on Testnet; production deployments must budget for it explicitly.

## Event Taxonomy

Every state change emits a Soroban event with at-most-2 indexed topics. We follow a flat namespace — no nested hierarchies:

| Contract         | Events                                                                 |
|------------------|------------------------------------------------------------------------|
| `course-registry`| `MetadataUpdated`, `CourseCreated`, `CourseStatusChanged`, `OwnershipTransferred`, `ModuleCompleted`, `CourseCompleted`, `ContractUpgraded` |
| `quest-engine`   | `QuestCreated`, `ProofSubmitted`, `SubmissionReviewed`, `QuestRefunded`, `BatchReviewed`, `ExploreQuestVerified`, `ContractUpgraded` |
| `reward-pool`    | `PoolInitialized`, `SpenderAdded`, `RewardDistributed`, `PoolFunded`, `EmergencySweep`, `ContractUpgraded` |
| `badge-nft`      | `BadgeMinted`, `BadgeRevoked`, `ContractUpgraded`                       |
| `governance`     | `ProposalExecuted`, `ProposalCancelled`, `ContractUpgraded`             |
| `stake-vault`    | `StakeVaultInitialized`, `Staked`, `Unstaked`, `ContractUpgraded`       |

Indexer code SHOULD subscribe by `(contract_id, event_name)` and avoid relying on raw topic ordering.

## Failure Modes & Defensive Calls

- **`reward_pool.distribute_reward` when contract is paused**: panics early — *caller* must check pause state ahead-of-time.
- **`reward_pool.distribute_reward` when caller is NOT a whitelisted spender**: panics with `"Caller is not an authorized spender"`.
- **`course_registry.complete_module` when `RewardPoolAddress` not configured**: gracefully *skips* reward distribution — completion still succeeds, no token transfer.
- **`course_registry.complete_module` on a deactivated course**: panics — `course_not_found` or `course_inactive`.
- **`stake_vault.unstake` while lock period active**: panics — `lock_period_active`.

Graceful degradation is preferred to silent failure wherever possible: the protocol must let honest users complete their work even if integrations are partially wired up.

---

*For deployment specifics, see [`DEPLOYMENT.md`](./DEPLOYMENT.md). For threat model, see [`SECURITY.md`](./SECURITY.md).*
