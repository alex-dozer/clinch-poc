use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

use crate::lunch::index::pipeline::PipelineIndex;
use crate::lunch::lowering::{
    clinch::lower_clinch, operations::lower_operations, signals::lower_signals,
};
use syn::parse_quote;

pub fn lower_pipeline(index: &PipelineIndex, component: &str) -> TokenStream2 {
    let fn_ident = format_ident!("run_{}_pipeline", component);


    let ops_crate_ident = format_ident!("{}_ops", component);
    let ops_path: syn::Path = syn::parse_quote! {
        crate::#ops_crate_ident
    };

    let ops = lower_operations(&index.operation_index, &ops_path);
    let signals = lower_signals(&index.signal_index, &index.operation_index);
    let clinch = lower_clinch(&index.clinch_index);

    quote! {
        #[allow(non_snake_case)]
        pub fn #fn_ident(
            artifact: &Artifact,
        ) -> LuciusContext{
            let mut ctx = LuciusContext::new();

            // --- operations ---
            #ops

            // --- signals ---
            #signals

            // --- clinch ---
            #clinch

            ctx
        }
    }
}
