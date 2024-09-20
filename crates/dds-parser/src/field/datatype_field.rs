use std::fmt::Display;

use super::idk_field::IdkField;
use super::result::FieldResult;
use crate::meta::{Meta, IHighlight, ISpan, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Datatype {
    Empty,
    P,
    S,
    B,
    F,
    A,
    H,
    L,
    T,
    Z,
    Five,
}
impl Display for Datatype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Datatype::Empty => ' '.to_string(),
            Datatype::P => "P".to_string(),
            Datatype::S => "S".to_string(),
            Datatype::B => "B".to_string(),
            Datatype::F => "F".to_string(),
            Datatype::A => "A".to_string(),
            Datatype::H => "H".to_string(),
            Datatype::L => "L".to_string(),
            Datatype::T => "T".to_string(),
            Datatype::Z => "Z".to_string(),
            Datatype::Five => "5".to_string(),
        };
        write!(f, "{}", msg)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatatypeField {
    pub value: Datatype,
    pub meta: Meta,
}
impl Display for DatatypeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.meta.text)
    }
}
impl From<(Position, &[char; 1])> for FieldResult<DatatypeField> {
    fn from(value: (Position, &[char; 1])) -> Self {
        let chars = value.1;
        let maybe = match chars[0] {
            ' ' => Some(Datatype::Empty),
            'P' => Some(Datatype::P),
            'S' => Some(Datatype::S),
            'B' => Some(Datatype::B),
            'F' => Some(Datatype::F),
            'A' => Some(Datatype::A),
            'H' => Some(Datatype::H),
            'L' => Some(Datatype::L),
            'T' => Some(Datatype::T),
            'Z' => Some(Datatype::Z),
            '5' => Some(Datatype::Five),
            _ => None,
        };
        if let Some(x) = maybe {
            let fld = DatatypeField {
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
impl IHighlight for DatatypeField {
    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.meta.span, "@type.qualifier".to_string())]
    }
}
impl ISpan for DatatypeField {
    fn span(&self) -> Span {
        self.meta.span
    }
}
