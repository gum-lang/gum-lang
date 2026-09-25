# Plan: Automated Multi-Registry Publishing

## Goal

Create GitHub Actions workflows that trigger on git tags (e.g., `v0.2.0`) and automatically publish all packages to their respective registries with synchronized versioning.

---

## Registries & Packages

| Registry | Package | Source |
|----------|---------|--------|
| npm | `@gum-lang/parser` | `parser-ts/` |
| npm | `tree-sitter-gum` | `tree-sitter-lang/` |
| JSR | `@gum-lang/parser` | `parser-ts/` |
| PyPI | `gum-parser` | `parser-python/` |
| VSCode Marketplace | `gum-language` | `vscode-extension/` |
| OpenVSX | `gum-language` | `vscode-extension/` |
| crates.io | `gum-core` | `core/` |
| crates.io | `gum-cli` | `cli/` |
| crates.io | `gum-lsp` | `lsp/` |
| crates.io | `gum-nix` | `nix-generator/` |
| Go modules | `github.com/gum-lang/parser-go` | `parser-go/` (git tag) |

---

## Phase 1: JSR Compatibility — Add Exports to parser-ts

JSR requires explicit `exports` in `package.json`. Add:

```json
"exports": {
  ".": {
    "import": "./dist/index.js",
    "types": "./dist/index.d.ts"
  }
}
```

---

## Phase 2: Create `publish-npm.yml`

Triggers on `v*` tags. Two jobs:

### Job: `publish-parser`
- Build `parser-ts/` (npm install + tsc)
- Sync version from git tag (strip `v` prefix)
- Publish `@gum-lang/parser` to **npm** using `NPM_TOKEN`
- Publish `@gum-lang/parser` to **JSR** using `JSR_TOKEN`

### Job: `publish-tree-sitter`
- Build `tree-sitter-lang/` (tree-sitter generate + prebuildify)
- Sync version from git tag
- Publish `tree-sitter-gum` to **npm** using `NPM_TOKEN`

---

## Phase 3: Create `publish-pypi.yml`

Triggers on `v*` tags.

- Install Rust toolchain + Python matrix (3.8–3.12)
- Use `maturin action` to build wheels for Linux (manylinux), macOS, Windows
- Sync version from git tag in `pyproject.toml`
- Publish to **PyPI** using `PYPI_TOKEN`

---

## Phase 4: Create `publish-vscode.yml`

Triggers on `v*` tags.

- Install Node.js, compile TypeScript
- Sync version in `vscode-extension/package.json`
- Build LSP binaries for target platforms (or bundle as dependency)
- Package with `@vscode/vsce`
- Publish to **VSCode Marketplace** using `VSCE_TOKEN`
- Publish to **OpenVSX** using `OPENVSX_TOKEN`

---

## Phase 5: Create `publish-crates.yml`

Triggers on `v*` tags.

**Sequential publishing** (dependency order):
1. `gum-core` — no local deps, publish first
2. `gum-cli` — depends on `gum-core`
3. `gum-lsp` — depends on `gum-core`
4. `gum-nix` — depends on `gum-core`

Each step:
- Sync version in `Cargo.toml`
- `cargo publish --token $CARGO_REGISTRY_TOKEN`

---

## Phase 6: Go Module (No Workflow Needed)

Go modules are published via git tags. When `v*` tag is pushed, `go get github.com/gum-lang/parser-go@vX.Y.Z` works automatically. No workflow required — just ensure `go.mod` module path is correct (already is: `github.com/gum-lang/parser-go`).

---

## Phase 7: Update `release.yml`

- Add version sync step: strip `v` prefix from tag, update all `package.json`, `Cargo.toml`, `pyproject.toml` versions before building binaries
- This ensures binary releases match published package versions

---

## Required Secrets (to be configured in repo Settings → Secrets)

| Secret | Registry | Purpose |
|--------|----------|---------|
| `NPM_TOKEN` | npm | Publish `@gum-lang/parser`, `tree-sitter-gum` |
| `JSR_TOKEN` | JSR | Publish `@gum-lang/parser` |
| `PYPI_TOKEN` | PyPI | Publish `gum-parser` |
| `VSCE_TOKEN` | VSCode Marketplace | Publish `gum-language` extension |
| `OPENVSX_TOKEN` | OpenVSX | Publish `gum-language` extension |
| `CARGO_REGISTRY_TOKEN` | crates.io | Publish `gum-core`, `gum-cli`, `gum-lsp`, `gum-nix` |

---

## Execution Order

1. **Phase 1** — Add JSR exports to parser-ts/package.json
2. **Phase 2** — Create publish-npm.yml
3. **Phase 3** — Create publish-pypi.yml
4. **Phase 4** — Create publish-vscode.yml
5. **Phase 5** — Create publish-crates.yml
6. **Phase 7** — Update release.yml with version sync

---

## Version Sync Strategy

All workflows extract the version from the git tag:
```yaml
- name: Extract version
  run: echo "VERSION=${GITHUB_REF_NAME#v}" >> $GITHUB_ENV
```

Then use `jq` / `sed` / `toml-cli` to update respective config files before publishing. This ensures every registry receives the exact same version number derived from the single git tag.
