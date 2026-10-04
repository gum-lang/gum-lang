# Playground JSON Output & Starlight Landing Page Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor the website playground to produce clean JSON output instead of raw AST structures, and integrate the landing page into the Starlight documentation theme.

**Architecture:** Add a new WASM function `parse_to_json()` in the Rust core that transforms the Document AST into clean JSON format. The playground UI is simplified to show only JSON output with inline error display. The landing page is converted to use Starlight's built-in hero layout.

**Tech Stack:** Rust (wasm-bindgen, serde_json), TypeScript/Astro, Starlight, WASM

## Global Constraints

- Rust Edition 2021+
- All WASM functions must be gated with `#[cfg(target_arch = "wasm32")]`
- Use idiomatic Rust with zero clippy warnings
- Playground must use Starlight CSS variables for consistency
- Landing page must use Starlight's hero component (no custom HTML)

---

## File Structure

**Core Rust Engine:**
- Modify: `core/src/wasm.rs` - Add `parse_to_json()` function and `ast_to_json()` helper
- Modify: `core/src/lib.rs` - Export new function (if needed)

**Playground:**
- Modify: `website/src/pages/playground/index.astro` - Remove tabs, update JavaScript
- Modify: `website/src/styles/playground.css` - Remove tab styles

**Landing Page:**
- Modify: `website/src/pages/index.astro` - Convert to Starlight hero layout
- Delete: `website/src/styles/main.css` - No longer needed

---

### Task 1: Implement ast_to_json() Helper Function (TDD)

**Files:**
- Modify: `core/src/wasm.rs` - Add `ast_to_json()` helper function
- Test: `core/src/wasm.rs` - Add unit tests in `#[cfg(test)]` module

**Interfaces:**
- Consumes: `Document` AST from `crate::ast`
- Produces: `serde_json::Value` representing clean JSON structure

**Context:**
The `ast_to_json()` function converts the internal AST representation into a clean JSON structure that matches user expectations (TOML-style). It must handle:
- `Document` → JSON object
- `Assignment` → key-value pairs
- `NonAssignment` → skipped (comments, whitespace)
- `Key::Bare("name")` → `"name"`
- `Key::Quoted("a.b")` → `"a.b"` (single key, even with dots)
- `Key::Dotted([Bare("a"), Bare("b")])` → nested `{"a": {"b": value}}`
- `Value::String` variants → JSON string
- `Value::Number` variants → JSON number
- `Value::Boolean` → JSON boolean
- `Value::Null` → JSON null
- `Value::Table` → nested JSON object
- `Value::List` → JSON array

- [ ] **Step 1: Write failing test for simple assignment**

Add a test module at the end of `core/src/wasm.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use serde_json::json;

    #[test]
    fn test_ast_to_json_simple_assignment() {
        let doc = Document {
            elements: vec![DocumentElement::Assignment(Assignment {
                key: Key::Bare("name".to_string()),
                value: Value::String(StringValue::Basic("Gum".to_string())),
            })],
        };
        
        let result = ast_to_json(&doc);
        let expected = json!({"name": "Gum"});
        
        assert_eq!(result, expected);
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --package gum-core --lib wasm::tests::test_ast_to_json_simple_assignment`
Expected: FAIL with "cannot find function `ast_to_json` in this scope"

- [ ] **Step 3: Implement ast_to_json() function**

Add the following to `core/src/wasm.rs` before the test module:

