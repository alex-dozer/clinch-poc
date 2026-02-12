mod expanders;
mod lunch;

use crate::{
    expanders::lunch_expander,
    lunch::{index::pipeline::PipelineIndex, parse::pipeline::LunchGenInput},
};
use proc_macro::TokenStream;

#[proc_macro]
pub fn lunch(input: TokenStream) -> TokenStream {
    let (parsed, index) = match lunch_expander(input.into()) {
        Ok((p, i)) => (p, i),
        Err(e) => return e.to_compile_error().into(),
    };

    match lunch::codegen::generate(&parsed.pipeline, &index, parsed.component) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
