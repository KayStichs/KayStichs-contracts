#!/usr/bin/env bash
#
# scripts/build.sh — wrap `stellar contract build` for the workspace.
#
# Usage:
#   ./scripts/build.sh             # build every contract in release mode
#   ./scripts/build.sh -- --package reward-pool   # forwarded
#
# Output:
#   contracts/target/wasm32v1-none/release/*.wasm

set -euo pipefail

cd "$(dirname "$0")/.."

echo "→ stellar contract build --workspace"
exec stellar contract build --workspace "$@"
