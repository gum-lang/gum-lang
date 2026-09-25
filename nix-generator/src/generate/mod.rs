//! Nix expression generation: produce generated.nix and generated.json.

mod nix;
mod json;

use crate::version::VersionInfo;
use crate::config::Package;

pub use nix::generate_nix;
pub use json::generate_json;

/// A resolved package with version info (prefetch not yet implemented for MVP).
pub type ResolvedPackage = (Package, VersionInfo);
