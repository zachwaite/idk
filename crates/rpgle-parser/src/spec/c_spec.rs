use crate::field::{CodeField, FieldResult};
use crate::free::Op;
use crate::meta::{PMixin, Span};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::line::{
    ExtF2CSpecLine, ExtF2CSpecLineContinuation, FreeCSpecLine, FreeCSpecLineContinuation,
    TraditionalCSpecLine,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl From<(&FreeCSpecLine, Vec<&FreeCSpecLineContinuation>)> for CSpec {
    fn from(value: (&FreeCSpecLine, Vec<&FreeCSpecLineContinuation>)) -> Self {
        let op = Op::from(value);
        let fld = CodeField { op };
        let code = FieldResult::Ok(fld);
        Self { code }
    }
}
impl From<(&ExtF2CSpecLine, Vec<&ExtF2CSpecLineContinuation>)> for CSpec {
    fn from(value: (&ExtF2CSpecLine, Vec<&ExtF2CSpecLineContinuation>)) -> Self {
        let op = Op::from(value);
        let fld = CodeField { op };
        let code = FieldResult::Ok(fld);
        Self { code }
    }
}
impl From<&TraditionalCSpecLine> for CSpec {
    fn from(value: &TraditionalCSpecLine) -> Self {
        let op = Op::from(value);
        let fld = CodeField { op };
        let code = FieldResult::Ok(fld);
        Self { code }
    }
}
