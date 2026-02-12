# Lunch Macro Validation Model

## Overview

This document describes how validation is handled inside the `lunch!` procedural macro pipeline.

Validation is intentionally separated into distinct phases to preserve architectural clarity and prevent semantic drift.

Pipeline flow:

1. Parsing (`syn::Parse` implementations)
2. Validation + Indexing (`validate_and_index`)
3. Lowering (code generation)
4. Rust compiler symbol resolution

Validation is responsible only for DSL semantic correctness, not Rust symbol resolution.

---

## Phase Separation

### Parsing Phase

Parsing guarantees:

- Grammar structure is correct.
- Sections (`meta`*coming soon*, `operations`, `signals`, `clinch`) are well-formed.
- No duplicate top-level sections.
- Identifiers are syntactically valid.

Parsing does NOT:

- Validate cross-references.
- Validate operation/signal relationships.
- Validate function existence.
- Perform semantic checks.

Parsing produces a structured `PipelineAst` (or `PipelineBlock`).

---

### Validation Phase

Validation operates on the parsed AST before indexing is finalized.

Validation ensures:

- `operations {}` block exists.
- Operation names are unique.
- Each operation contains at least one `do` statement.
- `do <step> output <binding>` is structurally valid.
- Output bindings are unique per operation.
- Mutable references to `Context` are disallowed.
- Signals reference existing operations.
- Signals reference existing steps.
- Clinch actions are structurally valid.
- Score operators (`+=`, `-=`, `*=`, `=`) are valid.
- Score values are numeric.

Validation does NOT:

- Check that referenced Rust functions exist.
- Resolve module paths.
- Perform type checking.

All Rust-level symbol validation is delegated to the Rust compiler during macro expansion.

---

## Operations Validation

File: `validate/operations.rs`

Validates:

- Presence of `operations` block.
- Unique operation names.
- Unique output bindings within an operation.
- At least one `do` statement per operation.
- No `&mut Context` references in operation bodies.

This phase enforces DSL-level invariants only.

---

## Signal Validation

Signal validation ensures:

- Referenced operations exist in the DSL.
- Referenced steps exist within those operations.

Rust function existence is not validated here.

---

## Clinch Validation

Clinch validation ensures:

- Each clause references a known signal.
- Actions (`emit`, `tag`, `score`, `run deferred`) are syntactically valid.
- Score operators are structurally correct.
- Score values parse as numeric types.

Clinch validation does not execute or interpret actions.

---

## Rust-Level Enforcement

After lowering, the Rust compiler enforces:

- Module existence (e.g., `crate::lstran_ops`).
- Function existence (e.g., `inspect_magic`).
- Type correctness.
- Borrowing rules.
- Lifetime rules.

The macro intentionally does not duplicate Rust’s symbol resolution.

This separation avoids shadow registries and maintains deterministic compile-time behavior.

---

## Architectural Guarantees

By maintaining strict phase boundaries:

- Parsing handles structure.
- Validation handles DSL semantics.
- Lowering emits deterministic code.
- Rust enforces symbol and type correctness.

This prevents validation logic from leaking into code generation and preserves long-term maintainability.

---

## Future Improvements

- Replace string-based keyword matching with `syn::custom_keyword!`.
- Improve error spans for multi-token operators.
- Transition away from token flattening toward structured AST validation.
- Expand cross-block semantic validation coverage.
- Improve diagnostic clarity for DSL misuse.

---

End of document.
