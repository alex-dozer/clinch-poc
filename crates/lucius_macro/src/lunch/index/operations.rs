use core::str;
use std::collections::{HashMap, HashSet};
use syn::{Error, Result};

use crate::{
    lunch::index::common::StepInfo,
    lunch::parse::{operations::OperationsBlock, pipeline::PipelineAst, signals::SignalsBlock},
};

// #[derive(Debug)]
// pub struct PipelineIndex {
//     pub operations: HashMap<String, OperationInfo>,
// }

#[derive(Debug)]
pub struct OperationIndex {
    pub index: HashMap<String, OperationInfo>,
}

#[derive(Debug)]
pub struct OperationInfo {
    pub steps: HashMap<String, StepInfo>,
}

pub fn build_operation_index(
    operations_block: &OperationsBlock,
) -> Result<HashMap<String, OperationInfo>> {
    let mut operations = HashMap::new();

    for op in &operations_block.definitions {
        let op_name = op.name.to_string();

        let mut steps = HashMap::new();

        // Reuse your existing flattening + scanning logic
        let mut tokens = Vec::new();
        crate::lunch::validate::core_validate::flatten(&op.body.content, &mut tokens);

        let mut i = 0;
        while i < tokens.len() {
            if let proc_macro2::TokenTree::Ident(ident) = &tokens[i] {
                if ident == "do" {
                    let step = match tokens.get(i + 1) {
                        Some(proc_macro2::TokenTree::Ident(id)) => id.to_string(),
                        _ => {
                            return Err(Error::new_spanned(
                                ident.clone(),
                                "expected step name after `do`",
                            ));
                        }
                    };

                    let output_kw = tokens.get(i + 2);
                    let output = match tokens.get(i + 3) {
                        Some(proc_macro2::TokenTree::Ident(id)) => id.to_string(),
                        _ => {
                            return Err(Error::new_spanned(
                                ident.clone(),
                                "expected output name after `output`",
                            ));
                        }
                    };

                    if !matches!(
                        output_kw,
                        Some(proc_macro2::TokenTree::Ident(kw)) if kw == "output"
                    ) {
                        return Err(Error::new_spanned(
                            ident.clone(),
                            "expected `do <step> output <name>`",
                        ));
                    }

                    if steps.contains_key(&step) {
                        return Err(Error::new_spanned(
                            ident.clone(),
                            format!("step `{}` already declared", step),
                        ));
                    }

                    steps.insert(
                        step.clone(),
                        StepInfo {
                            binding: output,             // ← `moop`
                            luop_fn: Some(step.clone()), // ← function name
                        },
                    );

                    i += 4;
                    continue;
                }
            }

            i += 1;
        }

        operations.insert(op_name, OperationInfo { steps });
    }

    Ok(operations)
}
