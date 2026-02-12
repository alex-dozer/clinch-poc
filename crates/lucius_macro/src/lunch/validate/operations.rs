use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use std::collections::HashSet;
use syn::{Error, Result};

use crate::lunch::parse::{operations::OperationsBlock, pipeline::PipelineAst};
use crate::lunch::validate::core_validate::{
    flatten, validate_common_block_rules, validate_unique_names,
};

pub fn validate_operations(ast: &PipelineAst) -> Result<()> {
    let ops = ast.operations.as_ref().ok_or_else(|| {
        Error::new(
            proc_macro2::Span::call_site(),
            "missing `operations { ... }` block",
        )
    })?;

    validate_unique_names(
        ops.definitions.iter().map(|op| op.name.clone()),
        "operation",
    )?;

    validate_operation_bodies(ops)?;

    Ok(())
}

fn validate_operation_bodies(ops: &OperationsBlock) -> Result<()> {
    for op in &ops.definitions {
        validate_common_block_rules(&op.body.content)?;
        validate_no_mut_context(&op.body.content)?; //abstract later
        validate_unique_outputs(&op.body.content)?;
        validate_do_statements(&op.body.content)?;
    }

    Ok(())
}

fn validate_no_mut_context(body: &TokenStream2) -> Result<()> {
    let mut tokens = Vec::new();
    flatten(body, &mut tokens);

    for window in tokens.windows(3) {
        let is_amp = matches!(window[0], TokenTree::Punct(ref p) if p.as_char() == '&');
        let is_mut = matches!(window[1], TokenTree::Ident(ref i) if i == "mut");
        let is_context = matches!(window[2], TokenTree::Ident(ref i) if i == "Context");

        if is_amp && is_mut && is_context {
            return Err(Error::new_spanned(
                body.clone(),
                "mutable references to Context are not allowed in operations",
            ));
        }
    }
    Ok(())
}

fn validate_unique_outputs(body: &TokenStream2) -> Result<()> {
    let mut tokens = Vec::new();
    flatten(body, &mut tokens);

    let mut outputs = HashSet::new();

    let mut i = 0;
    while i < tokens.len() {
        if let TokenTree::Ident(ref ident) = tokens[i] {
            if ident == "output" {
                if let Some(TokenTree::Ident(out_ident)) = tokens.get(i + 1) {
                    let out_name = out_ident.to_string();
                    if !outputs.insert(out_name.clone()) {
                        return Err(Error::new_spanned(
                            out_ident.clone(),
                            format!("duplicate output name `{}`", out_name),
                        ));
                    }
                    i += 2;
                    continue;
                } else {
                    return Err(Error::new_spanned(
                        tokens[i].clone(),
                        "expected output name after `output`",
                    ));
                }
            }
        }
        i += 1;
    }

    Ok(())
}

fn validate_do_statements(body: &TokenStream2) -> Result<()> {
    let mut tokens = Vec::new();
    flatten(body, &mut tokens);

    let mut i = 0;
    let mut saw_do = false;

    while i < tokens.len() {
        match &tokens[i] {
            TokenTree::Ident(ident) if ident == "do" => {
                saw_do = true;

                // do <ident> output <ident>
                let step = tokens.get(i + 1);
                let output_kw = tokens.get(i + 2);
                let output_name = tokens.get(i + 3);

                match (step, output_kw, output_name) {
                    (
                        Some(TokenTree::Ident(_step)),
                        Some(TokenTree::Ident(output)),
                        Some(TokenTree::Ident(_out)),
                    ) if output == "output" => {
                        i += 4;
                    }

                    _ => {
                        return Err(Error::new_spanned(
                            tokens[i].clone(),
                            "expected `do <step> output <name>`",
                        ));
                    }
                }
            }
            _ => i += 1,
        }
    }

    if !saw_do {
        return Err(Error::new_spanned(
            body.clone(),
            "operations must declare at least one `do` statement",
        ));
    }

    Ok(())
}
