# KayStichs Roadmap

> **Audience:** anyone curious where KayStichs is heading.
> **Cadence:** this document is updated at every minor / major release.

## Table of Contents

1. [Now (v1.0 — released)](#now-v10--released)
2. [Next (v1.1 — Q3 2026)](#next-v11--q3-2026)
3. [Later (v1.2+)](#later-v12)
4. [Deferred (no commitment)](#deferred-no-commitment)
5. [Anti-Roadmap](#anti-roadmap)

---

## Now (v1.0 — released)

| Item                                         | Status |
|----------------------------------------------|--------|
| Six-contract workspace compiles clean        | ✅      |
| Cargo fmt / clippy / test green              | ✅      |
| Soroban SDK 23 surface                       | ✅      |
| Wasm outputs produced for every member       | ✅      |
| Cross-contract composition wired             | ✅      |
| Bootstrap docs (this file + README etc.)     | ✅      |
| Rebrand artifacts (`BRANDING.md`, `wasm-info.json`) | ✅  |

## Next (v1.1 — Q3 2026)

| Item                                                              | Owner      |
|-------------------------------------------------------------------|------------|
| `governance.create_proposal` so proposals can be opened on-chain | governance |
| `reward-pool.remove_approved_spender` for revocation flows       | reward-pool|
| `quest-engine.get_leaderboard(quest_id, top_n)`                   | quest-engine|
| Pagination primitives (cursor-based) for course / quest / voter lists          | cross-cutting |
| Snapshot of test_snapshots in CI via `cargo install cargo-insta`  | ci         |
| Audit review with external firm on `reward-pool` + `badge-nft`   | ops        |

## Later (v1.2+)

| Item                                                                            | Rationale |
|---------------------------------------------------------------------------------|-----------|
| Native multi-currency reward pools (USDC + EURC baskets)                        | Composability |
| Pluggable multiplier policy (replacing the hard-coded tiers in `stake-vault`)  | Operations more flexibility |
| Cross-chain credential relay to the Soroban-equivalent of an off-ramp protocol | Reach |
| Soulbound reputation score derived from badge recency                            | UX       |

## Deferred (no commitment)

| Item                                                                | Why deferred |
|---------------------------------------------------------------------|--------------|
| On-chain course content (full modules in storage)                   | Storage cost; off-chain IPFS is fine |
| Native fiat onramps                                                 | Regulatory moat; out of engineering scope |
| Anonymous credentials (zk-attestations)                             | Requires Soroban feature still on roadmap |
| DAO-as-Service integration with non-KayStichs protocols             | Lacks partner commitment |
| `governance.create_proposal`'s parameterized dynamic payload        | Hard to bound attack surface; defer until needed |

## Anti-Roadmap

We will **not** in 2026–2027:

- ✗ Add a "token pre-mint" path. New tokens go through `mint_policy = strict_sbt`.
- ✗ Implement semi-transferable NFTs. The badge soulbound surface is documented and will stay.
- ✗ Add a rehypothecation of stakes (i.e. using `stake-vault` tokens as collateral elsewhere). We freeze stakes to encourage long horizon.
- ✗ Deploy to Mainnet from anything but a hardware wallet + 3-of-5 multisig.

---

## Milestone Cadence

| Quarter | Planned major work                                                  |
|---------|---------------------------------------------------------------------|
| 2026-Q3 | `reward-pool` per-spender caps; `governance.create_proposal` (v1.1). |
| 2026-Q4 | On-chain proposal execution hooks; cross-crate snapshot testing.    |
| 2027-Q1 | Multi-currency reward pools (basket SAC).                           |
| 2027-Q2 | Cross-chain credential relay; `soulbound-reputation` derived from `minted_at`. |

> A milestone migrates *between* buckets when scope changes. A bump from
> `Later` to `Next` must be approved by at least one CODEOWNER before
> the change lands in `ROADMAP.md`.

---

*Anyone can open a `roadmap` PR proposing changes — please include a section under "Why deferred" or "Anti-roadmap" when downgrading a candidate.*
