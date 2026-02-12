use crate::{LunchGenInput, PipelineIndex};
use proc_macro2::TokenStream as TokenStream2;

pub fn lunch_expander(input: TokenStream2) -> syn::Result<(LunchGenInput, PipelineIndex)> {
    let parsed = match syn::parse2::<LunchGenInput>(input) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let index = match parsed.pipeline.validate_and_index() {
        Ok(i) => i,
        Err(e) => return Err(e),
    };

    Ok((parsed, index))
}
