# Playground Starlight Integration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Integrate the playground into Starlight's layout so it shares the same header, footer, dark mode, and typography as documentation pages.

**Architecture:** Replace the standalone HTML layout with Starlight's `<StarlightPage>` component. The playground retains its interactive editor/output panels but inherits Starlight's page shell. CSS uses Starlight's design tokens (CSS variables) for automatic dark mode support.

**Tech Stack:** Astro 7.3.5, @astrojs/starlight 0.42.5, WebAssembly

## Global Constraints

- Use `<StarlightPage>` component from `@astrojs/starlight/components`
- Replace hardcoded colors with Starlight CSS variables: `--sl-color-text`, `--sl-color-text-secondary`, `--sl-color-accent`, `--sl-color-bg-nav`, `--sl-color-hairline`
- Preserve all WASM loading logic and interactive functionality
- Build must succeed with `npm run build`

---

### Task 1: Rewrite Playground Page with StarlightPage

**Files:**
- Modify: `website/src/pages/playground/index.astro`

**Interfaces:**
- Consumes: Starlight's `<StarlightPage>` component
- Produces: Playground page using Starlight layout

- [ ] **Step 1: Read current playground page**

Read `website/src/pages/playground/index.astro` to understand the current structure.

- [ ] **Step 2: Rewrite the page to use StarlightPage**

Replace the entire file with:

```astro
---
import { StarlightPage } from '@astrojs/starlight/components';
import '../../styles/playground.css';
---

<StarlightPage
  frontmatter={{
    title: 'Interactive Playground',
    description: 'Write Gum markup and see the parsed AST in real-time.'
  }}
  hasSidebar={false}
>
  <div class="playground-container">
    <div class="editor-panel">
      <div class="panel-header">
        <span>Editor</span>
        <button id="format-btn" class="btn-small">Format</button>
      </div>
      <textarea id="editor" spellcheck="false"></textarea>
    </div>
    <div class="output-panel">
      <div class="panel-header">
        <span>Output</span>
        <div class="status" id="status">Ready</div>
      </div>
      <div class="tabs">
        <button class="tab active" data-tab="ast">AST</button>
        <button class="tab" data-tab="json">JSON</button>
        <button class="tab" data-tab="errors">Errors</button>
      </div>
      <div class="tab-content">
        <pre id="ast-output" class="active">Loading parser...</pre>
        <pre id="json-output"></pre>
        <pre id="errors-output"></pre>
      </div>
    </div>
  </div>

  <script type="module">
    let parse_to_ast, format_document, validate_document, is_valid, init;

    async function loadWasm() {
      try {
        const response = await fetch('/pkg/gum_core.js');
        const code = await response.text();
        const blob = new Blob([code], { type: 'application/javascript' });
        const url = URL.createObjectURL(blob);
        const wasm = await import(url);
        URL.revokeObjectURL(url);
        init = wasm.default;
        parse_to_ast = wasm.parse_to_ast;
        format_document = wasm.format_document;
        validate_document = wasm.validate_document;
        is_valid = wasm.is_valid;
        return true;
      } catch (e) {
        console.error('Failed to load WASM:', e);
        return false;
      }
    }

    async function initParser() {
      const loaded = await loadWasm();
      if (!loaded) {
        astOutput.textContent = 'Failed to load WASM parser. Run `just build-wasm` first.';
        status.textContent = 'Error';
        status.className = 'status error';
        return;
      }
      try {
        await init('/pkg/gum_core_bg.wasm');
        status.textContent = 'Ready';
        status.className = 'status valid';
        update();
      } catch (e) {
        astOutput.textContent = 'Failed to initialize WASM parser.';
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
  </script>
</StarlightPage>
```

- [ ] **Step 3: Verify the build**

```bash
cd website && npm run build
```

Expected: Build succeeds, `/playground/index.html` is generated.

- [ ] **Step 4: Commit**

