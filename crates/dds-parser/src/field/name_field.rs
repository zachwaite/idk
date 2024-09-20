use std::fmt::Display;

use super::result::FieldResult;
use crate::meta::{Meta, IHighlight, ISpan, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameField {
    pub value: String,
    pub meta: Meta,
}
impl From<(Position, &[char; 10])> for FieldResult<NameField> {
    fn from(value: (Position, &[char; 10])) -> Self {
            Self::Ok(NameField { 
                value: value.1.iter().filter(|c| **c != ' ').collect::<String>(),
                meta: Meta::from((value.0, value.1.as_slice()))
            })
    }
}
impl Display for NameField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}
impl IHighlight for NameField {
    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.meta.span, "Identifier".to_string())]
    }
}
impl ISpan for NameField {
    fn span(&self) -> Span {
        self.meta.span
    }
}
