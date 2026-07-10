# Quest Engine — per-contract Makefile. Mirrors course-registry/Makefile.

default: build

all: test

.PHONY: build test fmt fmt-check lint doc clean clean-deep watch verify
build:
	stellar contract build
	@ls -l target/wasm32v1-none/release/quest_engine.wasm || true

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
