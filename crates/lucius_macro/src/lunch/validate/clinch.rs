use crate::lunch::index::signals::{FamilyInfo, SignalIndex};

use crate::{
    lunch::parse::{
        clinch::{ClinchBody, SignalPath},
        pipeline::PipelineAst,
    },
    lunch::validate::core_validate::flatten,
};
use proc_macro2::TokenTree;
use syn::{Error, Result};

pub fn validate_clinch(ast: &PipelineAst, signal_index: &SignalIndex) -> Result<()> {
    let clinch_block = ast.clinch.as_ref().ok_or_else(|| {
        Error::new(
            proc_macro2::Span::call_site(),
            "missing `clinch { ... }` block",
        )
    })?;

    for clause in &clinch_block.clauses {
        validate_signal_path(&clause.signal, signal_index)?;
        validate_actions_non_empty(&clause.body)?;
        validate_actions(&clause.body)?;
    }

    Ok(())
}

fn validate_signal_path(signal: &SignalPath, index: &SignalIndex) -> Result<()> {
    let family = index
        .families
        .get(&signal.family.to_string())
        .ok_or_else(|| Error::new_spanned(&signal.family, "unknown signal family"))?;

    if !family.signals.contains_key(&signal.name.to_string()) {
        return Err(Error::new_spanned(&signal.name, "unknown signal in family"));
    }

    Ok(())
}

fn validate_actions_non_empty(body: &ClinchBody) -> Result<()> {
    let mut tokens = Vec::new();
    flatten(&body.actions, &mut tokens);

    if tokens.is_empty() {
        return Err(Error::new_spanned(
            &body.actions,
            "clinch body must contain at least one action",
        ));
    }

    Ok(())
}

fn validate_actions(body: &ClinchBody) -> Result<()> {
    let mut tokens = Vec::new();
    flatten(&body.actions, &mut tokens);

    let mut i = 0;

    while i < tokens.len() {
        if let TokenTree::Ident(ident) = &tokens[i] {
            match ident.to_string().as_str() {
                "emit" => {
                    let (event_path, next_i) = parse_path(&tokens, i + 1)?;

                    if event_path.is_empty() {
                        return Err(Error::new_spanned(
                            ident.clone(),
                            "expected event name after `emit`",
                        ));
                    }

                    i = next_i;
                }
                "run" => {
                    // expect: run deferred Handler
                    if tokens.get(i + 1).map(|t| t.to_string()) != Some("deferred".into()) {
                        return Err(Error::new_spanned(
                            ident.clone(),
                            "expected `deferred` after `run`",
                        ));
                    }

                    i += 3; // consume `run deferred Handler`
                }
                "tag" => {
                    // expect: tag += <value>
                    if tokens.get(i + 1).map(|t| t.to_string()) != Some("+".into())
                        || tokens.get(i + 2).map(|t| t.to_string()) != Some("=".into())
                    {
                        return Err(Error::new_spanned(
                            ident.clone(),
                            "expected `tag += <value>`",
                        ));
                    }

                    if tokens.get(i + 3).is_none() {
                        return Err(Error::new_spanned(
                            ident.clone(),
                            "expected value after `tag +=`",
                        ));
                    }

                    i += 4;
                }
                "score" => {
                    // expect: score <key> <op> <number>
                    let key = match tokens.get(i + 1) {
                        Some(TokenTree::Ident(id)) => id,
                        _ => {
                            return Err(Error::new_spanned(
                                ident.clone(),
                                "expected score key after `score`",
                            ));
                        }
                    };

                    let op = match (tokens.get(i + 2), tokens.get(i + 3)) {
                        (Some(TokenTree::Punct(a)), Some(TokenTree::Punct(b)))
                            if format!("{}{}", a.as_char(), b.as_char()) == "+="
                                || format!("{}{}", a.as_char(), b.as_char()) == "-="
                                || format!("{}{}", a.as_char(), b.as_char()) == "*=" =>
                        {
                            2 // consume both puncts
                        }
                        (Some(TokenTree::Punct(eq)), _) if eq.as_char() == '=' => 1,
                        _ => {
                            return Err(Error::new_spanned(
                                ident.clone(),
                                "expected score operator (`=`, `+=`, `-=`, `*=`)",
                            ));
                        }
                    };

                    match tokens.get(i + 2 + op) {
                        Some(TokenTree::Literal(_)) => {}
                        _ => {
                            return Err(Error::new_spanned(
                                key.clone(),
                                "expected numeric literal after score operator",
                            ));
                        }
                    }

                    i += 3 + op;
                }
                _ => {
                    return Err(Error::new_spanned(
                        ident.clone(),
                        "unknown action in clinch body",
                    ));
                }
            }
        } else {
            i += 1;
        }
    }

    Ok(())
}

fn parse_path(tokens: &[TokenTree], start: usize) -> Result<(String, usize)> {
    let mut parts: Vec<String> = Vec::new();
    let mut i = start;

    loop {
        match tokens.get(i) {
            Some(TokenTree::Ident(id)) => {
                parts.push(id.to_string());
                i += 1;
            }
            Some(TokenTree::Punct(p)) if p.as_char() == ':' => {
                // expect '::'
                if matches!(
                    tokens.get(i + 1),
                    Some(TokenTree::Punct(p2)) if p2.as_char() == ':'
                ) {
                    i += 2;
                } else {
                    break;
                }
            }
            _ => break,
        }
    }

    if parts.is_empty() {
        return Err(Error::new_spanned(
            tokens.get(start).cloned().unwrap_or_else(|| {
                TokenTree::Ident(proc_macro2::Ident::new(
                    "emit",
                    proc_macro2::Span::call_site(),
                ))
            }),
            "expected a valid path (e.g. Emit::PdfMagic)",
        ));
    }

    Ok((parts.join("::"), i))
}
