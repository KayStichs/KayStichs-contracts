# KayStichs Contracts — Changelog

> All notable changes to this workspace are recorded here, in [Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/) format.
> The project adheres to [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html).

## Table of Contents

- [Unreleased](#unreleased)
- [1.0.0 — 2026-07-10](#100--2026-07-10)
- [0.x — pre-release history](#0x--pre-release-history)

---

## Unreleased

### Added

- **Per-contract READMEs** for `reward-pool`, `badge-nft`, `governance`,
  `quest-engine`, `stake-vault`. Each is 90+ lines and aligns public API
  with the actual `BytesN<32>`-based implementation.
- **`course-registry` README realigned** with the current `BytesN<32>` API;
  Symbol-based API example replaced with full-learner journey sample.
- Drift source: this CHANGELOG now records commit-by-commit summaries so
  release notes have a clean pre-canned input.

### Changed

- `README.md`: added Why KayStichs section, Highlights, Protocol Overview,
  Repository-Layout script table, and Quickstart deeper workflow.
- `ARCHITECTURE.md`: added sequence diagrams, storage cost reference,
  upgrade mechanics, error reference, and a glossary; expanded ToC.
- `CONTRIBUTING.md`: added reviewer OCR checklist, rustdoc minimum standard,
  and contract recipe checklist.
- `DEPLOYMENT.md`: added Smoke Tests detail and Post-deploy Monitoring section.
- `INTEGRATION.md`: added Patterns F–I (TypeScript, Rust, Python, raw HTTP/curl).
- `SECURITY.md`: added Contributor OCR checklist.
- `FAQ.md`: added Integrations section with `getEvents` recipe.
- `ROADMAP.md`: added Milestone Cadence and bucket-promotion discipline.
- `RELEASING.md`: added Breaking-change discipline section.
- `BRANDING.md`: added Usage Do/Don't table.

### Deprecated

_None._

### Removed

_None._

### Fixed

- **`course-registry/README.md` API drift** — replaced Symbol-shaped example
  with `BytesN::from_array(&env, &[0u8; 32])`-shaped example to match current code.

### Security

- Documentation surface for emergency-sweep procedure reinforced
  across `README.md`, `ARCHITECTURE.md`, and per-contract READMEs
  (see `SECURITY.md` §Emergency Sweep).

## [1.0.0] — 2026-07-10

> **Codename:** *Stitch One*

> First stable release of the **KayStichs Protocol** smart contracts. The previous legacy codebase was published under the **`Learnault`** name; this release marks the cut-over to the KayStichs brand and sign-off on the 6-contract architecture.

### Added

- **Brand & docs bootstrap**
  - Top-level `README.md`, `ARCHITECTURE.md`, `CONTRIBUTING.md`, `SECURITY.md`
  - `BRANDING.md` as the canonical brand contract
  - `wasm-info.json` for build-time metadata
  - `scripts/verify-rebrand.sh` to fail CI on forbidden legacy tokens
- **Additive view functions** (one logical commit per function):
  - `course-registry`: `is_course_active`, `get_course_instructor`, `is_enrolled`, `get_module_count`
  - `badge-nft`: `has_any_badges`, `get_latest_mint_time`, `is_initialized`
  - `reward-pool`: `is_spender_approved`, `is_pool_paused`
  - `stake-vault`: `get_staked_amount`, `get_lock_timestamp`
  - `governance`: `is_proposal_executable`, `get_proposal_votes`
  - `quest-engine`: `get_quest_reward_amount`, `is_quest_active`
- **Ergonomic scripts under `scripts/`** mapping every CI step to a local command.
- **GitHub templates**: bug report, feature request, PR template, CODEOWNERS, dependabot.yml.

### Changed

- Rebrand metadata across the workspace from `Learnault` → `KayStichs`.
- CI working-directory pinned to `./contracts/` (no change, documented).

### Deprecated

_None._

### Removed

_None._

### Fixed

- Removed a stale comment in `course-registry/src/lib.rs` referencing the legacy `get_course` tuple surface.

### Security

- Reinforced the documentation surface for the **emergency sweep** procedure in `reward-pool`.

## [0.x] — pre-release history

The pre-1.0 history was published under the legacy `Learnault` brand. Tagged releases for migration reference are:

- `0.6.0` — last release under the legacy brand.
- `0.5.0` — soulbound badge enforcement + course-token integration.
- `0.4.0` — initial Soroban SDK 23 workspace consolidation.
- `0.3.0` — quest-engine staking multiplier wired through.
- `0.2.0` — governance `execute_proposal` landed.
- `0.1.0` — initial workspace split (this monorepo).

Pre-1.0 history is preserved in git tags for traceability but is **not** in this changelog.

---

[1.0.0]: #100--2026-07-10
[0.x]: #0x--pre-release-history
[Unreleased]: #unreleased
