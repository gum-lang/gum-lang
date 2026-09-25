//! Manual version source: returns the configured version as-is.

use anyhow::Result;
use super::VersionInfo;

/// Return the manually configured version.
pub fn get_manual_version(version: &str) -> Result<VersionInfo> {
    Ok(VersionInfo {
        version: version.to_string(),
        rev: None,
    })
}
