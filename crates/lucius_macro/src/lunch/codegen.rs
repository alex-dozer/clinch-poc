use proc_macro2::TokenStream as TokenStream2;
use syn::Ident;

use crate::lunch::{
    index::pipeline::PipelineIndex, lowering::pipeline::lower_pipeline,
    parse::pipeline::PipelineAst,
};

pub fn generate(
    _ast: &PipelineAst,
    index: &PipelineIndex,
    component: Ident,
) -> syn::Result<TokenStream2> {
    let lowered = lower_pipeline(index, &component.to_string());

    Ok(lowered)
}
