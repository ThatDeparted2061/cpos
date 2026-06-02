# CPOS troubleshooting

Common issues across the **VS Code extension**, **browser companion**, and **terminal app**.  
If something isn’t listed here, open a [GitHub issue](https://github.com/Soham109/cpos/issues) with your OS, CPOS versions, and the exact error text.

| Component | Version file |
| --- | --- |
| VS Code extension | `extensions/vscode/package.json` |
| Browser companion | `extensions/chrome/manifest.json` |
| Terminal app | `Cargo.toml` |

---

## VS Code — Run All fails: `spawn sh ENOENT`

**What it means:** CPOS could not start a shell to compile or run your file. This is **not** a wrong answer (WA) or compilation error (CE) in your solution.

**Fix:** Update to VS Code extension **0.3.21+** (uses `/bin/sh` on macOS/Linux and `cmd.exe` on Windows).

### macOS

1. Install a compiler if needed:
   ```bash
   brew install gcc
   ```
2. If Cursor or VS Code was opened from the Dock, launch from Terminal so PATH includes `/bin` and Homebrew:
   ```bash
   cursor .
   # or: code .
   ```
3. Open **View → Output → CPOS** to see the exact command CPOS ran.

### Windows

1. Install a C++ toolchain — e.g. [MSYS2](https://www.msys2.org/):
   ```bash
   pacman -S mingw-w64-ucrt-x86_64-gcc
   ```
   Add `C:\msys64\ucrt64\bin` to your system **PATH** (or use MinGW-w64 / Scoop `gcc`).
2. **Python:** ensure `python` is on PATH (Windows often has no `python3` command).
3. Check **Output → CPOS** for the compile/run command and errors.

### All platforms

- **CE in the panel** = the toolchain ran but the build failed — read stderr in the test row or CPOS output.
- Override commands: **Settings → Extensions → CPOS → Compile Commands** (`{source}`, `{output}`, `{dir}` placeholders).

---

## VS Code — Compilation errors (CE)

### macOS: `#include <bits/stdc++.h>` not found

Apple’s default `g++` is Clang and does not ship `bits/stdc++.h`. Install GNU g++:

```bash
brew install gcc
```

CPOS auto-detects Homebrew’s `g++-14`, `g++-15`, etc. when the GUI app’s PATH is thin.

### Custom compiler or flags

Sync with the terminal app via `~/Library/Application Support/cpos/config.toml` (macOS) or `%APPDATA%\cpos\config.toml` (Windows), or set `cpos.compileCommands` in VS Code settings.

---

## Submit — nothing happens or wrong language

### Submit does nothing

1. Install the [browser companion](https://chromewebstore.google.com/detail/gjnbapmjonegeeamdeahcoojgokeogmm) in **Chrome** (Edge/Brave work too).
2. Stay **logged in** to Codeforces or CSES in that browser.
3. CPOS only talks to Chrome on `127.0.0.1` — not Safari or Firefox.
4. Keep the VS Code extension running (capture server on port `27122`).

### Codeforces submits with wrong compiler (e.g. C++17 instead of C++23)

Update the browser companion to **0.6.12+**. It picks the **newest matching compiler** from the submit dropdown (e.g. G++23 before G++17) instead of a stale numeric id.

Rebuild and upload from `extensions/chrome` if you install unpacked:

```bash
./package-store.sh
```

### Submit opens but code is empty or form not filled

- Log in on Codeforces/CSES in the same Chrome profile as the extension.
- Disable other extensions that might block scripting on judge pages.
- Retry after a full page load on the submit URL.

---

## Capture — samples or file not appearing

1. **VS Code:** open the folder where you want solution files before capturing.
2. **Browser companion** installed and enabled on `codeforces.com` / `cses.fi`.
3. **One CPOS server:** only one VS Code window should own the capture port (`27122`); the panel shows if another window is active.
4. Refresh the problem page after installing the extension.

Terminal app capture uses port `27121` — VS Code and TUI can both receive captures; VS Code is enough for most workflows.

---

## Terminal app — `cargo install` fails: `link.exe` not found

**What it means:** Rust on Windows is using the **MSVC** toolchain but **Visual C++ build tools** are not installed. VS Code/Cursor does **not** include the linker.

This is a Rust/Windows setup issue, not a CPOS bug.

### Fix A — MSVC (recommended on Windows)

1. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
2. Select **Desktop development with C++** (or at least **MSVC** + **Windows SDK**).
3. Restart your terminal.
4. Retry:
   ```bash
   cargo install --git https://github.com/Soham109/cpos
   ```

**Verify:**

```bash
rustup show          # active toolchain, e.g. x86_64-pc-windows-msvc
where link.exe       # should find Visual Studio's linker
```

### Fix B — GNU toolchain (MSYS2)

1. Install [MSYS2](https://www.msys2.org/) and GCC:
   ```bash
   pacman -S mingw-w64-ucrt-x86_64-gcc
   ```
2. Switch Rust to the GNU target:
   ```bash
   rustup toolchain install stable-x86_64-pc-windows-gnu
   rustup default stable-x86_64-pc-windows-gnu
   ```
3. Retry `cargo install --git https://github.com/Soham109/cpos`.

### Don’t need the terminal app?

**VS Code extension + Chrome companion** is enough for capture, Run All, and submit. The TUI is optional (catalog, contests, analytics, recommendations).

---

## Terminal app — other issues

### Config location

| OS | Path |
| --- | --- |
| macOS | `~/Library/Application Support/cpos/config.toml` |
| Windows | `%APPDATA%\cpos\config.toml` |
| Linux | `~/.config/cpos/config.toml` |

### Submit from TUI opens wrong browser

The terminal app queues submit for the **Chrome companion** only (same as VS Code). Install the extension and use Chrome while logged in.

### CSES progress not syncing

Set `cses_session` in config (see README) and use the browser companion on the CSES problem list.

---

## Panel UI

### Native theme — Run All button looks wrong on light VS Code themes

Update to VS Code extension **0.3.20+**. Native theme uses VS Code button colors (white label on the theme button background).

### Extension README screenshot broken in Extensions view

Update to **0.3.20+** or reinstall from the latest VSIX/Marketplace build.

---

## Still stuck?

1. Note your versions (extension, Chrome companion, terminal app if used).
2. Copy the **full error** from **Output → CPOS** or your terminal.
3. Say which step failed: capture, Run All, submit, or `cargo install`.
4. [Open an issue](https://github.com/Soham109/cpos/issues) or ask in your community with that info.
