# NONOS Browser

Privacy-first browser with native Nym mixnet integration. All traffic routes through a 5-hop anonymity network before reaching the destination.

## Features

- Native Nym mixnet routing (SOCKS5 on port 1080)
- Built-in NOX wallet with Ethereum/Sepolia support
- Staking interface for NOXStakingPool contracts
- Tracker blocking and fingerprint protection
- Zero-knowledge identity management

## Requirements

- Rust 1.75+
- Node.js 20+
- macOS 12+ / Ubuntu 22.04+ / Windows 11

## Quick Start

```bash
# Install dependencies
make setup

# Run in development mode
make dev

# Build release binary
make build
```

## Project Structure

```
nonos-browser/
├── ui/                     # Tauri application
│   ├── src/               # SvelteKit frontend
│   └── src-tauri/         # Rust backend
├── crates/
│   ├── nonos-types/       # Shared type definitions
│   ├── nonos-crypto/      # Cryptographic primitives
│   └── nonos-wallet/      # HD wallet implementation
└── Makefile               # Build automation
```

## Development

See [DEVELOPMENT.md](DEVELOPMENT.md) for build instructions and contribution guidelines.

## Staking Contracts (Sepolia)

| Contract | Address |
|----------|---------|
| NOX Token | `0xC87799c4517Dcdfc65bfefa3Be64Beb89668c66c` |
| NOXStakingPool | `0xb27DAe7EbE628ebe2E9302D7D2C71eF34Dd01705` |
| PrivacyWorkEconomy | `0xAf8018e21Eff6F21BE305941f6d595Bd337c8bCA` |

## License

AGPL-3.0 - See [LICENSE](LICENSE) for details.

## Links

- Website: https://nonos.systems
- Documentation: https://docs.nonos.systems
- Twitter: https://x.com/noaboringmoney
