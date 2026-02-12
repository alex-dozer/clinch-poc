use std::collections::HashSet;

#[derive(Debug)]
pub struct StepInfo {
    pub binding: String,         // ← replaces outputs HashSet
    pub luop_fn: Option<String>, // e.g. "inspect_magic"
}
