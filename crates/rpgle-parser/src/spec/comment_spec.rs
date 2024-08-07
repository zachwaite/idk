use std::fmt::Display;

use crate::line::CommentSpecLine;

pub struct CommentSpec {
    pub line: CommentSpecLine,
}

impl Display for CommentSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.line.to_string();
        write!(f, "{}", out)
    }
}
