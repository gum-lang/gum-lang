//! Fetcher module: download and prefetch source archives.

mod github;
mod git;
mod url;

use anyhow::Result;
use crate::config::Fetcher;

pub use github::prefetch_github;
pub use git::prefetch_git;
pub use url::prefetch_url;

/// Result of a prefetch operation.
#[derive(Debug, Clone)]
pub struct PrefetchResult {
    /// SHA256 hash of the source.
    pub sha256: String,
    /// The resolved revision/tag.
    pub rev: String,
    /// The URL used for fetching.
    pub url: String,
}

/// Prefetch a package source based on its fetcher configuration.
pub async fn prefetch(fetcher: &Fetcher, version: &str, rev: Option<&str>) -> Result<PrefetchResult> {
    match fetcher {
        Fetcher::GitHub { owner, repo } => {
            prefetch_github(owner, repo, version).await
        }
        Fetcher::Git { url } => {
            let rev = rev.unwrap_or(version);
            prefetch_git(url, rev).await
        }
        Fetcher::Url { url } => {
            let url = interpolate_url(url, version);
            prefetch_url(&url).await
        }
    }
}

/// Interpolate `$ver` in a URL pattern.
pub fn interpolate_url(pattern: &str, version: &str) -> String {
    pattern.replace("$ver", version)
}
