use std::fmt::Display;

use super::idk_field::IdkField;
use super::result::{FieldResult, PMixin};
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Formtype {
    Empty,
    H,
    F,
    D,
    I,
    C,
    O,
    P,
}

impl Display for Formtype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Empty => " ".to_string(),
            Self::H => "H".to_string(),
            Self::F => "F".to_string(),
            Self::D => "D".to_string(),
            Self::I => "I".to_string(),
            Self::C => "C".to_string(),
            Self::O => "O".to_string(),
            Self::P => "P".to_string(),
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
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 1])> for FieldResult<FormtypeField> {
    fn from(value: (Position, &[char; 1])) -> Self {
        let chars = value.1;
        let maybe = match chars[0] {
            ' ' => Some(Formtype::Empty),
            'H' => Some(Formtype::H),
            'F' => Some(Formtype::F),
            'D' => Some(Formtype::D),
            'I' => Some(Formtype::I),
            'C' => Some(Formtype::C),
            'O' => Some(Formtype::O),
            'P' => Some(Formtype::P),
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

impl PMixin for FormtypeField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "@keyword.directive".to_string())]
    }
}
