# CPOS Companion for Firefox

Firefox build of the CPOS browser companion for Codeforces and CSES. It mirrors the Chrome companion: capture public samples, sync CSES progress, and autofill judge submit pages through the local CPOS endpoints.

**Current version:** 0.0.1 (see [CHANGELOG.md](../../CHANGELOG.md)).

## Install From Source

This Firefox build is not listed on addons.mozilla.org yet. For now, load it
from source for local use:

1. Open `about:debugging#/runtime/this-firefox`.
2. Click **Load Temporary Add-on...**.
3. Select `extensions/firefox/manifest.json`.

Firefox 142 or newer is required. Temporary add-ons are removed when Firefox restarts; reload this manifest when needed.

## Package

```bash
./package-firefox.sh
```

This produces `cpos-companion.xpi` in this directory for local testing or
Mozilla signing. Normal Firefox release builds require add-ons to be signed by
Mozilla before permanent installation, even when the add-on is self-distributed
instead of publicly listed.

Mozilla's `web-ext` tooling is also configured:

```bash
npx --yes web-ext lint
npx --yes web-ext build
```
