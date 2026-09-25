//! Configuration parsing: parse GUM documents into typed Package definitions.

use anyhow::{Context, Result};
use gum_core::{Document, DocumentElement, Key, Value, StringValue, Table};

/// A complete parsed configuration.
#[derive(Debug, Clone)]
pub struct Config {
    /// All packages defined in the config.
    pub packages: Vec<Package>,
}

/// A single package definition.
#[derive(Debug, Clone)]
pub struct Package {
    /// Package name (top-level key in the config).
    pub name: String,
    /// How to determine the latest version.
    pub src: VersionSource,
    /// How to fetch the source archive.
    pub fetch: Fetcher,
    /// Cargo.lock glob patterns to extract.
    pub cargo_lock: Vec<String>,
    /// File glob patterns to extract from source.
    pub extract: Vec<String>,
    /// Git branch to track (for git sources).
    pub git_branch: Option<String>,
}

/// Version source configuration.
#[derive(Debug, Clone)]
pub enum VersionSource {
    /// Latest GitHub release: src.github = "owner/repo"
    GitHub { owner: String, repo: String },
    /// Latest git commit: src.git = "url"
    Git { url: String },
    /// Fixed version: src.manual = "version"
    Manual { version: String },
}

/// Fetcher configuration.
#[derive(Debug, Clone)]
pub enum Fetcher {
    /// fetchFromGitHub: fetch.github = "owner/repo"
    GitHub { owner: String, repo: String },
    /// fetchgit: fetch.git = "url"
    Git { url: String },
    /// fetchurl: fetch.url = "url" (supports $ver interpolation)
    Url { url: String },
}

impl Config {
    /// Parse a GUM document string into a typed Config.
    pub fn parse(input: &str) -> Result<Self> {
        let doc = gum_core::parse(input)
            .context("Failed to parse Gum document")?;

        let packages = parse_packages(&doc)?;

        Ok(Config { packages })
    }
}

/// Extract package definitions from the document.
fn parse_packages(doc: &Document) -> Result<Vec<Package>> {
    let mut packages = Vec::new();

    for element in &doc.elements {
        if let DocumentElement::Assignment(assignment) = element {
            let name = match &assignment.key {
                Key::Bare(s) | Key::Quoted(s) => s.clone(),
                Key::Dotted(_) => anyhow::bail!("Dotted keys are not allowed at the top level"),
            };

            if let Value::Table(table) = &assignment.value {
                let pkg = parse_package(&name, table)?;
                packages.push(pkg);
            } else {
                anyhow::bail!("Package '{}' must be a table, got {:?}", name, assignment.value);
            }
        }
    }

    Ok(packages)
}

/// Parse a single package table into a Package struct.
fn parse_package(name: &str, table: &Table) -> Result<Package> {
    let mut src: Option<VersionSource> = None;
    let mut fetch: Option<Fetcher> = None;
    let mut cargo_lock = Vec::new();
    let mut extract = Vec::new();
    let mut git_branch: Option<String> = None;

    for entry in &table.entries {
        let key_str = entry.key.as_str();

        match key_str.as_str() {
            "src.github" => {
                let (owner, repo) = parse_owner_repo(&entry.value)?;
                src = Some(VersionSource::GitHub { owner, repo });
            }
            "src.git" => {
                let url = parse_string_value(&entry.value)?;
                src = Some(VersionSource::Git { url });
            }
            "src.manual" => {
                let version = parse_string_value(&entry.value)?;
                src = Some(VersionSource::Manual { version });
            }
            "fetch.github" => {
                let (owner, repo) = parse_owner_repo(&entry.value)?;
                fetch = Some(Fetcher::GitHub { owner, repo });
            }
            "fetch.git" => {
                let url = parse_string_value(&entry.value)?;
                fetch = Some(Fetcher::Git { url });
            }
            "fetch.url" => {
                let url = parse_string_value(&entry.value)?;
                fetch = Some(Fetcher::Url { url });
            }
            "cargo_lock" => {
                cargo_lock = parse_string_list(&entry.value)?;
            }
            "extract" => {
                extract = parse_string_list(&entry.value)?;
            }
            "git.branch" => {
                git_branch = Some(parse_string_value(&entry.value)?);
            }
            _ => {
                anyhow::bail!("Unknown config key '{}' in package '{}'", key_str, name);
            }
        }
    }

    let src = src.ok_or_else(|| anyhow::anyhow!("Package '{}' is missing a src.* field", name))?;
    let fetch = fetch.ok_or_else(|| anyhow::anyhow!("Package '{}' is missing a fetch.* field", name))?;

    Ok(Package {
        name: name.to_string(),
        src,
        fetch,
        cargo_lock,
        extract,
        git_branch,
    })
}

