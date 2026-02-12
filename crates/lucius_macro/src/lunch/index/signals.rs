use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::TokenTree;
use serde_json::to_string;
use std::collections::HashMap;
use syn::{Error, Result};

use crate::lunch::parse::signals::SignalsBlock;

#[derive(Debug)]
pub struct SignalIndex {
    pub families: HashMap<String, FamilyInfo>,
}
impl SignalIndex {
    pub fn new() -> Self {
        SignalIndex {
            families: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct FamilyInfo {
    pub signals: HashMap<String, SignalInfo>,
}

#[derive(Debug)]
pub struct SignalInfo {
    pub derives_from: DeriveFrom,
    pub when: TokenStream2,
}

#[derive(Debug)]
pub struct DeriveFrom {
    pub operation: String,
    pub step: String,
}

pub fn build_signal_index(signals_block: &SignalsBlock) -> Result<HashMap<String, FamilyInfo>> {
    let mut families: HashMap<String, FamilyInfo> = HashMap::new();

    for family in &signals_block.families {
        let family_name = family.name.to_string();

        let family_entry = families
            .entry(family_name.clone())
            .or_insert_with(|| FamilyInfo {
                signals: HashMap::new(),
            });

        for signal in &family.signals {
            let signal_name = signal.name.to_string();

            if family_entry.signals.contains_key(&signal_name) {
                return Err(Error::new_spanned(
                    &signal.name,
                    format!(
                        "duplicate signal `{}` in family `{}`",
                        signal_name, family_name
                    ),
                ));
            }

            family_entry.signals.insert(
                signal_name,
                SignalInfo {
                    derives_from: DeriveFrom {
                        operation: signal.body.derive_from.operation.to_string(),
                        step: signal.body.derive_from.step.to_string(),
                    },
                    when: signal.body.when.clone(),
                },
            );
        }
    }

    Ok(families)
}
