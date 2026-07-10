# KayStichs Brand Guide

> **Project brand:** KayStichs Protocol
> **Repository:** kaystichs-contracts
> **Adopted:** v1.0.0 (post-`Learnault` migration)

This document captures the canonical brand identity for this workspace. It exists so the rebrand is **auditable**, **treatable as a contract**, and **grep-verifiable** in CI.

## Table of Contents

1. [Canonical Tokens](#canonical-tokens)
2. [Forbidden Tokens](#forbidden-tokens)
3. [Visual Identity](#visual-identity)
4. [Brand Architecture](#brand-architecture)
5. [Provenance](#provenance)
6. [Maintenance](#maintenance)

---

## Canonical Tokens

Use these spelling, casing, and capitalization forms everywhere — in source, docs, ADRs, scripts, and CI artifacts.

| Surface        | Canonical Token | Sample                                |
|----------------|-----------------|---------------------------------------|
| Product name   | `KayStichs`     | "KayStichs Protocol"                  |
| Slug           | `kaystichs`     | `kaystichs-contracts`                 |
| Reserve casing | `KAYSTICHS`     | `KAYSTICHS_V1_ARCHITECTURE` header    |
| Repository     | `kaystichs/kaystichs-contracts` | GitHub URL                |
| Cargo org label| `KayStichs Team` | `authors = ["KayStichs Team"]`        |

The reserved casing `KAYSTICHS` should appear **only** in machine-generated artifacts (build hashes, audit IDs, lockfile fingerprints).

## Forbidden Tokens

The following strings are forbidden anywhere in the workspace outside of historical audit artifacts:

- `Learnault`, `learnault`, `LEARNAULT`
- Any historical slug that pre-dates the 2026 `Learnault → KayStichs` migration

Run `scripts/verify-rebrand.sh` before tagging a release. CI must fail if any forbidden token is reintroduced outside `BRANDING.md`, `CHANGELOG.md` migration notes, or `course_completion_payout_audit.md` / `FINAL_CI_REPORT.md` historical references.

## Visual Identity

The KayStichs mark represents the moment-by-moment craft of a learner mastering a skill — "stitch" being the atomic action of a tailors-programmable economy.

When generating docs, ads or social posts:

- **Headline color**: indigo `#4F46E5`
- **Accent color**: rose `#F43F5E`
- **Type**: humanist sans, e.g. Inter / Noto Sans
- **Voice**: instructive but warm — never gamified

## Brand Architecture

KayStichs is composed of:

1. **KayStichs Protocol** — the on-chain layer (this repo).
2. **KayStichs Studio** — the instructor-facing UI (out of scope, separate repo).
3. **KayStichs Registry** — the public catalog of soulbound badges (out of scope, separate repo).

Anything that talks about *all three* must be titled **KayStichs**.

## Provenance

This rebrand was adopted after the workspace migrated off `Learnault`. The migration commit is the boundary — everything before is `Learnault`, everything after is `KayStichs`. Audit documents pre-dating the migration (`course_completion_payout_audit.md`, the original `FINAL_CI_REPORT.md`) may still reference `Learnault` for historical accuracy and must not be edited.

## Maintenance

- Bumping the brand version? Update this document **before** opening the PR.
- Adding a new product line under the KayStichs family? Add a row to §Brand Architecture.
- The CI rebrand-verification script lives at `scripts/verify-rebrand.sh` — keep it in lock-step with §Forbidden Tokens.

---

*This file is the canonical contract for brand identity. If reality disagrees with this file, reality wins — but the next PR must reconcile.*
