//! GitHub fetcher: prefetch from GitHub release tarballs.

use anyhow::{Context, Result};
use super::PrefetchResult;

/// Prefetch a GitHub source tarball.
pub async fn prefetch_github(owner: &str, repo: &str, version: &str) -> Result<PrefetchResult> {
    let url = format!(
        "https://github.com/{}/{}/archive/refs/tags/{}.tar.gz",
        owner, repo, version
    );

    super::url::prefetch_url(&url).await
        .with_context(|| format!("Failed to prefetch GitHub tarball for {}/{}@{}", owner, repo, version))
}
