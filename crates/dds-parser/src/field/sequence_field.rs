use std::fmt::Display;

use super::result::FieldResult;
use crate::meta::{Meta, IHighlight, ISpan, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceField {
    pub value: String,
    pub meta: Meta,
}
impl From<(Position, &[char; 5])> for FieldResult<SequenceField> {
    fn from(value: (Position, &[char; 5])) -> Self {
        let fld = SequenceField {
            value: value.1.iter().collect::<String>(),
            meta: Meta::from((value.0, value.1.as_slice()))
        };
        Self::Ok(fld)
    }
}
impl Display for SequenceField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}
impl IHighlight for SequenceField {
    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.meta.span, "Normal".to_string())]
    }
}
impl ISpan for SequenceField {
    fn span(&self) -> Span {
        self.meta.span
    }
}
