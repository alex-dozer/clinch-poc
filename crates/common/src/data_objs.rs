use std::collections::HashMap;

#[derive(Debug)]
pub struct Artifact {
    /// Raw bytes of the artifact (file, payload, stream, etc.)
    pub bytes: Vec<u8>,

    /// Optional decoded text (if applicable)
    pub text: Option<String>,

    /// Optional metadata (MIME, filename, etc.)
    pub meta: HashMap<String, String>,
}

#[derive(Debug, Default)]
pub struct LuciusContext {
    pub tags: Vec<String>,
    pub emits: Vec<String>,
    pub deferred: Vec<String>,
    pub scores: HashMap<String, f64>,
}
impl LuciusContext {
    pub fn new() -> Self {
        Self {
            tags: Vec::new(),
            emits: Vec::new(),
            deferred: Vec::new(),
            scores: std::collections::HashMap::new(),
        }
    }
}
