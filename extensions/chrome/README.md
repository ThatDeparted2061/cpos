# CPOS Companion

Browser extension for Codeforces and CSES. Captures sample tests and relays them to CPOS on your machine.

**Install:** [Chrome Web Store](https://chromewebstore.google.com/detail/gjnbapmjonegeeamdeahcoojgokeogmm)

Requires the [CPOS VS Code extension](https://marketplace.visualstudio.com/items?itemName=sohamaggarwal.cpos-vscode) and/or the CPOS terminal app running locally.

## What it does

- Captures public samples when you open a Codeforces or CSES problem
- Sends data only to `127.0.0.1:27122` (VS Code) or `127.0.0.1:27121` (TUI)
- Autofills submit pages when you submit from CPOS

## Development

1. Open `chrome://extensions`
2. Enable **Developer mode**
3. **Load unpacked** → select this folder

## Publish

```bash
./package-store.sh
```

Listing copy and privacy justifications: [`STORE_LISTING.md`](STORE_LISTING.md) · [`PRIVACY.md`](PRIVACY.md)
