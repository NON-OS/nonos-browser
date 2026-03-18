# NONOS Browser

Privacy-first browser with native Nym mixnet integration. All traffic routes through encrypted multi-hop relays before reaching the destination.

## Features

- Native Nym mixnet routing (SOCKS5 on port 1080)
- Built-in NOX wallet with Ethereum/Sepolia support
- Staking interface for NOXStakingPool contracts
- LP staking with tiered lock periods
- Tracker blocking and fingerprint protection
- Zero-knowledge identity management

## Requirements

- Rust 1.75+
- Node.js 20+
- Platform-specific dependencies (see below)

## Platform Setup

### macOS

```bash
brew install node rust
xcode-select --install
```

### Ubuntu/Debian (24.04+)

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev build-essential curl wget libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows

- Install [Node.js LTS](https://nodejs.org/)
- Install [Rust](https://rustup.rs/)
- Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

## Build

```bash
git clone https://github.com/NON-OS/nonos-browser.git
cd nonos-browser

# Install dependencies
make setup

# Build Nym sidecar binary
# Linux x86_64:
mkdir -p ui/src-tauri/binaries
curl -L "https://github.com/nymtech/nym/releases/download/nym-binaries-v2026.5-raclette/nym-socks5-client" \
  -o ui/src-tauri/binaries/anon-x86_64-unknown-linux-gnu
cp ui/src-tauri/binaries/anon-x86_64-unknown-linux-gnu \
   ui/src-tauri/binaries/anon-gencert-x86_64-unknown-linux-gnu
chmod +x ui/src-tauri/binaries/*

# macOS (build from source):
git clone --depth 1 --branch nym-binaries-v2026.5-raclette https://github.com/nymtech/nym nym-src
cd nym-src && cargo build --release -p nym-socks5-client
mkdir -p ../ui/src-tauri/binaries
cp target/release/nym-socks5-client ../ui/src-tauri/binaries/anon-aarch64-apple-darwin
cp target/release/nym-socks5-client ../ui/src-tauri/binaries/anon-gencert-aarch64-apple-darwin
cp target/release/nym-socks5-client ../ui/src-tauri/binaries/anon-x86_64-apple-darwin
cp target/release/nym-socks5-client ../ui/src-tauri/binaries/anon-gencert-x86_64-apple-darwin
chmod +x ../ui/src-tauri/binaries/*
cd ..

# Run development server
make dev

# Build release
make build
```

## Project Structure

```
nonos-browser/
├── ui/
│   ├── src/               # SvelteKit frontend
│   └── src-tauri/         # Rust backend
│       └── binaries/      # Nym sidecar binaries
├── crates/
│   ├── nonos-types/       # Shared type definitions
│   ├── nonos-crypto/      # Cryptographic primitives
│   └── nonos-wallet/      # HD wallet implementation
├── docs/                  # Documentation
└── Makefile               # Build automation
```

## Development

```bash
make dev      # Start development server with hot reload
make build    # Build release binary
make test     # Run tests
make lint     # Run clippy
make fmt      # Format code
```

See [DEVELOPMENT.md](DEVELOPMENT.md) for contribution guidelines.

## Contracts (Sepolia Testnet)

| Contract | Address |
|----------|---------|
| NOX Token | `0xC87799c4517Dcdfc65bfefa3Be64Beb89668c66c` |
| NOXStakingPool | `0xb27DAe7EbE628ebe2E9302D7D2C71eF34Dd01705` |
| PrivacyWorkEconomy | `0xAf8018e21Eff6F21BE305941f6d595Bd337c8bCA` |
| PrivacyLiquidityPool | `0x4838b106fce9647bdf1e7877bf73ce8b0bad5f97` |

## License

AGPL-3.0 - See [LICENSE](LICENSE) for details.

## Links

- Website: https://nonos.systems
- Documentation: https://docs.nonos.systems
- Twitter: https://x.com/noaboringmoney
