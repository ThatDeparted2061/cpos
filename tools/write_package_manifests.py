#!/usr/bin/env python3
"""Generate Homebrew and Scoop package files from release artifacts."""

import argparse
import hashlib
import json
from pathlib import Path

ASSETS = {
    "mac_arm": "cpos-aarch64-apple-darwin.tar.gz",
    "mac_intel": "cpos-x86_64-apple-darwin.tar.gz",
    "linux_x64": "cpos-x86_64-unknown-linux-gnu.tar.gz",
    "windows_x64": "cpos-x86_64-pc-windows-msvc.zip",
}


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def release_url(repo: str, tag: str, asset: str) -> str:
    return f"https://github.com/{repo}/releases/download/{tag}/{asset}"


def write_homebrew(out_dir: Path, repo: str, version: str, tag: str, hashes: dict[str, str]) -> None:
    formula_dir = out_dir / "Formula"
    formula_dir.mkdir(parents=True, exist_ok=True)
    formula = formula_dir / "cpos.rb"
    formula.write_text(
        f'''class Cpos < Formula
  desc "Competitive Programming Operating System terminal app"
  homepage "https://github.com/{repo}"
  version "{version}"
  license "MIT"

  on_macos do
    on_arm do
      url "{release_url(repo, tag, ASSETS["mac_arm"])}"
      sha256 "{hashes["mac_arm"]}"
    end

    on_intel do
      url "{release_url(repo, tag, ASSETS["mac_intel"])}"
      sha256 "{hashes["mac_intel"]}"
    end
  end

  on_linux do
    on_intel do
      url "{release_url(repo, tag, ASSETS["linux_x64"])}"
      sha256 "{hashes["linux_x64"]}"
    end
  end

  def install
    bin.install "cpos"
  end

  test do
    assert_match "CPOS v{version}", shell_output("#{{bin}}/cpos help 2>&1")
  end
end
''',
        encoding="utf-8",
    )


def write_scoop(out_dir: Path, repo: str, version: str, tag: str, hashes: dict[str, str]) -> None:
    bucket_dir = out_dir / "bucket"
    bucket_dir.mkdir(parents=True, exist_ok=True)
    manifest = {
        "version": version,
        "description": "Competitive Programming Operating System terminal app",
        "homepage": f"https://github.com/{repo}",
        "license": "MIT",
        "architecture": {
            "64bit": {
                "url": release_url(repo, tag, ASSETS["windows_x64"]),
                "hash": hashes["windows_x64"],
            }
        },
        "bin": "cpos.exe",
        "checkver": {"github": f"https://github.com/{repo}"},
        "autoupdate": {
            "architecture": {
                "64bit": {
                    "url": f"https://github.com/{repo}/releases/download/v$version/{ASSETS['windows_x64']}"
                }
            }
        },
    }
    (bucket_dir / "cpos.json").write_text(
        json.dumps(manifest, indent=2) + "\n",
        encoding="utf-8",
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo", default="Soham109/cpos")
    parser.add_argument("--version", required=True)
    parser.add_argument("--assets", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    tag = args.version if args.version.startswith("v") else f"v{args.version}"
    version = tag.removeprefix("v")
    args.out.mkdir(parents=True, exist_ok=True)

    hashes = {}
    sum_lines = []
    for key, asset in ASSETS.items():
        path = args.assets / asset
        digest = sha256(path)
        hashes[key] = digest
        sum_lines.append(f"{digest}  {asset}")

    write_homebrew(args.out, args.repo, version, tag, hashes)
    write_scoop(args.out, args.repo, version, tag, hashes)
    (args.out / "SHA256SUMS").write_text("\n".join(sum_lines) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
