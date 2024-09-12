use crate::field::{CodeField, FieldResult};
use crate::free::{tokenize, tokenize_extf2, tokenize_traditional_f2};
use crate::meta::{PMixin, Span};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::line::{CSpecLine, CSpecLineContinuation};

#[derive(Debug, Serialize, Deserialize)]
pub struct CSpec {
    pub code: FieldResult<CodeField>,
}

impl Display for CSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        msg.push_str(&self.code.to_string());
        write!(f, "{}", msg)
    }
}

impl PMixin for CSpec {
    fn span(&self) -> Span {
        self.code.span()
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        let mut out = vec![];
        out.append(&mut self.code.highlight());
        out
    }
}

impl From<(&CSpecLine, Vec<&CSpecLineContinuation>)> for CSpec {
    fn from(value: (&CSpecLine, Vec<&CSpecLineContinuation>)) -> Self {
        let line = value.0;
        let continuations = value.1;

        let tokens = match line {
            CSpecLine::Free(line) => tokenize(line, vec![]),
            CSpecLine::Traditional(line) => tokenize_traditional_f2(line), // no conts
            CSpecLine::ExtF2(line) => tokenize_extf2(line, vec![]),
        };
        let codefield = CodeField { tokens };

        Self {
            code: FieldResult::Ok(codefield),
        }
    }
}
