# CPOS

**Competitive programming for Codeforces and CSES — in VS Code.**

Open a problem in your browser. CPOS creates your solution file in the folder you have open, loads the sample tests, and provides a side panel to run samples and submit.

Part of the **CPOS** project — works with the [terminal app](https://github.com/Soham109/cpos) and [browser companion](https://chromewebstore.google.com/detail/gjnbapmjonegeeamdeahcoojgokeogmm). All components sync over localhost.

![CPOS VS Code panel with test cases and a Codeforces solution](media/vscode-panel-ui.png)

## How it works

1. Install from the **[VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=sohamaggarwal.cpos-vscode)** and the **[browser companion](https://chromewebstore.google.com/detail/gjnbapmjonegeeamdeahcoojgokeogmm)**
2. Open the folder where you want solution files
3. Open a Codeforces or CSES problem in your browser
4. CPOS creates the solution file (for example `1982C.cpp`) with samples attached
5. Write code and use the **CPOS panel** to **Run All** or **Submit**

## The CPOS panel

Open the **CPOS** view in the activity bar:

- **Run All** — compile and run every sample; verdicts shown inline (`AC`, `WA`, `TLE`, `RE`, `CE`)
- **Submit** — queue submission and autofill the judge page in your logged-in browser
- **Problem** — open the problem statement
- **Test cases** — edit, add, or remove samples; Codeforces multi-case inputs can show linked input/output blocks
- **Themes** — CPOS, Midnight, Amber, Paper, or Native (matches your VS Code theme)

Keep the **terminal app** running for browsing, recommendations, and analytics. Captures and submissions work with either app.

## Settings

`Settings → Extensions → CPOS` — save folder, language, template, compile commands, timeouts.

By default, files are created in the **currently open workspace folder**.

Submit requires the [browser companion](https://chromewebstore.google.com/detail/gjnbapmjonegeeamdeahcoojgokeogmm) and an active login on the judge site.

## Links

- [CPOS on GitHub](https://github.com/Soham109/cpos)
- [Changelog](../../CHANGELOG.md)
- [Architecture](../../ARCHITECTURE.md)
- [Report an issue](https://github.com/Soham109/cpos/issues)
