use anyhow::{Result, bail};
use std::path::Path;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

pub struct StressResult {
    pub iterations: u64,
    pub found_mismatch: bool,
    pub failing_input: Option<String>,
    pub expected_output: Option<String>,
    pub actual_output: Option<String>,
}

async fn run_program(cmd: &str, dir: &Path, input: &str, timeout_ms: u64) -> Result<String> {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .current_dir(dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(input.as_bytes()).await?;
        stdin.shutdown().await?;
    }

    let timeout = tokio::time::Duration::from_millis(timeout_ms);
    let result = tokio::time::timeout(timeout, child.wait_with_output()).await;

    match result {
        Ok(Ok(output)) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                bail!("Runtime error: {stderr}");
            }
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        }
        Ok(Err(e)) => bail!("Process error: {e}"),
        Err(_) => bail!("Time limit exceeded"),
    }
}

pub async fn stress_test(
    generator_cmd: &str,
    solution_cmd: &str,
    brute_cmd: &str,
    work_dir: &Path,
    max_iterations: u64,
    timeout_ms: u64,
) -> Result<StressResult> {
    for i in 1..=max_iterations {
        let gen_cmd_with_seed = format!("{generator_cmd} {i}");
        let input = run_program(&gen_cmd_with_seed, work_dir, "", timeout_ms).await?;

        let expected = run_program(brute_cmd, work_dir, &input, timeout_ms).await?;
        let actual = run_program(solution_cmd, work_dir, &input, timeout_ms).await?;

        if expected != actual {
            return Ok(StressResult {
                iterations: i,
                found_mismatch: true,
                failing_input: Some(input),
                expected_output: Some(expected),
                actual_output: Some(actual),
            });
        }
    }

    Ok(StressResult {
        iterations: max_iterations,
        found_mismatch: false,
        failing_input: None,
        expected_output: None,
        actual_output: None,
    })
}
