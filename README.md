# KayStichs Protocol — Smart Contracts

> **The on-chain credential & micro-grant layer for the KayStichs Protocol.**

[![Soroban SDK](https://img.shields.io/badge/soroban--sdk-23-4F46E5)](https://developers.stellar.org/docs/build/smart-contracts/)
[![format](https://img.shields.io/badge/cargo--fmt-passing-22C55E)](#)
[![lint](https://img.shields.io/badge/cargo--clippy-clean-22C55E)](#)
[![tests](https://img.shields.io/badge/cargo--test-passing-22C55E)](#)
[![brand](https://img.shields.io/badge/brand-KayStichs-F43F5E)](./BRANDING.md)

This workspace houses the six Soroban smart contracts that compose the **KayStichs Protocol** — a learner-owned credential and skill-to-bounty economy on Stellar.

## Table of Contents

1. [What's Inside](#whats-inside)
2. [Quickstart](#quickstart)
3. [Repository Layout](#repository-layout)
4. [Contract Matrix](#contract-matrix)
5. [Build, Test, Lint](#build-test-lint)
6. [Documentation Map](#documentation-map)
7. [Contributing](#contributing)
8. [License](#license)

---

## What's Inside

| Contract         | Symbol | Purpose                                                            |
|------------------|--------|--------------------------------------------------------------------|
| `course-registry`| `CR`   | On-chain course catalog + module completion + reward trigger.      |
| `quest-engine`   | `QE`   | B2B bounties (build / explore) with staking-multiplier rewards.    |
| `reward-pool`    | `RP`   | Whitelisted reward vault for credentialed payouts.                 |
| `badge-nft`      | `BN`   | Soulbound course-completion badges (non-transferable).             |
| `governance`     | `GV`   | Badge-weighted proposal + voting + execution.                      |
| `stake-vault`    | `SV`   | Time-locked stake → multi-tier quest multiplier (100 / 120 / 200). |

## Quickstart

```bash
# 1. Install Soroban CLI (>= v23.0.1) and Rust with `wasm32-unknown-unknown` target.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install stellar-cli --version "^23.0.1"
rustup target add wasm32-unknown-unknown wasm32v1-none

# 2. Clone and test.
git clone https://github.com/kaystichs/kaystichs-contracts.git
cd kaystichs-contracts/contracts
cargo test

# 3. Build wasm artifacts.
stellar contract build --workspace
# Outputs:  contracts/target/wasm32v1-none/release/*.wasm
```

Full testnet deployment walkthrough lives at [`DEPLOYMENT.md`](./DEPLOYMENT.md).

## Repository Layout

```text
.
├── README.md                ← you are here
├── ARCHITECTURE.md          ← cross-contract data-flow diagrams
├── BRANDING.md              ← canonical brand contract
├── CHANGELOG.md             ← semver release notes
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md          ← PRs, issues, commit conventions
├── DEPLOYMENT.md            ← testnet/mainnet deployment playbook
├── FAQ.md                   ← frequently asked questions
├── INTEGRATION.md           ← how contracts compose end-to-end
├── LICENSE
├── RELEASING.md             ← release processes & version bumps
├── ROADMAP.md               ← what's next
├── SECURITY.md              ← threat model & disclosure policy
├── wasm-info.json           ← machine-readable brand + build metadata
├── scripts/                 ← ergonomics: build / test / lint / deploy
└── contracts/               ← the actual Soroban workspace
    ├── Cargo.toml           ← workspace root
    ├── course-registry/
    ├── quest-engine/
    ├── reward-pool/
    ├── badge-nft/
    ├── governance/
    └── stake-vault/
```

## Contract Matrix

| Contract         | Public fn count | Wasm target         | Storage model              |
|------------------|----------------:|---------------------|----------------------------|
| `course-registry`| 12              | `wasm32v1-none`     | Persistent + Instance      |
| `quest-engine`   | 11              | `wasm32v1-none`     | Persistent + Instance      |
| `reward-pool`    | 7               | `wasm32v1-none`     | Persistent + Instance      |
| `badge-nft`      | 7               | `wasm32v1-none`     | Persistent + Instance      |
| `governance`     | 6               | `wasm32v1-none`     | Persistent + Instance      |
| `stake-vault`    | 5               | `wasm32v1-none`     | Persistent + Instance      |

> Counts include `initialize`, `upgrade_contract` and view functions.

## Build, Test, Lint

From `contracts/`:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
stellar contract build --workspace
```

Equivalent wrappers live under [`scripts/`](./scripts). CI runs the same steps on push & PR.

## Documentation Map

| File                                  | Audience                |
|---------------------------------------|-------------------------|
| [`ARCHITECTURE.md`](./ARCHITECTURE.md)| Protocol integrators    |
| [`DEPLOYMENT.md`](./DEPLOYMENT.md)    | Operators               |
| [`INTEGRATION.md`](./INTEGRATION.md)  | Front-end / backend devs|
| [`SECURITY.md`](./SECURITY.md)        | Auditors, contributors  |
| [`CONTRIBUTING.md`](./CONTRIBUTING.md)| New contributors        |
| [`ROADMAP.md`](./ROADMAP.md)          | Anyone                  |
| [`FAQ.md`](./FAQ.md)                  | Anyone                  |
| [`RELEASING.md`](./RELEASING.md)      | Release managers        |
| [`BRANDING.md`](./BRANDING.md)        | Comm / marketing        |
| [`CHANGELOG.md`](./CHANGELOG.md)      | Anyone tracking changes |

## Contributing

Read [`CONTRIBUTING.md`](./CONTRIBUTING.md) — commits follow conventional commit prefixes (`feat`, `fix`, `chore`, `docs`, `test`, `ci`, `refactor`), and every PR must pass `cargo fmt --check`, `cargo clippy -D warnings`, and `cargo test`.

## License

Released under the MIT License — see [`LICENSE`](./LICENSE).

---

*KayStichs is part of the broader KayStichs Protocol. If you ship integration code that uses these contracts, please add a link back to this repo in your docs.*
