# Autonomous Implementation Plan: Markup Language Ecosystem

This execution plan guides agentic developers through bootstrapping a complete markup language ecosystem from an initial ABNF specification file. The ecosystem utilizes a Rust-based core engine, multi-language parsers (TypeScript, Python, Rust, Go), native editor tooling (VSCode, Neovim), a CLI, a documentation website, and a meta-repository `Justfile` for orchestration.

If appropriate, each repository should have a flake.nix for development and a Justfile to standardize developers' tasks. Code should be well-commented in a manner suitable for human review.

---

## Phase 1: Meta-Repository & Workspace Bootstrapping

Establish the root orchestrator repository that manages all child repositories via Git submodules and provides a unified task runner.

### Tasks

1. **Initialize Workspace Repo (`workspace`)**:
* Create a root directory structure with a master `Justfile`.
* Set up Git submodules pointing to empty companion repositories (`core`, `cli`, `parser-ts`, `parser-python`, `parser-go`, `tree-sitter-lang`, `vscode-extension`, `website`).


2. **Implement Root `Justfile**`:
* Write recipes for cloning, cross-repo building, and synchronized testing.


3. **Configure Organization Defaults**:
* Create a `.github` repository containing global `.github/workflows/` templates, standard `CONTRIBUTING.md`, and an organization profile `README.md`.



### Deliverables

* `workspace` repo containing the master `Justfile`.
* Submodule links established to all child repositories.

---

## Phase 2: Core Engine & ABNF Parser (`core`)

Translate the provided ABNF specification into a robust, high-performance Rust parser that generates an Abstract Syntax Tree (AST), validates documents, and serves as the ecosystem's source of truth.

### Tasks

1. **Grammar & Lexer Implementation**:
* Parse the provided ABNF specification file into tokens (using `pest`, `nom`, or hand-written lexer/parser logic in Rust).
* Define a strongly typed AST structure (`struct` and `enum` hierarchies).


2. **Validator & Formatter Logic**:
* Build a traversal engine to inspect AST nodes for semantic correctness (Validator).
* Build a formatting engine that normalizes AST nodes back into standard markup strings (Formatter).


3. **Test Suite Generation**:
* Create an `integration_tests/` directory with a robust corpus of valid and invalid markup `.ext` files alongside expected AST outputs (golden files).


4. **WASM Compilation Target**:
* Configure `wasm-pack` compilation targets so the core engine can compile to WebAssembly for the browser playground and TypeScript wrapper usage.



### Deliverables

* Rust crate `core` published/ready locally with a comprehensive test suite.
* WASM build output pipeline.

---

## Phase 3: CLI & Language Server Protocol (`cli`, `lsp`)

Build the command-line interface and the Language Server Protocol (LSP) daemon using the Rust `core` engine.

### Tasks

1. **CLI Implementation (`cli`)**:
* Build a command-line interface using `clap`.
* Implement three mandatory subcommands:
* `validate <file>`: Runs syntax and semantic checks, exiting with non-zero codes on error.
* `format <file>`: Rewrites the file to stdout or in-place based on standard formatting rules.
* `convert <file> --to <format>`: Converts markup into output formats like HTML or JSON.




2. **LSP Implementation (`lsp`)**:
* Build a language server using `tower-lsp` that wraps the `core` parser.
* Implement handlers for `textDocument/didOpen`, `textDocument/didChange`, diagnostics (real-time error squiggles), and formatting requests.


3. **CI Matrix Builds**:
* Create GitHub Actions workflows to cross-compile CLI and LSP binaries for Linux, macOS (Intel/Apple Silicon), and Windows.



### Deliverables

* Fully functional Rust CLI executable.
* Standalone LSP binary capable of driving IDE intelligence.
* GitHub Actions release matrix for binaries.

---

## Phase 4: Native Multi-Language Parsers (`parser-ts`, `parser-python`, `parser-go`, `parser-rust`)

Build ecosystem-native parsers for host languages, ensuring they parse documents identically to the core Rust spec.

### Tasks

1. **Rust Parser Wrapper (`parser-rust`)**:
* Expose the `core` engine directly as an idiomatic Rust library crate.


2. **TypeScript Parser (`parser-ts`)**:
* Implement a native TypeScript parser (or wrap the WASM core build if optimal for lightweight environments) packaged for npm.


3. **Python Parser (`parser-python`)**:
* Implement a native Python parser packaged for PyPI (incorporating PyO3/maturin if wrapping Rust, or pure Python).


4. **Go Parser (`parser-go`)**:
* Implement a native Go parser packaged as a standard Go module.


5. **Shared Test Harness Sync**:
* Configure each parser repo's CI to download or submodule the `core` spec's test suite, running cross-language compliance tests to guarantee 100% AST parity.



### Deliverables

* Four independent package repositories ready for publishing (`crates.io`, `npm`, `PyPI`, `GoProxy`).

---

## Phase 5: Editor Integration & Tooling (`tree-sitter-lang`, `vscode-extension`)

Enable syntax highlighting, autocompletion, and real-time error checking inside Neovim and VSCode.

### Tasks

1. **Tree-sitter Grammar (`tree-sitter-lang`)**:
* Write the equivalent Tree-sitter C/WASM grammar matching the ABNF specification to support Neovim (`nvim-treesitter`) and GitHub code views.


2. **VSCode Extension (`vscode-extension`)**:
* Build a TypeScript-based VSCode extension (`vscode-languageclient`).
* Configure the extension to automatically bundle or fetch the platform-appropriate Rust LSP binary compiled in Phase 3.
* Bundle the `tree-sitter` grammar for syntax highlighting.


3. **Neovim Configuration Guide**:
* Document setup steps for `nvim-lspconfig` and `nvim-treesitter` in the organization docs.



### Deliverables

* `tree-sitter-lang` repository for syntax highlighting.
* `vscode-extension` published to the VSCode Marketplace.

---

## Phase 6: Documentation Website & Playground (`website`)

Create a public portal and interactive playground for the markup language.

### Tasks

1. **Framework Setup**:
* Initialize a documentation site using Astro, VitePress, or Docusaurus.


2. **Interactive Playground**:
* Integrate the WASM build of the `core` parser into a web page component, enabling real-time AST generation and validation directly in the browser without server roundtrips.


3. **Hosting & Deployment**:
* Configure GitHub Pages or Vercel deployment workflows triggered on main branch updates.



### Deliverables

* Public-facing documentation site with an interactive playground.
