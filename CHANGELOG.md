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

_(none yet — opened with workspace bootstrap on 2026-07-10)_

### Changed

_(none yet)_

### Deprecated

_(none yet)_

### Removed

_(none yet)_

### Fixed

_(none yet)_

### Security

_(none yet)_

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
