agents_md_content = """# Developer Guide & Agent Instructions (`AGENTS.md`)

This document serves as the primary technical directive for autonomous development agents and human contributors working within the `@your-org` markup language ecosystem. Read this entire document before executing tasks across the meta-repository or its submodules.

---

## 1. Core Architecture & Design Principles

- **Language of Truth:** The formal grammar is derived from the ABNF specification. The Rust `core` engine serves as the canonical reference implementation for AST generation, validation, and formatting.
- **Language Stack:**
  - **Tooling, CLI, LSP & Core Engine:** Rust (Edition 2021+).
  - **Ecosystem Parsers:** TypeScript (`parser-ts`), Python (`parser-python`), Go (`parser-go`), Rust (`parser-rust`).
  - **Editor Tooling:** TypeScript (VSCode Extension), C/WASM (Tree-sitter).
  - **Documentation & Playground:** TypeScript / Astro (or Docusaurus) running WASM.
- **Cross-Language Parity:** All language parsers **must** produce identical Abstract Syntax Trees (ASTs) for the same input text, validated continuously against the shared test suite.

---

## 2. Repository Layout & Workspace Management

The organization uses a meta-repository workspace structure with independent child repositories managed via Git submodules and orchestrated via a root `Justfile`.

````text
workspace/
├── Justfile               # Master orchestrator
├── .github/               # Organization-wide workflows & templates
├── core/                  # Canonical Rust parser, AST, Validator, Formatter, WASM target
├── cli/                   # Rust CLI (`validate`, `format`, `convert`)
├── lsp/                   # Rust Language Server Protocol implementation
├── parser-ts/             # TypeScript parser package (npm)
├── parser-python/         # Python parser package (PyPI)
├── parser-go/             # Go parser package (GoProxy)
├── tree-sitter-lang/      # Tree-sitter grammar for Neovim / syntax highlighting
├── vscode-extension/      # VSCode extension bundling LSP & Tree-sitter
└── website/               # Documentation portal & WASM interactive playground

```python
agents_md_content = """# Developer Guide & Agent Instructions (`AGENTS.md`)

This document serves as the primary technical directive for autonomous development agents and human contributors working within the `@your-org` markup language ecosystem. Read this entire document before executing tasks across the meta-repository or its submodules.
````

---

## 1. Core Architecture & Design Principles

- **Language of Truth:** The formal grammar is derived from the ABNF specification. The Rust `core` engine serves as the canonical reference implementation for AST generation, validation, and formatting.
- **Language Stack:**
  - **Tooling, CLI, LSP & Core Engine:** Rust (Edition 2021+).
  - **Ecosystem Parsers:** TypeScript (`parser-ts`), Python (`parser-python`), Go (`parser-go`), Rust (`parser-rust`).
  - **Editor Tooling:** TypeScript (VSCode Extension), C/WASM (Tree-sitter).
  - **Documentation & Playground:** TypeScript / Astro (or Docusaurus) running WASM.
- **Cross-Language Parity:** All language parsers **must** produce identical Abstract Syntax Trees (ASTs) for the same input text, validated continuously against the shared test suite.

---

## 2. Repository Layout & Workspace Management

The organization uses a meta-repository workspace structure with independent child repositories managed via Git submodules and orchestrated via a root `Justfile`.

```text
workspace/
├── Justfile               # Master orchestrator
├── .github/               # Organization-wide workflows & templates
├── core/                  # Canonical Rust parser, AST, Validator, Formatter, WASM target
├── cli/                   # Rust CLI (`validate`, `format`, `convert`)
├── lsp/                   # Rust Language Server Protocol implementation
├── parser-ts/             # TypeScript parser package (npm)
├── parser-python/         # Python parser package (PyPI)
├── parser-go/             # Go parser package (GoProxy)
├── tree-sitter-lang/      # Tree-sitter grammar for Neovim / syntax highlighting
├── vscode-extension/      # VSCode extension bundling LSP & Tree-sitter
└── website/               # Documentation portal & WASM interactive playground
```

---

## 3. Master `Justfile` Commands

Agents should use the root `Justfile` for workspace-wide operations. Always run commands through `just <recipe>`:

- `just clone-all` — Initializes and updates all Git submodules.
- `just test-all` — Executes test suites across Rust, TypeScript, Python, and Go repositories.
- `just build-tools` — Builds optimized release binaries for the Rust CLI and LSP.
- `just format-all` — Formats all codebases using their respective native formatters (`cargo fmt`, `npm run format`, `black`, `go fmt`).
- `just integration-test` — Runs cross-repo validation compliance tests against the shared test suite.

---

## 4. Development Workflow Rules for Agents

### Rule 1: Isolation & Context

- When assigned a task targeting a specific repository (e.g., `cli` or `parser-python`), navigate to that submodule directory or treat it as an isolated workspace context. Do not modify unrelated repositories unless executing cross-repo updates (e.g., updating spec test vectors).

### Rule 2: Test-Driven Parity

- Every bug fix or feature addition to parser libraries **must** include a corresponding test case added to the shared spec test corpus.
- Run `just test-all` before submitting any pull request or patch to ensure no regressions occur across host languages.

### Rule 3: Rust Code Standards

- Use idiomatic Rust. Avoid unsafe code blocks unless explicitly required for FFI/WASM performance bounds.
- Document public APIs using standard rustdoc comments (`///`).
- Ensure all Rust crates compile cleanly under `cargo clippy --all-targets --all-features` with zero warnings.

### Rule 4: Multi-Platform Binaries

- The CLI and LSP are compiled via GitHub Actions matrix workflows for Linux (x86_64, aarch64), macOS (Intel, Apple Silicon), and Windows (x86_64). Avoid adding platform-specific OS calls that break cross-compilation unless properly feature-flagged.

### Rule 5: Error Handling & Exit Codes

- The CLI (`cli`) must adhere to strict UNIX exit code standards:
- `0`: Success (validation passed, formatting completed cleanly).
- `1`: Syntax or semantic validation error detected (prints structured diagnostics to `stderr`).
- `2`: Internal runtime or I/O error.

---

## 5. Execution Checklist for Agents

When implementing a feature across the ecosystem, follow this sequence:

1. **Spec Update:** Update ABNF rules or test vectors in `core/spec/`.
2. **Core Implementation:** Implement changes in `core/` and verify via `cargo test`.
3. **Tooling & CLI:** Update `cli/` or `lsp/` handlers if syntax changes impact commands or IDE diagnostics.
4. **Parser Sync:** Mirror syntax/AST changes in `parser-ts`, `parser-python`, and `parser-go`.
5. **Validation:** Run `just test-all` and `just integration-test` to guarantee 100% ecosystem alignment.
