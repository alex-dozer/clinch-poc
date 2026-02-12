# What and Why We Have a Lowering Phase

## Overview

Lowering is the final macro-controlled phase in the `lunch!` pipeline architecture.

It transforms a validated and indexed semantic model into deterministic Rust code.

Pipeline flow:

1. Parse → structured syntax
2. Validate + Index → resolved semantic graph
3. **Lower → emit Rust code**
4. Rust compiler → enforce symbols and types

Lowering is where the DSL becomes executable.

---

## What Lowering Does

Lowering consumes resolved index structures such as:

- `OperationIndex`
- `SignalIndex`
- `ClinchIndex`

It generates concrete Rust code that:

- Calls operation functions
- Binds step outputs
- Evaluates signal conditions
- Mutates `LuciusContext`
- Pushes emits, tags, deferred handlers, and scores

Lowering produces plain Rust statements.

There is no interpretation at runtime.
There is no dynamic dispatch.
There is no reflection.

The result is static, readable Rust code.

---

## Why Lowering Exists

Without lowering, the DSL would need to:

- Be interpreted at runtime, or
- Generate behavior dynamically.

That would introduce:

- Runtime overhead
- Implicit state
- Hard-to-debug behavior
- Reduced compile-time guarantees

Lowering ensures:

- Deterministic execution
- Zero runtime DSL engine
- Compile-time enforcement
- Clear separation of concerns

---

## What Lowering Does NOT Do

Lowering does not:

- Validate DSL structure
- Resolve cross-references
- Check that operations exist
- Perform semantic inference

All semantic guarantees must already be satisfied during validation and indexing.

Lowering assumes correctness and emits code accordingly.

If lowering performs validation, phase boundaries are broken.

---

## Architectural Role

Lowering is the boundary between:

- DSL semantics
- Concrete Rust execution

It turns a resolved wiring diagram (indexing) into a compiled machine (Rust code).

After lowering, the Rust compiler enforces:

- Module existence
- Function existence
- Type correctness
- Borrow rules
- Lifetime rules

The macro does not duplicate Rust’s responsibilities.

---

## Determinism and Explicitness

Lowered code is:

- Fully explicit
- Path-qualified (e.g., `crate::lstran_ops::inspect_magic`)
- Deterministic in ordering
- Free of hidden runtime behavior

This makes debugging straightforward:

- Expand the macro
- Inspect generated Rust
- Trace execution normally

---

## Why Lowering Improves Maintainability

By isolating lowering:

- DSL semantics stay independent of execution details
- Validation remains focused
- Indexing remains declarative
- Generated code remains predictable

This prevents architectural drift and accidental coupling between phases.

---

## Mental Model

Parsing gives you structure.
Indexing gives you meaning.
Lowering gives you execution.

Lowering is where intention becomes concrete behavior.

---

## Future Improvements

- Improve span information in generated code
- Refine deterministic ordering guarantees
- Expand structured lowering patterns
- Add optional debug expansion helpers
