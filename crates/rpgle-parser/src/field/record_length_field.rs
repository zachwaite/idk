use std::fmt::Display;

use super::idk_field::IdkField;
use super::result::{FieldBehavior, FieldResult};
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RecordLength {
    Empty,
    Value(u32),
}

impl Display for RecordLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Empty => " ".to_string(),
            Self::Value(x) => x.to_string(),
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecordLengthField {
    pub value: RecordLength,
    pub meta: Meta,
}

impl Display for RecordLengthField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 5])> for FieldResult<RecordLengthField> {
    fn from(value: (Position, &[char; 5])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let txt = chars.iter().filter(|c| **c != ' ').collect::<String>();
        let maybe = match txt.len() {
            0 => Some(RecordLength::Empty),
            _ => match txt.parse::<u32>() {
                Ok(x) => Some(RecordLength::Value(x)),
                Err(_) => None,
            },
        };
        if let Some(x) = maybe {
            let fld = RecordLengthField { value: x, meta };
            Self::Ok(fld)
        } else {
            let fld = IdkField::from((value.0, chars.as_slice()));
            Self::Idk(fld)
        }
    }
}

impl FieldBehavior for RecordLengthField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "@number".to_string())]
    }
}