```rust
use serde_json::{json, Value as JsonValue};

fn ast_to_json(doc: &crate::ast::Document) -> JsonValue {
    let mut map = serde_json::Map::new();
    
    for element in &doc.elements {
        if let crate::ast::DocumentElement::Assignment(assignment) = element {
            let key_str = assignment.key.as_str();
            let value_json = value_to_json(&assignment.value);
            
            // Handle dotted keys by creating nested objects
            if let crate::ast::Key::Dotted(keys) = &assignment.key {
                insert_dotted_key(&mut map, keys, value_json);
            } else {
                map.insert(key_str, value_json);
            }
        }
    }
    
    JsonValue::Object(map)
}

fn value_to_json(value: &crate::ast::Value) -> JsonValue {
    match value {
        crate::ast::Value::String(s) => {
            let s = match s {
                crate::ast::StringValue::Basic(s) => s,
                crate::ast::StringValue::MultilineBasic(s) => s,
                crate::ast::StringValue::Literal(s) => s,
                crate::ast::StringValue::MultilineLiteral(s) => s,
            };
            JsonValue::String(s.clone())
        }
        crate::ast::Value::Number(n) => {
            match n {
                crate::ast::NumberValue::Integer(i) => json!(*i),
                crate::ast::NumberValue::Float(f) => json!(*f),
            }
        }
        crate::ast::Value::Boolean(b) => JsonValue::Bool(*b),
        crate::ast::Value::Null => JsonValue::Null,
        crate::ast::Value::Table(table) => {
            let mut map = serde_json::Map::new();
            for assignment in &table.entries {
                let key_str = assignment.key.as_str();
                let value_json = value_to_json(&assignment.value);
                
                if let crate::ast::Key::Dotted(keys) = &assignment.key {
                    insert_dotted_key(&mut map, keys, value_json);
                } else {
                    map.insert(key_str, value_json);
                }
            }
            JsonValue::Object(map)
        }
        crate::ast::Value::List(list) => {
            let array: Vec<JsonValue> = list.elements.iter().map(value_to_json).collect();
            JsonValue::Array(array)
        }
    }
}

fn insert_dotted_key(map: &mut serde_json::Map<String, JsonValue>, keys: &[crate::ast::Key], value: JsonValue) {
    if keys.is_empty() {
        return;
    }
    
    if keys.len() == 1 {
        let key_str = keys[0].as_str();
        map.insert(key_str, value);
        return;
    }
    
    let first_key = keys[0].as_str();
    let remaining_keys = &keys[1..];
    
    let nested = if let Some(existing) = map.get_mut(&first_key) {
        if let JsonValue::Object(ref mut nested_map) = existing {
            insert_dotted_key(nested_map, remaining_keys, value);
        }
        return;
    } else {
        let mut nested_map = serde_json::Map::new();
        insert_dotted_key(&mut nested_map, remaining_keys, value);
        JsonValue::Object(nested_map)
    };
    
    map.insert(first_key, nested);
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --package gum-core --lib wasm::tests::test_ast_to_json_simple_assignment`
Expected: PASS

- [ ] **Step 5: Add test for dotted keys**

```rust
#[test]
fn test_ast_to_json_dotted_key() {
    let doc = Document {
        elements: vec![DocumentElement::Assignment(Assignment {
            key: Key::Dotted(vec![
                Key::Bare("a".to_string()),
                Key::Bare("b".to_string()),
            ]),
            value: Value::Number(NumberValue::Integer(1)),
        })],
    };
    
    let result = ast_to_json(&doc);
    let expected = json!({"a": {"b": 1}});
    
    assert_eq!(result, expected);
}
```

- [ ] **Step 6: Run test to verify it passes**

Run: `cargo test --package gum-core --lib wasm::tests::test_ast_to_json_dotted_key`
Expected: PASS

- [ ] **Step 7: Add test for quoted key with dots**

```rust
#[test]
fn test_ast_to_json_quoted_dotted_key() {
    let doc = Document {
        elements: vec![DocumentElement::Assignment(Assignment {
            key: Key::Quoted("a.b".to_string()),
            value: Value::Number(NumberValue::Integer(1)),
        })],
    };
    
    let result = ast_to_json(&doc);
    let expected = json!({"a.b": 1});
    
    assert_eq!(result, expected);
}
```

- [ ] **Step 8: Run test to verify it passes**

Run: `cargo test --package gum-core --lib wasm::tests::test_ast_to_json_quoted_dotted_key`
Expected: PASS

- [ ] **Step 9: Add test for nested table**

