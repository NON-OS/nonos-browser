.PHONY: all setup dev build test lint fmt clean release

all: build

setup:
	@echo "Installing dependencies..."
	cd ui && npm install
	rustup component add clippy rustfmt

dev:
	@echo "Starting development server..."
	cd ui && npm run tauri dev

build:
	@echo "Building release binary..."
	cd ui && npm run tauri build

test:
	@echo "Running tests..."
	cargo test --workspace

lint:
	@echo "Running clippy..."
	cargo clippy --workspace --all-targets -- -D warnings

fmt:
	@echo "Formatting code..."
	cargo fmt --all

fmt-check:
	@echo "Checking format..."
	cargo fmt --all -- --check

check:
	@echo "Running cargo check..."
	cargo check --workspace

clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	cd ui && rm -rf build .svelte-kit node_modules

release:
	@echo "Building optimized release..."
	cd ui && npm run tauri build -- --release

install-deps:
	@echo "Installing system dependencies..."
ifeq ($(shell uname), Darwin)
	brew install node rust
endif
ifeq ($(shell uname), Linux)
	sudo apt update && sudo apt install -y nodejs npm libwebkit2gtk-4.1-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
endif

ci: fmt-check lint test
	@echo "CI checks passed"
