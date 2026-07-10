# Pull Request

> **Title prefix**: `feat`, `fix`, `docs`, `chore`, `refactor`, `test`, or `ci`.

## Description

<!-- Answer: what does this PR do, and *why*? -->

## Type of Change

<!-- Pick one. -->

- [ ] Bug fix (non-breaking change that fixes an issue)
- [ ] New feature (non-breaking change that adds functionality)
- [ ] Breaking change (fix or feature that **would** cause existing functionality to break)
- [ ] Documentation only

## Affected Contracts

<!-- One per row that applies. -->

| Contract          | Touched? | Notes |
|-------------------|----------|-------|
| `course-registry` |          |       |
| `quest-engine`    |          |       |
| `reward-pool`     |          |       |
| `badge-nft`       |          |       |
| `governance`      |          |       |
| `stake-vault`     |          |       |

## Checks

The 4 local CI steps *must* pass before requesting review:

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test`
- [ ] `stellar contract build --workspace`
- [ ] `scripts/verify-rebrand.sh` (no forbidden tokens)

## Test Plan

<!-- How did you verify the change? New tests added? Manual? Edge cases? -->

## Public-ABI Impact

<!-- Critical for reviewers. -->

- [ ] No public-ABI change (function signatures, storage layout, event topics intact)
- [ ] Public-ABI change → see the **Breaking Changes** section in `CHANGELOG.md` for the v1.x→v2.x plan

## Linked Issues

<!-- `Closes #123`, `Refs #456`. -->

## Screenshots / Event Logs

<!-- Optional. -->

---

<!--
After opening the PR:
  • CI will auto-assign reviewers via `.github/CODEOWNERS`.
  • Maintainers squash-merge with a conventional-commit message at the root.
-->
