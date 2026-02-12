use crate::lunch::index::{
    common::StepInfo,
    operations::{OperationIndex, OperationInfo},
};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

pub fn lower_operations(ops: &OperationIndex, ops_path: &syn::Path) -> TokenStream2 {
    let mut lowered = Vec::new();

    for (op_name, op_info) in &ops.index {
        lowered.push(lower_operation(op_name, op_info, ops_path));
    }

    quote! { #(#lowered)* }
}

fn lower_operation(op_name: &str, op_info: &OperationInfo, ops_path: &syn::Path) -> TokenStream2 {
    let mut step_calls = Vec::new();

    for (step_name, step_info) in &op_info.steps {
        step_calls.push(crate::lunch::lowering::operations::lower_step(
            op_name, step_name, step_info, ops_path,
        ));
    }

    quote! {
        #(#step_calls)*
    }
}

pub fn lower_step(
    op_name: &str,
    step_name: &str,
    step_info: &StepInfo,
    ops_path: &syn::Path,
) -> TokenStream2 {
    let result_ident = format_ident!("__op_{}_step_{}", op_name, step_name);
    let fn_name = step_info.ops_fn.as_deref().unwrap_or(step_name);

    let fn_ident = format_ident!("{}", fn_name);

    quote! {
        let #result_ident = #ops_path::#fn_ident(artifact);
    }
}