```rust
#[test]
fn test_ast_to_json_nested_table() {
    let doc = Document {
        elements: vec![DocumentElement::Assignment(Assignment {
            key: Key::Bare("server".to_string()),
            value: Value::Table(crate::ast::Table {
                entries: vec![
                    Assignment {
                        key: Key::Bare("host".to_string()),
                        value: Value::String(StringValue::Basic("localhost".to_string())),
                    },
                    Assignment {
                        key: Key::Bare("port".to_string()),
                        value: Value::Number(NumberValue::Integer(8080)),
                    },
                ],
            }),
        })],
    };
    
    let result = ast_to_json(&doc);
    let expected = json!({"server": {"host": "localhost", "port": 8080}});
    
    assert_eq!(result, expected);
}
```

- [ ] **Step 10: Run test to verify it passes**

Run: `cargo test --package gum-core --lib wasm::tests::test_ast_to_json_nested_table`
Expected: PASS

- [ ] **Step 11: Add test for list**

```rust
#[test]
fn test_ast_to_json_list() {
    let doc = Document {
        elements: vec![DocumentElement::Assignment(Assignment {
            key: Key::Bare("items".to_string()),
            value: Value::List(crate::ast::List {
                elements: vec![
                    Value::Number(NumberValue::Integer(1)),
                    Value::Number(NumberValue::Integer(2)),
                    Value::Number(NumberValue::Integer(3)),
                ],
            }),
        })],
    };
    
    let result = ast_to_json(&doc);
    let expected = json!({"items": [1, 2, 3]});
    
    assert_eq!(result, expected);
}
```

- [ ] **Step 12: Run test to verify it passes**

Run: `cargo test --package gum-core --lib wasm::tests::test_ast_to_json_list`
Expected: PASS

- [ ] **Step 13: Add test for comments being ignored**

```rust
#[test]
fn test_ast_to_json_ignores_comments() {
    let doc = Document {
        elements: vec![
            DocumentElement::NonAssignment(crate::ast::NonAssignment {
                content: "# This is a comment".to_string(),
            }),
            DocumentElement::Assignment(Assignment {
                key: Key::Bare("name".to_string()),
                value: Value::String(StringValue::Basic("Gum".to_string())),
            }),
        ],
    };
    
    let result = ast_to_json(&doc);
    let expected = json!({"name": "Gum"});
    
    assert_eq!(result, expected);
}
```

- [ ] **Step 14: Run test to verify it passes**

Run: `cargo test --package gum-core --lib wasm::tests::test_ast_to_json_ignores_comments`
Expected: PASS

- [ ] **Step 15: Run all tests**

Run: `cargo test --package gum-core`
Expected: All tests PASS

- [ ] **Step 16: Commit**

```bash
git add core/src/wasm.rs
git commit -m "feat: add ast_to_json() helper function with tests"
```

---

### Task 2: Add parse_to_json() WASM Function

**Files:**
- Modify: `core/src/wasm.rs` - Add `parse_to_json()` WASM function

**Interfaces:**
- Consumes: Gum document string input
- Produces: Pretty-printed JSON string or error

**Context:**
The `parse_to_json()` function is the public WASM interface that parses a Gum document and returns clean JSON. It uses the `ast_to_json()` helper from Task 1.

- [ ] **Step 1: Write failing test for parse_to_json()**

Add to the test module in `core/src/wasm.rs`:

```rust
#[test]
fn test_parse_to_json_simple() {
    let input = r#"name = "Gum""#;
    let result = parse_to_json_impl(input).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    let expected = json!({"name": "Gum"});
    
    assert_eq!(parsed, expected);
}

#[test]
fn test_parse_to_json_error() {
    let input = r#"invalid syntax"#;
    let result = parse_to_json_impl(input);
    
    assert!(result.is_err());
}
```

