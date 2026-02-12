use core::str;
use proc_macro2::Span;
use std::collections::{HashMap, HashSet};
use syn::{Error, Result, spanned};

use crate::lunch::{
    index::{
        clinch::{self, ClinchAction, ClinchIndex, SignalId, build_clinch_index},
        common::StepInfo,
        operations::{OperationIndex, OperationInfo, build_operation_index},
        signals::{FamilyInfo, SignalIndex, build_signal_index},
    },
    parse::pipeline::PipelineAst,
}; // adjust paths if needed, add signals block later.

#[derive(Debug)]
pub struct PipelineIndex {
    pub operation_index: OperationIndex,
    pub signal_index: SignalIndex,
    pub clinch_index: ClinchIndex,
}

/*
pub struct PipelineIndex {
    pub meta: PipelineMeta,
    pub operations: HashMap<…>,
    pub signals: HashMap<…>,
    pub clinch: ClinchInfo,
}

pub struct PipelineMeta {
    pub name: Option<String>,
    pub author: Option<String>,
    pub version: Option<String>,
    pub scope: Option<String>,
    pub raw: HashMap<String, String>, // future-proof
}

*/

impl PipelineIndex {
    // pub fn from_ast(ast: &PipelineAst) -> Result<Self> {
    //     let ops = match &ast.operations {
    //         Some(ops) => ops,
    //         None => {
    //             return Err(Error::new(
    //                 Span::call_site(),
    //                 "missing `operations` block in pipeline definition",
    //             ));
    //         } // no ops is allowed (for now)
    //     };

    //     let signals_block = match &ast.signals {
    //         Some(s) => s,
    //         None => {
    //             return Err(Error::new(
    //                 Span::call_site(),
    //                 "missing `operations` block in pipeline definition",
    //             ));
    //         }
    //     };

    //     let operation_index = build_operation_index(ops)?;
    //     let signal_index = build_signal_index(signals_block)?;

    //     Ok(Self {
    //         operation_index,
    //         signal_index,
    //     })
    // }

    pub fn from_operations(ast: &PipelineAst) -> Result<Self> {
        let ops = match &ast.operations {
            Some(ops) => ops,
            None => {
                return Err(Error::new(
                    Span::call_site(),
                    "missing `operations` block in pipeline definition",
                ));
            } // no ops is allowed (for now)
        };

        let operation_infos = build_operation_index(ops)?;

        Ok(Self {
            operation_index: OperationIndex {
                index: operation_infos,
            },
            signal_index: SignalIndex::new(),
            clinch_index: ClinchIndex::new(),
        })
    }

    pub fn extend_with_signals(&mut self, ast: &PipelineAst) -> Result<()> {
        let signals_block = match &ast.signals {
            Some(s) => s,
            None => {
                return Err(Error::new(
                    Span::call_site(),
                    "missing `signals` block in pipeline definition",
                ));
            }
        };

        let signal_index = build_signal_index(signals_block)?;
        self.signal_index.families = signal_index;

        Ok(())
    }

    pub fn extend_with_clinch(&mut self, ast: &PipelineAst) -> Result<()> {
        let clinch_block = match &ast.clinch {
            Some(c) => c,
            None => {
                return Err(Error::new(
                    Span::call_site(),
                    "missing `clinch` block in pipeline definition",
                ));
            }
        };

        let clinch_index: clinch::ClinchIndex = build_clinch_index(clinch_block)?;

        self.clinch_index.by_signal = clinch_index.by_signal;

        Ok(())
    }

    pub fn get_operation(&self, name: &str) -> Option<&OperationInfo> {
        self.operation_index.index.get(name)
    }

    pub fn get_step(&self, op: &str, step: &str) -> Option<&StepInfo> {
        self.operation_index
            .index
            .get(op)
            .and_then(|op| op.steps.get(step))
    }

    pub fn has_binding(&self, op: &str, step: &str) -> bool {
        self.get_step(op, step).map(|s| s.binding.clone()).is_some()
    }
}
