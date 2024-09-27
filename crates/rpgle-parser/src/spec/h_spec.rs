use crate::field::{FieldResult, FormtypeField, HKeywordsField, SequenceField};
use crate::free::tokenize_hspec_kw;
use crate::meta::{PMixin, Span};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::line::{HSpecLine, HSpecLineContinuation};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HSpec {
    pub sequence: FieldResult<SequenceField>,
    pub form_type: FieldResult<FormtypeField>,
    pub keywords: FieldResult<HKeywordsField>,
}

impl Display for HSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        msg.push_str(&self.sequence.to_string());
        msg.push_str(&self.form_type.to_string());
        msg.push_str(&self.keywords.to_string());
        write!(f, "{}", msg)
    }
}

impl PMixin for HSpec {
    fn span(&self) -> Span {
        let start = self.sequence.span();
        let end = self.keywords.span();
        Span::from((start, end))
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        let mut out = vec![];
        out.append(&mut self.sequence.highlight());
        out.append(&mut self.form_type.highlight());
        out.append(&mut self.keywords.highlight());
        out
    }
}

impl From<(&HSpecLine, Vec<&HSpecLineContinuation>)> for HSpec {
    fn from(value: (&HSpecLine, Vec<&HSpecLineContinuation>)) -> Self {
        let line = value.0;
        let continuations = value.1;

        let tokens = tokenize_hspec_kw(line, continuations);
        let kwfield = HKeywordsField { tokens };

        Self {
            sequence: line.sequence.clone(),
            form_type: line.form_type.clone(),
            keywords: FieldResult::Ok(kwfield),
        }
    }
}
