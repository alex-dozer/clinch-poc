# What and Why We Index

## Overview

Indexing is the semantic resolution phase that occurs after parsing and before lowering in the `lunch!` macro pipeline.

It transforms parsed DSL structure into a resolved, validated graph that can be deterministically lowered into Rust code.

Pipeline flow:

1. Parse -> structured blocks
2. **Index -> resolved semantic graph**
3. Lower -> emit Rust
4. Rust compiler → enforce symbols and types

Indexing is the phase where meaning is established.

---

## What Indexing Does

Indexing consumes the parsed `PipelineAst` and produces structured index types such as:

- `OperationIndex`
- `SignalIndex`
- `ClinchIndex`

These indices:

- Map operation names to their step definitions
- Map signals to their derived operation outputs
- Map signals to Clinch actions
- Resolve cross-references
- Attach validated bindings

Indexing converts loose structure into a resolved model that lowering can trust.

---

## Why Indexing Exists

Parsing only ensures grammar correctness.

Example:

```
signal pdf_magic {
    derive from operation.magic.inspect_magic
}
```

Parsing confirms that syntax is valid.

It does NOT confirm:

- That `magic` exists as an operation
- That `inspect_magic` is a declared step
- That the referenced output binding exists

Indexing performs these semantic checks.

Without indexing:

- Lowering would need to validate relationships
- Errors would surface late and unclearly
- Phase boundaries would collapse

Indexing isolates semantic validation cleanly.

---

## Indexing Responsibilities

Indexing ensures:

- Referenced operations exist in the DSL
- Referenced steps exist within operations
- Output bindings are valid
- Signal references are resolvable
- Clinch clauses map to known signals

After indexing, the graph is coherent.

Lowering no longer performs semantic checks.

---

## What Indexing Does NOT Do

Indexing does not:

- Resolve Rust function paths
- Check module existence
- Validate types
- Execute code

Rust itself performs symbol resolution during compilation.

Indexing remains strictly DSL-scoped.

---

## Architectural Role

Indexing is the boundary between:

- **Structure (parse phase)**
- **Execution (lower phase)**

It prevents semantic drift by enforcing:

- Explicit contracts
- Deterministic relationships
- Clean separation of concerns

Without indexing, the macro would either:

- Duplicate Rust’s symbol resolution, or
- Defer semantic errors until lowering

Both approaches weaken architecture.

Indexing keeps the system disciplined.

---

## Mental Model

Think of indexing as building a verified wiring diagram.

Parsing gives you labeled components.

Indexing connects them correctly.

Lowering then generates the machine based on that wiring.

---

## Future Improvements

- Expand index-level diagnostics
- Strengthen cross-family validation
- Move from token flattening to structured node resolution
- Improve error span precision

---

End of document.
