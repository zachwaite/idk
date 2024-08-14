use std::fmt::Display;

use super::idk_field::IdkField;
use super::result::FieldResult;
use crate::meta::{Meta, Position};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Datatype {
    Empty,
    A,
    B,
    C,
    D,
    F,
    G,
    I,
    N,
    O,
    P,
    S,
    T,
    U,
    Z,
    Star,
}

impl Display for Datatype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Empty => " ".to_string(),
            Self::A => "A".to_string(),
            Self::B => "B".to_string(),
            Self::C => "C".to_string(),
            Self::D => "D".to_string(),
            Self::F => "F".to_string(),
            Self::G => "G".to_string(),
            Self::I => "I".to_string(),
            Self::N => "N".to_string(),
            Self::O => "O".to_string(),
            Self::P => "P".to_string(),
            Self::S => "S".to_string(),
            Self::T => "T".to_string(),
            Self::U => "U".to_string(),
            Self::Z => "Z".to_string(),
            Self::Star => "Star".to_string(),
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
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 1])> for FieldResult<DatatypeField> {
    fn from(value: (Position, &[char; 1])) -> Self {
        let chars = value.1;
        let maybe = match chars[0] {
            ' ' => Some(Datatype::Empty),
            'A' => Some(Datatype::A),
            'B' => Some(Datatype::B),
            'C' => Some(Datatype::C),
            'D' => Some(Datatype::D),
            'F' => Some(Datatype::F),
            'G' => Some(Datatype::G),
            'I' => Some(Datatype::I),
            'N' => Some(Datatype::N),
            'O' => Some(Datatype::O),
            'P' => Some(Datatype::P),
            'S' => Some(Datatype::S),
            'T' => Some(Datatype::T),
            'U' => Some(Datatype::U),
            'Z' => Some(Datatype::Z),
            '*' => Some(Datatype::Star),
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
