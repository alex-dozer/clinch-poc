use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

use crate::lunch::index::clinch::{ClinchAction, ClinchIndex, ScoreOp};

pub fn lower_clinch(index: &ClinchIndex) -> TokenStream2 {
    let mut blocks = Vec::new();

    for (signal_id, actions) in &index.by_signal {
        blocks.push(lower_clinch_clause(
            &signal_id.family,
            &signal_id.name,
            actions,
        ));
    }

    quote! {
        #(#blocks)*
    }
}

fn lower_clinch_clause(family: &str, name: &str, actions: &[ClinchAction]) -> TokenStream2 {
    let sig_ident = format_ident!("__signal_{}_{}", family, name);

    let lowered_actions: Vec<TokenStream2> = actions.iter().map(lower_action).collect();

    quote! {
        if #sig_ident {
            #(#lowered_actions)*
        }
    }
}

fn lower_action(action: &ClinchAction) -> TokenStream2 {
    match action {
        ClinchAction::Tag { key, value } => {
            let tag = format!("{}:{}", key, value);
            quote! {
                ctx.tags.push(#tag.to_string());
            }
        }

        ClinchAction::Emit { event } => {
            quote! {
                ctx.emits.push(#event.to_string());
            }
        }

        ClinchAction::RunDeferred { handler } => {
            quote! {
                ctx.deferred.push(#handler.to_string());
            }
        }

        ClinchAction::Score {
            key,
            operator,
            value,
        } => {
            let v = *value;
            match operator {
                ScoreOp::Add => quote! {
                    *ctx.scores.entry(#key.to_string()).or_insert(0.0) += #v;
                },
                ScoreOp::Sub => quote! {
                    *ctx.scores.entry(#key.to_string()).or_insert(0.0) -= #v;
                },
                ScoreOp::Mul => quote! {
                    *ctx.scores.entry(#key.to_string()).or_insert(0.0) *= #v;
                },
                ScoreOp::Set => quote! {
                    ctx.scores.insert(#key.to_string(), #v);
                },
            }
        }
    }
}
