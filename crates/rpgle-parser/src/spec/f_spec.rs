use crate::meta::{PMixin, Span};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::line::{FSpecLine, FSpecLineContinuation};

#[derive(Debug, Serialize, Deserialize)]
pub struct FSpec {
    pub line: FSpecLine,
    pub continuations: Vec<FSpecLineContinuation>,
}

impl Display for FSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut outs = vec![self.line.to_string()];
        for cont in self.continuations.iter() {
            outs.push(cont.to_string());
        }
        let out = outs.join("\n");
        write!(f, "{}", out)
    }
}

impl PMixin for FSpec {
    fn highlight(&self) -> Vec<(Span, String)> {
        let mut out = self.line.highlight();
        for cont in &self.continuations {
            out.append(&mut cont.highlight())
        }
        out
    }

    fn span(&self) -> Span {
        if self.continuations.len() == 0 {
            self.line.span()
        } else {
            let start = self.line.span();
            let end = self.continuations.last().expect("FSpec expected").span();
            Span::from((start, end))
        }
    }
}
