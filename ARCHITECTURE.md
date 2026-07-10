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

## Sequence: Course Completion → Badge → Reward

```text
verifier       course-registry         badge-nft       reward-pool
  │                  │                       │               │
  │ complete_module  │                       │               │
  ├─────────────────►│                       │               │
  │                  │ increment_progress    │               │
  │                  │ ModuleCompleted event │               │
  │                  │ (if final module)     │               │
  │                  │ mint_badge            │               │
  │                  ├──────────────────────►│               │
  │                  │ (if RewardPool set)   │ BadgeMinted   │
  │                  │ distribute_reward     ├──────────────►│
  │                  ├─────────────────────────────────────►│
  │                  │ CourseCompleted event │RewardDistributed
  │ ◄────────────────┤                       │               │
```

A single `complete_module` call on the final module triggers **two** cross-contract hops.
If either fails, the prior state is rolled back by the host VM — there is no scenario in
which a learner obtains a *badge* but not a *reward* (or vice-versa) for the same completion.

---

## Sequence: Stake Multiplier on Quest Approval

```text
employer       quest-engine          stake-vault     reward-pool   learner
  │                  │                    │               │           │
  │ review_submission│                    │               │           │
  ├─────────────────►│                    │               │           │
  │                  │ get_multiplier     │               │           │
  │                  ├──────────────────►│                │           │
  │                  │◄──── 200 / 120 / 100                │           │
  │                  │ compute amount    │               │           │
  │                  │ + 15% protocol fee│               │           │
  │                  ├─── transfer fee ──────────────────►│           │
  │                  ├─── transfer boosted amount ────────────────────►│
  │ ◄SubmissionReviewed event              │               │           │
```

The contract **caps** the boosted payout to the post-fee balance, by design: employers
are expected to size bounties conservatively until the boost overflow rules are upgraded.

---

## Storage Cost Reference

> All sizes are approximate and will tighten once tests pin concrete keys.

| Contract         | Instance keys (init) | Persistent keys (per-record) | Hot path (read) |
|------------------|---------------------|------------------------------|-----------------|
| `course-registry`| 4                   | 1 per course + 1 per enrollment | `get_course`, `get_progress` |
| `quest-engine`   | 5                   | 1 per quest + 1 per submission | `get_quest`, `get_submission` |
| `reward-pool`    | 3                   | 1 per approved spender      | `distribute_reward` |
| `badge-nft`      | 1                   | 1 vec per learner           | `get_badges`, `has_badge` |
| `governance`     | 2                   | 1 per proposal + 1 per vote | `cast_vote`, `execute_proposal` |
| `stake-vault`    | 2                   | 1 per staker                | `get_multiplier` |

---

## Upgrade Mechanics

Every contract exposes `upgrade_contract(admin, new_wasm_hash)`. The flow is:

```text
Admin off-chain              Contract (on-chain)
  │                                │
  │ 1. Build new wasm offline      │
  │ 2. Upload hash via Stellar CLI │
  ├───────────────────────────────►│
  │                                │ 3. require_auth on admin
  │                                │ 4. assert admin == stored admin
  │                                │ 5. deployer.update_current_contract_wasm(new)
  │                                │ 6. emit ContractUpgraded event
```

**Confirmation:** always subscribe to `ContractUpgraded` events across all six
contracts and alert on drift. Indexers MUST treat the event as the *only* signal
that wasm changed.

### Why `BytesN<32>` and not `String` for the hash?

The wasm hash is fixed-width 32 bytes for a reason — Soroban's `deployer` API
expects it. Provide it via off-chain tooling (e.g. `stellar contract upload`
returns the hash you want to pass in).

---

## Error Reference

| Panic message                                  | Where                                    | Meaning                                                |
|------------------------------------------------|------------------------------------------|--------------------------------------------------------|
| `Already initialized`                           | every `initialize(...)`                  | Called twice; second call reverts.                     |
| `Contract not initialized` / `Not initialized` | most state-changing calls                | Instance `Admin` (or similar) absent.                  |
| `Unauthorized` / `Unauthorized: ...`           | any admin-gated call                     | Caller's address ≠ stored admin.                       |
| `Amount must be positive`                       | `reward_pool.distribute_reward`, `stake_vault.stake` | `amount <= 0`.                                |
| `Caller is not an authorized spender`          | `reward_pool.distribute_reward`          | Spender whitelist missing the caller.                 |
| `Contract is paused`                            | `reward_pool.distribute_reward`, `quest-engine.review_submission`, `quest-engine.batch_review_submissions` | Admin toggled pause. |
| `Lock period active`                            | `stake_vault.unstake`                    | Less than 604 800 s (~7 days) since last stake.        |
| `Course already completed`                      | `course_registry.complete_module`        | `progress >= total_modules`.                            |
| `Learner already enrolled`                      | `course_registry.enroll`                 | Duplicate `Progress(learner, id)` key.                 |
| `Submission is not pending review`              | `quest-engine.review_submission`         | Status != Pending.                                     |
| `Voting still active` / `Voting ended`         | `governance.execute_proposal` / `cancel_proposal` | Timestamp conditions not met.                  |

> Anything not in the table above is a bug — please open an issue.

---

*For deployment specifics, see [`DEPLOYMENT.md`](./DEPLOYMENT.md). For threat model, see [`SECURITY.md`](./SECURITY.md).*
