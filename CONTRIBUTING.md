# Contributing to KayStichs Contracts

> **TL;DR** — fork → branch → small atomic commit → `cargo fmt/clippy/test` clean → open a PR against `main`.
> All new code lands in **conventional-commit** style and **passes CI** before review.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Pull Request Process](#pull-request-process)
3. [Commit Conventions](#commit-conventions)
4. [Local Toolchain](#local-toolchain)
5. [Running Tests Locally](#running-tests-locally)
6. [Adding New View Functions](#adding-new-view-functions)
7. [Adding a New Contract](#adding-a-new-contract)
8. [Style Guide](#style-guide)

---

## Code of Conduct

This project follows the Contributor Covenant — see [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md).

By participating you agree to:

- Be welcoming to newcomers.
- Focus on **what is best** for the protocol.
- Show empathy towards other community members.

## Pull Request Process

1. **Branch off `main`.** Use the prefix `feat/`, `fix/`, `docs/`, `chore/`, etc. matching the change.
2. **One logical change per PR.** Avoid drive-by refactors; split if you can describe them with "and".
3. **All four CI checks must pass:**
   - `cargo fmt --all -- --check`
   - `cargo clippy --all-targets --all-features -- -D warnings`
   - `cargo test`
   - `stellar contract build --workspace` (no Wasm errors)
4. **Open the PR with the template.** The bot will auto-assign reviewers based on `.github/CODEOWNERS`.
5. **Squash-merge** with a conventional-commit message at merge time. The bot will re-write your commit subject.

### Reviewer's checklist

Reviewers should run through this checklist before approval:

- [ ] **API additive only** — no renames or removals on the contract surface.
- [ ] **OCR (Ownership / Cross-call / Reentrancy)** — admin calls check `require_auth`,
  cross-contract entrypoints use `current_contract_address()`, state writes precede external calls.
- [ ] **Documentation** — public function has rustdoc; new events logged in `ARCHITECTURE.md`.
- [ ] **Tests** — at least one positive and one negative test per new branch.
- [ ] **CI green** — commits the conventional-prefix policy.

If any item is unchecked, request changes — do not merge with TODO comments in code.

## Commit Conventions

We use **Conventional Commits 1.0.0** with these prefixes:

| Prefix     | When                                                       |
|------------|------------------------------------------------------------|
| `feat`     | A new user-facing feature                                  |
| `fix`      | A bug fix                                                  |
| `docs`     | Doc-only changes (no source code change)                    |
| `style`    | Format-only (whitespace, missing semicolons)               |
| `refactor` | Internal change that neither fixes a bug nor adds a feature |
| `test`     | Adding tests only                                          |
| `chore`    | Build / CI / tooling / release / housekeeping              |
| `ci`       | GitHub Actions / Dependabot / pre-commit                   |

Examples:

```text
feat(course-registry): add is_enrolled view function
fix(reward-pool): prevent reward distribution when learner == caller
docs(stake-vault): document multiplier tiers
chore(rebrand): refresh brand assets
test(quest-engine): cover explore-quest pause interaction
ci: bump stellar-cli to v23.0.2
```

Keep commits **atomic** — one logical change per commit, even if the PR is multi-commit.

## Local Toolchain

Run the bootstrap once:

```bash
rustup target add wasm32-unknown-unknown wasm32v1-none
cargo install stellar-cli --version "^23.0.1"
```

A handy wrapper:

```bash
./scripts/check-tools.sh
./scripts/install-tools.sh       # if anything is missing
```

## Running Tests Locally

```bash
cd contracts/
cargo test
```

Or the entire workflow via `scripts/`:

```bash
./scripts/test.sh       # cargo test
./scripts/fmt.sh        # cargo fmt
./scripts/lint.sh       # cargo clippy
./scripts/build.sh      # stellar contract build
```

## Adding New View Functions

1. The function MUST be **additive** — never rename or remove an existing public function.
2. The function SHOULD live within the existing `#[contractimpl] impl X` block (lets `stellar contract build` re-export it as a wasm symbol).
3. For `badge-nft` and `reward-pool` (feature-gated modules), put your function INSIDE `#[cfg(feature = "contract")] mod contract_impl { ... }`.
4. Add tests in `src/test.rs` of the same crate.
5. Update the relevant README and `ARCHITECTURE.md` event / function lists.

### Rustdoc minimum standard

Every new *public* function in a contract MUST have:

```rust
/// One-line summary.
///
/// # Arguments
/// * `param_name` - what it represents
///
/// # Returns
/// What the function returns (omit if void).
///
/// # Panics
/// Conditions that trigger a panic + the literal message.
///
/// # Events
/// Which event(s) this function emits (omit if none).
pub fn foo(...) { ... }
```

Use `cargo doc --workspace --no-deps --open` locally to preview.

---

## Adding a New Contract

1. Create `contracts/<your-contract>/` with the standard sub-tree (`src/lib.rs`, `src/types.rs`, `src/test.rs`, `Cargo.toml`, `Makefile`, `README.md`).
2. Add the crate to `contracts/Cargo.toml` `[workspace] members` list.
3. Cross-contract clients go in the new crate's `src/lib.rs` using `#[contractclient(name = "XClient")]`.
4. Add CI checks: `cargo fmt`, `cargo clippy`, `cargo test` (the workspace already does this).
5. Update the README contract matrix in this file.

### Contract recipe checklist

| Item                                              | ✓ |
|---------------------------------------------------|---|
| `Cargo.toml`: `soroban-sdk = { workspace = true }` |   |
| `Cargo.toml`: `[lib] crate-type = ["lib","cdylib"]`|   |
| `src/lib.rs`: module-level rustdoc on top         |   |
| `src/types.rs`: typed `DataKey` enum with `#[contracttype]` |   |
| `src/test.rs`: helper `setup()` that mocks auths  |   |
| `tests/` folder empty or contains integration tests |   |
| `README.md`: One-page Tour + Build section         |   |

## Style Guide

- **All public functions MUST have a rustdoc comment** describing arguments, return, panic conditions.
- **Panic messages MUST be human-readable.** No `"unauthorized"`, prefer `"Unauthorized: caller is not the protocol admin"`.
- **Avoid `unwrap()` and `expect()` on storage reads**. Always `expect("Human message")` or `.unwrap_or(default)`.
- **No magic numbers**. Constants like `BASE_REWARD: i128 = 10_0000000;` belong at the top of the file.

---

*If anything in this file is wrong or unclear, open an issue — labeling `docs` — and propose the fix in the same PR if it's small.*
