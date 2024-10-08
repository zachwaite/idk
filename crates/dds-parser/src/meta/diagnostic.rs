use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub span: String,
    pub level: DiagnosticLevel,
    pub msg: String,
}

impl Diagnostic {
    pub fn empty() -> Self {
        Self {
            span: "TODO".to_string(),
            level: DiagnosticLevel::Info,
            msg: "TODO MSG".to_string(),
        }
    }
}
