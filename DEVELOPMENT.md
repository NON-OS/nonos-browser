# Development Guide

## Prerequisites

### macOS
```bash
brew install node rust
xcode-select --install
```

### Ubuntu/Debian
```bash
sudo apt update
sudo apt install -y nodejs npm libwebkit2gtk-4.1-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows
- Install [Node.js LTS](https://nodejs.org/)
- Install [Rust](https://rustup.rs/)
- Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

## Setup

```bash
git clone https://github.com/NON-OS/nonos-browser.git
cd nonos-browser
make setup
```

## Development Workflow

### Running locally
```bash
make dev
```

This starts the Tauri development server with hot-reload enabled.

### Code formatting
```bash
make fmt
```

### Linting
```bash
make lint
```

### Running tests
```bash
make test
```

## Architecture

### Frontend (SvelteKit)

Located in `ui/src/`. Key directories:

- `routes/` - Page components
- `lib/components/` - Reusable UI components
- `lib/styles/` - Global CSS and theme

### Backend (Rust/Tauri)

Located in `ui/src-tauri/src/`. Key modules:

- `browser/` - Tab management and navigation
- `wallet/` - HD wallet and transaction signing
- `network/` - Nym mixnet connection
- `staking/` - Contract interactions
- `proxy/` - SOCKS5 and HTTP proxy handling

### Shared Crates

- `nonos-types` - Common type definitions
- `nonos-crypto` - Blake3, secp256k1, key derivation
- `nonos-wallet` - BIP39/BIP32 wallet implementation

## Building for Release

```bash
make release
```

Output binaries are in `ui/src-tauri/target/release/`.

## Nym Mixnet

The browser connects to Nym network on startup. Connection status shows in the sidebar.

Default configuration:
- Mode: Two-hop with fastmode
- SOCKS5 proxy: `127.0.0.1:1080`
- Client ID: `nonos-client`

## Troubleshooting

### Port 5173 already in use
```bash
lsof -ti :5173 | xargs kill -9
```

### Nym connection issues
Check if the Nym client is running:
```bash
ps aux | grep nym-socks5
```

### Webkit issues on Linux
```bash
sudo apt install --reinstall libwebkit2gtk-4.1-dev
```

## Code Style

- Rust: Follow standard rustfmt conventions
- TypeScript/Svelte: Prettier with tabs
- Commits: Conventional commits format

## Pull Requests

1. Fork the repository
2. Create a feature branch
3. Run `make ci` before pushing
4. Submit PR against `main`
