# Clinch POC

Clinch is a proof-of-concept compile-time DSL for structural artifact analysis built using the `lunch!` procedural macro.

This repository demonstrates deterministic pipeline generation, semantic validation, and context mutation without runtime interpretation or dynamic registration.

---

## Core Idea

The DSL allows you to define:

- **Meta** (eventual add, just meta about the pipeline)
- **Operations** (Rust functions)
- **Signals** (derived logical conditions)
- **Clinch rules** (context mutations: tags, emits, deferred actions, scores)

All logic is resolved at compile time.

No runtime engine.
No reflection.
No dynamic dispatch.

---

## Pipeline Architecture

The system is intentionally phase-separated:

1. **Parsing** - Convert DSL into structured blocks.
2. **Validation** - Enforce DSL semantic correctness.
3. **Indexing** - Resolve cross-references and build a coherent graph.
4. **Lowering** - Emit deterministic Rust code.
5. **Rust Compiler** - Enforce symbol resolution and type safety.

Each phase has a single responsibility.

---

## Example DSL

```rust
lunch! {

    component = lstran

    {

    operations {
        operation magic {
            do inspect_magic   output magic_probe
            do classify_format output format_probe
        }
    }

    signals {
        family format {
            signal pdf_magic {
                derive from operation.magic.inspect_magic
                    when magic_probe.magic == [0x25, 0x50, 0x44, 0x46]
            }
        }
    }

    clinch {
        when signal.format.pdf_magic {
            tag += "type:pdf"
            emit Emit::PdfMagic
            run deferred PdfMagicHandler
            score risk += 1.0
        }
    }

}
}
```

---

## What This POC Demonstrates

- Compile-time pipeline expansion
- Explicit operation binding via module paths
- Signal derivation from operation outputs
- Separation of emit vs deferred actions
- Deterministic context mutation
- Score accumulation logic
- Strict phase boundaries

---

## What This Is Not

This is not:

- A runtime rule engine
- A plugin system
- A dynamic registry
- A production ingestion pipeline

It is an architectural proof that structural analysis pipelines can be:

- Declarative
- Deterministic
- Compile-time verified
- Rust-native

---

## Operations

Operations live in:

```
lstran_ops.rs
```

They are plain Rust functions.  
The Rust module system acts as the operation registry.

If an operation function does not exist, the Rust compiler will emit an error during macro expansion.

---

## Running the POC

```bash
cargo run
```

The `main` function constructs a deterministic test `Artifact` and prints the resulting `LuciusContext`.

---

## Design Principles

- No shadow registries
- No runtime interpretation
- No implicit state
- Explicit module paths
- Rust enforces symbol existence
- Validation enforces DSL semantics

---

## Documentation

Additional architectural documentation:

- Validation Model
- Indexing Model
- Lowering Model

These documents explain the internal structure and reasoning behind the phase separation.

---

## Future Work

- Replace string keyword matching with `syn::custom_keyword!`
- Improve structured parsing (reduce token flattening)
- Strengthen diagnostics and error spans
- Add configurable ops path support
- Expand multi-artifact demonstration cases

---

This POC is intended as a foundation for further evolution of deterministic telemetry and structural analysis pipelines.
