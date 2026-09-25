//! Gum Nix Generator — Generate Nix source expressions from Gum markup config files.
//!
//! This crate reads `.gum` configuration files that define package sources,
//! checks for latest versions, prefetches sources with SHA256 hashes, and
//! generates ready-to-use Nix expressions.

pub mod config;
pub mod fetch;
pub mod generate;
pub mod version;

pub use config::{Config, Package, VersionSource, Fetcher};
pub use generate::{generate_json, generate_nix, ResolvedPackage};
pub use version::{check_version, VersionInfo};
