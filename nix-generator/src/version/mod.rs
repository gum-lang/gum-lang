//! Version source checking: determine the latest version for a package.

mod github;
mod git;
mod manual;

use anyhow::Result;
use crate::config::{Package, VersionSource};

pub use github::check_github_release;
pub use git::check_git_remote;
pub use manual::get_manual_version;

/// Result of a version check.
#[derive(Debug, Clone)]
pub struct VersionInfo {
    /// The resolved version string.
    pub version: String,
    /// The git revision (SHA or tag) to fetch, if applicable.
    pub rev: Option<String>,
}

/// Check the version for a package based on its version source.
pub async fn check_version(pkg: &Package) -> Result<VersionInfo> {
    match &pkg.src {
        VersionSource::Manual { version } => {
            get_manual_version(version)
        }
        VersionSource::GitHub { owner, repo } => {
            check_github_release(owner, repo).await
        }
        VersionSource::Git { url } => {
            check_git_remote(url, pkg.git_branch.as_deref()).await
        }
    }
}