```bash
git add website/src/pages/playground/index.astro
git commit -m "refactor: use StarlightPage for playground layout"
```

---

### Task 2: Update Playground CSS to Use Starlight Design Tokens

**Files:**
- Modify: `website/src/styles/playground.css`

**Interfaces:**
- Consumes: Starlight CSS variables
- Produces: Playground styles with dark mode support

- [ ] **Step 1: Read current playground CSS**

Read `website/src/styles/playground.css` to understand the current styles.

- [ ] **Step 2: Replace hardcoded colors with Starlight CSS variables**

Replace the entire file with:

```css
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
  border: 1px solid var(--sl-color-hairline);
  border-radius: 8px;
  overflow: hidden;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1rem;
  background: var(--sl-color-bg-nav);
  border-bottom: 1px solid var(--sl-color-hairline);
}

.panel-header span {
  font-weight: 500;
  color: var(--sl-color-text);
}

.btn-small {
  padding: 0.25rem 0.75rem;
  background: var(--sl-color-accent);
  color: #fff;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.875rem;
}

.btn-small:hover {
  opacity: 0.9;
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
  background: var(--sl-color-bg);
  color: var(--sl-color-text);
}

textarea:focus {
  outline: none;
}

.tabs {
  display: flex;
  border-bottom: 1px solid var(--sl-color-hairline);
}

.tab {
  padding: 0.75rem 1.5rem;
  background: none;
  border: none;
  cursor: pointer;
  color: var(--sl-color-text-secondary);
  border-bottom: 2px solid transparent;
}

.tab.active {
  color: var(--sl-color-text);
  border-bottom-color: var(--sl-color-accent);
}

.tab-content {
  flex: 1;
  overflow: auto;
  background: var(--sl-color-bg);
}

.tab-content pre {
  display: none;
  padding: 1rem;
  margin: 0;
  font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.5;
  color: var(--sl-color-text);
}

.tab-content pre.active {
  display: block;
}
```

- [ ] **Step 3: Verify the build**

```bash
cd website && npm run build
```

Expected: Build succeeds.

- [ ] **Step 4: Commit**

```bash
git add website/src/styles/playground.css
git commit -m "style: use Starlight CSS variables for playground"
```

---

### Task 3: Add Playground to Navigation

**Files:**
- Modify: `website/astro.config.mjs`

**Interfaces:**
- Consumes: Starlight configuration
- Produces: Playground link in navigation

- [ ] **Step 1: Add playground link to Starlight config**

Add the playground as a top-level sidebar link so it's accessible from all doc pages.

Update `website/astro.config.mjs` — add `{ label: 'Playground', link: '/playground/' }` as the first item in the sidebar array:

```javascript
sidebar: [
  { label: 'Playground', link: '/playground/' },
  {
    label: 'Getting Started',
    items: [
      { label: 'Introduction', link: '/docs/' },
      { label: 'Installation', link: '/docs/installation/' },
      { label: 'Quick Start', link: '/docs/quick-start/' },
    ],
  },
  // ... rest unchanged
],
```

- [ ] **Step 2: Verify the build**

```bash
cd website && npm run build
```

Expected: Build succeeds, playground link appears in the header.

- [ ] **Step 3: Commit**

```bash
git add website/astro.config.mjs
git commit -m "feat: add playground link to navigation"
```

---

### Task 4: Final Verification

**Files:**
- None (verification only)

- [ ] **Step 1: Run dev server and verify**

```bash
cd website && npm run dev
```

Visit `http://localhost:4321/playground/` and verify:
- Page uses Starlight header and footer
- Dark mode toggle works and playground colors update
- Editor and output panels display correctly
- WASM parser loads and functions
- Navigation between docs and playground is seamless

- [ ] **Step 2: Build for production**

```bash
cd website && npm run build
```

Expected: Build succeeds with no errors.

- [ ] **Step 3: Commit any final fixes**

If any issues were found, fix and commit them.
