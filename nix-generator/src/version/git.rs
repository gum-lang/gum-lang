//! Git version source: use `git ls-remote` to find latest tags/commits.

use anyhow::{Context, Result};
use semver::Version;
use super::VersionInfo;

/// Check a git remote for the latest version.
///
/// If a branch is specified, returns the latest commit SHA.
/// Otherwise, returns the latest tag (sorted by semver).
pub async fn check_git_remote(url: &str, branch: Option<&str>) -> Result<VersionInfo> {
    let output = tokio::process::Command::new("git")
        .args(["ls-remote", "--tags", url])
        .output()
        .await
        .context("Failed to run git ls-remote")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git ls-remote failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    if let Some(branch) = branch {
        // For branch mode, get the latest commit on the branch
        let branch_output = tokio::process::Command::new("git")
            .args(["ls-remote", url, format!("refs/heads/{}", branch).as_str()])
            .output()
            .await
            .context("Failed to run git ls-remote for branch")?;

        if !branch_output.status.success() {
            anyhow::bail!("Failed to find branch '{}'", branch);
        }

        let branch_stdout = String::from_utf8_lossy(&branch_output.stdout);
        let sha = branch_stdout.split_whitespace().next()
            .context("No commit found for branch")?
            .to_string();

        Ok(VersionInfo {
            version: sha[..7].to_string(),
            rev: Some(sha),
        })
    } else {
        // Parse tags and find the latest by semver
        let mut tags: Vec<(String, String)> = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let sha = parts[0].to_string();
                let ref_name = parts[1];
                // Extract tag name from refs/tags/v1.2.3 or refs/tags/1.2.3
                if let Some(tag) = ref_name.strip_prefix("refs/tags/") {
                    // Skip annotated tags (they end with ^{})
                    if !tag.ends_with("^{}") {
                        tags.push((tag.to_string(), sha));
                    }
                }
            }
        }

        if tags.is_empty() {
            anyhow::bail!("No tags found in remote");
        }

        // Sort by semver, fallback to lexicographic
        tags.sort_by(|a, b| {
            let va = Version::parse(a.0.strip_prefix('v').unwrap_or(&a.0));
            let vb = Version::parse(b.0.strip_prefix('v').unwrap_or(&b.0));
            match (va, vb) {
                (Ok(a), Ok(b)) => a.cmp(&b),
                _ => a.0.cmp(&b.0),
            }
        });

        let (latest_tag, sha) = tags.last().unwrap();

        Ok(VersionInfo {
            version: latest_tag.strip_prefix('v')
                .unwrap_or(latest_tag)
                .to_string(),
            rev: Some(sha.clone()),
        })
    }
}
