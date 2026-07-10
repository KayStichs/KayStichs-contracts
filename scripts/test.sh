#!/usr/bin/env bash
#
# scripts/test.sh — wrap `cargo test` for the KayStichs contracts workspace.
#
# Usage:
#   ./scripts/test.sh             # run all tests
#   ./scripts/test.sh -p reward-pool   # run a single contract
#   ./scripts/test.sh -- --nocapture   # forward extra args to cargo test
#
# Exit codes mirror `cargo test`.

set -euo pipefail

cd "$(dirname "$0")/../contracts"

echo "→ cargo test (workspace)"
exec cargo test "$@"
