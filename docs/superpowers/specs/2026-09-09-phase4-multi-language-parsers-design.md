# Phase 4: Multi-Language Parser Implementation Design

## Overview

Implement 4 native parsers for the Gum markup language, each producing identical ASTs to the Rust `core` engine. This ensures cross-language parity and enables ecosystem adoption across TypeScript, Python, Go, and Rust environments.

## Architecture

### parser-rust: Thin Wrapper Crate
- **Approach**: Re-exports `gum-core` with ergonomic API
- **Dependencies**: `gum-core` (path dependency to `../core`)
- **Deliverable**: Idiomatic Rust crate publishable to crates.io
- **API**: `gum_parser::parse()`, `gum_parser::validate()`, `gum_parser::format()`
- **Tests**: Re-uses core's test suite via path symlink

### parser-ts: WASM Wrapper
- **Approach**: Loads WASM build from `core/` compiled via `wasm-pack`
- **Dependencies**: `@gum-lang/core` WASM package, TypeScript type definitions
- **Deliverable**: npm package with TypeScript types mirroring AST
- **API**: `import { parse, validate, format } from '@gum-lang/parser'`
- **Tests**: Copies `core/integration_tests/` files, runs via Jest/Vitest

### parser-python: PyO3/maturin Bindings
- **Approach**: Rust + PyO3 bindings compiled via maturin
- **Dependencies**: `gum-core` (path dependency), `pyo3`, `maturin`
- **Deliverable**: PyPI package with native extension
- **API**: `from gum_parser import parse, validate, format`
- **Tests**: Copies `core/integration_tests/` files, runs via pytest

### parser-go: Native Recursive-Descent Parser
- **Approach**: Hand-written Go parser matching ABNF grammar
- **Dependencies**: None (pure Go)
- **Deliverable**: Go module publishable to GoProxy
- **API**: `gum.Parse(input)`, `gum.Validate(doc)`, `gum.Format(doc)`
- **Tests**: Copies `core/integration_tests/` files, runs via `go test`

## Shared Test Strategy

Each parser gets:
1. Copy of `core/integration_tests/valid/*.gum` — must parse successfully
2. Copy of `core/integration_tests/invalid/*.gum` — must fail to parse/validate
3. Roundtrip test: parse → format → parse yields identical AST

## Implementation Order

All 4 parsers are independent and can be built in parallel.

## Data Flow

```
ABNF Grammar → core/ (Rust reference)
                    ↓ (WASM)          ↓ (PyO3)         ↓ (re-export)    ↓ (independent)
              parser-ts/          parser-python/    parser-rust/      parser-go/
              (npm)               (PyPI)            (crates.io)       (GoProxy)
                    ↓                  ↓                 ↓                 ↓
              Same AST ← Same AST ← Same AST ← Same AST (verified by tests)
```

## Error Handling

All parsers must:
- Return structured errors with line/column information
- Distinguish parse errors from validation errors
- Use language-idiomatic error types (Result<T, E> in Rust, exceptions in Python/TS, error return in Go)

## Success Criteria

1. All 4 parsers pass `just test-all`
2. All 4 parsers produce identical ASTs for the same input (verified by integration tests)
3. Each parser is publishable to its respective package registry
4. Cross-language parity: 100% test pass rate across all parsers
