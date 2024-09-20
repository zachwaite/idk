use std::fmt::Display;

use super::result::FieldResult;
use crate::meta::{Meta, IHighlight, ISpan, Position, Span};
use crate::field::IdkField;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LengthField {
    pub value: usize,
    pub meta: Meta,
}
impl From<(Position, &[char; 5])> for FieldResult<LengthField> {
    fn from(value: (Position, &[char; 5])) -> Self {
        let raw = value.1.iter().collect::<String>().trim().to_string();
        match raw.parse::<usize>() {
            Ok(cooked) => {
                Self::Ok(LengthField { 
                    value: cooked,
                    meta: Meta::from((value.0, value.1.as_slice()))
                })
            }
            Err(_) => {
                Self::Idk(IdkField { 
                    value: raw,
                    meta: Meta::from((value.0, value.1.as_slice()))
                })
            }
        }
    }
}
impl Display for LengthField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}
impl IHighlight for LengthField {
    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.meta.span, "@number".to_string())]
    }
}
impl ISpan for LengthField {
    fn span(&self) -> Span {
        self.meta.span
    }
}
