# KayStichs Integration Guide

> **Audience:** backend engineers wiring KayStichs into a UI, off-chain indexer, or third-party service.

The six contracts in this workspace are designed to **compose end-to-end** without bespoke glue. This guide covers the canonical integration patterns, in order of complexity.

## Table of Contents

1. [Pattern A — Read-only Front-end](#pattern-a--read-only-front-end)
2. [Pattern B — Course Completion Off-chain Verifier](#pattern-b--course-completion-off-chain-verifier)
3. [Pattern C — Explore-Quest Operator Backend](#pattern-c--explore-quest-operator-backend)
4. [Pattern D — DAO Operator](#pattern-d--dao-operator)
5. [Pattern E — Indexer](#pattern-e--indexer)
6. [Address Wiring Cheat-sheet](#address-wiring-cheat-sheet)

---

## Pattern A — Read-only Front-end

The simplest integration: a UI that subscribes to events and renders state.

```text
Paginated list of courses:
   course-registry.get_course(id)         for each (1..course_count)
                                          skip if course.active == false

A learner's progress in a course:
   course-registry.get_progress(learner, course_id)
                                          clamp to course.total_modules

A learner's badges:
   badge-nft.get_badges(learner)           render each by course metadata

A learner's stake tier:
   stake-vault.get_multiplier(learner)    100 / 120 / 200 display in UI
```

**No admin keys. No auth required.** Every call above is read-only and infallible.

## Pattern B — Course Completion Off-chain Verifier

You run an off-chain backend that issues quizzes and only then bumps the learner's progress on-chain.

```text
1. Learner enrolls
   course-registry.enroll(learner, course_id)

2. Backend verifies quiz → module N is done
   course-registry.complete_module(admin, learner, course_id)

3. On the final module:
   course-registry → badge-nft.mint_badge(...)   [automatic]
   course-registry → reward-pool.distribute_reward(...) [automatic if wired]
```

The verifier must hold the **course-registry admin key**. Consider:

- Splitting that key between a multisig and an HSM-backed signer.
- Rotating the key on verifier staff turnover — see [ARCHITECTURE — Authorization Model](./ARCHITECTURE.md#authorization-model).

## Pattern C — Explore-Quest Operator Backend

For quests that grade off-chain activity ("attend a workshop", "post a tweet"):

```text
1. Admin creates a quest
   quest-engine.create_explore_quest(admin, reward_amount, metadata_hash)
                                              → QuestCreated event with quest_id

2. Backend (oracle) verifies the off-chain action

3. Admin (or oracle same admin key) calls
   quest-engine.verify_explore_quest(admin, learner, quest_id)
                                              → distribute_reward from reward-pool
```

**Pre-flight**:

- `reward-pool.add_approved_spender(admin, quest_engine_address)` — once at deploy time.
- `reward-pool.fund_pool(donor, amount)` — donors deposit USDC into the pool.

## Pattern D — DAO Operator

For governance — a badge-weighted DAO decides where to take the protocol next.

```text
1. Proposer opens a proposal  (read ARCHITECTURE §3 consequence)

2. Badge holders cast votes
   governance.cast_vote(voter, proposal_id, support)

3. After the voting window ends
   governance.execute_proposal(proposal_id)    ← creates ProposalExecuted event
```

> **Note:** as of v1.0.0 `create_proposal` is intentionally out-of-scope — proposals are seeded by an off-chain pipeline that already knows the badge-holders. See [ROADMAP.md](./ROADMAP.md#deferred).

## Pattern E — Indexer

Subscribe to events emitted by the six contracts:

```text
for event in ledger_events:
    case event.contract_id of:
        course-registry → handle Course*, OwnershipTransferred, ContractUpgraded
        quest-engine   → handle Quest*, Submission*, BatchReviewed, ExploreQuestVerified, ContractUpgraded
        reward-pool    → handle Pool*, SpenderAdded, RewardDistributed, EmergencySweep, ContractUpgraded
        badge-nft      → handle BadgeMinted, BadgeRevoked, ContractUpgraded
        governance     → handle ProposalExecuted, ProposalCancelled, ContractUpgraded
        stake-vault    → handle StakeVaultInitialized, Staked, Unstaked, ContractUpgraded
```

`soroban_sdk::contractevent!` indexes events by name + topics; indexers SHOULD inspect both.

## Address Wiring Cheat-sheet

A canonical Testnet deploy wires:

```text
reward-pool → admin: governance multisig
            → approved spenders: { course-registry, quest-engine }
            → token: USDC SAC contract

course-registry → admin: governance multisig
               → reward_pool_address: <reward-pool address above>
               → badge_nft_address:   <badge-nft address>

badge-nft     → admin: course-registry (only the registry mints)

quest-engine  → admin: governance multisig
              → token: USDC SAC
              → reward_pool: <reward-pool>
              → stake_vault: <stake-vault>

stake-vault   → admin: governance multisig
              → token: KAYSTICHS staking SAC (Testnet mock)

governance    → admin: governance multisig
              → badge_contract_address: <badge-nft>
```

`scripts/deploy.sh` automates this on Testnet.

> **Hermetic test wiring** is identical except admins are random
> `Address::generate(&env)` values in the unit tests — see
> `contracts/*/test.rs`.

---

## Pattern F — TypeScript via @stellar/stellar-sdk

A minimal subscribe-to-events harness:

```typescript
import { StellarRpc } from "@stellar/stellar-sdk";

const rpc = new StellarRpc.Server("https://soroban-testnet.stellar.org");
const cursor = "now";

const events = await rpc.getEvents({
  cursor,
  topics: [["*", "module_completed"]],   // matches Course* / ModuleCompleted
  limit: 50,
});
for (const ev of events.events!) {
  console.log(ev.contractId, ev.topic, ev.value);
}
```

For state reads (read-only Pattern A from above):

```typescript
const courseRegistry = new StellarSdk.Contract(courseRegistryId);
call.course_registry.get_course({ id: 1 }, { simulate: true });
```

TypeScript bindings are produced by:

```bash
stellar contract bindings typescript \
    --wasm contracts/target/wasm32v1-none/release/course_registry.wasm \
    --network testnet \
    --output ./bindings/ts
```

---

## Pattern G — Rust via `soroban-sdk` Client

For a Rust server-side indexer or backend:

```rust,ignore
use course_registry::CourseRegistryClient;
use soroban_sdk::{Env, BytesN};

let env = Env::default();
let contract_id = BytesN::from_array(&env, &[0u8; 32]);
let client = CourseRegistryClient::new(&env, &contract_id);

let course = client.get_course(&1u32);
println!("course metadata hash = {:?}", course.metadata_hash);
```

Async via `tokio`:

```rust,ignore
let rpc = soroban_client::Client::new("https://soroban-testnet.stellar.org").await?;
let course = rpc.call(&course_registry_id, "get_course", vec![1u32.into()]).await?;
```

---

## Pattern H — Python via `py-stellar-base`

Useful for quick scripts and dashboards:

```python
from stellar_sdk import SorobanServer, scval

server = SorobanServer("https://soroban-testnet.stellar.org")
resp = server.simulate_transaction(
    source=admin_pubkey,
    contract_id=course_registry_id,
    function_name="get_course",
    parameters=[scval.to_uint32(1)],
)
print(resp.result)
```

---

## Pattern I — Raw HTTP / curl

For inspection when no SDK fits:

```bash
curl -sSf \
    -d '{"jsonrpc":"2.0","id":0,"method":"getEvents","params":{
        "startLedger":"latest",
        "filters": [{
            "contractIds":["'"$COURSE_REGISTRY"'"],
            "topics":[["*","course_created"]]
        }]
    }}' \
    -H "Content-Type: application/json" \
    https://soroban-testnet.stellar.org | jq .
```

> Requires `jq` for pretty-printing.

---

*Need help integrating? File a question in **Discussions** with the `integration` tag.*
