# Website Content Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Set up Starlight for documentation, integrate WASM parser into playground, polish landing page, and write content for all 12 documentation pages.

**Architecture:** Configure Astro + Starlight for the `/docs/` section using MDX content collections. Keep landing and playground as custom Astro pages. Integrate real WASM build from core Rust crate into playground. Write all documentation content as MDX files with Starlight frontmatter.

**Tech Stack:** Astro 4.x, @astrojs/starlight 0.29.x, MDX, WebAssembly (wasm-pack), TypeScript

## Global Constraints

- Starlight version: 0.29.x (already installed)
- Astro version: 4.x (already installed)
- Site URL: `https://gum-lang.github.io`
- Base path: `/gum-lang`
- WASM build command: `just build-wasm` (outputs to `website/public/pkg/`)
- Target audience: developers evaluating Gum (concise, example-driven, TOML/JSON comparisons)
- Documentation pages: 12 total (Introduction, Installation, Quick Start, Syntax, Types, Strings, Tables & Lists, CLI, LSP, Editor Support, Parsers, WASM)

---

## File Structure

```
website/
├── astro.config.mjs                    (Modify: add Starlight plugin)
├── src/
│   ├── content/
│   │   └── docs/
│   │       ├── index.mdx               (Create: Introduction)
│   │       ├── installation.mdx        (Create)
│   │       ├── quick-start.mdx         (Create)
│   │       ├── syntax.mdx              (Create)
│   │       ├── types.mdx               (Create)
│   │       ├── strings.mdx             (Create)
│   │       ├── tables-lists.mdx        (Create)
│   │       ├── cli.mdx                 (Create)
│   │       ├── lsp.mdx                 (Create)
│   │       ├── editors.mdx             (Create)
│   │       ├── parsers.mdx             (Create)
│   │       └── wasm.mdx                (Create)
│   ├── pages/
│   │   ├── index.astro                 (Modify: add footer, polish)
│   │   └── playground/
│   │       └── index.astro             (Modify: WASM integration)
│   └── styles/
│       ├── main.css                    (Create: landing page styles)
│       └── playground.css              (Create: playground styles)
└── public/
    └── pkg/                            (Generated: WASM build output)
```

---

### Task 1: Configure Starlight

**Files:**
- Modify: `website/astro.config.mjs`

**Interfaces:**
- Consumes: Starlight plugin from `@astrojs/starlight`
- Produces: Configured Astro project with Starlight integration

- [ ] **Step 1: Update astro.config.mjs with Starlight plugin**

```javascript
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  site: 'https://gum-lang.github.io',
  base: '/gum-lang',
  integrations: [
    starlight({
      title: 'Gum',
      description: 'A simple, human-readable markup language',
      sidebar: [
        {
          label: 'Getting Started',
          items: [
            { label: 'Introduction', link: '/docs/' },
            { label: 'Installation', link: '/docs/installation/' },
            { label: 'Quick Start', link: '/docs/quick-start/' },
          ],
        },
        {
          label: 'Language Reference',
          items: [
            { label: 'Syntax', link: '/docs/syntax/' },
            { label: 'Types', link: '/docs/types/' },
            { label: 'Strings', link: '/docs/strings/' },
            { label: 'Tables & Lists', link: '/docs/tables-lists/' },
          ],
        },
        {
          label: 'Tooling',
          items: [
            { label: 'CLI', link: '/docs/cli/' },
            { label: 'LSP', link: '/docs/lsp/' },
            { label: 'Editor Support', link: '/docs/editors/' },
          ],
        },
        {
          label: 'Ecosystem',
          items: [
            { label: 'Parsers', link: '/docs/parsers/' },
            { label: 'WASM', link: '/docs/wasm/' },
          ],
        },
      ],
    }),
  ],
});
```

- [ ] **Step 2: Create content directory structure**

```bash
mkdir -p website/src/content/docs
```

- [ ] **Step 3: Verify Starlight builds without errors**

```bash
cd website && npm run build
```

