# KayStichs Contracts — FAQ

> Likely questions, short answers.

## General

### What is KayStichs?

A learner-credential + skill-to-bounty protocol on Stellar. The smart contracts in this repo form its on-chain core.

### How does this differ from Learnault?

It doesn't — earlier work was branded `Learnault`. The current `KayStichs` brand is the canonical continuation; see [`CHANGELOG.md`](./CHANGELOG.md) and [`BRANDING.md`](./BRANDING.md) for the audit trail.

### Which Soroban SDK version?

`23`, pinned via the workspace `Cargo.toml`.

### Where do I ask questions?

GitHub Discussions → **Q&A** category. For bugs use GitHub Issues. For security use `security@kaystichs.dev` (see [`SECURITY.md`](./SECURITY.md)).

## Contracts

### Why six contracts and not one mega-contract?

Blast radius. Any one contract can be paused, replaced, or quarantined without taking the protocol down. The cost is more deployment ops; we accept that trade-off.

### Why is the badge soulbound?

If a badge is transferable, learners can sell credentials they didn't earn — that breaks the meaning of completion. Soulbound is a *feature*, not a limitation.

### Why a 7-day stake lock?

The 7-day lock aligns staking with medium-horizon commitments; shorter windows would let users game the multiplier between quests.

### Is there a way to remove an approved spender from `reward-pool`?

Not yet (v1.0). Tracked in [ROADMAP §Next](./ROADMAP.md#next-v11--q3-2026).

### Can I deploy only some of the contracts?

Yes. Each contract has its own `initialize(...)` and you can deploy them piecewise. The cross-contract wiring in [`DEPLOYMENT.md`](./DEPLOYMENT.md) is the canonical "all-in" config; partial-deploy configs are welcome in PRs.

## Operations

### How do I rotate the admin key?

Today: deploy a new "shadow" admin that calls `upgrade_contract` with patched wasm that updates storage. Daylight operation; no in-place admin rotation in v1.0. Tracked in [ROADMAP §Later](./ROADMAP.md#later-v12).

### How do I drain the pool in an emergency?

`reward-pool.set_pause(admin, true)` then `reward-pool.emergency_sweep(admin, recovery_wallet)`. See [`SECURITY.md §Emergency Sweep`](./SECURITY.md#emergency-sweep).

### My CI is failing on a `cargo fmt` check — what?

Run `cargo fmt --all` locally before pushing. CI expects zero diffs.

## Contributing

### Can I add a new view function to any contract?

Yes — please match the existing `#[contractimpl] impl X` block. Add tests in `src/test.rs`. Update README + this FAQ if the function is user-visible.

### How do I open my first PR?

Read [`CONTRIBUTING.md`](./CONTRIBUTING.md) carefully. Use the PR template and confirm all four CI checks pass *before* requesting review.

### What's the bug bounty?

See [`SECURITY.md` §Disclosure Process](./SECURITY.md#disclosure-process) — formal bounty TBD. Critical issues receive coordinated credit.

---

*Have a question that's not here? File it as a new `docs` issue and we'll grow this list.*
