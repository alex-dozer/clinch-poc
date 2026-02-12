use proc_macro2::TokenStream as TokenStream2;
use syn::{
    Ident, Token, braced,
    parse::{Parse, ParseStream, Result},
};

#[derive(Debug)]
pub struct ClinchBlock {
    pub clauses: Vec<ClinchClause>,
}

#[derive(Debug)]
pub struct ClinchClause {
    pub signal: SignalPath,
    pub body: ClinchBody,
}

#[derive(Debug)]
pub struct SignalPath {
    pub family: Ident,
    pub name: Ident,
}

#[derive(Debug)]
pub struct ClinchBody {
    pub actions: TokenStream2,
}

impl Parse for ClinchBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);

        let mut clauses = Vec::new();

        while !content.is_empty() {
            // `when`
            let kw_when: Ident = content.parse()?;
            if kw_when != "when" {
                return Err(syn::Error::new_spanned(
                    kw_when,
                    "expected `when signal.<family>.<name> { ... }`",
                ));
            }

            // `signal`
            let kw_signal: Ident = content.parse()?;
            if kw_signal != "signal" {
                return Err(syn::Error::new_spanned(
                    kw_signal,
                    "expected `signal` after `when`",
                ));
            }

            // `.`
            content.parse::<Token![.]>()?;

            // `<family>`
            let family: Ident = content.parse()?;

            // `.`
            content.parse::<Token![.]>()?;

            // `<signal>`
            let name: Ident = content.parse()?;

            // `{ ... }`
            let body_content;
            braced!(body_content in content);
            let actions: TokenStream2 = body_content.parse()?;

            clauses.push(ClinchClause {
                signal: SignalPath {
                    family: family,
                    name: name,
                },
                body: ClinchBody { actions },
            });
        }

        Ok(Self { clauses })
    }
}

impl Parse for ClinchClause {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse `when`
        let kw_when: Ident = input.parse()?;
        if kw_when != "when" {
            return Err(syn::Error::new_spanned(
                kw_when,
                "expected `when signal.<family>.<name> { ... }`",
            ));
        }

        // Parse `signal.<family>.<name>`
        let signal: SignalPath = input.parse()?;

        // Parse `{ ... }`
        let body: ClinchBody = input.parse()?;

        Ok(Self { signal, body })
    }
}

impl Parse for SignalPath {
    fn parse(input: ParseStream) -> Result<Self> {
        let kw_signal: Ident = input.parse()?;
        if kw_signal != "signal" {
            return Err(syn::Error::new_spanned(
                kw_signal,
                "expected `signal.<family>.<name>`",
            ));
        }

        input.parse::<syn::Token![.]>()?;
        let family: Ident = input.parse()?;

        input.parse::<syn::Token![.]>()?;
        let name: Ident = input.parse()?;

        Ok(Self { family, name })
    }
}

impl Parse for ClinchBody {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        syn::braced!(content in input);
        let actions: TokenStream2 = content.parse()?;

        Ok(Self { actions })
    }
}