Expected: Build succeeds (will show warnings about missing content files, which we'll fix in next tasks)

- [ ] **Step 4: Commit**

```bash
git add website/astro.config.mjs
git commit -m "feat: configure Starlight for documentation"
```

---

### Task 2: Create Introduction Page

**Files:**
- Create: `website/src/content/docs/index.mdx`

**Interfaces:**
- Consumes: Starlight frontmatter schema
- Produces: Introduction documentation page

- [ ] **Step 1: Create introduction MDX file**

```markdown
---
title: Introduction
description: Learn about Gum, a simple human-readable markup language
---

Gum is a simple, human-readable markup language for configuration and data serialization. Inspired by TOML and JSON, it combines the readability of TOML with the flexibility of JSON.

## Why Gum?

- **Simple syntax** — Easy to read and write by hand
- **Strong typing** — Strings, numbers, booleans, null, tables, and lists
- **Cross-language** — Parsers in Rust, TypeScript, Python, and Go
- **Full tooling** — CLI, LSP, VSCode extension, Neovim support

## Example

```gum
# A Gum document
name = "My Project"
version = 1
enabled = true

[server]
host = "localhost"
port = 8080

items = [1, 2, 3]
```

## Next Steps

- [Installation guide](/docs/installation/)
- [Language syntax reference](/docs/syntax/)
- [Try it in the playground](/playground/)
```

- [ ] **Step 2: Remove old docs page**

```bash
rm website/src/pages/docs/index.astro
```

- [ ] **Step 3: Verify the page renders**

```bash
cd website && npm run dev
```

Visit `http://localhost:4321/gum-lang/docs/` and verify the introduction page displays with Starlight styling and sidebar navigation.

- [ ] **Step 4: Commit**

```bash
git add website/src/content/docs/index.mdx
git rm website/src/pages/docs/index.astro
git commit -m "docs: add introduction page"
```

---

### Task 3: Create Installation Page

**Files:**
- Create: `website/src/content/docs/installation.mdx`

**Interfaces:**
- Consumes: Starlight frontmatter schema
- Produces: Installation documentation page

- [ ] **Step 1: Create installation MDX file**

```markdown
---
title: Installation
description: How to install Gum CLI, parser libraries, and editor extensions
---

## CLI

### Pre-built Binaries

Download pre-built binaries from [GitHub Releases](https://github.com/gum-lang/gum-lang/releases):

- Linux (x86_64, aarch64)
- macOS (Intel, Apple Silicon)
- Windows (x86_64)

### Cargo

```bash
cargo install gum-cli
```

### Verify Installation

```bash
gum --version
```

## Parser Libraries

### TypeScript

```bash
npm install @gum-lang/parser
```

### Python

```bash
pip install gum-lang
```

### Go

```bash
go get github.com/gum-lang/parser-go
```

### Rust

```bash
cargo add gum-core
```

## Editor Extensions

### VSCode

Install the [Gum extension](https://marketplace.visualstudio.com/items?itemName=gum-lang.gum) from the VSCode Marketplace.

### Neovim

Use your preferred plugin manager:

```lua
-- lazy.nvim
{
  "gum-lang/vscode-extension",
  ft = "gum",
}
```

Configure LSP:

```lua
require('lspconfig').gum.setup{}
```

## Next Steps

- [Quick Start guide](/docs/quick-start/)
```

- [ ] **Step 2: Verify the page renders**

```bash
cd website && npm run dev
```

Visit `http://localhost:4321/gum-lang/docs/installation/` and verify the page displays correctly.

- [ ] **Step 3: Commit**

```bash
git add website/src/content/docs/installation.mdx
git commit -m "docs: add installation page"
```

---

### Task 4: Create Quick Start Page

**Files:**
- Create: `website/src/content/docs/quick-start.mdx`

**Interfaces:**
- Consumes: Starlight frontmatter schema
- Produces: Quick Start documentation page

- [ ] **Step 1: Create quick-start MDX file**

```markdown
---
title: Quick Start
description: Get started with Gum in 5 minutes
---

This guide walks you through creating your first Gum document and using the CLI tools.

## Step 1: Create a Gum File

Create a file named `config.gum`:

```gum
# Application configuration
name = "My App"
version = 1
debug = false

[server]
host = "localhost"
port = 8080

[database]
url = "postgres://localhost/mydb"
pool_size = 10

features = ["auth", "logging", "metrics"]
```

## Step 2: Validate

Check your document for syntax errors:

```bash
gum validate config.gum
```

If valid, you'll see no output and exit code 0. If there are errors, they'll be printed to stderr.

## Step 3: Format

Normalize formatting:

```bash
gum format config.gum
```

This prints the formatted document to stdout. To format in-place:

```bash
gum format config.gum --in-place
```

## Step 4: Convert to JSON

Convert your Gum document to JSON:

```bash
gum convert config.gum --to json
```

Output:

```json
{
  "name": "My App",
  "version": 1,
  "debug": false,
  "server": {
    "host": "localhost",
    "port": 8080
  },
  "database": {
    "url": "postgres://localhost/mydb",
    "pool_size": 10
  },
  "features": ["auth", "logging", "metrics"]
}
```

## Step 5: Try the Playground

Visit the [interactive playground](/playground/) to experiment with Gum syntax in your browser.

## Next Steps

- [Syntax reference](/docs/syntax/) — Learn the full grammar
- [Types](/docs/types/) — Explore the type system
```

- [ ] **Step 2: Verify the page renders**

```bash
cd website && npm run dev
```

Visit `http://localhost:4321/gum-lang/docs/quick-start/` and verify the page displays correctly.

- [ ] **Step 3: Commit**

```bash
git add website/src/content/docs/quick-start.mdx
git commit -m "docs: add quick start page"
```

---

### Task 5: Create Syntax Page

**Files:**
- Create: `website/src/content/docs/syntax.mdx`

**Interfaces:**
- Consumes: Starlight frontmatter schema, ABNF grammar from LANGUAGE.abnf
- Produces: Syntax reference documentation page

- [ ] **Step 1: Create syntax MDX file**

```markdown
---
title: Syntax
description: Gum language syntax reference
---

Gum's syntax is inspired by TOML, with a focus on readability and simplicity.

## Documents

A Gum document consists of assignments and comments:

```gum
# This is a comment
name = "Gum"
version = 1
```

## Assignments

An assignment maps a key to a value:

```gum
key = value
```

### Keys

Keys can be **bare**, **quoted**, or **dotted**:

```gum
# Bare key
name = "Gum"

# Quoted key (allows special characters)
"my-key" = "value"

# Dotted key (creates nested structure)
server.host = "localhost"
```

Dotted keys are equivalent to nested tables:

```gum
# These are equivalent:
server.host = "localhost"

[server]
host = "localhost"
```

### Values

Values can be strings, numbers, booleans, null, tables, or lists:

```gum
string = "hello"
number = 42
float = 3.14
boolean = true
null_value = null
table = { key = "value" }
list = [1, 2, 3]
```

## Comments

Comments start with `#` and continue to the end of the line:

```gum
# This is a comment
name = "Gum"  # Inline comment
```

## Whitespace and Separators

Assignments can be separated by newlines or commas:

```gum
# Newline-separated
name = "Gum"
version = 1

# Comma-separated (inline)
name = "Gum", version = 1
```

Whitespace around `=` is optional but recommended for readability.

## Comparison to TOML

Gum syntax is similar to TOML with a few differences:

| Feature | TOML | Gum |
|---------|------|-----|
| Table headers | `[table]` | `[table]` or `table.key = value` |
| Inline tables | `{ key = "value" }` | `{ key = "value" }` |
| Arrays | `[1, 2, 3]` | `[1, 2, 3]` |
| Separators | Newlines required | Newlines or commas |
| Multiline strings | `"""..."""` | `"""..."""` (with dedent) |

## Next Steps

- [Types](/docs/types/) — Deep dive into the type system
- [Strings](/docs/strings/) — String variants and escape sequences
```

- [ ] **Step 2: Verify the page renders**

```bash
cd website && npm run dev
```

Visit `http://localhost:4321/gum-lang/docs/syntax/` and verify the page displays correctly.

- [ ] **Step 3: Commit**

```bash
git add website/src/content/docs/syntax.mdx
git commit -m "docs: add syntax reference page"
```

---

### Task 6: Create Types Page

**Files:**
- Create: `website/src/content/docs/types.mdx`

**Interfaces:**
- Consumes: Starlight frontmatter schema
- Produces: Types documentation page

- [ ] **Step 1: Create types MDX file**

```markdown
---
title: Types
description: Gum type system reference
---

Gum has a strong type system with six value types: strings, numbers, booleans, null, tables, and lists.

## Strings

Strings represent text data:

```gum
basic = "hello"
literal = 'hello'
multiline = """
  hello
  world
"""
```

See [Strings](/docs/strings/) for details on string variants and escape sequences.

## Numbers

Numbers can be integers or floats:

```gum
integer = 42
negative = -17
positive = +100
float = 3.14
scientific = 1.23e-4
```

## Booleans

Booleans are `true` or `false`:

```gum
enabled = true
disabled = false
```

## Null

Null represents the absence of a value:

```gum
optional = null
```

## Tables

Tables are key-value collections:

```gum
# Inline table
server = { host = "localhost", port = 8080 }

# Block table
[server]
host = "localhost"
port = 8080
```

See [Tables & Lists](/docs/tables-lists/) for more details.

## Lists

Lists are ordered sequences of values:

```gum
# Inline list
items = [1, 2, 3]

# Multiline list
items = [
  "apple",
  "banana",
  "cherry"
]
```

Lists can contain mixed types:

```gum
mixed = [1, "two", true, null]
```

See [Tables & Lists](/docs/tables-lists/) for more details.

## Type Inference

Gum infers types from values:

```gum
name = "Gum"      # String
version = 1       # Integer
pi = 3.14         # Float
enabled = true    # Boolean
nothing = null    # Null
```

## Next Steps

- [Strings](/docs/strings/) — String variants and escape sequences
- [Tables & Lists](/docs/tables-lists/) — Complex data structures
```

- [ ] **Step 2: Verify the page renders**

```bash
cd website && npm run dev
```

Visit `http://localhost:4321/gum-lang/docs/types/` and verify the page displays correctly.

- [ ] **Step 3: Commit**

```bash
git add website/src/content/docs/types.mdx
git commit -m "docs: add types page"
```

---

### Task 7: Create Strings Page

**Files:**
- Create: `website/src/content/docs/strings.mdx`

**Interfaces:**
- Consumes: Starlight frontmatter schema
- Produces: Strings documentation page

- [ ] **Step 1: Create strings MDX file**

```markdown
---
title: Strings
description: String variants, escape sequences, and multiline strings
---

Gum supports four string variants, each with different escape behavior.

## Basic Strings

Basic strings are enclosed in double quotes:

```gum
name = "Hello, World!"
```

### Escape Sequences

Basic strings support escape sequences:

| Escape | Meaning |
|--------|---------|
| `\"` | Double quote |
| `\\` | Backslash |
| `\b` | Backspace |
| `\f` | Form feed |
| `\n` | Newline |
| `\r` | Carriage return |
| `\t` | Tab |
| `\uXXXX` | Unicode (4 hex digits) |
| `\UXXXXXXXX` | Unicode (8 hex digits) |

Example:

```gum
newline = "Line 1\nLine 2"
unicode = "Smile: \u{1F600}"
```

## Literal Strings

Literal strings are enclosed in single quotes and do not support escape sequences:

```gum
path = 'C:\Users\name\file.txt'
regex = '\d+\.\d+'
```

Literal strings are useful for values that contain backslashes or other characters that would need escaping.

## Multiline Strings

### Multiline Basic Strings

Multiline basic strings are enclosed in triple double quotes:

```gum
description = """
This is a
multiline string.
"""
```

**Features:**

- Supports escape sequences
- Automatically dedents common leading whitespace (like Python's `textwrap.dedent()`)
- Line continuations with `\` at end of line

Example with dedent:

```gum
# All three lines are dedented to the same level
message = """
  Hello
  World
"""
# Result: "Hello\nWorld"
```

Line continuation:

```gum
long = """
  This is a \
  single line \
  string.
"""
# Result: "This is a single line string."
```

### Multiline Literal Strings

Multiline literal strings are enclosed in triple single quotes:

```gum
code = '''
def hello():
    print("Hello")
'''
```

**Features:**

- No escape sequences (literal)
- Automatically dedents common leading whitespace
- Line continuations with `\` at end of line

## String Comparison

| Variant | Quotes | Escapes | Multiline | Use Case |
|---------|--------|---------|-----------|----------|
| Basic | `"..."` | Yes | No | General text with escapes |
| Literal | `'...'` | No | No | Paths, regex, Windows paths |
| Multiline Basic | `"""..."""` | Yes | Yes | Long text, documentation |
| Multiline Literal | `'''...'''` | No | Yes | Code snippets, templates |

## Next Steps

- [Tables & Lists](/docs/tables-lists/) — Complex data structures
```

- [ ] **Step 2: Verify the page renders**

```bash
cd website && npm run dev
```

Visit `http://localhost:4321/gum-lang/docs/strings/` and verify the page displays correctly.

- [ ] **Step 3: Commit**

```bash
git add website/src/content/docs/strings.mdx
git commit -m "docs: add strings page"
```

---

### Task 8: Create Tables & Lists Page

**Files:**
- Create: `website/src/content/docs/tables-lists.mdx`

**Interfaces:**
- Consumes: Starlight frontmatter schema
- Produces: Tables & Lists documentation page

- [ ] **Step 1: Create tables-lists MDX file**

```markdown
---
title: Tables & Lists
description: Working with tables and lists in Gum
---

Tables and lists are Gum's composite data structures.

## Tables

Tables are key-value collections. They can be defined inline or as blocks.

### Inline Tables

Inline tables are defined on a single line:

```gum
point = { x = 1, y = 2 }
server = { host = "localhost", port = 8080 }
```

### Block Tables

Block tables use the `[table]` syntax:

```gum
[server]
host = "localhost"
port = 8080

[database]
url = "postgres://localhost/mydb"
pool_size = 10
```

### Nested Tables

Tables can be nested using dotted keys:

```gum
[server]
host = "localhost"

[server.tls]
enabled = true
cert = "/path/to/cert"
```

Or inline:

```gum
server = {
  host = "localhost"
  tls = {
    enabled = true
    cert = "/path/to/cert"
  }
}
```

### Comparison to TOML

Gum tables are similar to TOML, but with more flexibility:

```gum
# Gum: Both are valid
server.host = "localhost"

[server]
host = "localhost"

# TOML: Only the second form is valid
[server]
host = "localhost"
```

## Lists

Lists are ordered sequences of values.

### Inline Lists

```gum
numbers = [1, 2, 3]
names = ["Alice", "Bob", "Charlie"]
mixed = [1, "two", true, null]
```

### Multiline Lists

```gum
items = [
  "apple",
  "banana",
  "cherry",
]
```

Trailing commas are allowed.

### Nested Lists

```gum
matrix = [
  [1, 2, 3],
  [4, 5, 6],
  [7, 8, 9]
]
```

### Lists of Tables

```gum
servers = [
  { host = "server1", port = 8080 },
  { host = "server2", port = 8081 }
]
```

Or with block syntax:

```gum
[[servers]]
host = "server1"
port = 8080

[[servers]]
host = "server2"
port = 8081
```

## Separators

Tables and lists use commas or newlines to separate elements:

```gum
# Comma-separated
point = { x = 1, y = 2 }

# Newline-separated
point = {
  x = 1
  y = 2
}

# Mixed (trailing comma allowed)
items = [
  1,
  2,
  3,
]
```

## Next Steps

- [CLI](/docs/cli/) — Command-line tools
```

---

### Task 9: Create CLI Page

**Files:**
- Create: `website/src/content/docs/cli.mdx`

- [ ] **Step 1: Create CLI MDX file**

```markdown
---
title: CLI
description: Gum command-line interface reference
---

The Gum CLI provides commands for validating, formatting, and converting Gum documents.

## Installation

See [Installation](/docs/installation/) for how to install the CLI.

## Commands

### validate

Check a document for syntax and semantic errors:

```bash
gum validate config.gum
```

**Exit codes:**
- `0` — Document is valid
- `1` — Syntax or validation error (details printed to stderr)
- `2` — Internal error (file not found, I/O error)

### format

Format a document to standard style:

```bash
# Print to stdout
gum format config.gum

# Format in-place
gum format config.gum --in-place
```

### convert

Convert a Gum document to another format:

```bash
# Convert to JSON
gum convert config.gum --to json

# Convert to HTML
gum convert config.gum --to html
```

## Flags

| Flag | Description |
|------|-------------|
| `--help` | Show help information |
| `--version` | Show version |
| `--in-place` | Modify file in-place (format only) |
| `--to <format>` | Output format (convert only) |

## Next Steps

- [LSP](/docs/lsp/) — Language server for editor integration
```

- [ ] **Step 2: Verify the page renders**

```bash
cd website && npm run dev
```

Visit `http://localhost:4321/gum-lang/docs/cli/` and verify the page displays correctly.

- [ ] **Step 3: Commit**

```bash
git add website/src/content/docs/cli.mdx
git commit -m "docs: add CLI page"
```

---

### Task 10: Create LSP Page

**Files:**
- Create: `website/src/content/docs/lsp.mdx`

- [ ] **Step 1: Create LSP MDX file**

```markdown
---
title: LSP
description: Gum Language Server Protocol implementation
---

The Gum LSP provides editor integration with real-time diagnostics and formatting.

## Features

- **Real-time diagnostics** — Syntax and validation errors as you type
- **Formatting on save** — Automatically format documents when saving
- **Hover information** — Type information and documentation on hover

## Installation

The LSP binary is included with the CLI installation. See [Installation](/docs/installation/).

## Connection

Most users connect to the LSP through editor extensions:

- **VSCode** — The [Gum extension](/docs/editors/#vscode) bundles and manages the LSP automatically
- **Neovim** — Configure via [nvim-lspconfig](/docs/editors/#neovim)

### Manual Connection

For advanced setups, the LSP binary can be run directly:

```bash
gum-lsp
```

The server communicates over stdin/stdout using the Language Server Protocol.

## Next Steps

- [Editor Support](/docs/editors/) — Setup guides for specific editors
```

- [ ] **Step 2: Verify the page renders**

```bash
cd website && npm run dev
```

Visit `http://localhost:4321/gum-lang/docs/lsp/` and verify the page displays correctly.

- [ ] **Step 3: Commit**

```bash
git add website/src/content/docs/lsp.mdx
git commit -m "docs: add LSP page"
```

---

### Task 11: Create Editor Support Page

**Files:**
- Create: `website/src/content/docs/editors.mdx`

- [ ] **Step 1: Create editors MDX file**

```markdown
---
title: Editor Support
description: Setup guides for VSCode, Neovim, and other editors
---

## VSCode

### Installation

Install the [Gum extension](https://marketplace.visualstudio.com/items?itemName=gum-lang.gum) from the VSCode Marketplace.

### Features

- Syntax highlighting
- Real-time error checking (via LSP)
- Format on save
- Code completion

### Configuration

The extension works out of the box. Optional settings in `settings.json`:

```json
{
  "gum.formatOnSave": true,
  "gum.lsp.path": "/path/to/gum-lsp"
}
```

## Neovim

### Requirements

- Neovim 0.8+
- [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig)
- [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter) (optional, for syntax highlighting)

### LSP Setup

Using lazy.nvim:

```lua
{
  "neovim/nvim-lspconfig",
  config = function()
    require("lspconfig").gum.setup({})
  end,
}
```

### Tree-sitter Setup

```lua
{
  "nvim-treesitter/nvim-treesitter",
  build = ":TSUpdate",
  config = function()
    require("nvim-treesitter.configs").setup({
      ensure_installed = { "gum" },
      highlight = { enable = true },
    })
  end,
}
```

## Other Editors

The Tree-sitter grammar can be used with any editor that supports Tree-sitter grammars (Emacs, Helix, etc.). See the [tree-sitter-lang](https://github.com/gum-lang/tree-sitter-lang) repository for details.

## Next Steps

- [Parsers](/docs/parsers/) — Language-specific parser libraries
```

- [ ] **Step 2: Verify the page renders**

```bash
cd website && npm run dev
```

Visit `http://localhost:4321/gum-lang/docs/editors/` and verify the page displays correctly.

- [ ] **Step 3: Commit**

```bash
git add website/src/content/docs/editors.mdx
git commit -m "docs: add editor support page"
```

---

### Task 12: Create Parsers Page

**Files:**
- Create: `website/src/content/docs/parsers.mdx`

- [ ] **Step 1: Create parsers MDX file**

```markdown
---
title: Parsers
description: Multi-language parser libraries for Gum
---

Gum provides parser libraries for multiple languages, all producing identical ASTs for the same input.

## AST Parity Guarantee

All parsers produce the same Abstract Syntax Tree for any given input. This is validated continuously against the shared test suite from the core Rust implementation.

## TypeScript

```bash
npm install @gum-lang/parser
```

```typescript
import { parse } from '@gum-lang/parser';

const doc = parse('name = "Gum"');
console.log(doc.elements);
```

## Python

```bash
pip install gum-lang
```

```python
import gum

doc = gum.parse('name = "Gum"')
print(doc.elements)
```

## Go

```bash
go get github.com/gum-lang/parser-go
```

```go
package main

import "github.com/gum-lang/parser-go"

func main() {
    doc, err := gum.Parse(`name = "Gum"`)
    if err != nil {
        panic(err)
    }
    fmt.Println(doc.Elements)
}
```

## Rust

```bash
cargo add gum-core
```

```rust
use gum_core::parse;

fn main() {
    let doc = parse(r#"name = "Gum""#).unwrap();
    println!("{:?}", doc.elements());
}
```

## Next Steps

- [WASM](/docs/wasm/) — Browser and Node.js usage
```

- [ ] **Step 2: Verify the page renders**

```bash
cd website && npm run dev
```

Visit `http://localhost:4321/gum-lang/docs/parsers/` and verify the page displays correctly.

- [ ] **Step 3: Commit**

```bash
git add website/src/content/docs/parsers.mdx
git commit -m "docs: add parsers page"
```

---

### Task 13: Create WASM Page

**Files:**
- Create: `website/src/content/docs/wasm.mdx`

- [ ] **Step 1: Create WASM MDX file**

```markdown
---
title: WASM
description: Using the Gum parser in browser and Node.js via WebAssembly
---

The Gum core engine compiles to WebAssembly for use in browsers and Node.js.

## Building

Build the WASM module from the workspace root:

```bash
just build-wasm
```

This outputs the WASM module to `website/public/pkg/`.

## Browser Usage

```html
<script type="module">
  import init, { parse_to_ast, format_document, validate_document } from './pkg/gum_core.js';

  async function main() {
    await init();

    const ast = parse_to_ast('name = "Gum"');
    console.log(ast);

    const formatted = format_document('name="Gum"');
    console.log(formatted);

    const errors = validate_document('name = "Gum"');
    console.log(errors);
  }

  main();
</script>
```

## API

### `parse_to_ast(input: string) → object`

Parse a Gum document and return the AST as a JavaScript object.

### `format_document(input: string) → string`

Parse and format a Gum document, returning the normalized string.

### `validate_document(input: string) → array`

Parse and validate a Gum document, returning an array of errors (empty if valid).

### `is_valid(input: string) → boolean`

Check if a Gum document is valid.

## Playground

The [interactive playground](/playground/) uses the WASM build to parse Gum documents in real-time in the browser.

## Next Steps

- [Try the playground](/playground/)
```

- [ ] **Step 2: Verify the page renders**

```bash
cd website && npm run dev
```

Visit `http://localhost:4321/gum-lang/docs/wasm/` and verify the page displays correctly.

- [ ] **Step 3: Commit**

```bash
git add website/src/content/docs/wasm.mdx
git commit -m "docs: add WASM page"
```

---

### Task 14: Polish Landing Page

**Files:**
- Modify: `website/src/pages/index.astro`
- Create: `website/src/styles/main.css`

**Interfaces:**
- Consumes: Existing landing page structure
- Produces: Polished landing page with footer and consistent nav

- [ ] **Step 1: Create main.css with landing page styles**

```css
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
  line-height: 1.6;
  color: #333;
}

header {
  background: #fff;
  border-bottom: 1px solid #eaeaea;
  padding: 1rem 2rem;
}

nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
  max-width: 1200px;
  margin: 0 auto;
}

.logo {
  font-size: 1.5rem;
  font-weight: bold;
  text-decoration: none;
  color: #333;
}

.nav-links {
  display: flex;
  gap: 2rem;
}

.nav-links a {
  text-decoration: none;
  color: #666;
  transition: color 0.2s;
}

.nav-links a:hover {
  color: #333;
}

main {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
}

.hero {
  text-align: center;
  padding: 4rem 0;
}

.hero h1 {
  font-size: 3rem;
  margin-bottom: 1rem;
}

.hero p {
  font-size: 1.25rem;
  color: #666;
  margin-bottom: 2rem;
}

.hero-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
}

.btn {
  padding: 0.75rem 1.5rem;
  border-radius: 6px;
  text-decoration: none;
  font-weight: 500;
  transition: all 0.2s;
}

.btn-primary {
  background: #0070f3;
  color: #fff;
}

.btn-primary:hover {
  background: #0051cc;
}

.btn-secondary {
  background: #fff;
  color: #333;
  border: 1px solid #eaeaea;
}

.btn-secondary:hover {
  background: #f5f5f5;
}

.features {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 2rem;
  margin-top: 4rem;
}

.feature {
  padding: 2rem;
  background: #f9f9f9;
  border-radius: 8px;
}

.feature h3 {
  margin-bottom: 0.5rem;
}

footer {
  margin-top: 4rem;
  padding: 2rem;
  background: #f9f9f9;
  text-align: center;
  color: #666;
}

footer a {
  color: #666;
  text-decoration: none;
  margin: 0 1rem;
}

footer a:hover {
  color: #333;
}
```

- [ ] **Step 2: Update index.astro to add footer**

Add footer before closing `</body>` tag:

```html
<footer>
  <p>
    <a href="https://github.com/gum-lang/gum-lang">GitHub</a>
    <a href="/docs/">Documentation</a>
    <a href="/playground/">Playground</a>
  </p>
  <p>&copy; 2026 Gum Language Project</p>
</footer>
```

- [ ] **Step 3: Verify the landing page renders**

```bash
cd website && npm run dev
```

Visit `http://localhost:4321/gum-lang/` and verify the landing page displays with proper styling and footer.

- [ ] **Step 4: Commit**

```bash
git add website/src/pages/index.astro website/src/styles/main.css
git commit -m "feat: polish landing page with footer and styles"
```

---

### Task 15: Create Playground Styles

**Files:**
- Create: `website/src/styles/playground.css`

**Interfaces:**
- Consumes: Existing playground page structure
- Produces: Playground styles

- [ ] **Step 1: Create playground.css**

```css
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
  line-height: 1.6;
  color: #333;
}

header {
  background: #fff;
  border-bottom: 1px solid #eaeaea;
  padding: 1rem 2rem;
}

nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
  max-width: 1200px;
  margin: 0 auto;
}

.logo {
  font-size: 1.5rem;
  font-weight: bold;
  text-decoration: none;
  color: #333;
}

.nav-links {
  display: flex;
  gap: 2rem;
}

.nav-links a {
  text-decoration: none;
  color: #666;
  transition: color 0.2s;
}

.nav-links a:hover {
  color: #333;
}

.nav-links a.active {
  color: #333;
  font-weight: 500;
}

.playground {
  max-width: 1400px;
  margin: 0 auto;
  padding: 2rem;
}

.playground-header {
  margin-bottom: 2rem;
}

.playground-header h1 {
  font-size: 2rem;
  margin-bottom: 0.5rem;
}

.playground-header p {
  color: #666;
}

.playground-container {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
  height: 600px;
}

.editor-panel,
.output-panel {
  display: flex;
  flex-direction: column;
  border: 1px solid #eaeaea;
  border-radius: 8px;
  overflow: hidden;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1rem;
  background: #f9f9f9;
  border-bottom: 1px solid #eaeaea;
}

.panel-header span {
  font-weight: 500;
}

.btn-small {
  padding: 0.25rem 0.75rem;
  background: #0070f3;
  color: #fff;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.875rem;
}

.btn-small:hover {
  background: #0051cc;
}

.status {
  padding: 0.25rem 0.75rem;
  border-radius: 4px;
  font-size: 0.875rem;
}

.status.valid {
  background: #d4edda;
  color: #155724;
}

.status.error {
  background: #f8d7da;
  color: #721c24;
}

textarea {
  flex: 1;
  padding: 1rem;
  border: none;
  resize: none;
  font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.5;
}

textarea:focus {
  outline: none;
}

.tabs {
  display: flex;
  border-bottom: 1px solid #eaeaea;
}

.tab {
  padding: 0.75rem 1.5rem;
  background: none;
  border: none;
  cursor: pointer;
  color: #666;
  border-bottom: 2px solid transparent;
}

.tab.active {
  color: #333;
  border-bottom-color: #0070f3;
}

.tab-content {
  flex: 1;
  overflow: auto;
}

.tab-content pre {
  display: none;
  padding: 1rem;
  margin: 0;
  font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.5;
}

.tab-content pre.active {
  display: block;
}
```

- [ ] **Step 2: Verify the playground renders**

```bash
cd website && npm run dev
```

Visit `http://localhost:4321/gum-lang/playground/` and verify the playground displays with proper styling.

- [ ] **Step 3: Commit**

```bash
git add website/src/styles/playground.css
git commit -m "feat: add playground styles"
```

---

### Task 16: Integrate WASM into Playground

**Files:**
- Modify: `website/src/pages/playground/index.astro`

**Interfaces:**
- Consumes: WASM module from `public/pkg/` (built via `just build-wasm`)
- Produces: Playground with real WASM parser integration

**Prerequisites:** WASM must be built first with `just build-wasm`

- [ ] **Step 1: Build WASM module**

```bash
just build-wasm
```

This compiles the Rust core to WebAssembly and outputs to `website/public/pkg/`.

- [ ] **Step 2: Update playground script to use WASM**

Replace the entire `<script>` section in `playground/index.astro` with:

```javascript
import init, { parse_to_ast, format_document, validate_document, is_valid } from '/gum-lang/pkg/gum_core.js';

async function initParser() {
  try {
    await init();
    status.textContent = 'Ready';
    status.className = 'status valid';
    update();
  } catch (e) {
    astOutput.textContent = 'Failed to load WASM parser. Run `just build-wasm` first.';
    status.textContent = 'Error';
    status.className = 'status error';
  }
}

const editor = document.getElementById('editor');
const astOutput = document.getElementById('ast-output');
const jsonOutput = document.getElementById('json-output');
const errorsOutput = document.getElementById('errors-output');
const status = document.getElementById('status');
const formatBtn = document.getElementById('format-btn');

function update() {
  const input = editor.value;
  
  try {
    const ast = parse_to_ast(input);
    astOutput.textContent = JSON.stringify(ast, null, 2);
    jsonOutput.textContent = JSON.stringify(ast, null, 2);
    
    const errors = validate_document(input);
    if (errors && errors.length > 0) {
      errorsOutput.textContent = errors.map(e => e.message || e).join('\n');
      status.textContent = 'Error';
      status.className = 'status error';
    } else {
      errorsOutput.textContent = 'No errors';
      status.textContent = 'Valid';
      status.className = 'status valid';
    }
  } catch (e) {
    errorsOutput.textContent = e.message || e.toString();
    astOutput.textContent = '';
    jsonOutput.textContent = '';
    status.textContent = 'Error';
    status.className = 'status error';
  }
}

editor.addEventListener('input', update);

formatBtn.addEventListener('click', () => {
  try {
    const formatted = format_document(editor.value);
    editor.value = formatted;
    update();
  } catch (e) {
    // Keep current content on error
  }
});

document.querySelectorAll('.tab').forEach(tab => {
  tab.addEventListener('click', () => {
    document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
    document.querySelectorAll('.tab-content > pre').forEach(p => p.classList.remove('active'));
    tab.classList.add('active');
    document.getElementById(`${tab.dataset.tab}-output`).classList.add('active');
  });
});

editor.value = `# Welcome to Gum!
name = "Gum Playground"
version = 1
enabled = true

server = {
  host = "localhost"
  port = 8080
}

items = [1, 2, 3]`;

initParser();
```

- [ ] **Step 3: Add script type="module" attribute**

Update the `<script>` tag to include `type="module"`:

```html
<script type="module">
```

- [ ] **Step 4: Verify WASM integration**

```bash
cd website && npm run dev
```

Visit `http://localhost:4321/gum-lang/playground/` and verify:
- The parser loads successfully (status shows "Ready" or "Valid")
- Editing the textarea updates the AST/JSON output in real-time
- The Format button formats the content
- Errors are displayed correctly

- [ ] **Step 5: Commit**

```bash
git add website/src/pages/playground/index.astro
git commit -m "feat: integrate WASM parser into playground"
```

---

### Task 17: Final Verification

**Files:**
- All files created/modified in previous tasks

**Interfaces:**
- Consumes: Complete website implementation
- Produces: Verified, working website

- [ ] **Step 1: Build the website**

```bash
cd website && npm run build
```

Expected: Build succeeds without errors.

- [ ] **Step 2: Test all documentation pages**

```bash
cd website && npm run dev
```

Visit each page and verify it renders correctly:
- `http://localhost:4321/gum-lang/` — Landing page
- `http://localhost:4321/gum-lang/docs/` — Introduction
- `http://localhost:4321/gum-lang/docs/installation/` — Installation
- `http://localhost:4321/gum-lang/docs/quick-start/` — Quick Start
- `http://localhost:4321/gum-lang/docs/syntax/` — Syntax
- `http://localhost:4321/gum-lang/docs/types/` — Types
- `http://localhost:4321/gum-lang/docs/strings/` — Strings
- `http://localhost:4321/gum-lang/docs/tables-lists/` — Tables & Lists
- `http://localhost:4321/gum-lang/docs/cli/` — CLI
- `http://localhost:4321/gum-lang/docs/lsp/` — LSP
- `http://localhost:4321/gum-lang/docs/editors/` — Editor Support
- `http://localhost:4321/gum-lang/docs/parsers/` — Parsers
- `http://localhost:4321/gum-lang/docs/wasm/` — WASM
- `http://localhost:4321/gum-lang/playground/` — Playground

- [ ] **Step 3: Test playground functionality**

In the playground:
- Edit the textarea and verify AST updates in real-time
- Click Format button and verify content is formatted
- Introduce a syntax error and verify error is displayed
- Switch between AST/JSON/Errors tabs

- [ ] **Step 4: Verify navigation**

Test navigation between:
- Landing page → Docs
- Landing page → Playground
- Docs → Landing page
- Docs → Playground
- Playground → Landing page
- Playground → Docs

- [ ] **Step 5: Final commit (if any fixes needed)**

```bash
git add .
git commit -m "feat: complete website content implementation"
```
