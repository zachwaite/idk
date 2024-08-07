use crate::field::{CommentField, FieldResult, FormtypeField, SequenceField};
use crate::meta::pluck_array3 as pluck;
use crate::meta::Position;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct CommentSpecLine {
    pub sequence: FieldResult<SequenceField>,
    pub form_type: FieldResult<FormtypeField>,
    pub comment: FieldResult<CommentField>,
}

impl Display for CommentSpecLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        msg.push_str(&self.sequence.to_string());
        msg.push_str(&self.form_type.to_string());
        msg.push_str(&self.comment.to_string());
        write!(f, "{}", msg)
    }
}

impl From<(usize, &[char; 100])> for CommentSpecLine {
    fn from(value: (usize, &[char; 100])) -> Self {
        let row = value.0;
        let start = Position::from((row, 0));
        let chars = value.1;
        Self {
            sequence: FieldResult::from((start, pluck::<100, 0, 6, 94>(chars))),
            form_type: FieldResult::from((start, pluck::<100, 6, 1, 93>(chars))),
            comment: FieldResult::from((start, pluck::<100, 7, 93, 0>(chars))),
        }
    }
}
