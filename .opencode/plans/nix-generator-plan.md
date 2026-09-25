# Plan: `gum-lang/nix-generator` — nvfetcher Alternative

## Goal

Build a Rust crate that reads GUM markup config files defining package sources, checks for latest versions, prefetches sources with SHA256 hashes, and generates ready-to-use Nix expressions — serving as a drop-in alternative to `nvfetcher`.

---

## GUM Config Schema (Design)

A `packages.gum` file replaces `nvfetcher.toml`. Each top-level key is a package name, with a table value defining its version source, fetcher, and optional config:

```gum
# packages.gum

rustdesk = {
    src.github = "rustdesk/rustdesk"
    fetch.github = "rustdesk/rustdesk"
    cargo_lock = ["Cargo.lock"]
}

hello = {
    src.manual = "2.12.1"
    fetch.url = "https://ftp.gnu.org/gnu/hello/hello-$ver.tar.gz"
}

mylib = {
    src.git = "https://git.example.com/repo.git"
    fetch.git = "https://git.example.com/repo.git"
    git.branch = "main"
}
```

### Supported fields (MVP)

**Version sources (`src.*`):**
- `src.github = "owner/repo"` — latest GitHub release
- `src.git = "url"` — latest commit of a git repo
- `src.manual = "version"` — fixed version, never updates

**Fetchers (`fetch.*`):**
- `fetch.github = "owner/repo"` — `fetchFromGitHub`
- `fetch.git = "url"` — `fetchgit`
- `fetch.url = "url"` — `fetchurl` (with `$ver` interpolation)

**Optional:**
- `cargo_lock = ["glob1", "glob2"]` — extract Cargo.lock files and compute outputHashes
- `git.branch = "branch"` — branch to track for git sources
- `extract = ["glob1"]` — extract additional files from source

---

## Phase 1: Crate Scaffolding

### 1.1 Create `nix-generator/` crate

```
nix-generator/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Public API: generate(config_path, output_dir)
│   ├── main.rs         # CLI binary: `gum-nix` command
│   ├── config.rs       # GUM config parsing → typed Package definitions
│   ├── version/
│   │   ├── mod.rs      # Version source trait + dispatcher
│   │   ├── github.rs   # GitHub releases API
│   │   ├── git.rs      # git ls-remote
│   │   └── manual.rs   # Fixed version
│   ├── fetch/
│   │   ├── mod.rs      # Fetcher trait + dispatcher
│   │   ├── github.rs   # fetchFromGitHub + nix-prefetch
│   │   ├── git.rs      # fetchgit + nix-prefetch-git
│   │   └── url.rs      # fetchurl + nix-prefetch-url
│   ├── generate/
│   │   ├── mod.rs      # Nix expression generator
│   │   ├── nix.rs      # Generate generated.nix
│   │   └── json.rs     # Generate generated.json
│   └── cargo_lock.rs   # Cargo.lock extraction + outputHash computation
├── tests/
│   └── integration.rs  # End-to-end test with sample .gum files
└── example/
    └── packages.gum    # Example config file
```

### 1.2 `Cargo.toml` dependencies

- `gum-core` — parse .gum config files
- `reqwest` + `tokio` — HTTP client for GitHub API
- `serde` + `serde_json` — serialization for generated.json
- `clap` — CLI argument parsing
- `glob` — file pattern matching for cargo_lock/extract
- `sha2` — SHA256 computation for cargoLock outputHashes
- `url` — URL parsing and `$ver` interpolation
- `anyhow` — error handling
- `tracing` — logging

### 1.3 CLI interface

```
gum-nix [OPTIONS] [TARGET]

Targets:
  build    — generate sources (default)
  clean    — remove generated files
  purge    — remove state/cache

Options:
  -c, --config FILE     Path to packages.gum (default: "packages.gum")
  -o, --output-dir DIR  Output directory (default: "_sources")
  -f, --filter REGEX    Filter packages by name
  -j, --jobs NUM        Parallel jobs (default: 0 = auto)
  -v, --verbose         Verbose output
  --keep-going          Don't stop on first failure
  --timing              Show build timing
```

### 1.4 Justfile + flake integration

- Add `test-nix-generator`, `build-nix-generator`, `lint-nix-generator` recipes to root Justfile
- Add `nix-generator` to flake.nix dev shell
- Flake outputs: `packages.gum-nix` (the binary), `devShells.with-nix-generator`

---

## Phase 2: Config Parsing

### 2.1 Typed config model

```rust
pub struct Package {
    pub name: String,
    pub src: VersionSource,
    pub fetch: Fetcher,
    pub cargo_lock: Vec<String>,
    pub extract: Vec<String>,
    pub git_branch: Option<String>,
}

pub enum VersionSource {
    GitHub { owner: String, repo: String },
    Git { url: String },
    Manual { version: String },
}

pub enum Fetcher {
    GitHub { owner: String, repo: String },
    Git { url: String },
    Url { url: String },
}
```

### 2.2 Parse from GUM Document

- Walk the parsed `gum_core::Document`
- Each top-level `Assignment` with a `Table` value becomes a `Package`
- Extract `src.*`, `fetch.*`, and optional fields from table entries
- Validate required fields (each package must have exactly one `src.*` and one `fetch.*`)
- Support `$ver` string interpolation in URL patterns

### 2.3 Config validation errors

