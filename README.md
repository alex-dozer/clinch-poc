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

## Crates

### Common

Just data objects necessary for `lunch!`.

### lstran

Pipeline-ish thing. No normalizing or anything. `lstan_ops.rs` is the ops file. It corresponds to the component that you list in `lunch!`. So if you do `component = moopy` you'd need a `moopy_ops.rs` file. Can be re-exports or whatever. The `main.rs` just runs the pipeline. The `ops` file is necessary to register operations. Haven't tested wasm yet, but I don't *foresee* issues. Though there could be issues.

### lucius_macro

Just the macro housing crate. It is where `lunch!` resides. Originally had more macros planned but got rid of them. So, it's just `lunch!` which is large enough.

The `meta` section isn't done. It would be simple enough, I'm just not 100% on exactly what to do with it yet. I do think it needs to exist. I could lower it into a function that attaches itself to the context at the end. Unsure....

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

Haven't tested on WASM yet but foresee no issues.

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

Future code will have a lot more comments. I just felt it was time to show it because there is always "1 more thing".

NOTE: You'll see `kw` places, that stands for keyword.

---

## Future Work

- Replace string keyword matching with `syn::custom_keyword!`
- Improve structured parsing (reduce token flattening)
- Strengthen diagnostics and error spans
- Add configurable ops path support
- Expand multi-artifact demonstration cases
- Need to work on better operation validation, or validation in general.
- Enforce enums for actions and emits. Though that may be cumbersome users. Unsure. Every macro adds complexity and I wonder if I'm already teetering at the edge of reasonable.

---

This POC is intended as a foundation for further evolution of deterministic telemetry and structural analysis pipelines.  It exists as proving that coding clinch would work.
