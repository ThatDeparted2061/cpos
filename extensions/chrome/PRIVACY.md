# CPOS Companion — Privacy Policy

**Last updated:** May 31, 2026  
**Contact:** [github.com/Soham109/cpos/issues](https://github.com/Soham109/cpos/issues)

CPOS Companion is a browser extension for competitive programming. It works together with the [CPOS VS Code extension](https://marketplace.visualstudio.com/items?itemName=sohamaggarwal.cpos-vscode) and/or the CPOS desktop app on your computer.

## Summary

- **No accounts.** The extension does not create user accounts.
- **No analytics.** The extension does not use Google Analytics or any third-party tracking.
- **No remote servers.** The extension does not send your data to the developer or any cloud service.
- **Local only.** Problem samples and source code are sent only to `127.0.0.1` on your machine (ports `27121` and `27122`) when CPOS is running.

## Data the extension accesses

When you open a supported problem page on **Codeforces** or **CSES**, the extension may read from that page:

- Problem identifier and title
- Public sample test inputs and expected outputs shown on the page
- On CSES problem list pages: which tasks appear solved or attempted (read from the page DOM only)

When you submit a solution from CPOS in VS Code or the CPOS app, the extension may:

- Receive your source code from the local CPOS app via localhost
- Fill the submit form on Codeforces or CSES in your already-logged-in browser tab
- Click Submit on your behalf (same as you would manually)

The extension **does not** read passwords, cookies, or browsing history outside the matched Codeforces/CSES pages.

## Where data goes

| Data | Destination |
|------|-------------|
| Captured samples & problem metadata | `http://127.0.0.1:27122` (CPOS VS Code) and/or `http://127.0.0.1:27121` (CPOS desktop app) |
| Pending submission source code | From localhost CPOS → browser submit page only |
| Anything else | **Nowhere** — not transmitted off your device |

If CPOS is not running locally, captures fail gracefully and nothing is stored by the extension.

## Data storage

The extension does not persist problem data, source code, or personal information in browser storage. Temporary in-memory state is used while processing a page or submission and is discarded afterward.

## Permissions

| Permission | Why it is needed |
|------------|------------------|
| `scripting` | Autofill submit forms on Codeforces/CSES when you submit from CPOS |
| `tabs` | Find or open the correct browser tab for submission autofill |
| `alarms` | Wake the background worker periodically so queued submissions from CPOS are picked up quickly; no data is read or sent by this alarm |
| `127.0.0.1:27121/27122` | Talk to CPOS running on your computer |
| `codeforces.com`, `cses.fi` | Read problem pages you visit and interact with submit pages |

## Children

This extension is not directed at children under 13 and does not knowingly collect information from children.

## Changes

Material changes to this policy will be reflected in the extension repository. Continued use after an update constitutes acceptance of the revised policy.

## Open source

Source code: [github.com/Soham109/cpos/tree/main/extensions/chrome](https://github.com/Soham109/cpos/tree/main/extensions/chrome)
