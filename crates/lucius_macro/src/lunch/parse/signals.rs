use proc_macro2::TokenStream as TokenStream2;
use syn::{
    Ident, braced,
    parse::{self, Parse, ParseStream, Result},
};

pub struct SignalsBlock {
    pub families: Vec<SignalFamily>,
}

pub struct SignalFamily {
    pub name: Ident,
    pub signals: Vec<SignalDef>,
}

pub struct SignalDef {
    pub name: Ident,
    pub body: SignalBody,
}

pub struct SignalBody {
    pub derive_from: DeriveFrom,
    pub when: TokenStream2,
}

pub struct DeriveFrom {
    pub operation: Ident,
    pub step: Ident,
}

impl Parse for SignalsBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);

        let mut families = Vec::new();

        while !content.is_empty() {
            let kw_family: Ident = content.parse()?;
            if kw_family != "family" {
                return Err(syn::Error::new_spanned(
                    kw_family,
                    "expected `family <name> { ... }`",
                ));
            }

            let name: Ident = content.parse()?;

            let family_content;
            braced!(family_content in content);

            let mut signals = Vec::new();
            while !family_content.is_empty() {
                signals.push(family_content.parse()?);
            }

            families.push(SignalFamily { name, signals });
        }

        Ok(Self { families })
    }
}

impl Parse for SignalDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let kw_signal: Ident = input.parse()?;
        if kw_signal != "signal" {
            return Err(syn::Error::new_spanned(
                kw_signal,
                "expected `signal <name> { ... }`",
            ));
        }

        let name: Ident = input.parse()?;
        let body: SignalBody = input.parse()?;

        Ok(Self { name, body })
    }
}

impl Parse for SignalBody {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);

        let mut derive_from = None;
        let mut when = None;

        while !content.is_empty() {
            let ident: Ident = content.parse()?;

            match ident.to_string().as_str() {
                "derive" => {
                    if derive_from.is_some() {
                        return Err(syn::Error::new_spanned(ident, "duplicate `derive from`"));
                    }
                    derive_from = Some(content.parse()?);
                }
                "when" => {
                    if when.is_some() {
                        return Err(syn::Error::new_spanned(ident, "duplicate `when` clause"));
                    }
                    let ts: TokenStream2 = content.parse()?;
                    when = Some(ts);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ident,
                        "unexpected token in signal body",
                    ));
                }
            }
        }

        Ok(Self {
            derive_from: derive_from.ok_or_else(|| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "signal must declare `derive from`",
                )
            })?,
            when: when.ok_or_else(|| {
                syn::Error::new(proc_macro2::Span::call_site(), "signal must declare `when`")
            })?,
        })
    }
}

impl Parse for DeriveFrom {
    fn parse(input: ParseStream) -> Result<Self> {
        let kw_from: Ident = input.parse()?;
        if kw_from != "from" {
            return Err(syn::Error::new_spanned(
                kw_from,
                "expected `from operation.<op>.<step>`",
            ));
        }

        let kw_operation: Ident = input.parse()?;
        if kw_operation != "operation" {
            return Err(syn::Error::new_spanned(
                kw_operation,
                "expected `operation`",
            ));
        }

        input.parse::<syn::Token![.]>()?;
        let operation: Ident = input.parse()?;
        input.parse::<syn::Token![.]>()?;
        let step: Ident = input.parse()?;

        Ok(Self { operation, step })
    }
}
