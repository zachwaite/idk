use std::fmt::Display;

use super::idk_field::IdkField;
use super::result::{Field, FieldResult};
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RecordAddressType {
    Empty,
    A,
    P,
    G,
    K,
    D,
    T,
    Z,
    F,
}

impl Display for RecordAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Empty => " ".to_string(),
            Self::A => "A".to_string(),
            Self::P => "P".to_string(),
            Self::G => "G".to_string(),
            Self::K => "K".to_string(),
            Self::D => "D".to_string(),
            Self::T => "T".to_string(),
            Self::Z => "Z".to_string(),
            Self::F => "F".to_string(),
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordAddressTypeField {
    pub value: RecordAddressType,
    pub meta: Meta,
}

impl Display for RecordAddressTypeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 1])> for FieldResult<RecordAddressTypeField> {
    fn from(value: (Position, &[char; 1])) -> Self {
        let chars = value.1;
        let maybe = match chars[0] {
            ' ' => Some(RecordAddressType::Empty),
            'A' => Some(RecordAddressType::A),
            'P' => Some(RecordAddressType::P),
            'G' => Some(RecordAddressType::G),
            'K' => Some(RecordAddressType::K),
            'D' => Some(RecordAddressType::D),
            'T' => Some(RecordAddressType::T),
            'Z' => Some(RecordAddressType::Z),
            'F' => Some(RecordAddressType::F),

            _ => None,
        };
        if let Some(x) = maybe {
            let fld = RecordAddressTypeField {
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

impl Field for RecordAddressTypeField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "Normal".to_string())]
    }
}
