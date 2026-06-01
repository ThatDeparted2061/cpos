use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

const REPO: &str = "https://github.com/Soham109/cpos";

pub fn run() -> Result<()> {
    eprintln!("CPOS v{}", env!("CARGO_PKG_VERSION"));
    eprintln!("Updating terminal app…\n");

    ensure_cargo()?;

    match detect_method()? {
        UpdateMethod::Git => git_install()?,
        UpdateMethod::LocalPath(path) => path_install(&path)?,
        UpdateMethod::DevTree(path) => dev_build(&path)?,
    }

    eprintln!("\nDone. Run `cpos` to start.");
    eprintln!("VS Code extension and browser companion update via their stores.");
    Ok(())
}

enum UpdateMethod {
    Git,
    LocalPath(PathBuf),
    DevTree(PathBuf),
}

fn ensure_cargo() -> Result<()> {
    let ok = Command::new("cargo")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if ok {
        return Ok(());
    }
    bail!(
        "Rust/cargo not found on PATH.\n\
         Install Rust from https://rustup.rs, then run `cpos update` again."
    );
}

fn detect_method() -> Result<UpdateMethod> {
    if let Some(path) = cargo_install_path() {
        return Ok(UpdateMethod::LocalPath(path));
    }
    if let Some(path) = dev_tree_from_exe() {
        return Ok(UpdateMethod::DevTree(path));
    }
    Ok(UpdateMethod::Git)
}

fn cargo_install_path() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    let text = std::fs::read_to_string(home.join(".cargo/.crates2.json")).ok()?;
    let json: serde_json::Value = serde_json::from_str(&text).ok()?;
    let packages = json.get("packages")?.as_object()?;

    let mut best: Option<PathBuf> = None;
    for key in packages.keys() {
        if let Some(path) = parse_path_from_crate_key(key) {
            if path.join("Cargo.toml").is_file() {
                best = Some(path);
            }
        }
    }
    best
}

fn parse_path_from_crate_key(key: &str) -> Option<PathBuf> {
    if !key.starts_with("cpos ") {
        return None;
    }
    let start = key.find("(path+file://")? + "(path+file://".len();
    let rest = &key[start..];
    let end = rest.find('#').unwrap_or(rest.len());
    let encoded = &rest[..end];
    Some(percent_decode_path(encoded))
}

fn percent_decode_path(encoded: &str) -> PathBuf {
    let mut out = String::new();
    let bytes = encoded.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(v) = u8::from_str_radix(&encoded[i + 1..i + 3], 16) {
                out.push(v as char);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    PathBuf::from(out)
}

fn dev_tree_from_exe() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let lossy = exe.to_string_lossy();
    if !lossy.contains("/target/") && !lossy.contains("\\target\\") {
        return None;
    }
    let dir = exe.parent()?.parent()?.parent()?.to_path_buf();
    let manifest = dir.join("Cargo.toml");
    if !manifest.is_file() {
        return None;
    }
    let text = std::fs::read_to_string(manifest).ok()?;
    if text.contains("name = \"cpos\"") {
        Some(dir)
    } else {
        None
    }
}

fn git_install() -> Result<()> {
    run_cargo(&["install", "--git", REPO, "--force"])
        .context("failed to update from GitHub — check your network and try again")
}

fn path_install(root: &Path) -> Result<()> {
    eprintln!("Detected local install at {}", root.display());
    git_pull(root)?;
    run_cargo(&["install", "--path", &root.display().to_string(), "--force"])
        .context("failed to rebuild from local clone")
}

fn dev_build(root: &Path) -> Result<()> {
    eprintln!("Detected dev build in {}", root.display());
    git_pull(root)?;
    run_cargo(&["build", "--release"])
        .context("failed to rebuild dev binary")?;
    eprintln!("Built {}", root.join("target/release/cpos").display());
    Ok(())
}

fn git_pull(root: &Path) -> Result<()> {
    if !root.join(".git").is_dir() {
        return Ok(());
    }
    eprintln!("Pulling latest changes…");
    let status = Command::new("git")
        .args(["-C"])
        .arg(root)
        .args(["pull", "--ff-only"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("failed to run git pull")?;
    if !status.success() {
        bail!("git pull failed");
    }
    Ok(())
}

fn run_cargo(args: &[&str]) -> Result<()> {
    let status = Command::new("cargo")
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("failed to run cargo")?;
    if !status.success() {
        bail!("cargo {} failed", args.first().copied().unwrap_or("command"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_path_install_key() {
        let key = "cpos 0.1.0 (path+file:///Users/dev/cpos#0.1.0)";
        let path = parse_path_from_crate_key(key);
        assert_eq!(path, Some(PathBuf::from("/Users/dev/cpos")));
    }

    #[test]
    fn ignores_git_install_key() {
        let key = "cpos 0.1.0 (git+https://github.com/Soham109/cpos#abc123)";
        assert!(parse_path_from_crate_key(key).is_none());
    }
}
