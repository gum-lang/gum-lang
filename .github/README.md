# Gum Markup Language

A high-performance markup language ecosystem with a Rust-based core engine, multi-language parsers, native editor tooling, and comprehensive developer tooling.

## Ecosystem Repositories

| Repository | Description | Language |
|---|---|---|
| [`core`](../../core) | Canonical parser, AST, validator, formatter, WASM target | Rust |
| [`cli`](../../cli) | Command-line interface (`validate`, `format`, `convert`) | Rust |
| [`lsp`](../../lsp) | Language Server Protocol implementation | Rust |
| [`parser-ts`](../../parser-ts) | TypeScript parser (npm) | TypeScript |
| [`parser-python`](../../parser-python) | Python parser (PyPI) | Python |
| [`parser-go`](../../parser-go) | Go parser (GoProxy) | Go |
| [`parser-rust`](../../parser-rust) | Rust parser wrapper (crates.io) | Rust |
| [`tree-sitter-lang`](../../tree-sitter-lang) | Tree-sitter grammar for Neovim / syntax highlighting | C |
| [`vscode-extension`](../../vscode-extension) | VSCode extension bundling LSP & Tree-sitter | TypeScript |
| [`website`](../../website) | Documentation portal & WASM interactive playground | TypeScript / Astro |

## Quick Start

```bash
# Clone the workspace with all submodules
git clone --recurse-submodules <repo-url>
cd gum-lang

# Install dependencies and run all tests
just clone-all
just test-all

# Build CLI and LSP binaries
just build-tools
```

## Architecture

The formal grammar is defined in ABNF. The Rust `core` engine is the canonical reference implementation for AST generation, validation, and formatting. All ecosystem parsers must produce identical ASTs for the same input, validated continuously against the shared test suite.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.
