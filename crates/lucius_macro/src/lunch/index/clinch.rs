use std::collections::HashMap;

use crate::{lunch::parse::clinch::ClinchBlock, lunch::validate::core_validate::flatten};
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use syn::Result;

#[derive(Debug)]
pub struct ClinchIndex {
    pub by_signal: HashMap<SignalId, Vec<ClinchAction>>,
}

impl ClinchIndex {
    pub fn new() -> Self {
        ClinchIndex {
            by_signal: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SignalId {
    pub family: String,
    pub name: String,
}

#[derive(Debug)]
pub enum ClinchAction {
    Emit {
        event: String,
    },
    Tag {
        key: String,
        value: String,
    },
    Score {
        key: String,
        operator: ScoreOp,
        value: f64,
    },
    RunDeferred {
        handler: String,
    },
}

#[derive(Debug)]
pub enum ScoreOp {
    Add,
    Sub,
    Mul,
    Set,
}

pub fn build_clinch_index(clinch: &ClinchBlock) -> Result<ClinchIndex> {
    let mut by_signal: HashMap<SignalId, Vec<ClinchAction>> = HashMap::new();

    for clause in &clinch.clauses {
        let signal = SignalId {
            family: clause.signal.family.to_string().clone(),
            name: clause.signal.name.to_string().clone(),
        };

        let actions = parse_clinch_actions(&clause.body.actions)?;
        by_signal.entry(signal).or_default().extend(actions);
    }

    Ok(ClinchIndex { by_signal })
}

fn parse_clinch_actions(ts: &TokenStream2) -> Result<Vec<ClinchAction>> {
    let mut tokens = Vec::new();
    flatten(ts, &mut tokens);

    let mut actions = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            TokenTree::Ident(ident) if ident == "emit" => {
                let (event, next_i) = parse_path(&tokens, i + 1)?;

                actions.push(ClinchAction::Emit { event });

                i = next_i;
            }

            TokenTree::Ident(ident) if ident == "tag" => {
                // expect: tag += <literal>
                if tokens.get(i + 1).map(|t| t.to_string()) != Some("+".into())
                    || tokens.get(i + 2).map(|t| t.to_string()) != Some("=".into())
                {
                    return Err(syn::Error::new_spanned(
                        tokens[i].clone(),
                        "expected `tag += <value>`",
                    ));
                }

                let value = tokens
                    .get(i + 3)
                    .ok_or_else(|| syn::Error::new_spanned(tokens[i].clone(), "missing tag value"))?
                    .to_string();

                actions.push(ClinchAction::Tag {
                    key: "tag".into(),
                    value,
                });

                i += 4;
            }

            TokenTree::Ident(ident) if ident == "score" => {
                let (op, value_index, advance_by) = match (tokens.get(i + 2), tokens.get(i + 3)) {
                    (Some(TokenTree::Punct(p1)), Some(TokenTree::Punct(p2)))
                        if p1.as_char() == '+' && p2.as_char() == '=' =>
                    {
                        (ScoreOp::Add, i + 4, 5)
                    }
                    (Some(TokenTree::Punct(p1)), Some(TokenTree::Punct(p2)))
                        if p1.as_char() == '-' && p2.as_char() == '=' =>
                    {
                        (ScoreOp::Sub, i + 4, 5)
                    }
                    (Some(TokenTree::Punct(p1)), Some(TokenTree::Punct(p2)))
                        if p1.as_char() == '*' && p2.as_char() == '=' =>
                    {
                        (ScoreOp::Mul, i + 4, 5)
                    }
                    (Some(TokenTree::Punct(p1)), _) if p1.as_char() == '=' => {
                        (ScoreOp::Set, i + 3, 4)
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(
                            tokens[i + 2].clone(),
                            "invalid score operator",
                        ));
                    }
                };

                let value_token = tokens.get(value_index).ok_or_else(|| {
                    syn::Error::new_spanned(tokens[i].clone(), "missing score value")
                })?;

                let value: f64 = value_token.to_string().parse().map_err(|_| {
                    syn::Error::new_spanned(value_token.clone(), "score value must be a number")
                })?;

                actions.push(ClinchAction::Score {
                    key: tokens[i + 1].to_string(),
                    operator: op,
                    value,
                });

                i += advance_by;
            }

            TokenTree::Ident(ident) if ident == "run" => {
                if tokens.get(i + 1).map(|t| t.to_string()) != Some("deferred".into()) {
                    return Err(syn::Error::new_spanned(
                        tokens[i + 1].clone(),
                        "expected `run deferred <Handler>`",
                    ));
                }

                let handler = tokens
                    .get(i + 2)
                    .ok_or_else(|| syn::Error::new_spanned(tokens[i].clone(), "missing handler"))?
                    .to_string();

                actions.push(ClinchAction::RunDeferred { handler });

                i += 3;
            }

            _ => {
                return Err(syn::Error::new_spanned(
                    tokens[i].clone(),
                    "invalid clinch action",
                ));
            }
        }
    }

    Ok(actions)
}

fn parse_path(tokens: &[TokenTree], start: usize) -> Result<(String, usize)> {
    let mut parts = Vec::new();
    let mut i = start;

    loop {
        match tokens.get(i) {
            Some(TokenTree::Ident(id))
                if id != "run" && id != "emit" && id != "tag" && id != "score" =>
            {
                parts.push(id.to_string());
                i += 1;
            }
            Some(TokenTree::Punct(p)) if p.as_char() == ':' => {
                // expect ::
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
        return Err(syn::Error::new_spanned(
            tokens[start].clone(),
            "expected path",
        ));
    }

    Ok((parts.join("::"), i))
}
