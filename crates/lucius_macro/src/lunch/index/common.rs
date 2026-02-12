use std::collections::HashSet;

#[derive(Debug)]
pub struct StepInfo {
    pub binding: String,
    pub ops_fn: Option<String>, // e.g. "inspect_magic"
}
