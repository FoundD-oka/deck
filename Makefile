.PHONY: build test lint run release

build:
	cargo build

test:
	cargo test

lint:
	cargo clippy -- -D warnings
	cargo fmt -- --check

run:
	cargo run

release:
	cargo build --release
