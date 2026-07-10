# Course Registry — per-contract Makefile.
#
# Targets:
#   make            - alias for `build` (wasm artefact)
#   make test       - cargo test (after a build)
#   make build      - `stellar contract build` + show resulting .wasm sizes
#   make fmt        - cargo fmt --all (write mode)
#   make fmt-check  - cargo fmt --all -- --check (CI gate)
#   make lint       - cargo clippy -- -D warnings
#   make doc        - cargo doc --no-deps --open
#   make clean      - cargo clean
#   make clean-deep - cargo clean + remove target/wasm*-none balances
#   make watch      - cargo watch (re-test on edit) — requires `cargo install cargo-watch`
#   make verify     - end-to-end check: fmt + clippy + test

default: build

all: test

.PHONY: build test fmt fmt-check lint doc clean clean-deep watch verify
build:
	stellar contract build
	@ls -l target/wasm32v1-none/release/course_registry.wasm || true

test: build
	cargo test

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --all-targets --all-features -- -D warnings

doc:
	cargo doc --no-deps --open

clean:
	cargo clean

clean-deep: clean
	@rm -rf contracts/target/wasm*-none

watch:
	cargo watch -x test

verify: fmt-check lint test
