//! GitHub version source: fetch latest release via GitHub API.

use anyhow::{Context, Result};
use serde::Deserialize;
use super::VersionInfo;

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
}

/// Check the latest GitHub release for a repository.
pub async fn check_github_release(owner: &str, repo: &str) -> Result<VersionInfo> {
    let url = format!("https://api.github.com/repos/{}/{}/releases/latest", owner, repo);

    let client = reqwest::Client::builder()
        .user_agent("gum-nix/0.1.0")
        .build()
        .context("Failed to create HTTP client")?;

    let mut request = client.get(&url);

    // Use GITHUB_TOKEN if available for higher rate limits
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        request = request.header("Authorization", format!("Bearer {}", token));
    }

    let release: GitHubRelease = request
        .send()
        .await
        .context("Failed to send request to GitHub API")?
        .error_for_status()
        .context("GitHub API request failed")?
        .json()
        .await
        .context("Failed to parse GitHub API response")?;

    // Strip leading 'v' if present
    let version = release.tag_name.strip_prefix('v')
        .unwrap_or(&release.tag_name)
        .to_string();

    Ok(VersionInfo {
        version: version.clone(),
        rev: Some(release.tag_name),
    })
}
