use crate::meta::{PMixin, Span};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::line::{HSpecLine, HSpecLineContinuation};

#[derive(Serialize, Deserialize)]
pub struct HSpec {
    pub line: HSpecLine,
    pub continuations: Vec<HSpecLineContinuation>,
}

impl Display for HSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = self.line.to_string();
        for cont in self.continuations.iter() {
            out.push_str(&cont.to_string());
        }
        write!(f, "{}", out)
    }
}

impl PMixin for HSpec {
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
            let end = self.continuations.last().expect("HSpec expected").span();
            Span::from((start, end))
        }
    }
}
