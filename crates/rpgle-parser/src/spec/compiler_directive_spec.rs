use crate::meta::{PMixin, Span};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::line::CompilerDirectiveSpecLine;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerDirectiveSpec {
    pub line: CompilerDirectiveSpecLine,
}

impl Display for CompilerDirectiveSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.line.to_string();
        write!(f, "{}", out)
    }
}

impl PMixin for CompilerDirectiveSpec {
    fn highlight(&self) -> Vec<(Span, String)> {
        self.line.highlight()
    }

    fn span(&self) -> Span {
        self.line.span()
    }
}
