# Gum Language Ecosystem — Master Orchestrator
# ================================================
#
# This Justfile coordinates development across all workspace submodules.
# Each submodule (core, cli, lsp, parser-*, tree-sitter-lang, vscode-extension,
# website) is an independent repository managed via Git submodules.

# Default recipe — show available commands
default:
    @just --list

# ── Submodule Management ─────────────────────────────────────────────

# Clone or update all sub-repositories
clone-all:
    git submodule update --init --recursive

# ── Testing ──────────────────────────────────────────────────────────

# Run tests across all parser implementations and tooling
test-all: test-core test-cli test-lsp test-parser-ts test-parser-python test-parser-go

test-core:
    cargo test --manifest-path core/Cargo.toml

test-cli:
    cargo test --manifest-path cli/Cargo.toml

test-lsp:
    cargo test --manifest-path lsp/Cargo.toml

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
