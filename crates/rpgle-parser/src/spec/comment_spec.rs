use crate::field::PMixin;
use crate::meta::Span;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::line::CommentSpecLine;

#[derive(Serialize, Deserialize)]
pub struct CommentSpec {
    pub line: CommentSpecLine,
}

impl Display for CommentSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.line.to_string();
        write!(f, "{}", out)
    }
}

impl PMixin for CommentSpec {
    fn highlight(&self) -> Vec<(Span, String)> {
        self.line.highlight()
    }

    fn span(&self) -> Span {
        self.line.span()
    }
}
