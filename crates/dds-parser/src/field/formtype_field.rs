use std::fmt::Display;

use super::idk_field::IdkField;
use super::result::FieldResult;
use crate::meta::{Meta, IHighlight, ISpan, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Formtype {
    Empty,
    A,
}
impl Display for Formtype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Empty => " ".to_string(),
            Self::A => "A".to_string(),
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormtypeField {
    pub value: Formtype,
    pub meta: Meta,
}
impl Display for FormtypeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.meta.text)
    }
}
impl From<(Position, &[char; 1])> for FieldResult<FormtypeField> {
    fn from(value: (Position, &[char; 1])) -> Self {
        let chars = value.1;
        let maybe = match chars[0] {
            ' ' => Some(Formtype::Empty),
            'A' => Some(Formtype::A),
            _ => None,
        };
        if let Some(x) = maybe {
            let fld = FormtypeField {
                value: x,
                meta: Meta::from((value.0, chars.as_slice())),
            };
            Self::Ok(fld)
        } else {
            let fld = IdkField::from((value.0, chars.as_slice()));
            Self::Idk(fld)
        }
    }
}
impl IHighlight for FormtypeField {
    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.meta.span, "@keyword.directive".to_string())]
    }
}
impl ISpan for FormtypeField {
    fn span(&self) -> Span {
        self.meta.span
    }
}
