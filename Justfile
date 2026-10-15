# Gum Language Ecosystem — Master Orchestrator
# ================================================
#
# This Justfile coordinates development across all workspace submodules.
# Each submodule (core, cli, lsp, parser-*, tree-sitter-lang, vscode-extension,
# website) is an independent repository managed via Git submodules.

# Default recipe — show available commands
default:
    @just --list

# ── Git Operations ───────────────────────────────────────────────────

# Clone or update all sub-repositories
clone-all:
    git submodule update --init --recursive

# Show git status for the main repo and all submodules
status-all:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "=== gum-lang (main) ==="
    git status --short
    for dir in core cli lsp parser-ts parser-python parser-go parser-rust tree-sitter-lang vscode-extension website; do
        echo
        echo "=== $dir ==="
        git -C "$dir" status --short 2>/dev/null || echo "  (not initialized)"
    done

# Fetch all remotes for main repo and submodules
fetch-all:
    git fetch --all
    git submodule foreach 'git fetch --all 2>/dev/null || true'

# Pull main repo and update all submodules to their tracked branches
pull-all:
    git pull
    git submodule update --remote --recursive

# Push main repo and all submodules (each to its own remote)
push-all:
    #!/usr/bin/env bash
    set -euo pipefail
    for dir in core cli lsp parser-ts parser-python parser-go parser-rust tree-sitter-lang vscode-extension website; do
        if [ -d "$dir/.git" ] || [ -f "$dir/.git" ]; then
            branch=$(git -C "$dir" rev-parse --abbrev-ref HEAD 2>/dev/null)
            if [ -n "$branch" ] && [ "$branch" != "HEAD" ]; then
                echo "Pushing $dir ($branch)..."
                git -C "$dir" push origin "$branch" 2>/dev/null || echo "  (skipped: no remote or up to date)"
            fi
        fi
    done
    echo "Pushing gum-lang (main)..."
    git push

# Create or switch to a branch across all repos
branch-all name:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Switching gum-lang (main) to '{{name}}'..."
    git checkout -b "{{name}}" 2>/dev/null || git checkout "{{name}}"
    for dir in core cli lsp parser-ts parser-python parser-go parser-rust tree-sitter-lang vscode-extension website; do
        echo "Switching $dir to '{{name}}'..."
        git -C "$dir" checkout -b "{{name}}" 2>/dev/null || git -C "$dir" checkout "{{name}}" 2>/dev/null || echo "  (skipped)"
    done

# Show recent commits across all repos
log-all:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "=== gum-lang (main) ==="
    git log --oneline -5
    for dir in core cli lsp parser-ts parser-python parser-go parser-rust tree-sitter-lang vscode-extension website; do
        echo
        echo "=== $dir ==="
        git -C "$dir" log --oneline -5 2>/dev/null || echo "  (no commits)"
    done

# Stash changes across all repos
stash-all:
    #!/usr/bin/env bash
    set -euo pipefail
    git stash
    for dir in core cli lsp parser-ts parser-python parser-go parser-rust tree-sitter-lang vscode-extension website; do
        git -C "$dir" stash 2>/dev/null || true
    done

# Pop stashed changes across all repos
stash-pop-all:
    #!/usr/bin/env bash
    set -euo pipefail
    git stash pop 2>/dev/null || true
    for dir in core cli lsp parser-ts parser-python parser-go parser-rust tree-sitter-lang vscode-extension website; do
        git -C "$dir" stash pop 2>/dev/null || true
    done

# Sync submodule URLs with .gitmodules (useful after remote changes)
sync-submodules:
    git submodule sync --recursive
    git submodule update --init --recursive

# ── Testing ──────────────────────────────────────────────────────────

# Run tests across all parser implementations and tooling
test-all: test-core test-cli test-lsp test-nix-generator test-parser-ts test-parser-python test-parser-go

test-core:
    cargo test --manifest-path core/Cargo.toml

test-cli:
    cargo test --manifest-path cli/Cargo.toml

test-lsp:
    cargo test --manifest-path lsp/Cargo.toml

test-nix-generator:
    cargo test --manifest-path nix-generator/Cargo.toml

test-parser-ts:
    npm test --prefix parser-ts

test-parser-python:
    pytest parser-python

test-parser-go:
    #!/usr/bin/env bash
    set -euo pipefail
    cd parser-go && go test ./...

# ── Building ─────────────────────────────────────────────────────────

# Build the Rust CLI and LSP binaries for local development
build-tools:
    cargo build --release --manifest-path core/Cargo.toml
    cargo build --release --manifest-path cli/Cargo.toml
    cargo build --release --manifest-path lsp/Cargo.toml
    cargo build --release --manifest-path nix-generator/Cargo.toml

# Build all parser packages
build-parsers: build-parsers-ts build-parsers-python build-parsers-go

build-parsers-ts:
    npm run build --prefix parser-ts

build-parsers-python:
    #!/usr/bin/env bash
    set -euo pipefail
    cd parser-python && pip install -e .

build-parsers-go:
    #!/usr/bin/env bash
    set -euo pipefail
    cd parser-go && go build ./...

# ── Formatting ───────────────────────────────────────────────────────

# Format all codebases according to their respective standards
format-all:
    cargo fmt --manifest-path core/Cargo.toml
    cargo fmt --manifest-path cli/Cargo.toml
    cargo fmt --manifest-path lsp/Cargo.toml
    cargo fmt --manifest-path nix-generator/Cargo.toml
    npm run format --prefix parser-ts
    #!/usr/bin/env bash
    set -euo pipefail
    black parser-python
    cd parser-go && go fmt ./...

# ── Linting ──────────────────────────────────────────────────────────

# Lint all codebases
lint-all:
    cargo clippy --all-targets --all-features --manifest-path core/Cargo.toml -- -D warnings
    cargo clippy --all-targets --all-features --manifest-path cli/Cargo.toml -- -D warnings
    cargo clippy --all-targets --all-features --manifest-path lsp/Cargo.toml -- -D warnings
    cargo clippy --all-targets --all-features --manifest-path nix-generator/Cargo.toml -- -D warnings
    npm run lint --prefix parser-ts
    #!/usr/bin/env bash
    set -euo pipefail
    cd parser-go && go vet ./...

# ── Integration ──────────────────────────────────────────────────────

# Run cross-repo integration tests against the shared test suite
integration-test:
    cargo test --manifest-path core/Cargo.toml --test integration

# ── WASM ─────────────────────────────────────────────────────────────

# Build the core engine to WebAssembly for browser playground
build-wasm:
    #!/usr/bin/env bash
    set -euo pipefail
    cd core && wasm-pack build --target web --out-dir ../website/public/pkg

# ── Website ──────────────────────────────────────────────────────────

# Build the documentation website
build-website:
    npm run build --prefix website

# Start the website dev server
serve-website:
    npm run dev --prefix website