/// Parse an "owner/repo" string from a value.
fn parse_owner_repo(value: &Value) -> Result<(String, String)> {
    let s = parse_string_value(value)?;
    let parts: Vec<&str> = s.splitn(2, '/').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        anyhow::bail!("Expected 'owner/repo' format, got '{}'", s);
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

/// Extract a string from a Value.
fn parse_string_value(value: &Value) -> Result<String> {
    match value {
        Value::String(s) => Ok(match s {
            StringValue::Basic(s)
            | StringValue::MultilineBasic(s)
            | StringValue::Literal(s)
            | StringValue::MultilineLiteral(s) => s.clone(),
        }),
        _ => anyhow::bail!("Expected a string value, got {:?}", value),
    }
}

/// Parse a list of strings from a Value (must be a List of Strings).
fn parse_string_list(value: &Value) -> Result<Vec<String>> {
    match value {
        Value::List(list) => {
            let mut items = Vec::new();
            for elem in &list.elements {
                items.push(parse_string_value(elem)?);
            }
            Ok(items)
        }
        _ => anyhow::bail!("Expected a list value, got {:?}", value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_config() {
        let input = r#"
hello = {
    src.manual = "2.12.1"
    fetch.url = "https://ftp.gnu.org/gnu/hello/hello-$ver.tar.gz"
}
"#;
        let config = Config::parse(input).unwrap();
        assert_eq!(config.packages.len(), 1);
        let pkg = &config.packages[0];
        assert_eq!(pkg.name, "hello");
        assert!(matches!(pkg.src, VersionSource::Manual { .. }));
        assert!(matches!(pkg.fetch, Fetcher::Url { .. }));
    }

    #[test]
    fn test_parse_github_config() {
        let input = r#"
rustdesk = {
    src.github = "rustdesk/rustdesk"
    fetch.github = "rustdesk/rustdesk"
    cargo_lock = ["Cargo.lock"]
}
"#;
        let config = Config::parse(input).unwrap();
        assert_eq!(config.packages.len(), 1);
        let pkg = &config.packages[0];
        assert_eq!(pkg.name, "rustdesk");
        assert!(matches!(pkg.src, VersionSource::GitHub { .. }));
        assert!(matches!(pkg.fetch, Fetcher::GitHub { .. }));
        assert_eq!(pkg.cargo_lock, vec!["Cargo.lock"]);
    }

    #[test]
    fn test_parse_multiple_packages() {
        let input = r#"
pkg1 = {
    src.manual = "1.0"
    fetch.url = "https://example.com/pkg1-$ver.tar.gz"
}

pkg2 = {
    src.manual = "2.0"
    fetch.url = "https://example.com/pkg2-$ver.tar.gz"
}
"#;
        let config = Config::parse(input).unwrap();
        assert_eq!(config.packages.len(), 2);
    }

    #[test]
    fn test_parse_missing_src() {
        let input = r#"
bad = {
    fetch.url = "https://example.com/pkg.tar.gz"
}
"#;
        let result = Config::parse(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("src.*"));
    }

    #[test]
    fn test_parse_missing_fetch() {
        let input = r#"
bad = {
    src.manual = "1.0"
}
"#;
        let result = Config::parse(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("fetch.*"));
    }

    #[test]
    fn test_parse_unknown_key() {
        let input = r#"
bad = {
    src.manual = "1.0"
    fetch.url = "https://example.com/pkg.tar.gz"
    unknown.field = "value"
}
"#;
        let result = Config::parse(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unknown.field"));
    }

    #[test]
    fn test_parse_git_with_branch() {
        let input = r#"
mylib = {
    src.git = "https://git.example.com/repo.git"
    fetch.git = "https://git.example.com/repo.git"
    git.branch = "main"
}
"#;
        let config = Config::parse(input).unwrap();
        let pkg = &config.packages[0];
        assert!(matches!(pkg.src, VersionSource::Git { .. }));
        assert!(matches!(pkg.fetch, Fetcher::Git { .. }));
        assert_eq!(pkg.git_branch, Some("main".to_string()));
    }

    #[test]
    fn test_parse_extract_list() {
        let input = r#"
pkg = {
    src.manual = "1.0"
    fetch.url = "https://example.com/pkg.tar.gz"
    extract = ["**/*.rs", "Cargo.toml"]
}
"#;
        let config = Config::parse(input).unwrap();
        let pkg = &config.packages[0];
        assert_eq!(pkg.extract, vec!["**/*.rs", "Cargo.toml"]);
    }
}
