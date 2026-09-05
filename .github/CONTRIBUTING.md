# Contributing to Gum Language

Thank you for your interest in contributing to the Gum markup language ecosystem.

## Development Setup

1. Clone the workspace with all submodules:
   ```bash
   git clone --recurse-submodules <repo-url>
   cd gum-lang
   ```

2. Ensure you have the following tools installed:
   - **Rust** (Edition 2021+) — `rustup` recommended
   - **Node.js** (LTS) — for TypeScript parser and website
   - **Python 3.10+** — for Python parser
   - **Go 1.21+** — for Go parser
   - **just** — task runner ([installation](https://github.com/casey/just#installation))
   - **Nix** (optional) — reproducible dev environments via `flake.nix`

3. Run the full test suite to verify your setup:
   ```bash
   just test-all
   ```

## Workflow

### Making Changes

1. Create a feature branch from `main`.
2. Make your changes in the relevant submodule(s).
3. Follow the coding standards of each language (see below).
4. Add or update tests as appropriate.
5. Run `just test-all` to ensure no regressions.
6. Run `just format-all` and `just lint-all` before committing.

### Cross-Language Parity

All parser implementations **must** produce identical ASTs for the same input. When modifying syntax:

1. Update the ABNF specification and test vectors in `core/spec/` first.
2. Implement changes in `core/` and verify with `cargo test`.
3. Mirror changes in `parser-ts`, `parser-python`, `parser-go`, and `parser-rust`.
4. Run `just integration-test` to validate cross-language compliance.

### Coding Standards

| Language | Formatter | Linter |
|---|---|---|
| Rust | `cargo fmt` | `cargo clippy -- -D warnings` |
| TypeScript | Prettier (`npm run format`) | ESLint (`npm run lint`) |
| Python | `black` | `ruff` |
| Go | `go fmt` | `go vet` |

### Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(core): add support for nested block quotes
fix(cli): exit with code 1 on validation errors
docs(website): update playground usage guide
```

### Pull Requests

1. Ensure all CI checks pass.
2. Keep PRs focused — one feature or fix per PR.
3. Reference related issues in the PR description.
4. Request review from at least one maintainer.

## Reporting Issues

- **Bug reports**: Include the input that triggers the bug, expected vs actual output, and your environment (OS, tool versions).
- **Feature requests**: Describe the use case and any relevant ABNF grammar changes.
- **Cross-parser discrepancies**: Include the input and the differing AST outputs from each parser.

## License

By contributing, you agree that your contributions will be licensed under the project's license.
