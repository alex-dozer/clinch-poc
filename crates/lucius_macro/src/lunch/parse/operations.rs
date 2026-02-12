// -------------------------------------------------------------------------
// Operations
// -------------------------------------------------------------------------

use proc_macro2::TokenStream as TokenStream2;
use syn::{
    Ident, braced,
    parse::{Parse, ParseStream, Result},
};

pub struct OperationBody {
    /// Raw DSL tokens inside `{ ... }`.
    pub content: TokenStream2,
}

impl Parse for OperationBody {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);
        let ts: TokenStream2 = content.parse()?;
        Ok(Self { content: ts })
    }
}

pub struct OperationDef {
    pub kw_operation: Ident,
    pub name: Ident,
    pub body: OperationBody,
}

impl Parse for OperationDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let kw_operation: Ident = input.parse()?;
        if kw_operation != "operation" {
            return Err(syn::Error::new_spanned(
                kw_operation,
                "expected `operation <name> { ... }`",
            ));
        }

        let name: Ident = input.parse()?;
        let body: OperationBody = input.parse()?;

        Ok(Self {
            kw_operation,
            name,
            body,
        })
    }
}

pub struct OperationsBlock {
    pub definitions: Vec<OperationDef>,
}

impl Parse for OperationsBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);

        let mut definitions = Vec::new();
        while !content.is_empty() {
            definitions.push(content.parse()?);
        }

        Ok(Self { definitions })
    }
}
