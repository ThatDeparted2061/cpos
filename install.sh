#!/usr/bin/env sh
set -eu

repo="${CPOS_REPO:-Soham109/cpos}"
version="${CPOS_VERSION:-latest}"
bin_dir="${CPOS_INSTALL_DIR:-$HOME/.local/bin}"

need_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "error: '$1' is required to install CPOS" >&2
    exit 1
  fi
}

need_cmd curl
need_cmd tar

os="$(uname -s)"
arch="$(uname -m)"

case "$os:$arch" in
  Darwin:arm64|Darwin:aarch64)
    target="aarch64-apple-darwin"
    ;;
  Darwin:x86_64)
    target="x86_64-apple-darwin"
    ;;
  Linux:x86_64|Linux:amd64)
    target="x86_64-unknown-linux-gnu"
    ;;
  *)
    echo "error: unsupported platform: $os $arch" >&2
    echo "Try the source install instead: cargo install --git https://github.com/$repo" >&2
    exit 1
    ;;
esac

asset="cpos-$target.tar.gz"

if [ "$version" = "latest" ]; then
  url="https://github.com/$repo/releases/latest/download/$asset"
else
  case "$version" in
    v*) tag="$version" ;;
    *) tag="v$version" ;;
  esac
  url="https://github.com/$repo/releases/download/$tag/$asset"
fi

tmp="$(mktemp -d 2>/dev/null || mktemp -d -t cpos)"
cleanup() {
  rm -rf "$tmp"
}
trap cleanup EXIT INT TERM

echo "Installing CPOS TUI for $target"
echo "Downloading $url"

if ! curl -fsSL "$url" -o "$tmp/$asset"; then
  echo "error: could not download CPOS release asset" >&2
  echo "Make sure a GitHub release exists with asset '$asset'." >&2
  exit 1
fi

tar -xzf "$tmp/$asset" -C "$tmp"

if [ ! -f "$tmp/cpos" ]; then
  echo "error: release archive did not contain the cpos binary" >&2
  exit 1
fi

mkdir -p "$bin_dir"
cp "$tmp/cpos" "$bin_dir/cpos"
chmod 755 "$bin_dir/cpos"

echo "Installed CPOS to $bin_dir/cpos"

case ":$PATH:" in
  *":$bin_dir:"*) ;;
  *)
    echo "Add this to your shell profile if 'cpos' is not found:"
    echo "  export PATH=\"$bin_dir:\$PATH\""
    ;;
esac

echo "Run: cpos"
