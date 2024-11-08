use std::fmt::Display;

use super::idk_field::IdkField;
use super::result::{FieldBehavior, FieldResult};
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum POS {
    Empty,
    Value(u32), // TDE: constrain value to 1-2000
}

impl Display for POS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Empty => " ".to_string(),
            Self::Value(x) => x.to_string(),
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct POSField {
    pub value: POS,
    pub meta: Meta,
}

impl Display for POSField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 7])> for FieldResult<POSField> {
    fn from(value: (Position, &[char; 7])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let txt = chars.iter().filter(|c| **c != ' ').collect::<String>();
        let maybe = match txt.len() {
            0 => Some(POS::Empty),
            _ => match txt.parse::<u32>() {
                Ok(x) => Some(POS::Value(x)),
                Err(_) => None,
            },
        };
        if let Some(x) = maybe {
            let fld = POSField { value: x, meta };
            Self::Ok(fld)
        } else {
            let fld = IdkField::from((value.0, chars.as_slice()));
            Self::Idk(fld)
        }
    }
}

impl FieldBehavior for POSField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "@number".to_string())]
    }
}
