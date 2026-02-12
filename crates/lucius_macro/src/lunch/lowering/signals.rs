use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

use crate::lunch::index::{
    operations::OperationIndex,
    signals::{SignalIndex, SignalInfo},
};

pub fn lower_signals(index: &SignalIndex, ops: &OperationIndex) -> TokenStream2 {
    let mut lowered = Vec::new();

    for (family_name, family) in &index.families {
        for (signal_name, signal) in &family.signals {
            lowered.push(lower_signal(family_name, signal_name, signal, ops));
        }
    }

    quote! {
        // --- signals ---
        #(#lowered)*
    }
}

fn lower_signal(
    family_name: &str,
    signal_name: &str,
    sig: &SignalInfo,
    ops: &OperationIndex,
) -> TokenStream2 {
    let sig_ident = format_ident!("__signal_{}_{}", family_name, signal_name);

    let op_name = &sig.derives_from.operation;
    let step_name = &sig.derives_from.step;

    let step_result_ident = format_ident!("__op_{}_step_{}", op_name, step_name);

    let binding_name = ops
        .index
        .get(op_name)
        .and_then(|op| op.steps.get(step_name))
        .and_then(|step| Some(step.binding.clone()))
        .unwrap_or(step_name.to_string());

    let step_alias_ident = format_ident!("{}", binding_name);

    let when_tokens = &sig.when;

    quote! {
        let #step_alias_ident = &#step_result_ident;
        let #sig_ident: bool = { #when_tokens };
    }
}
