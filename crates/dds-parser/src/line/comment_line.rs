use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use crate::meta::{IHighlight, ISpan, Span, pos};
use crate::field::{FieldResult, SequenceField, FormtypeField, CommentField};
use crate::meta::pluck_array3 as pluck;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentLine {
    pub sequence: FieldResult<SequenceField>,
    pub form_type: FieldResult<FormtypeField>,
    pub comment: FieldResult<CommentField>,
}
impl From<(usize, &[char; 80])> for CommentLine {
    fn from(value: (usize, &[char; 80])) -> Self {
        let row = value.0;
        let chars = value.1;
        Self {
            sequence: FieldResult::from((pos(row, 0), pluck::<80, 0, 5, 75>(chars))),
            form_type: FieldResult::from((pos(row, 5), pluck::<80, 5, 1, 74>(chars))),
            comment: FieldResult::from((pos(row, 6), pluck::<80, 6, 74, 0>(chars))),
        }
    }
}
impl Display for CommentLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", [
            self.sequence.to_string(),
            self.form_type.to_string(),
            self.comment.to_string(),
        ].concat())
    }
}
impl IHighlight for CommentLine {
    fn highlight(&self) -> Vec<(Span, String)> {
        let mut out = vec![];
        out.append(&mut self.sequence.highlight());
        out.append(&mut self.form_type.highlight());
        out.append(&mut self.comment.highlight());
        out
    }
}
impl ISpan for CommentLine {
    fn span(&self) -> Span {
        Span::from((
            self.sequence.span(),
            self.comment.span(),
        ))
    }
}

