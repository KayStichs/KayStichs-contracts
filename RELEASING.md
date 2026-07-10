# KayStichs Release Playbook

> **Audience:** release managers (and curious contributors).
> **Goal:** standardize how the KayStichs smart-contract workspace cuts a release end-to-end.

## Table of Contents

1. [When to Release](#when-to-release)
2. [Versioning Rules](#versioning-rules)
3. [Pre-release Checklist](#pre-release-checklist)
4. [Release Steps](#release-steps)
5. [Post-release Verification](#post-release-verification)
6. [Hot-fix Procedure](#hot-fix-procedure)
7. [Yanking a Release](#yanking-a-release)

---

## When to Release

- **Minor** (`vMAJOR.MINOR.0`): when a unit of user-visible functionality lands. Aim for 6-week cadence.
- **Patch** (`vMAJOR.MINOR.PATCH`): for bug fixes that don't change the public ABI or storage layout.
- **Major** (`vMAJOR.0.0`): when the public ABI **or** any `DataKey` enum **or** any public function signature changes. Coordinate with auditors.

## Versioning Rules

- All contracts share the **same workspace version** (`vX.Y.Z`) — keep them synchronized.
- `repr` of enums (e.g. `quest-engine::QuestType`) MUST NOT change between patch versions — it's an ABI break.
- If a public function's signature changes, the contract gets a major bump *and* the existing function must remain (deprecated) for one major.

## Pre-release Checklist

- [ ] `CHANGELOG.md` updated with the new release section.
- [ ] `wasm-info.json` bumped to the new version.
- [ ] `cargo fmt --all -- --check` passes.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes.
- [ ] `cargo test` is green.
- [ ] `stellar contract build --workspace` produces Wasm without warnings.
- [ ] `scripts/verify-rebrand.sh` passes.
- [ ] Any new public functions documented in the contract's README and `ARCHITECTURE.md`.

## Release Steps

```bash
# 1. Cut a release branch.
git checkout main
git pull
git checkout -b release/vX.Y.Z

# 2. Bump versions.
#    - contracts/*/Cargo.toml
#    - wasm-info.json
./scripts/bump-version.sh vX.Y.Z       # TBD - see ROADMAP

# 3. Final CHANGELOG pass.
$EDITOR CHANGELOG.md

# 4. Smoke test.
./scripts/test.sh && ./scripts/lint.sh && ./scripts/build.sh

# 5. Tag + push branch (NO direct tag to main).
git tag -a vX.Y.Z -m "KayStichs contracts vX.Y.Z"
git push origin release/vX.Y.Z

# 6. Open a PR titled "Release vX.Y.Z" and let CI run.
gh pr create --title "Release vX.Y.Z" --body "CHANGELOG.md" --base main

# 7. After merge, tag main.
git checkout main && git pull
git tag -a vX.Y.Z -m "KayStichs contracts vX.Y.Z"
git push origin vX.Y.Z
```

## Post-release Verification

- [ ] CI is green on the tag commit.
- [ ] `stellar contract install --wasm target/wasm32v1-none/release/<x>.wasm --source-account <test>` succeeds.
- [ ] Pinned `wasm-info.json` matches the tag.
- [ ] `BRANDING.md` reflect the actual published architecture (if changed).

## Hot-fix Procedure

Skip the branch dance for **patch** hot-fixes:

```bash
git checkout main && git pull
git checkout -b hotfix/<short-tag>
# ...fix
$EDITOR CHANGELOG.md
git commit -am 'fix: short description'
./scripts/test.sh
git push origin HEAD
gh pr create --title "Hotfix vX.Y.Z+1" --base main
```

After merge, tag and follow steps 6–7 above.

## Yanking a Release

If a release has a critical flaw:

1. Mark the tag as deprecated in `CHANGELOG.md`.
2. Pin everyone to a newer tag.
3. **Do not delete** the tag — historical reproducibility matters more than cleanliness.

---

*Anyone with merge rights can drive a release — please coordinate in `#releases` first.*
