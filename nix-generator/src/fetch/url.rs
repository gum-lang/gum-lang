//! URL fetcher: prefetch via nix-prefetch-url.

use anyhow::{Context, Result};
use super::PrefetchResult;

/// Prefetch a URL source using nix-prefetch-url.
pub async fn prefetch_url(url: &str) -> Result<PrefetchResult> {
    let output = tokio::process::Command::new("nix-prefetch-url")
        .args(["--type", "sha256", url])
        .output()
        .await
        .context("Failed to run nix-prefetch-url. Is it in your PATH?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("nix-prefetch-url failed: {}", stderr);
    }

    let sha256 = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    if sha256.is_empty() {
        anyhow::bail!("nix-prefetch-url returned empty output");
    }

    Ok(PrefetchResult {
        sha256,
        rev: String::new(),
        url: url.to_string(),
    })
}
