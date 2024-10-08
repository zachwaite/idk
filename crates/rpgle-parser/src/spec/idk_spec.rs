use crate::meta::{PMixin, Span};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::line::IdkSpecLine;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdkSpec {
    pub line: IdkSpecLine,
}

impl Display for IdkSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.line.to_string();
        write!(f, "{}", out)
    }
}

impl PMixin for IdkSpec {
    fn highlight(&self) -> Vec<(Span, String)> {
        self.line.highlight()
    }

    fn span(&self) -> Span {
        self.line.span()
    }
}
