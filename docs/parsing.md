# Lunch Macro Parsing Flow --- Recursive Call Model

## Overview

This document explains how the `lunch!` procedural macro parses its
input using `syn::Parse` implementations and how recursive descent/section dispatch
parsing occurs implicitly through trait dispatch.

The key idea:

Parsing is initiated exactly once via `syn::parse2::<LunchGenInput>()`.\
From that point forward, parsing proceeds through a section dispatch/recurtsive descent through
`impl Parse` implementations for nested AST types.

---


## Where Parsing Actually Begins

Parsing starts here:

``` rust
syn::parse2::<LunchGenInput>(input)
```

Conceptually, this performs:

``` rust
LunchGenInput::parse(ParseStream)
```

The `Parse` trait implementation for `LunchGenInput` defines the
top-level grammar.

Then in `PipeLineAst` we have:

```rust
            match name.to_string().as_str() {
                "meta" => {
                    if meta.is_some() {
                        return Err(syn::Error::new_spanned(name, "duplicate `meta` block"));
                    }
                    let content;
                    braced!(content in input);
                    let ts: TokenStream2 = content.parse()?;
                    meta = Some(MetaBlock { name, content: ts });
                }
                "operations" => {
                    if operations.is_some() {
                        return Err(syn::Error::new_spanned(
                            name,
                            "duplicate `operations` block",
                        ));
                    }
                    operations = Some(input.parse()?);
                }
                "signals" => {
                    if signals.is_some() {
                        return Err(syn::Error::new_spanned(name, "duplicate `signals` block"));
                    }
                    signals = Some(input.parse()?);
                }
                "clinch" => {
                    if clinch.is_some() {
                        return Err(syn::Error::new_spanned(name, "duplicate `clinch` block"));
                    }
                    clinch = Some(input.parse()?);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        name,
                        "expected one of: meta, operations, signals, clinch",
                    ));
                }
            }
```

---

## Top-Level Parse Structure

### `LunchGenInput::parse`

Responsible for parsing:

    component = <ident> { ...pipeline... }

It performs:

1.  Parse `component`
2.  Parse `=`
3.  Parse component identifier
4.  Enter `{ ... }`
5.  Delegate parsing of the inner block to `PipelineAst`

Specifically:

``` rust
let pipeline: PipelineAst = content.parse()?;
```

This calls:

``` rust
PipelineAst::parse(content_stream)
```

---

## Important Distinction: Parsing vs Indexing

Parsing: - Converts token stream into structured AST. - Enforces
grammar. - Does not resolve cross-references.

Indexing (`validate_and_index`): - Validates references. - Builds
semantic graph. - Resolves bindings and relationships. - Produces
`PipelineIndex`.

Lowering: - Emits Rust code from the resolved `PipelineIndex`. -
Performs no semantic inference.

---

## Why This Feels Non-Intuitive

The call to `LunchGenInput::parse` is never written explicitly.

Instead, it is invoked via generic dispatch:

``` rust
syn::parse2::<LunchGenInput>(...)
```

This uses the `Parse` trait to dynamically select the correct parsing
implementation.

The recursion is implicit through nested `parse()` calls inside each AST
node.

---

## Mental Model Summary

-   Parsing defines grammar.
-   Indexing defines meaning.
-   Lowering defines execution.

Each phase must remain isolated.

The parsing process is simply structured delegation across AST
node boundaries using the `Parse` trait.


---

## Future Work

### 1. Replace String Matching with `syn::custom_keyword!`

Current parsing matches identifiers via string comparison:

```rust
match name.to_string().as_str()
```

This allows any identifier and rejects invalid ones manually.

Future improvement:
- Use syn::custom_keyword! to define grammar keywords explicitly.
- This strengthens the grammar contract.
- Removes string-based dispatch.
- Produces clearer compiler diagnostics.

### 2. Decide on Section Ordering Rules

Currently, section ordering (e.g., signals before operations) is likely allowed.

Future decision:
- Explicitly enforce ordering at parse time, or
- Clearly document that ordering is semantically irrelevant.

Ambiguity in ordering can lead to subtle semantic confusion later.

### 3. Clarify Required Section Validation Phase

Parsing currently enforces:
- No duplicate sections.

Indexing likely enforces:
- Required sections exist.
- Semantic correctness.

Future improvement:
- Explicitly document which phase guarantees what.
- Keep structural validation in parsing.
- Keep semantic validation in indexing.

### 4. Consider Enum-Based Section Representation

Instead of storing sections as individual `Option<T>` fields:

```rust
Option<MetaBlock>
Option<OperationsBlock>
Option<SignalsBlock>
Option<ClinchBlock>
```

Do:
```rust
enum Section {
    Meta(MetaBlock),
    Operations(OperationsBlock),
    Signals(SignalsBlock),
    Clinch(ClinchBlock),
}
```

Advantages:
- Cleaner extensibility.
- More uniform parsing logic.
- Improved future DSL evolution.

### 5. Improve Error Diagnostics

Enhancements may include:
- Using lookahead1() for clearer expected-token errors.
- Providing better span coverage for nested structures.
- Standardizing error messaging style.

Better error reporting significantly improves DSL usability.

### 6 Document Phase Invariants

Formalize guarantees for each phase.

Parse Phase Guarantees:
- Sections are structurally valid.
- No duplicate sections.
- No semantic validation performed.

Index Phase Guarantees:
- All cross-references resolved.
- All bindings attached.
- Graph is semantically coherent.
- Required sections validated.

Lowering Phase Guarantees:
- Emits deterministic Rust code.
- Performs no semantic validation.
- Consumes only the resolved PipelineIndex.

Maintaining strict phase separation prevents architectural drift.

