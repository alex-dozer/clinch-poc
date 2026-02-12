//use common::luop_descriptor::LuopDescriptor;
use proc_macro2::TokenStream as TokenStream2;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, Token, braced};

use crate::lunch::index::pipeline::PipelineIndex;
use crate::lunch::parse::clinch::ClinchBlock;
use crate::lunch::parse::operations::OperationsBlock;
use crate::lunch::parse::signals::SignalsBlock;
use crate::lunch::validate::clinch::validate_clinch;
use crate::lunch::validate::operations::validate_operations;
use crate::lunch::validate::signals::validate_signals;

// -------------------------------------------------------------------------
// Top-level blocks
// Important file
// -------------------------------------------------------------------------

pub struct MetaBlock {
    pub name: Ident,
    pub content: TokenStream2,
}

pub struct PipelineAst {
    pub meta: Option<MetaBlock>,
    pub operations: Option<OperationsBlock>,
    pub signals: Option<SignalsBlock>,
    pub clinch: Option<ClinchBlock>,
}

impl PipelineAst {
    /*

    Very important method

     */

    pub fn validate_and_index(&self) -> Result<PipelineIndex> {
        // 1. Parse already happened

        // 2. Validate operations (purely local)
        validate_operations(self)?;

        // 3. Build operation index
        let mut index = PipelineIndex::from_operations(self)?;

        // 4. Validate signals *against operation index*
        validate_signals(self, &index.operation_index)?;

        //validate_operations_against_luops(&mut index.operation_index, luops)?;

        // 5. Build signal index
        index.extend_with_signals(self)?;

        // 6. Validate clinch *against signal index*
        validate_clinch(self, &index.signal_index)?;

        // 7. Build clinch index
        index.extend_with_clinch(self)?;

        Ok(index)
    }
}
// -------------------------------------------------------------------------
// Pipeline parser: meta/operations/signals/clinch
// -------------------------------------------------------------------------

impl Parse for PipelineAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut meta: Option<MetaBlock> = None;
        let mut operations: Option<OperationsBlock> = None;
        let mut signals: Option<SignalsBlock> = None;
        let mut clinch: Option<ClinchBlock> = None;

        while !input.is_empty() {
            let name: Ident = input.parse()?;

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
        }

        Ok(Self {
            meta,
            operations,
            signals,
            clinch,
        })
    }
}

// -------------------------------------------------------------------------
// clinchgen!(component = <ident> { ... })
// -------------------------------------------------------------------------

pub struct LunchGenInput {
    pub component: Ident,
    pub pipeline: PipelineAst,
}

impl Parse for LunchGenInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        if key != "component" {
            return Err(syn::Error::new_spanned(
                key,
                "expected `component = <ident>`",
            ));
        }

        input.parse::<Token![=]>()?;
        let component: Ident = input.parse()?;

        let content;
        braced!(content in input);
        let pipeline: PipelineAst = content.parse()?;

        Ok(Self {
            component,
            pipeline,
        })
    }
}
