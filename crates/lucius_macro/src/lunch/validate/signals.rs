use std::collections::HashMap;

use crate::lunch::{
    index::operations::{OperationIndex, OperationInfo},
    parse::{
        pipeline::PipelineAst,
        signals::{SignalDef, SignalsBlock},
    },
    validate::core_validate::{flatten, validate_unique_names},
};
use proc_macro2::TokenTree;
use syn::{Error, Result};

pub fn validate_signals(ast: &PipelineAst, op_index: &OperationIndex) -> Result<()> {
    let signals_block = ast.signals.as_ref().ok_or_else(|| {
        Error::new(
            proc_macro2::Span::call_site(),
            "missing `signals { ... }` block",
        )
    })?;

    validate_unique_names(
        signals_block.families.iter().map(|f| f.name.clone()),
        "signal family",
    )?;

    validate_signal_name_unique(signals_block)?;
    validate_family_non_empty(signals_block)?;

    for family in &signals_block.families {
        for signal in &family.signals {
            validate_single_when(signal)?;
            validate_derive_from(signal, &op_index.index)?;
            validate_ops_exist(signal, &op_index.index)?;
            validate_when_references(signal, &op_index.index)?;
        }
    }

    Ok(())
}

fn validate_signal_name_unique(signals_block: &SignalsBlock) -> Result<()> {
    for family in &signals_block.families {
        validate_unique_names(family.signals.iter().map(|sig| sig.name.clone()), "signal")?;
    }
    Ok(())
}

fn validate_single_when(signal: &SignalDef) -> Result<()> {
    let mut tokens = Vec::new();
    flatten(&signal.body.when, &mut tokens);

    if tokens.is_empty() {
        return Err(Error::new_spanned(
            signal.body.when.clone(),
            "empty `when` condition in signal definition",
        ));
    }

    Ok(())
}

fn validate_family_non_empty(signals_block: &SignalsBlock) -> Result<()> {
    for family in &signals_block.families {
        if family.signals.is_empty() {
            return Err(Error::new_spanned(
                family.name.clone(),
                "signal family must contain at least one signal",
            ));
        }
    }
    Ok(())
}

fn validate_ops_exist(signal: &SignalDef, op_index: &HashMap<String, OperationInfo>) -> Result<()> {
    let op_name = signal.body.derive_from.operation.to_string();
    let step_name = signal.body.derive_from.step.to_string();

    // 1. Operation exists
    let op = op_index.get(&op_name).ok_or_else(|| {
        Error::new_spanned(
            &signal.body.derive_from.operation,
            format!("operation `{}` does not exist", op_name),
        )
    })?;

    // 2. Step exists
    let step = op.steps.get(&step_name).ok_or_else(|| {
        Error::new_spanned(
            &signal.body.derive_from.step,
            format!(
                "step `{}` does not exist on operation `{}`",
                step_name, op_name
            ),
        )
    })?;

    // 3. Step declares exactly one binding (moop)
    if step.binding.is_empty() {
        return Err(Error::new_spanned(
            &signal.body.derive_from.step,
            "derived step declares no output binding",
        ));
    }

    Ok(())
}

fn validate_derive_from(
    signal: &SignalDef,
    op_index: &HashMap<String, OperationInfo>,
) -> Result<()> {
    let op_name = signal.body.derive_from.operation.to_string();
    let step_name = signal.body.derive_from.step.to_string();

    let op = op_index.get(&op_name).ok_or_else(|| {
        Error::new_spanned(
            &signal.body.derive_from.operation,
            format!("operation `{}` does not exist", op_name),
        )
    })?;

    if !op.steps.contains_key(&step_name) {
        return Err(Error::new_spanned(
            &signal.body.derive_from.step,
            format!(
                "step `{}` does not exist on operation `{}`",
                step_name, op_name
            ),
        ));
    }

    Ok(())
}

use proc_macro2::Delimiter;
fn validate_when_references(
    signal: &SignalDef,
    op_index: &HashMap<String, OperationInfo>,
) -> Result<()> {
    let op_name = signal.body.derive_from.operation.to_string();
    let step_name = signal.body.derive_from.step.to_string();

    let op = op_index.get(&op_name).ok_or_else(|| {
        Error::new_spanned(
            &signal.body.derive_from.operation,
            "derived operation does not exist",
        )
    })?;

    let step = op.steps.get(&step_name).ok_or_else(|| {
        Error::new_spanned(&signal.body.derive_from.step, "derived step does not exist")
    })?;

    // The ONLY allowed binding name
    let binding = &step.binding;

    let mut tokens = Vec::new();
    flatten(&signal.body.when, &mut tokens);

    let mut i = 0;
    while i + 1 < tokens.len() {
        if let TokenTree::Ident(ident) = &tokens[i] {
            // Look for `<ident>.`
            if let TokenTree::Punct(p) = &tokens[i + 1] {
                if p.as_char() == '.' {
                    let name = ident.to_string();

                    if name != *binding {
                        return Err(Error::new_spanned(
                            ident.clone(),
                            format!(
                                "unknown binding `{}` in `when`; expected `{}`",
                                name, binding
                            ),
                        ));
                    }
                }
            }
        }

        i += 1;
    }

    Ok(())
}
