# NONOS Browser

Privacy browser with Nym mixnet. All traffic goes through encrypted multi-hop relays.

## Download

[Latest Release](https://github.com/NON-OS/nonos-browser/releases)

| Platform | File |
|----------|------|
| macOS (Apple Silicon) | `.dmg` aarch64 |
| macOS (Intel) | `.dmg` x64 |
| Linux | `.AppImage` / `.deb` |
| Windows | `.msi` |

## What's Inside

- Nym mixnet routing (5-hop by default)
- NOX wallet (mainnet + Sepolia testnet)
- Staking dashboard
- Tracker blocking

## Build

```
brew install node rust        # macOS
# or
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev  # Linux

git clone https://github.com/NON-OS/nonos-browser.git
cd nonos-browser
make setup && make build
```

## Dev

```
make dev    # hot reload
make test   # tests
```

## Contracts (Sepolia)

- NOX: `0xC87799c4517Dcdfc65bfefa3Be64Beb89668c66c`
- Staking: `0xb27DAe7EbE628ebe2E9302D7D2C71eF34Dd01705`

## Links

- https://nonos.systems
- https://x.com/nonossystems
