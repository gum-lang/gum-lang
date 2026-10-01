# Website Content Design

**Date:** 2026-10-01
**Status:** Approved
**Approach:** Infrastructure-first, then content

---

## Overview

Fill out the Gum language documentation website by setting up Starlight for the docs section, integrating the real WASM parser into the playground, polishing the landing page, and writing content for all 12 documentation pages. Target audience: developers evaluating Gum.

---

## 1. Infrastructure & Starlight Setup

### Astro + Starlight Configuration

- Configure `astro.config.mjs` with the Starlight plugin
- Set `site: 'https://gum-lang.github.io'`, `base: '/gum-lang'`, `title: 'Gum'`
- Starlight manages the `/docs/` section via its content collection (MDX files with frontmatter)
- Landing page (`/`) and playground (`/playground/`) remain as custom Astro pages outside Starlight
- Shared navigation between Starlight docs and custom pages via Starlight's `components` override or a shared layout

### Content Structure

```
src/
├── content/
│   └── docs/
│       ├── index.mdx          (Introduction - migrate from existing)
│       ├── installation.mdx
│       ├── quick-start.mdx
│       ├── syntax.mdx
│       ├── types.mdx
│       ├── strings.mdx
│       ├── tables-lists.mdx
│       ├── cli.mdx
│       ├── lsp.mdx
│       ├── editors.mdx
│       ├── parsers.mdx
│       └── wasm.mdx
├── pages/
│   ├── index.astro            (Landing - keep custom)
│   └── playground/
│       └── index.astro        (Playground - keep custom)
├── components/                 (Shared components if needed)
└── styles/                     (Custom styles for landing/playground only)
```

### Starlight Sidebar

Configured in `astro.config.mjs` using Starlight's `sidebar` option with 4 groups:

- **Getting Started:** Introduction, Installation, Quick Start
- **Language Reference:** Syntax, Types, Strings, Tables & Lists
- **Tooling:** CLI, LSP, Editor Support
- **Ecosystem:** Parsers, WASM

### Styles

Starlight provides its own styling for docs. Custom CSS only needed for landing page and playground.

---

## 2. Landing Page Polish

Keep the current minimal structure (hero + 3 feature cards) and add:

- **Footer** with links to GitHub, docs, and a copyright line
- **Consistent nav** that matches the Starlight docs nav (seamless switching between landing/docs/playground)
- **Minor visual polish** — proper spacing, typography, responsive layout via a simple CSS file

No new sections or content blocks. The landing page stays a quick overview that funnels visitors to docs or playground.

---

## 3. Playground WASM Integration

Replace the JavaScript demo parser with the real WASM build from the `core` Rust crate:

- **WASM loading:** Import the WASM module from `public/pkg/` (built via `just build-wasm`)
- **Real-time parsing:** On editor input, call the WASM parser to generate the AST and display it
- **Tabs:**
  - **AST** — Raw AST structure from the core parser
  - **JSON** — JSON serialization of the parsed document
  - **Errors** — Validation errors/diagnostics from the core parser
- **Format button:** Use the core formatter to format the editor content
- **Status indicator:** Show valid/invalid status based on parser diagnostics
- **Fallback:** If WASM isn't built yet (no `public/pkg/`), show a message prompting the user to run `just build-wasm`

---

## 4. Documentation Content (12 Pages)

All pages written as MDX with Starlight frontmatter. Concise, example-driven, with comparisons to TOML/JSON where helpful.

### Getting Started

#### 4.1 Introduction (`index.mdx`)

Migrate existing content. What Gum is, why it exists, key features, a small example. Link to installation.

#### 4.2 Installation

- CLI: pre-built binaries via GitHub Releases, `cargo install`, package managers
- Parser libraries: `npm install @gum-lang/parser`, `pip install gum-lang`, `go get`, `cargo add`
- VSCode extension: marketplace link
- Neovim: plugin managers

#### 4.3 Quick Start

5-minute walkthrough: write a `.gum` file, validate with CLI, format it, convert to JSON. Show the feedback loop.

### Language Reference

#### 4.4 Syntax

Overview of the grammar: assignments, keys (bare, quoted, dotted), comments, whitespace rules, separators (commas/newlines). Side-by-side with TOML equivalents.

#### 4.5 Types

The value type system: strings, numbers (int/float), booleans, null, tables, lists. Brief description of each with examples.

#### 4.6 Strings

Deep dive into string variants:

- Basic quoted (`"..."`)
- Literal (`'...'`)
- Multiline quoted (`"""..."""`)
- Multiline literal (`'''...'''`)
- Escape sequences, unicode, line continuation
- Dedent behavior for multiline strings

#### 4.7 Tables & Lists

- Inline tables (`{ ... }`), inline lists (`[ ... ]`)
- Nesting, mixing
- Separator rules (commas vs newlines)
- Comparison to TOML's table/array-of-tables syntax

### Tooling

#### 4.8 CLI

Three subcommands: `validate`, `format`, `convert`. Exit codes, flags, usage examples.

#### 4.9 LSP

What the LSP provides: real-time diagnostics, formatting on save. How to connect (mostly via editor extensions, raw config for advanced users).

#### 4.10 Editor Support

- VSCode extension setup
- Neovim setup (nvim-lspconfig + nvim-treesitter)
- Tree-sitter grammar for syntax highlighting in other editors

### Ecosystem

#### 4.11 Parsers

Multi-language parser ecosystem overview. Installation and basic usage for each: `parser-ts`, `parser-python`, `parser-go`, `parser-rust`. Cross-language AST parity guarantee.

#### 4.12 WASM

How to use the WASM build in browser or Node.js. The playground as a reference implementation. API surface.