- Missing `src.*` or `fetch.*`
- Unknown `src.*` or `fetch.*` keys
- Invalid URL patterns
- Reserved keyword conflicts

---

## Phase 3: Version Sources

### 3.1 `src.github` — GitHub Releases

- Call `GET https://api.github.com/repos/{owner}/releases/latest`
- Extract `tag_name` as version
- Handle rate limiting (support `GITHUB_TOKEN` env var)
- Parse semver-compatible tags (strip leading `v`)

### 3.2 `src.git` — Git ls-remote

- Run `git ls-remote {url} refs/tags/* refs/heads/{branch}`
- For tags: pick the latest tag by version sort
- For branch: pick the latest commit SHA
- Return version = tag name or commit SHA (short)

### 3.3 `src.manual` — Fixed version

- Return the configured version string as-is

### 3.4 Version comparison

- Implement basic semver-aware sorting for tag selection
- Fallback to lexicographic comparison for non-semver

---

## Phase 4: Fetchers & Prefetch

### 4.1 `fetch.github` — GitHub source

- Construct tarball URL: `https://github.com/{owner}/repo/archive/refs/tags/{version}.tar.gz`
- Run `nix-prefetch-url --type sha256 {url}` to get hash
- Optionally handle `cargo_lock` globs

### 4.2 `fetch.git` — Generic git

- Run `nix-prefetch-git {url} {rev}` to get hash
- Support `--fetch-submodules` flag if configured

### 4.3 `fetch.url` — Direct URL

- Interpolate `$ver` in the URL pattern
- Run `nix-prefetch-url --type sha256 {url}` to get hash

### 4.4 Prefetch abstraction

- All prefetchers go through `nix-prefetch-*` CLI tools (available in nixpkgs)
- Cache results to avoid redundant prefetches when version hasn't changed
- Read `generated.json` from previous runs to detect version changes

---

## Phase 5: Nix Expression Generation

### 5.1 `generated.nix` output

```nix
# This file was generated by gum-nix, please do not modify it manually.
{
  fetchFromGitHub,
  fetchgit,
  fetchurl,
}:
{
  rustdesk = {
    pname = "rustdesk";
    version = "1.4.2";
    src = fetchFromGitHub {
      owner = "rustdesk";
      repo = "rustdesk";
      rev = "1.4.2";
      sha256 = "sha256-b/KuWECsmEcz9nPiuPqCcGQ1p+GdbaELScaIXnUTGv8=";
    };
    cargoLock."Cargo.lock" = {
      lockFile = ./. + "/sha256-.../Cargo.lock";
      outputHashes = {
        "filedescriptor-0.8.2" = "sha256-...";
      };
    };
  };
}
```

### 5.2 `generated.json` output

Metadata file tracking versions, hashes, and fetcher config for incremental builds:

```json
{
  "rustdesk": {
    "version": "1.4.2",
    "src": { "type": "github", "owner": "rustdesk", "repo": "rustdesk", "rev": "1.4.2", "sha256": "..." },
    "cargoLock": { "Cargo.lock": ["sha256-.../Cargo.lock", { "dep-1.0": "sha256-..." }] }
  }
}
```

### 5.3 Cargo.lock handling

- Extract Cargo.lock from prefetched source
- Parse lock file to find all dependencies
- Compute SHA256 for each dependency's source
- Generate `outputHashes` map in the Nix expression

---

## Phase 6: Flake & CI Integration

### 6.1 Flake outputs

```nix
{
  packages.gum-nix = rustPlatform.buildRustPackage { ... };
  apps.gum-nix = { type = "app"; program = "${packages.gum-nix}/bin/gum-nix"; };
}
```

### 6.2 GitHub Actions workflow

- Build `gum-nix` binary for Linux/macOS
- Run integration tests with sample `.gum` configs
- Publish binary releases

### 6.3 Usage as a flake input

```nix
# In a user's flake.nix
inputs.gum-nix.url = "github:gum-lang/nix-generator";

# Then use in devShell or packages
```

---

## Phase 7: Testing

### 7.1 Unit tests

- Config parsing from GUM documents (valid + invalid cases)
- Version source URL construction
- `$ver` interpolation
- Nix expression string generation

### 7.2 Integration tests

- End-to-end with `src.manual` + `fetch.url` (no network needed for manual)
- Verify `generated.nix` is syntactically valid Nix (via `nix-instantiate --parse`)
- Verify `generated.json` roundtrips correctly

### 7.3 Fixture-based tests

- Sample `.gum` config files in `tests/fixtures/`
- Expected `generated.nix` and `generated.json` as golden files

---

## Execution Order

1. **Phase 1** — Crate scaffolding, Cargo.toml, CLI skeleton
2. **Phase 2** — Config parsing from GUM → typed model
3. **Phase 3** — Version sources (manual first, then GitHub, then git)
4. **Phase 4** — Fetchers with nix-prefetch integration
5. **Phase 5** — Nix + JSON generation
6. **Phase 6** — Flake + CI
7. **Phase 7** — Tests throughout (TDD approach)

## MVP Definition

The MVP is complete when:
- A `packages.gum` file with `src.github`/`src.manual` + `fetch.github`/`fetch.url` packages can be processed
- `gum-nix build` produces valid `generated.nix` and `generated.json`
- The generated Nix expressions can be used in a real flake
- The tool is accessible via `nix run github:gum-lang/nix-generator`
