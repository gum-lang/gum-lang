# Playground JSON Output & Starlight Landing Page Integration

**Date:** 2026-10-04  
**Status:** Approved for Implementation

## Overview

This design refactors the website playground to produce clean JSON output instead of raw AST structures, and integrates the landing page into the Starlight documentation theme.

## Goals

1. **Clean JSON Output:** Transform Gum documents into standard JSON format (TOML-style) instead of exposing internal AST structures
2. **Simplified UI:** Remove AST and Errors tabs, showing only JSON output with inline error display
3. **Theme Integration:** Convert landing page to use Starlight's built-in hero layout for visual consistency

## Architecture

### Core Rust Engine

Add a new WASM function `parse_to_json()` in `core/src/wasm.rs` that serializes the Document AST to clean JSON.

**Function Signature:**
```rust
#[wasm_bindgen]
pub fn parse_to_json(input: &str) -> Result<String, JsValue>
```

**Implementation:**
- Parse input to Document AST
- Transform AST to `serde_json::Value` using `ast_to_json()` helper
- Serialize to pretty-printed JSON string
- Return error if parsing fails

**Key Transformation Logic:**

The `ast_to_json()` function converts AST nodes to JSON values:

| AST Type | JSON Output |
|----------|-------------|
| `Document` | JSON object |
| `Assignment` | Key-value pair in object |
| `NonAssignment` | Skipped (comments, whitespace) |
| `Key::Bare("name")` | `"name"` |
| `Key::Quoted("a.b")` | `"a.b"` (single key, even with dots) |
| `Key::Dotted([Bare("a"), Bare("b")])` | Nested: `{"a": {"b": value}}` |
| `Value::String(Basic("text"))` | `"text"` |
| `Value::Number(Integer(42))` | `42` |
| `Value::Number(Float(3.14))` | `3.14` |
| `Value::Boolean(true)` | `true` |
| `Value::Null` | `null` |
| `Value::Table(entries)` | Nested JSON object |
| `Value::List(elements)` | JSON array |

**Dotted Key Semantics:**
- Bare dotted keys: `a.b = 1` → `{"a": {"b": 1}}` (nested objects)
- Quoted keys with dots: `"a.b" = 1` → `{"a.b": 1}` (flat key)

This matches TOML semantics and user expectations.

### Playground UI Changes

**Simplified Structure:**
- Remove tab buttons (AST/JSON/Errors)
- Single output panel showing JSON only
- Inline error display in output panel when parsing fails
- Keep status indicator (Ready/Valid/Error) in header

**JavaScript Changes:**
- Call `parse_to_json()` instead of `parse_to_ast()`
- Display returned JSON string in output panel
- On error, display error message in output panel (replacing JSON)
- Update status indicator based on parse result

**CSS Changes:**
- Remove tab-related styles from `playground.css`
- Keep panel layout and status indicator styles

**Files Modified:**
- `website/src/pages/playground/index.astro` - Remove tabs, update JavaScript
- `website/src/styles/playground.css` - Remove tab styles

### Landing Page Changes

**Convert to Starlight Hero Layout:**

Replace custom HTML with Starlight's built-in hero component:

```astro
<StarlightPage
  frontmatter={{
    title: 'Gum Markup Language',
    description: 'A simple, human-readable configuration and data markup language.',
    hero: {
      title: 'Gum Markup Language',
      tagline: 'A simple, human-readable configuration and data markup language.',
      actions: [
        { text: 'Get Started', link: '/docs/', icon: 'right-arrow', variant: 'primary' },
        { text: 'Try it Live', link: '/playground/', icon: 'external', variant: 'secondary' }
      ]
    }
  }}
  hasSidebar={false}
>
</StarlightPage>
```

**Removed:**
- Custom `<header>`, `<main>`, `<footer>` HTML
- Features section (Simple Syntax, Multi-Language, Full Tooling)
- `src/styles/main.css` file

**Result:**
- Landing page inherits Starlight's header/nav/footer automatically
- Consistent visual theme across all pages
- Simpler maintenance

## Testing & Verification

### Rust Tests

Add unit tests in `core/src/wasm.rs` (or new test module) for `ast_to_json()`:

**Test Cases:**
- Simple assignment: `name = "Gum"` → `{"name": "Gum"}`
- Nested table: `server = { host = "localhost" }` → `{"server": {"host": "localhost"}}`
- Dotted key: `a.b = 1` → `{"a": {"b": 1}}`
- Quoted dotted key: `"a.b" = 1` → `{"a.b": 1}`
- List: `items = [1, 2, 3]` → `{"items": [1, 2, 3]}`
- Mixed types: strings, numbers, booleans, null
- Comments and whitespace ignored
- Complex nested structures

**Command:** `cargo test` in `core/`

### Playground Verification

**Steps:**
1. Run `just build-wasm` to rebuild WASM with new function
2. Open playground in browser
3. Verify JSON output matches expected format
4. Test with invalid input - verify error displays inline
5. Verify status indicator updates correctly
6. Test Format button still works

### Landing Page Verification

**Steps:**
1. Run `npm run dev` in `website/`
2. Confirm landing page renders with Starlight header/nav/footer
3. Confirm hero section displays correctly
4. Confirm CTA buttons work (Get Started, Try it Live)
5. Confirm no broken styles or missing assets

## Implementation Sequence

1. **Core Rust Changes**
   - Add `ast_to_json()` helper function
   - Add `parse_to_json()` WASM function
   - Add unit tests
   - Run `cargo test`

2. **WASM Build**
   - Run `just build-wasm` to compile new WASM

3. **Playground Updates**
   - Remove tab HTML structure
   - Update JavaScript to call `parse_to_json()`
   - Add inline error display logic
   - Remove tab styles from CSS

4. **Landing Page Conversion**
   - Rewrite `index.astro` with Starlight hero
   - Delete `main.css`
   - Verify rendering

5. **Final Verification**
   - Test playground end-to-end
   - Test landing page
   - Run `just test-all` to ensure no regressions

## Success Criteria

- Playground produces clean JSON output (not raw AST)
- Parse errors display inline in output panel
- Landing page uses Starlight theme consistently
- All existing tests pass
- No visual regressions in documentation pages
