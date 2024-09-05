use std::fmt::Display;

use super::idk_field::IdkField;
use super::result::{FieldResult, PMixin};
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ResultLength {
    Empty,
    Value(u32), // TODO: constrain value
}

impl Display for ResultLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Empty => " ".to_string(),
            Self::Value(x) => x.to_string(),
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultLengthField {
    pub value: ResultLength,
    pub meta: Meta,
}

impl Display for ResultLengthField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 5])> for FieldResult<ResultLengthField> {
    fn from(value: (Position, &[char; 5])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let txt = chars.iter().filter(|c| **c != ' ').collect::<String>();
        let maybe = match txt.len() {
            0 => Some(ResultLength::Empty),
            _ => match txt.parse::<u32>() {
                Ok(x) => Some(ResultLength::Value(x)),
                Err(_) => None,
            },
        };
        if let Some(x) = maybe {
            let fld = ResultLengthField { value: x, meta };
            Self::Ok(fld)
        } else {
            let fld = IdkField::from((value.0, chars.as_slice()));
            Self::Idk(fld)
        }
    }
}

impl PMixin for ResultLengthField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "@number".to_string())]
    }
}
