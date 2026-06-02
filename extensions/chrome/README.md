# CPOS Companion

Browser extension for Codeforces and CSES. Captures public sample tests and relays them to CPOS on your machine. Autofills judge submit pages when you submit from VS Code or the terminal app.

**Current version:** 0.6.12 (see [CHANGELOG.md](../../CHANGELOG.md)).

## Install

**[Chrome Web Store](https://chromewebstore.google.com/detail/gjnbapmjonegeeamdeahcoojgokeogmm)** — Chrome, Edge, Brave.

Also install the **[CPOS VS Code extension](https://marketplace.visualstudio.com/items?itemName=sohamaggarwal.cpos-vscode)** and/or the [terminal app](https://github.com/Soham109/cpos).

## What it does

- Reads public samples from Codeforces and CSES problem pages and sends them to `127.0.0.1:27122` (VS Code) and/or `127.0.0.1:27121` (terminal)
- On Codeforces, captures sub-test-case block structure when the statement provides it
- Polls for queued submissions and autofills the judge submit form in your logged-in browser session (picks the newest matching compiler on Codeforces, e.g. C++23 over C++17)
- Scrapes CSES solved and attempted status on the problem list when requested

Data is not sent to third-party servers—only to CPOS on localhost.

## Development

See [CONTRIBUTING.md](../../CONTRIBUTING.md) and [STORE_LISTING.md](STORE_LISTING.md).

```bash
./package-store.sh   # produces cpos-companion.zip for the Web Store
```