Note: We test `parse_to_json_impl()` which is the non-WASM version that can be tested without the WASM target.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --package gum-core --lib wasm::tests::test_parse_to_json_simple`
Expected: FAIL with "cannot find function `parse_to_json_impl` in this scope"

- [ ] **Step 3: Implement parse_to_json_impl() and parse_to_json()**

Add to `core/src/wasm.rs` before the test module:

```rust
fn parse_to_json_impl(input: &str) -> Result<String, String> {
    match crate::parse(input) {
        Ok(doc) => {
            let json_value = ast_to_json(&doc);
            serde_json::to_string_pretty(&json_value)
                .map_err(|e| format!("JSON serialization error: {}", e))
        }
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn parse_to_json(input: &str) -> Result<String, JsValue> {
    parse_to_json_impl(input).map_err(|e| JsValue::from_str(&e))
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --package gum-core --lib wasm::tests::test_parse_to_json`
Expected: All tests PASS

- [ ] **Step 5: Run all tests**

Run: `cargo test --package gum-core`
Expected: All tests PASS

- [ ] **Step 6: Check for clippy warnings**

Run: `cargo clippy --package gum-core --all-targets --all-features -- -D warnings`
Expected: No warnings

- [ ] **Step 7: Commit**

```bash
git add core/src/wasm.rs
git commit -m "feat: add parse_to_json() WASM function"
```

---

### Task 3: Build WASM and Verify

**Files:**
- Build: WASM artifacts in `website/public/pkg/`

**Interfaces:**
- Consumes: Rust core library
- Produces: WASM and JavaScript bindings

- [ ] **Step 1: Build WASM**

Run: `just build-wasm`
Expected: WASM builds successfully, artifacts appear in `website/public/pkg/`

- [ ] **Step 2: Verify parse_to_json is exported**

Run: `grep "parse_to_json" website/public/pkg/gum_core.js`
Expected: Function is present in the generated JavaScript bindings

- [ ] **Step 3: Commit (if any build artifacts changed)**

```bash
git add website/public/pkg/
git commit -m "build: rebuild WASM with parse_to_json()"
```

---

### Task 4: Update Playground UI

**Files:**
- Modify: `website/src/pages/playground/index.astro` - Remove tabs, update JavaScript
- Modify: `website/src/styles/playground.css` - Remove tab styles

**Interfaces:**
- Consumes: `parse_to_json()` WASM function from Task 3
- Produces: Simplified playground with JSON-only output

**Context:**
Remove the AST/JSON/Errors tabs and show only JSON output. Display errors inline in the output panel when parsing fails.

- [ ] **Step 1: Remove tab HTML structure**

In `website/src/pages/playground/index.astro`, replace lines 26-35 (the tabs and tab-content) with:

```html
<div class="output-content">
  <pre id="json-output">Loading parser...</pre>
</div>
```

- [ ] **Step 2: Update JavaScript to use parse_to_json()**

Replace the `update()` function (lines 101-126) with:

```javascript
function update() {
  const input = editor.value;
  
  try {
    const json = parse_to_json(input);
    jsonOutput.textContent = json;
    status.textContent = 'Valid';
    status.className = 'status valid';
  } catch (e) {
    jsonOutput.textContent = e.message || e.toString();
    status.textContent = 'Error';
    status.className = 'status error';
  }
}
```

- [ ] **Step 3: Remove unused WASM function imports**

In the `loadWasm()` function, remove these lines:

```javascript
parse_to_ast = wasm.parse_to_ast;
validate_document = wasm.validate_document;
is_valid = wasm.is_valid;
```

And add:

```javascript
parse_to_json = wasm.parse_to_json;
```

Update the variable declarations at line 40:

```javascript
let parse_to_json, format_document, init;
```

- [ ] **Step 4: Remove unused element references**

Remove these lines (95-97):

```javascript
const astOutput = document.getElementById('ast-output');
const errorsOutput = document.getElementById('errors-output');
```

- [ ] **Step 5: Remove tab event listeners**

Remove lines 139-146 (the tab click event listener code).

- [ ] **Step 6: Remove tab styles from CSS**

In `website/src/styles/playground.css`, remove lines 77-113 (all tab-related styles).

- [ ] **Step 7: Update output-content styles**

Add to `website/src/styles/playground.css`:

```css
.output-content {
  flex: 1;
  overflow: auto;
  background: var(--sl-color-bg);
}

.output-content pre {
  padding: 1rem;
  margin: 0;
  font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.5;
  color: var(--sl-color-text);
  white-space: pre-wrap;
  word-wrap: break-word;
}
```

- [ ] **Step 8: Test in browser**

Run: `npm run dev` in `website/`
Open: `http://localhost:4321/playground/`
Verify:
- JSON output displays correctly (not raw AST)
- Errors display inline in output panel
- Status indicator updates correctly
- Format button still works

- [ ] **Step 9: Commit**

```bash
git add website/src/pages/playground/index.astro website/src/styles/playground.css
git commit -m "feat: simplify playground to show JSON output only"
```

---

### Task 5: Convert Landing Page to Starlight

**Files:**
- Modify: `website/src/pages/index.astro` - Convert to Starlight hero layout
- Delete: `website/src/styles/main.css` - No longer needed

**Interfaces:**
- Consumes: Starlight's hero component
- Produces: Landing page with Starlight theme

**Context:**
Replace the custom landing page HTML with Starlight's built-in hero layout. Drop the features section entirely.

- [ ] **Step 1: Rewrite index.astro with Starlight hero**

Replace the entire contents of `website/src/pages/index.astro` with:

```astro
---
import StarlightPage from '@astrojs/starlight/components/StarlightPage.astro';
---

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

- [ ] **Step 2: Delete main.css**

```bash
rm website/src/styles/main.css
```

- [ ] **Step 3: Test in browser**

Run: `npm run dev` in `website/`
Open: `http://localhost:4321/`
Verify:
- Landing page renders with Starlight header/nav/footer
- Hero section displays correctly with title and tagline
- CTA buttons work (Get Started, Try it Live)
- No broken styles or missing assets

- [ ] **Step 4: Commit**

```bash
git add website/src/pages/index.astro
git rm website/src/styles/main.css
git commit -m "feat: convert landing page to Starlight hero layout"
```

---

### Task 6: Final Verification

**Files:**
- Test: All modified files

**Interfaces:**
- Consumes: All previous tasks
- Produces: Verified working implementation

- [ ] **Step 1: Run all Rust tests**

Run: `cargo test --package gum-core`
Expected: All tests PASS

- [ ] **Step 2: Run just test-all**

Run: `just test-all`
Expected: All tests across all repositories PASS

- [ ] **Step 3: Test playground end-to-end**

Open: `http://localhost:4321/playground/`
Test with this input:

```gum
# Welcome to Gum!
name = "Gum Playground"
version = 1
enabled = true

server = {
  host = "localhost"
  port = 8080
}

items = [1, 2, 3]
```

Verify JSON output:

```json
{
  "name": "Gum Playground",
  "version": 1,
  "enabled": true,
  "server": {
    "host": "localhost",
    "port": 8080
  },
  "items": [
    1,
    2,
    3
  ]
}
```

- [ ] **Step 4: Test error handling**

Enter invalid input: `invalid syntax here`
Verify: Error message displays inline in output panel, status shows "Error"

- [ ] **Step 5: Test landing page**

Open: `http://localhost:4321/`
Verify:
- Starlight header/nav/footer present
- Hero section with "Gum Markup Language" title
- "Get Started" button links to `/docs/`
- "Try it Live" button links to `/playground/`

- [ ] **Step 6: Test documentation pages**

Open: `http://localhost:4321/docs/`
Verify: Documentation pages still render correctly with no regressions

- [ ] **Step 7: Build for production**

Run: `npm run build` in `website/`
Expected: Build completes successfully with no errors

- [ ] **Step 8: Commit (if any final adjustments)**

```bash
git add .
git commit -m "chore: final verification and adjustments"
```

---

## Success Criteria

- [ ] Playground produces clean JSON output (not raw AST)
- [ ] Parse errors display inline in output panel
- [ ] Landing page uses Starlight theme consistently
- [ ] All existing tests pass (`just test-all`)
- [ ] No visual regressions in documentation pages
- [ ] Production build succeeds (`npm run build`)
