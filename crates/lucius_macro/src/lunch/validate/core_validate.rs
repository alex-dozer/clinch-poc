use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use std::collections::HashSet;
use syn::{Error, Result};

pub fn validate_common_block_rules(body: &TokenStream2) -> Result<()> {
    validate_no_loops(body)?;
    validate_no_threading(body)?;
    Ok(())
}
fn validate_no_loops(body: &TokenStream2) -> Result<()> {
    let mut tokens = Vec::new();
    flatten(body, &mut tokens);

    for tt in tokens {
        if let TokenTree::Ident(ref ident) = tt {
            let ident_str = ident.to_string();
            if ident_str == "for" || ident_str == "while" || ident_str == "loop" {
                return Err(Error::new_spanned(
                    TokenTree::Ident(ident.clone()),
                    "loops are not allowed in operations",
                ));
            }
        }
    }
    Ok(())
}

pub fn validate_unique_names<I>(items: I, kind: &str) -> Result<()>
where
    I: IntoIterator<Item = syn::Ident>,
{
    let mut seen = HashSet::new();

    for ident in items {
        let name = ident.to_string();
        if !seen.insert(name.clone()) {
            return Err(Error::new_spanned(
                ident,
                format!("duplicate {} name `{}`", kind, name),
            ));
        }
    }

    Ok(())
}
fn validate_no_threading(body: &TokenStream2) -> Result<()> {
    let mut tokens = Vec::new();
    flatten(body, &mut tokens);

    for tt in tokens {
        if let TokenTree::Ident(ref ident) = tt {
            let ident_str = ident.to_string();
            if ident_str == "thread" || ident_str == "spawn" {
                return Err(Error::new_spanned(
                    TokenTree::Ident(ident.clone()),
                    "threading is not allowed in operations",
                ));
            }
        }
    }
    Ok(())
}

pub fn flatten(ts: &TokenStream2, out: &mut Vec<TokenTree>) {
    for tt in ts.clone() {
        match tt {
            TokenTree::Group(g) => {
                flatten(&g.stream(), out);
            }
            other => out.push(other),
        }
    }
}
