//! CLI entry point for `gum-nix`.
//!
//! Usage:
//!   gum-nix [OPTIONS] [TARGET]
//!
//! Targets: build (default), clean, purge

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "gum-nix")]
#[command(about = "Generate Nix source expressions from Gum markup config files", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to packages.gum config file
    #[arg(short, long, default_value = "packages.gum")]
    config: PathBuf,

    /// Output directory for generated files
    #[arg(short, long, default_value = "_sources")]
    output_dir: PathBuf,

    /// Regex filter for package names
    #[arg(short, long)]
    filter: Option<String>,

    /// Number of parallel jobs (0 = auto)
    #[arg(short, long, default_value = "0")]
    jobs: usize,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Don't stop on first failure
    #[arg(long)]
    keep_going: bool,

    /// Show build timing
    #[arg(long)]
    timing: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate source expressions (default)
    Build,
    /// Remove generated files
    Clean,
    /// Remove state and cache
    Purge,
}

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();

    init_logging(cli.verbose);

    let command = cli.command.as_ref().unwrap_or(&Commands::Build);

    match command {
        Commands::Build => cmd_build(&cli),
        Commands::Clean => cmd_clean(&cli),
        Commands::Purge => cmd_purge(&cli),
    }
}

/// Initialize tracing/logging based on verbosity
fn init_logging(verbose: bool) {
    let env_filter = if verbose {
        "gum_nix=debug,info"
    } else {
        "gum_nix=info"
    };

    let _ = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .without_time()
        .try_init();
}

/// Build target: parse config, check versions, generate Nix
fn cmd_build(cli: &Cli) -> Result<ExitCode> {
    let start = if cli.timing { Some(std::time::Instant::now()) } else { None };

    tracing::info!("Reading config from {}", cli.config.display());

    let content = std::fs::read_to_string(&cli.config)
        .with_context(|| format!("Failed to read config file: {}", cli.config.display()))?;

    let config = gum_nix::Config::parse(&content)
        .with_context(|| "Failed to parse Gum config")?;

    tracing::info!("Found {} package(s) in config", config.packages.len());

    let packages = apply_filter(&config, cli.filter.as_deref())?;

    if packages.is_empty() {
        tracing::info!("No packages matched the filter");
        return Ok(ExitCode::SUCCESS);
    }

    let rt = tokio::runtime::Runtime::new()
        .context("Failed to create Tokio runtime")?;

    let results = rt.block_on(async {
        let mut results = Vec::new();
        for pkg in &packages {
            tracing::info!("Processing package: {}", pkg.name);
            let result = gum_nix::check_version(pkg).await;
            results.push((pkg.clone(), result));
        }
        results
    });

    let mut succeeded: Vec<gum_nix::ResolvedPackage> = Vec::new();
    let mut failed = Vec::new();

    for (pkg, version_result) in results {
        match version_result {
            Ok(version_info) => {
                tracing::info!("  {} -> version {}", pkg.name, version_info.version);
                succeeded.push((pkg, version_info));
            }
            Err(e) => {
                tracing::error!("  {} -> error: {}", pkg.name, e);
                if !cli.keep_going {
                    return Err(e);
                }
                failed.push(pkg.name.clone());
            }
        }
    }

    if !succeeded.is_empty() {
        std::fs::create_dir_all(&cli.output_dir)
            .with_context(|| format!("Failed to create output directory: {}", cli.output_dir.display()))?;

        let nix_expr = gum_nix::generate_nix(&succeeded)?;
        let json_meta = gum_nix::generate_json(&succeeded)?;

        let nix_path = cli.output_dir.join("generated.nix");
        let json_path = cli.output_dir.join("generated.json");

        std::fs::write(&nix_path, &nix_expr)
            .with_context(|| format!("Failed to write {}", nix_path.display()))?;
        std::fs::write(&json_path, &json_meta)
            .with_context(|| format!("Failed to write {}", json_path.display()))?;

        tracing::info!("Generated {} and {}", nix_path.display(), json_path.display());
    }

    if let Some(start) = start {
        tracing::info!("Total time: {:.2}s", start.elapsed().as_secs_f64());
    }

    if !failed.is_empty() {
        tracing::warn!("{} package(s) failed: {}", failed.len(), failed.join(", "));
    }

    Ok(ExitCode::SUCCESS)
}

/// Clean target: remove generated files
fn cmd_clean(cli: &Cli) -> Result<ExitCode> {
    let nix_path = cli.output_dir.join("generated.nix");
    let json_path = cli.output_dir.join("generated.json");

    for path in [&nix_path, &json_path] {
        if path.exists() {
            std::fs::remove_file(path)
                .with_context(|| format!("Failed to remove {}", path.display()))?;
            tracing::info!("Removed {}", path.display());
        }
    }

    Ok(ExitCode::SUCCESS)
}

/// Purge target: remove output directory entirely
fn cmd_purge(cli: &Cli) -> Result<ExitCode> {
    if cli.output_dir.exists() {
        std::fs::remove_dir_all(&cli.output_dir)
            .with_context(|| format!("Failed to remove {}", cli.output_dir.display()))?;
        tracing::info!("Removed {}", cli.output_dir.display());
    }

    Ok(ExitCode::SUCCESS)
}

/// Apply regex filter to packages
fn apply_filter(config: &gum_nix::Config, filter: Option<&str>) -> Result<Vec<gum_nix::Package>> {
    match filter {
        Some(pattern) => {
            let re = regex::Regex::new(pattern)
                .with_context(|| format!("Invalid filter regex: {}", pattern))?;
            Ok(config.packages.iter()
                .filter(|pkg| re.is_match(&pkg.name))
                .cloned()
                .collect())
        }
        None => Ok(config.packages.clone()),
    }
}
