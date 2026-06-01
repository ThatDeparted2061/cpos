# CPOS architecture

CPOS is three local clients plus a static website. Nothing runs in the cloud; the browser extension and apps only talk to `127.0.0.1`.

```
┌─────────────────┐     capture/submit      ┌──────────────────┐
│ Browser         │ ───────────────────────▶│ VS Code (:27122) │
│ (Chrome ext)    │                         │  extension       │
└────────┬────────┘                         └────────┬─────────┘
         │                                           │
         │ capture/submit                            │ forward capture
         ▼                                           ▼
┌─────────────────┐                         ┌──────────────────┐
│ Terminal TUI    │◀────── localhost ──────▶│ Shared data dirs │
│ (:27121)        │                         │ (config, cache)  │
└─────────────────┘                         └──────────────────┘
```

## Components

| Path | Role |
| --- | --- |
| `src/` | Rust terminal app (ratatui UI, sync, recommendations, test runner) |
| `extensions/vscode/` | VS Code webview panel + capture HTTP server |
| `extensions/chrome/` | Content scripts on Codeforces/CSES; posts to localhost |
| `docs/` | Static landing site |

## Localhost protocol

Both CPOS apps expose a tiny HTTP API on loopback only.

| Port | Owner | Default |
| --- | --- | --- |
| `27121` | Terminal TUI | `src/engine/capture.rs` |
| `27122` | VS Code extension | `extensions/vscode/src/extension.ts` |

The browser companion tries **both** ports so either app can receive captures.

Common endpoints:

| Method | Path | Purpose |
| --- | --- | --- |
| `POST` | `/capture/problem` | Problem metadata + sample tests from browser |
| `POST` | `/capture/cses-progress` | CSES solved/attempted task ids |
| `GET` | `/pending-submit` | Browser polls for code to autofill on submit |
| `POST` | `/pending-submit/consumed` | Ack after autofill |
| `GET` | `/health` | Liveness check |

CORS is open (`*`) because traffic never leaves the machine.

## Typical capture flow

1. User opens a Codeforces/CSES problem in the browser (logged in if submitting later).
2. Content script reads public samples from the DOM.
3. Extension `POST`s JSON to `127.0.0.1:27122` (VS Code) and/or `27121` (TUI).
4. Receiving app creates/updates a solution file, saves samples, opens the editor (optional).
5. VS Code forwards captures to the TUI when both are running.

## Submit flow

1. User runs **Submit** from VS Code or presses `s` in the TUI.
2. App queues `{ code, language, submitUrl, … }` on localhost.
3. Browser extension polls `/pending-submit`, opens the judge submit page, fills the form, clicks submit.
4. User stays logged in via their normal browser session — CPOS never sees passwords.

## Data on disk

| Location | Contents |
| --- | --- |
| `~/.config/cpos/` or `~/Library/Application Support/cpos/` | TUI config, SQLite cache, CSES progress |
| `~/.cpos-vscode/` | VS Code samples, problem metadata, build artifacts |
| User's open folder | Solution source files (e.g. `1971D.cpp`) |

Config keys like `default_language` and `[compile_commands.*]` are shared between TUI and VS Code where possible.

## Sync (terminal)

Background task fetches Codeforces API data (problems, submissions, rating, contests) into a local SQLite cache. Recommendations and analytics read from that cache — no repeated network calls during normal TUI use.

Press `r` to refresh. CSES progress can come from the browser companion when a session cookie isn't available server-side.

## Building each piece

See [CONTRIBUTING.md](CONTRIBUTING.md) for dev commands. CI runs `cargo test` and `npm run compile` in `extensions/vscode` on every push to `main`.
