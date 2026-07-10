# KayStichs Security Policy

> **Audience:** security researchers, integrators, auditors.
> If you find a bug, please follow the [Disclosure Process](#disclosure-process) — *do not* file a public issue.

## Table of Contents

1. [Supported Versions](#supported-versions)
2. [Threat Model](#threat-model)
3. [Disclosure Process](#disclosure-process)
4. [Known Mitigations](#known-mitigations)
5. [Emergency Sweep](#emergency-sweep)
6. [Secure Integration Checklist](#secure-integration-checklist)
7. [Audits](#audits)

---

## Supported Versions

| Version  | Status          | Bug fixes accepted? |
|----------|-----------------|---------------------|
| `1.x.x`  | Active          | ✅ Yes              |
| `0.x.x`  | Bug-fix only    | ⚠️ Critical only    |

Anything older is community-supported on a best-effort basis.

## Threat Model

The adversary we defend against is:

- **A malicious on-chain integrator** who tries to redirect reward flows or whitelist unauthorized spenders.
- **A malicious contract admin** who tries to upgrade wasm to a backdoored version *without* consent from governance.
- **A malicious learner** who tries to mint a duplicate badge or claim a reward modulo `total_modules`.
- **A compromised dependency** that injects code via a transitive crate.

What we **do not** defend against:

- Compromised admin private keys (those are the admin's problem).
- Loss of token due to a non-KayStichs token contract being wired in.
- Frontend UX issues (we ship wasm; the frontend is its own attack surface).

## Disclosure Process

1. **Email** `security@kaystichs.dev` with subject `[DISCLOSURE] <short summary>`.
2. Wait for an acknowledgement within **48 hours**.
3. After we patch + deploy a fix, we co-disclose with the reporter — credit given unless anonymity was requested.
4. Critical-severity issues *may* be eligible for a **bug bounty** (see [bounty policy TBD]).

> **Do not** open a public GitHub issue, GitHub Discussion, or tweet about a suspected vulnerability before disclosure. Doing so degrades our ability to patch and send coordinated disclosures.

## Known Mitigations

| Class                  | Mitigation in code                                                          |
|------------------------|-----------------------------------------------------------------------------|
| Cross-contract reentrancy| State updates written **before** external calls (checks-effects-interactions)|
| Whitelisted spenders   | `reward_pool.distribute_reward` requires `DataKey::Spender(caller) == true` |
| Role bypass            | Every privileged function triple-checks: signature + stored admin + equality |
| Wasm hijack            | `upgrade_contract` requires caller's `require_auth` and stored admin match  |
| Soulbound enforcement  | `badge-nft.mint_badge` panics on duplicate `(learner, course_id)`           |
| Pause / circuit-breaker| Both `reward-pool` and `quest-engine` have `set_pause` admin-gated kill switches |
| Time-lock bypass       | `stake-vault.unstake` enforces a 7-day lock (`lock_period_active`)          |

## Emergency Sweep

If a **critical vulnerability** is discovered in the wild:

1. Admin calls `reward_pool.set_pause(admin, true)` **first** — this freezes all distributions.
2. Admin calls `reward_pool.emergency_sweep(admin, recovery_wallet)` to drain the pool to a safe address.
3. Admin calls `quest_engine.set_pause(admin, true)` to freeze all flows.
4. Admin then coordinates with the governance contract for `upgrade_contract` of any compromised code path.

> **Always pause BEFORE sweeping.** Sweeping while still distributing is racy and tokens can leak.

## Secure Integration Checklist

Use this when integrating KayStichs into a UI or off-chain backend:

- [ ] Always check contract pause state before triggering UI flows.
- [ ] Pin wasm-hashes for `upgrade_contract` opcodes — alert on drift.
- [ ] Subscribe to all six `ContractUpgraded` events.
- [ ] Use the **`current_contract_address()`** of the caller in auth assertions, never its `Address` literal.
- [ ] Reject integration if `get_admin()` returns a stale / unexpected admin address.

### Contributor checklist (when writing new code)

- [ ] **OCR**: Ownership / Cross-call / Reentrancy — see [`CONTRIBUTING.md` §Reviewer's checklist](./CONTRIBUTING.md#reviewers-checklist).
- [ ] **Storage writes before external calls** — checks-effects-interactions pattern.
- [ ] **Auth always calls `require_auth` before reading stored admin.**
- [ ] **Magic numbers in `const` declarations**, not literal `123456789` sprinkles.
- [ ] **Panic messages are human-readable** + namespaced (`"...: <context>"`).
- [ ] **Events emitted AND topics chosen**, see [`ARCHITECTURE §Event Taxonomy`](./ARCHITECTURE.md#event-taxonomy).
- [ ] **Test the negative path** — every `panic(... expected = "X")` test exists.

## Audits

| Date         | Auditor    | Scope                              | Report                                                         |
|--------------|-----------|------------------------------------|----------------------------------------------------------------|
| 2026-04      | Internal  | `reward-pool`, `badge-nft`         | [`course_completion_payout_audit.md`](./course_completion_payout_audit.md) |
| 2026-06      | Internal  | Full workspace pre-release         | [`FINAL_CI_REPORT.md`](./FINAL_CI_REPORT.md)                   |

Future audits (to be commissioned): TBD.

---

*This file is normative for the workspace. If you find a mismatch, open a `docs` PR — not a security-only PR — so the rest of the protocol knows.*
