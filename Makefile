.PHONY: build test lint fmt clean

build:
	cargo build --workspace

test:
	cargo test --workspace

lint:
	cargo clippy --workspace -- -D warnings

fmt:
	cargo fmt --all

check: fmt lint test

clean:
	cargo clean
	rm -rf sdk/typescript/dist

sdk:
	cd sdk/typescript && npm run build

all: build sdk test
