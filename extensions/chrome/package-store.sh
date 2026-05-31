#!/usr/bin/env bash
# Build Chrome Web Store upload zip (extension files only).
set -euo pipefail
cd "$(dirname "$0")"

if [[ ! -f icons/icon128.png ]]; then
  echo "Generating icons..."
  npx --yes @resvg/resvg-js-cli --fit-width 128 icons/icon.svg icons/icon128.png
  npx --yes @resvg/resvg-js-cli --fit-width 48 icons/icon.svg icons/icon48.png
  npx --yes @resvg/resvg-js-cli --fit-width 16 icons/icon.svg icons/icon16.png
fi

rm -f cpos-companion.zip
zip -r cpos-companion.zip \
  manifest.json \
  background.js \
  content.js \
  icons/icon16.png \
  icons/icon48.png \
  icons/icon128.png

echo "Created cpos-companion.zip ($(du -h cpos-companion.zip | cut -f1))"
