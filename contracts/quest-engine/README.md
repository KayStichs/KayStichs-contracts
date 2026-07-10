# Quest Engine Contract

> B2B bounties (build & explore) with staking-multiplier rewards for the KayStichs Protocol.
>
> Symbol on dashboards: **QE** — employers hook here to fund learner quests.

## Overview

`quest-engine` is the marketplace layer. Employers lock USDC into the
contract to create **build quests** (peer-reviewed deliverables) or
**explore quests** (off-chain activities the admin vouches for). The
contract then pays out to learners on approval, with the paid amount
scaled by the learner's `stake-vault` multiplier.

Fees (15 % of every approved build quest payout) route to the
`reward-pool` contract, feeding the on-chain treasury that downstream
protocols tap into.

## Public API

| Function                              | Caller             | Notes                                                              |
|---------------------------------------|--------------------|--------------------------------------------------------------------|
| `initialize(admin, token, reward_pool, stake_vault)` | deployer | One-shot; stores all four addresses and the quest counter. |
| `set_pause(admin, status)`            | admin              | Emergency circuit breaker; freezes `review_submission` and `batch_review_submissions`. |
| `create_build_quest(employer, reward_amount, metadata_hash)` | employer | Locks `reward_amount` tokens into this contract. |
| `create_explore_quest(admin, reward_amount, metadata_hash)` | admin   | Uses `reward_pool` as funding source on verify. |
| `get_quest(quest_id)`                 | any                | Returns `Option<Quest>`; `None` for unknown ids.                  |
| `submit_proof(learner, quest_id, proof_hash)` | learner     | Learners must be enrolled via *this* quest; one proof per learner. |
| `get_submission(learner, quest_id)`   | any                | Returns the `Option<Submission>`; status field drives review.     |
| `review_submission(employer, learner, quest_id, approve)` | employer | Pays out with multiplier; fee to reward-pool; updates `Submission.status`. |
| `refund_quest(employer, quest_id)`    | employer           | Returns the locked USDC; only active build quests are refundable.  |
| `batch_review_submissions(employer, quest_id, learners)` | employer | Bulk approve; emits `BatchReviewed`.                 |
| `verify_explore_quest(admin, learner, quest_id)` | admin  | Pays out from `reward-pool` for an Explore quest.                  |
| `upgrade_contract(admin, new_wasm_hash)` | admin           | Standard wasm-upgrade flow.                                        |

### Quest types

```text
Build   employer-funded  submission  →  employer approves  →  payout
Explore reward-pool-funded admin-verified  →  payout from reward-pool
fee = 15% of approved build quest payout
```

### Multiplier math

```text
base_learner_amount = quest.reward_amount - fee
multiplier          = stake_vault.get_multiplier(learner)  // 100, 120, 200
boosted             = (base_learner_amount * multiplier) / 100
learner_amount      = min(boosted, base_learner_amount)   // capped
```

The cap exists because the contract only holds `reward_amount` funds —
the multiplier scales the *promised* amount, not funds it from outside.

## Storage

```text
DataKey::Admin            -> Address        (instance)
DataKey::Token            -> Address        (instance, USDC SAC)
DataKey::RewardPool       -> Address        (instance)
DataKey::StakeVault       -> Address        (instance)
DataKey::QuestCounter     -> u32            (instance)
DataKey::IsPaused         -> bool           (instance)
DataKey::Quest(id)        -> Quest          (persistent, one per quest)
DataKey::Submission(addr, id) -> Submission (persistent, one per learner per quest)
```

## Events

| Event                  | Topics                          | Data                        |
|------------------------|---------------------------------|-----------------------------|
| `QuestCreated`         | `employer`, `quest_id`          | `reward_amount: i128`       |
| `ProofSubmitted`       | `learner`, `quest_id`           | `proof_hash: BytesN<32>`    |
| `SubmissionReviewed`   | `employer`, `learner`, `quest_id` | `approved: bool`          |
| `QuestRefunded`        | `employer`, `quest_id`          | `amount: i128`              |
| `BatchReviewed`        | `employer`, `quest_id`          | `approved_count: u32`       |
| `ExploreQuestVerified` | `admin`, `learner`, `quest_id`  | `amount: i128`              |
| `ContractUpgraded`     | `admin`                         | `new_wasm_hash: BytesN<32>` |

## Safety guarantees

- **Pause gates both reviews.** `review_submission` and
  `batch_review_submissions` early-return on `IsPaused`.
- **Submission is single-fire.** A second `submit_proof` panics.
- **Batch review is per-submission atomic.** A bad learner in the
  vector rolls back the whole batch — there are no half-approvals.
- **Refund is single-trip.** Once `quest.active = false`, subsequent
  `refund_quest` calls panic with `"Quest already inactive"`.

## Building & testing

```bash
cargo build -p quest-engine --release --target wasm32v1-none
cargo test  -p quest-engine --lib
```

## Acceptance criteria

- ✅ Build quests require employer approval; payouts respect multiplier.
- ✅ Explore quests route through `reward-pool`.
- ✅ Pause flag halts reviews per contract.
- ✅ Refund + batch-review paths emit `QuestRefunded`/`BatchReviewed`.

## Future enhancements

- Pickup of multi-claim quests (one quest, many winners).
- Time-bounded employer escrow release (e.g. 30 days).
- Off-chain verifier signatures for explore quests (oracles).
