#!/usr/bin/env bash
#
# scripts/check-tools.sh — verify the local toolchain matches CI.
#
# Exit codes:
#   0 — all tools present at expected versions
#   1 — one or more tools missing or out of range
#   2 — could not parse `cargo --version` / `stellar --version`
#
# Usage:
#   ./scripts/check-tools.sh
#
# Pinned expectations live in [`CONTRIBUTING.md` §Local Toolchain](../CONTRIBUTING.md#local-toolchain).

set -euo pipefail

EXPECTED_STELLAR_CLI="^23\.0\.[0-9]+"
EXPECTED_RUSTC="^1\.[0-9]+"

err=0

echo "→ Checking Rust toolchain"
if ! command -v rustc >/dev/null 2>&1; then
    echo "  ✗ rustc not found. Install via:  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    err=1
else
    v="$(rustc --version | awk '{print $2}')"
    if [[ "$v" =~ $EXPECTED_RUSTC ]]; then
        echo "  ✓ rustc $v"
    else
        echo "  ✗ rustc $v (does not match $EXPECTED_RUSTC)"
        err=1
    fi
fi

echo "→ Checking wasm32 targets"
for target in wasm32-unknown-unknown wasm32v1-none; do
    if rustup target list --installed 2>/dev/null | grep -q "$target"; then
        echo "  ✓ $target"
    else
        echo "  ✗ $target (install via:  rustup target add $target)"
        err=1
    fi
done

echo "→ Checking Soroban / Stellar CLI"
if ! command -v stellar >/dev/null 2>&1; then
    echo "  ✗ stellar CLI not found. Install via:  cargo install stellar-cli --version \"^23.0.1\""
    err=1
else
    v="$(stellar --version | awk '{print $2}')"
    if [[ "$v" =~ $EXPECTED_STELLAR_CLI ]]; then
        echo "  ✓ stellar CLI $v"
    else
        echo "  ✗ stellar CLI $v (does not match $EXPECTED_STELLAR_CLI)"
        err=1
    fi
fi

echo "→ Checking cargo and rustfmt"
if ! command -v cargo >/dev/null 2>&1; then
    echo "  ✗ cargo not found alongside rustc — re-run rustup."
    err=1
else
    echo "  ✓ cargo $(cargo --version | awk '{print $2}')"
fi

if ! cargo fmt --version >/dev/null 2>&1; then
    echo "  ✗ cargo-fmt not installed."
    err=1
else
    echo "  ✓ cargo-fmt present"
fi

if ! cargo clippy --version >/dev/null 2>&1; then
    echo "  ✗ cargo-clippy not installed."
    err=1
else
    echo "  ✓ cargo-clippy present"
fi

echo
if [[ $err -eq 0 ]]; then
    echo "All checks passed. Run ./scripts/test.sh to verify the workspace."
else
    echo "One or more tools missing — follow the printed instructions."
fi

exit $err
