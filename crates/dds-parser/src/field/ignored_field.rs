use std::fmt::Display;

use super::result::FieldResult;
use crate::meta::{Meta, IHighlight, ISpan, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IgnoredField {
    pub value: String,
    pub meta: Meta,
}
impl From<(Position, &[char; 1])> for FieldResult<IgnoredField> {
    fn from(value: (Position, &[char; 1])) -> Self {
            Self::Ok(IgnoredField { 
                value: value.1.iter().collect::<String>(),
                meta: Meta::from((value.0, value.1.as_slice()))
            })
    }
}
impl From<(Position, &[char; 6])> for FieldResult<IgnoredField> {
    fn from(value: (Position, &[char; 6])) -> Self {
            Self::Ok(IgnoredField { 
                value: value.1.iter().collect::<String>(),
                meta: Meta::from((value.0, value.1.as_slice()))
            })
    }
}
impl From<(Position, &[char; 9])> for FieldResult<IgnoredField> {
    fn from(value: (Position, &[char; 9])) -> Self {
            Self::Ok(IgnoredField { 
                value: value.1.iter().collect::<String>(),
                meta: Meta::from((value.0, value.1.as_slice()))
            })
    }
}
impl From<(Position, &[char; 44])> for FieldResult<IgnoredField> {
    fn from(value: (Position, &[char; 44])) -> Self {
            Self::Ok(IgnoredField { 
                value: value.1.iter().collect::<String>(),
                meta: Meta::from((value.0, value.1.as_slice()))
            })
    }
}
impl Display for IgnoredField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}
impl IHighlight for IgnoredField {
    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.meta.span, "Error".to_string())]
    }
}
impl ISpan for IgnoredField {
    fn span(&self) -> Span {
        self.meta.span
    }
}
