# Governance Contract

> Badge-weighted DAO proposals for the KayStichs Protocol.
>
> Symbol on dashboards: **GV** — every action requires *earned* badge weight.

## Overview

`governance` is the *deliberation layer* of the protocol. Learners cast
votes on proposals using the number of badges they hold as their weight.
The contract does *not* actually mutate protocol parameters — execution
simply marks a proposal as accepted, signalling to the admin off-chain
that an upgrade or contract change is approved.

It is intentionally **minimal**:

- **No on-chain proposal creation yet** — proposals are seeded by an
  off-chain pipeline that has access to badge-holder data. Tracked in
  [`ROADMAP.md §Deferred`](../../ROADMAP.md#deferred).
- **No on-chain execution side-effects** — `execute_proposal` is a
  bookkeeping call that flips `executed = true`. The admin takes the
  approved proposal off-chain and applies it.
- **No quorum configuration** — every vote's weight *is* the quorum.

## Public API

| Function                          | Caller         | Notes                                                                  |
|-----------------------------------|----------------|------------------------------------------------------------------------|
| `initialize(admin, badge_contract)`| deployer      | Stores admin + `badge_contract_address`.                                |
| `get_proposal(proposal_id)`       | any            | Panics with `"Proposal not found"` if absent.                          |
| `cast_vote(voter, proposal_id, support)`| voter     | Weight = `badge_nft.get_badges(voter).len()`. Panics on double-vote.   |
| `cancel_proposal(caller, proposal_id)`| proposer or admin | Only before voting period ends, and only if not yet executed.    |
| `execute_proposal(proposal_id)`   | any            | Window-end + majority-passed + not-yet-executed → marks executed.       |
| `upgrade_contract(admin, new_wasm_hash)`| admin      | Standard wasm-upgrade flow.                                             |

### Voting weight (knobs)

```text
weight = badge_nft.get_badges(voter).len()  // integer >= 0
```

- A learner with no badges can vote but their vote contributes `0`.
- The contract does NOT scale weight by anything else; badge count *is*
  the design choice.
- `get_badges.length()` is a cross-contract call — `governance` waits for
  the response before mutating state.

### Execution semantics

`execute_proposal` is **idempotent** in the panic sense:

1. Voting window must have ended (`timestamp > end_time`).
2. `votes_for > votes_against` (strict majority).
3. `executed == false`.

If any condition is false, the function panics. There is no half-state.

## Storage

```text
DataKey::Admin                       -> Address         (instance)
badge (Symbol)                       -> Address         (instance, badge contract)
DataKey::Proposal(id)                -> Proposal        (persistent, one per proposal)
DataKey::UserVote(Address, id)       -> bool            (persistent, prevents double-vote)
```

## Events

| Event                | Topics                  | Data              |
|----------------------|-------------------------|-------------------|
| `ProposalExecuted`   | `proposal_id`           | `proposer: Address`|
| `ProposalCancelled`  | `proposal_id`           | `cancelled_by: Address` |
| `ContractUpgraded`   | `admin`                 | `new_wasm_hash: BytesN<32>` |

Cast votes do **not** emit an event — the increment of `votes_for` or
`votes_against` is the only authoritative record. Add vote-events via
indexer poll of `get_proposal(id)` instead.

## Safety guarantees

- **Double-vote prevention.** The first vote writes
  `DataKey::UserVote(voter, proposal_id) = true`; a second vote panics.
- **Time-gated execution.** `cancel_proposal` and `execute_proposal`
  both consult `env.ledger().timestamp()` to enforce temporal order.
- **Strict-majority rule.** Tied votes *reject* the proposal — there is
  no tie-breaker path on-chain.
- **Cross-contract auth.** `badge-nft` is not gated by an admin key
  itself in the read path; but `governance` is the only contract that
  uses badge count as weight, so the dependency is well-scoped.

## Building & testing

```bash
cargo build -p governance --release --target wasm32v1-none
cargo test  -p governance --lib
```

## Acceptance criteria

- ✅ Double-voting by the same address panics.
- ✅ Strict-majority executes; ties and rejections trigger no execute-event.
- ✅ Window-bound `cancel_proposal` and `execute_proposal`.
- ✅ Cancellation by either proposer *or* admin (via stored admin) works.

## Future enhancements

- `create_proposal(proposer, end_time)` for on-chain proposal creation.
- Quorum threshold with `quorum_votes` field on `Proposal`.
- Vote delegations: `delegate(vote_from, vote_to, proposal_id)`.
