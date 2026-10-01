# Playground Starlight Integration Design

**Date:** 2026-10-01  
**Status:** Draft

## Goal

Integrate the interactive playground into Starlight's layout so it shares the same header, footer, dark mode, and typography as the documentation pages, eliminating the visual disconnect between docs and playground.

## Approach

Replace the standalone HTML layout with Starlight's `StarlightPage` component. The playground retains its interactive editor/output panels but inherits Starlight's page shell and design tokens.

## Changes

### 1. `website/src/pages/playground/index.astro`

- Remove the standalone HTML structure (`<!doctype html>`, `<head>`, `<body>`, custom `<header>`, `<nav>`)
- Use `<StarlightPage>` wrapper with frontmatter for title/description
- Keep the interactive editor/output panels unchanged
- Keep the WASM loading logic unchanged
- Remove the `<link>` to `playground.css` from `<head>` (Starlight handles styles)

### 2. `website/src/styles/playground.css`

- Replace hardcoded colors with Starlight CSS variables:
  - `#333` → `var(--sl-color-text)`
  - `#666` → `var(--sl-color-text-secondary)`
  - `#0070f3` → `var(--sl-color-accent)`
  - `#f9f9f9` → `var(--sl-color-bg-nav)`
  - `#eaeaea` → `var(--sl-color-hairline)`
- Remove rules for `header`, `nav`, `.logo`, `.nav-links` (Starlight provides these)
- Keep rules for `.playground`, `.playground-container`, `.editor-panel`, `.output-panel`, tabs, textarea, status indicators
- This gives automatic dark mode support

### 3. Navigation

- Add playground link to Starlight's top navigation via the `header` slot or keep it as a standalone page accessible from docs
- Option A: Add to Starlight config as a custom page (appears in sidebar)
- Option B: Keep as standalone page, add link from docs pages manually
- **Decision:** Use Option A — add playground to Starlight sidebar under a "Tools" section

## What Stays the Same

- WASM loading logic (blob URL approach with explicit WASM path)
- Editor textarea with default content
- Output tabs (AST, JSON, Errors)
- Format button functionality
- Real-time parsing on input

## Layout

```
┌─────────────────────────────────────────┐
│ Starlight Header (logo, nav, dark mode) │
├─────────────────────────────────────────┤
│ Page Title: "Interactive Playground"    │
│ Description: "Write Gum markup..."      │
├──────────────────┬──────────────────────┤
│ Editor Panel     │ Output Panel         │
│ - Format button  │ - Status indicator   │
│ - Textarea       │ - Tabs (AST/JSON/Err)│
│                  │ - Pre blocks         │
├──────────────────┴──────────────────────┤
│ Starlight Footer                        │
└─────────────────────────────────────────┘
```

## Testing

- Verify playground loads and functions correctly
- Verify dark mode toggle works and playground colors update
- Verify navigation between docs and playground is seamless
- Verify responsive layout on mobile

## Migration Path

1. Rewrite `playground/index.astro` to use `StarlightPage`
2. Update `playground.css` to use Starlight CSS variables
3. Add playground to Starlight sidebar config
4. Test in dev mode, verify dark mode
5. Build and verify production output
