//! Generate JSON metadata (generated.json).

use anyhow::Result;
use serde::Serialize;
use crate::config::{Fetcher, Package};
use crate::version::VersionInfo;
use super::ResolvedPackage;

#[derive(Serialize)]
struct PackageMeta {
    name: String,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extract: Option<Vec<String>>,
    pinned: bool,
    src: SrcMeta,
}

#[derive(Serialize)]
struct SrcMeta {
    #[serde(rename = "type")]
    src_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    rev: String,
    sha256: String,
}

/// Generate JSON metadata from resolved packages.
pub fn generate_json(packages: &[ResolvedPackage]) -> Result<String> {
    let mut meta = serde_json::Map::new();

    for (pkg, version_info) in packages {
        let src_meta = build_src_meta(pkg, version_info);

        let package_meta = PackageMeta {
            name: pkg.name.clone(),
            version: version_info.version.clone(),
            date: None,
            extract: if pkg.extract.is_empty() { None } else { Some(pkg.extract.clone()) },
            pinned: false,
            src: src_meta,
        };

        meta.insert(pkg.name.clone(), serde_json::to_value(package_meta)?);
    }

    Ok(serde_json::to_string_pretty(&meta)?)
}

fn build_src_meta(pkg: &Package, version_info: &VersionInfo) -> SrcMeta {
    let (src_type, owner, repo, url) = match &pkg.fetch {
        Fetcher::GitHub { owner, repo } => (
            "github".to_string(),
            Some(owner.clone()),
            Some(repo.clone()),
            None,
        ),
        Fetcher::Git { url } => (
            "git".to_string(),
            None,
            None,
            Some(url.clone()),
        ),
        Fetcher::Url { url } => (
            "url".to_string(),
            None,
            None,
            Some(url.clone()),
        ),
    };

    let rev = version_info.rev.clone()
        .unwrap_or_else(|| version_info.version.clone());
    let sha256 = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".to_string();

    SrcMeta {
        src_type,
        owner,
        repo,
        url,
        rev,
        sha256,
    }
}
