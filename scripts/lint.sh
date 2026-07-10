#!/usr/bin/env bash
#
# scripts/lint.sh — wrap `cargo clippy` to match CI's strictness.
#
# Usage:
#   ./scripts/lint.sh
#   ./scripts/lint.sh -p reward-pool  # one contract
#
# Run from any directory.

set -euo pipefail

cd "$(dirname "$0")/../contracts"

echo "→ cargo fmt --all -- --check"
cargo fmt --all -- --check

echo "→ cargo clippy --all-targets --all-features -- -D warnings"
exec cargo clippy --all-targets --all-features -- -D warnings "$@"
