use std::fmt::Display;

use super::idk_field::IdkField;
use super::result::FieldResult;
use crate::meta::{Meta, IHighlight, ISpan, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Reference {
    Empty,
    R,
}
impl Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Empty => " ".to_string(),
            Self::R => "R".to_string(),
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceField {
    pub value: Reference,
    pub meta: Meta,
}
impl Display for ReferenceField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.meta.text)
    }
}
impl From<(Position, &[char; 1])> for FieldResult<ReferenceField> {
    fn from(value: (Position, &[char; 1])) -> Self {
        let chars = value.1;
        let maybe = match chars[0] {
            ' ' => Some(Reference::Empty),
            'R' => Some(Reference::R),
            _ => None,
        };
        if let Some(x) = maybe {
            let fld = ReferenceField {
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
impl IHighlight for ReferenceField {
    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.meta.span, "@keyword.directive".to_string())]
    }
}
impl ISpan for ReferenceField {
    fn span(&self) -> Span {
        self.meta.span
    }
}
