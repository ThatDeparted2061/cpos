# Chrome Web Store listing — copy & paste

Use this when uploading `cpos-companion.zip` at  
[Chrome Web Store Developer Dashboard](https://chrome.google.com/webstore/devconsole).

---

## Build the zip

```bash
cd extensions/chrome
./package-store.sh
```

Upload **`cpos-companion.zip`** (root must contain `manifest.json`).

---

## Store fields

| Field | Value |
|-------|-------|
| **Category** | Developer Tools |
| **Language** | English |
| **Privacy policy URL** | `https://github.com/Soham109/cpos/blob/main/extensions/chrome/PRIVACY.md` |
| **Official URL** (optional) | `https://github.com/Soham109/cpos` |
| **Support URL** (optional) | `https://github.com/Soham109/cpos/issues` |

### Title
```
CPOS Companion
```

### Summary (132 chars max)
```
Capture Codeforces & CSES samples and submit from CPOS in VS Code. Sends data only to CPOS on your computer (localhost).
```

### Description
```
CPOS Companion connects your browser to CPOS — a competitive programming workflow for VS Code and the CPOS desktop app.

WHAT IT DOES
• When you open a Codeforces or CSES problem, captures public sample tests and sends them to CPOS on your machine
• When you click Submit in CPOS (VS Code or TUI), autofills the submit page in your logged-in browser tab
• On CSES problem lists, can sync solved/attempted status to CPOS when you choose to sync

REQUIRES
• CPOS VS Code extension and/or CPOS desktop app running locally
• You must be logged in to Codeforces or CSES in this browser for submissions

PRIVACY
• No analytics, no accounts, no cloud servers
• All communication is to 127.0.0.1 on your computer only
• See privacy policy in the repository for full details

Open source: https://github.com/Soham109/cpos
```

### Single purpose
```
Capture competitive programming problem samples from Codeforces and CSES and relay them to the local CPOS editor on the user's computer; optionally autofill submit forms when the user submits from CPOS.
```

---

## Permission justifications (Privacy practices tab)

### Host permission: `http://127.0.0.1:27121/*` and `http://127.0.0.1:27122/*`
```
CPOS runs locally on the user's machine. The extension sends captured problem samples and receives pending submissions only via localhost HTTP. No data is sent to external servers.
```

### Host permission: `https://codeforces.com/*`
```
Reads public problem and sample data from Codeforces pages the user opens, and autofills the submit form when the user submits from CPOS while logged into Codeforces in this browser.
```

### Host permission: `https://cses.fi/*`
```
Reads public problem and sample data from CSES pages the user opens, syncs task progress from the problem list when requested, and autofills the submit form when the user submits from CPOS.
```

### Permission: `scripting`
```
Injects autofill logic only on Codeforces and CSES submit pages when CPOS queues a submission from the local editor. Required because submit pages use dynamic editors that cannot be filled from an ordinary content script alone.
```

### Permission: `tabs`
```
Finds an existing Codeforces/CSES tab or opens one when the user submits from CPOS, so the submit form can be autofilled in the correct logged-in session.
```

### Permission: `alarms`
```
Chrome may suspend the extension's background worker when idle. CPOS uses a local alarm every 30 seconds solely to wake that worker and check whether the user queued a submission from CPOS (VS Code or desktop app) on localhost. The alarm does not read browsing data, does not contact external servers, and does not run unless the extension is installed.
```

---

## Data usage certification

When asked whether you collect or use user data:

- **No**, the extension does not sell or transfer user data to third parties for unrelated purposes.
- **No** remote collection — data stays on the user's device (localhost CPOS only).
- Check **No personal data collected** / equivalent if the form allows, or disclose only: *problem metadata and source code transmitted locally to CPOS on 127.0.0.1 when the user uses the extension*.

---

## Screenshots

Chrome requires at least one screenshot (1280×800 or 640×400). Capture:

1. A Codeforces problem page with the green “CPOS · captured …” toast
2. VS Code CPOS panel with loaded samples (optional second screenshot)

Store assets live in `store/` after you add screenshots (not included in the zip).

---

## One-time fee

Chrome Web Store developer registration: **$5 one-time** at [Chrome Web Store Developer Dashboard](https://chrome.google.com/webstore/devconsole).
