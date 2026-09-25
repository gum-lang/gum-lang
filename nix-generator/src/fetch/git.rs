//! Git fetcher: prefetch via nix-prefetch-git.

use anyhow::{Context, Result};
use super::PrefetchResult;

/// Prefetch a git source using nix-prefetch-git.
pub async fn prefetch_git(url: &str, rev: &str) -> Result<PrefetchResult> {
    let output = tokio::process::Command::new("nix-prefetch-git")
        .args([url, rev])
        .output()
        .await
        .context("Failed to run nix-prefetch-git. Is it in your PATH?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("nix-prefetch-git failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let sha256 = extract_sha256(&stdout)
        .context("Failed to extract sha256 from nix-prefetch-git output")?;

    Ok(PrefetchResult {
        sha256,
        rev: rev.to_string(),
        url: url.to_string(),
    })
}

/// Extract the sha256 from nix-prefetch-git JSON output.
fn extract_sha256(output: &str) -> Option<String> {
    // nix-prefetch-git outputs JSON with a "sha256" field
    for line in output.lines() {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
            if let Some(sha) = json.get("sha256").and_then(|v| v.as_str()) {
                return Some(sha.to_string());
            }
        }
    }
    None
}
