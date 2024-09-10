use crate::field::{CompilerDirectiveField, FieldResult, FormtypeField, SequenceField};
use crate::meta::pluck_array3 as pluck;
use crate::meta::{PMixin, Position, Span};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerDirectiveSpecLine {
    pub sequence: FieldResult<SequenceField>,
    pub form_type: FieldResult<FormtypeField>,
    pub directive: FieldResult<CompilerDirectiveField>,
}

impl Display for CompilerDirectiveSpecLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        msg.push_str(&self.sequence.to_string());
        msg.push_str(&self.form_type.to_string());
        msg.push_str(&self.directive.to_string());
        write!(f, "{}", msg)
    }
}

impl From<(usize, &[char; 100])> for CompilerDirectiveSpecLine {
    fn from(value: (usize, &[char; 100])) -> Self {
        let row = value.0;
        let chars = value.1;
        Self {
            sequence: FieldResult::from((Position::from((row, 0)), pluck::<100, 0, 5, 95>(chars))),
            form_type: FieldResult::from((Position::from((row, 5)), pluck::<100, 5, 1, 94>(chars))),
            directive: FieldResult::from((Position::from((row, 6)), pluck::<100, 6, 94, 0>(chars))),
        }
    }
}

impl PMixin for CompilerDirectiveSpecLine {
    fn span(&self) -> Span {
        let start = self.sequence.span();
        let end = self.directive.span();
        Span::from((start, end))
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        let mut out = vec![];
        out.append(&mut self.sequence.highlight());
        out.append(&mut self.form_type.highlight());
        out.append(&mut self.directive.highlight());
        out
    }
}
