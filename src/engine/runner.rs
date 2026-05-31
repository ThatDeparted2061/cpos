use anyhow::{Result, bail};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Instant;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

use crate::data::config::{CompileConfig, Config};
use crate::data::models::{TestCase, TestResult};

/// Where compiler output (binaries, .class/.jar files) goes — in the app's
/// data dir, never in the user's solution folders.
fn build_dir() -> PathBuf {
    let dir = Config::data_dir().join("build");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

pub async fn compile(source: &Path, config: &CompileConfig) -> Result<Option<String>> {
    let compile_cmd = match &config.compile {
        Some(cmd) => cmd,
        None => return Ok(None),
    };

    let source_str = source.to_string_lossy();
    let stem = source.file_stem().unwrap_or_default().to_string_lossy();
    let dir = build_dir();

    // Build artifacts land in the data dir's build folder; `{output}` is a name
    // relative to that folder, `{source}` is the absolute path to the user's
    // file, so the workspace stays clean.
    let expanded = compile_cmd
        .replace("{source}", &source_str)
        .replace("{output}", &stem)
        .replace("{dir}", &dir.to_string_lossy())
        .replace("{classname}", &stem);

    let output = Command::new("sh")
        .arg("-c")
        .arg(&expanded)
        .current_dir(&dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Compilation failed:\n{}", stderr.trim());
    }

    Ok(Some(stem.to_string()))
}

pub async fn run_test(
    source: &Path,
    config: &CompileConfig,
    compiled_output: Option<&str>,
    test: &TestCase,
    index: usize,
    timeout_ms: u64,
) -> TestResult {
    let source_str = source.to_string_lossy();
    let stem = source.file_stem().unwrap_or_default().to_string_lossy();
    let dir = build_dir();
    let output_str = compiled_output.unwrap_or(&stem);

    let run_cmd = config
        .run
        .replace("{source}", &source_str)
        .replace("{output}", output_str)
        .replace("{dir}", &dir.to_string_lossy())
        .replace("{classname}", &stem);

    let start = Instant::now();

    let child = Command::new("sh")
        .arg("-c")
        .arg(&run_cmd)
        .current_dir(&dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let mut child = match child {
        Ok(c) => c,
        Err(e) => {
            return TestResult {
                test_index: index,
                passed: false,
                input: test.input.clone(),
                expected: test.expected_output.clone(),
                actual: String::new(),
                time_ms: 0,
                error: Some(format!("Failed to spawn: {e}")),
            };
        }
    };

    if let Some(stdin) = child.stdin.as_mut() {
        let _ = stdin.write_all(test.input.as_bytes()).await;
        let _ = stdin.shutdown().await;
    }

    let timeout = tokio::time::Duration::from_millis(timeout_ms);
    let result = tokio::time::timeout(timeout, child.wait_with_output()).await;

    let elapsed = start.elapsed().as_millis() as u64;

    match result {
        Ok(Ok(output)) => {
            let actual = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
            let expected = test.expected_output.trim().to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            let passed = actual == expected;
            let error = if !output.status.success() && !stderr.is_empty() {
                Some(stderr)
            } else {
                None
            };

            TestResult {
                test_index: index,
                passed,
                input: test.input.clone(),
                expected,
                actual,
                time_ms: elapsed,
                error,
            }
        }
        Ok(Err(e)) => TestResult {
            test_index: index,
            passed: false,
            input: test.input.clone(),
            expected: test.expected_output.clone(),
            actual: String::new(),
            time_ms: elapsed,
            error: Some(format!("Process error: {e}")),
        },
        Err(_) => TestResult {
            test_index: index,
            passed: false,
            input: test.input.clone(),
            expected: test.expected_output.clone(),
            actual: String::new(),
            time_ms: elapsed,
            error: Some("Time limit exceeded".to_string()),
        },
    }
}

pub async fn run_all_tests(
    source: &Path,
    config: &CompileConfig,
    tests: &[TestCase],
    timeout_ms: u64,
) -> Result<Vec<TestResult>> {
    let compiled = compile(source, config).await?;
    let compiled_ref = compiled.as_deref();

    let mut results = Vec::new();
    for (i, test) in tests.iter().enumerate() {
        let result = run_test(source, config, compiled_ref, test, i + 1, timeout_ms).await;
        results.push(result);
    }

    Ok(results)
}
