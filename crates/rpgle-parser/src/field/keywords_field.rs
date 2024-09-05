use std::fmt::Display;

use super::result::{FieldResult, PMixin};
use super::IdkField;
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordsField {
    pub value: String,
    pub meta: Meta,
}

impl Display for KeywordsField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

// for f-spec keywords
impl From<(Position, &[char; 57])> for FieldResult<KeywordsField> {
    fn from(value: (Position, &[char; 57])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        match chars[0] {
            '*' => {
                let value = chars.iter().collect::<String>();
                Self::Ok(KeywordsField { value, meta })
            }
            _ => {
                let fld = IdkField::from((value.0, chars.as_slice()));
                Self::Idk(fld)
            }
        }
    }
}

// TODO: for f-spec continuation keywords
impl From<(Position, &[char; 94])> for FieldResult<KeywordsField> {
    fn from(value: (Position, &[char; 94])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        match chars[0] {
            '*' => {
                let value = chars.iter().collect::<String>();
                Self::Ok(KeywordsField { value, meta })
            }
            _ => {
                let fld = IdkField::from((value.0, chars.as_slice()));
                Self::Idk(fld)
            }
        }
    }
}

// for d-spec keywords
impl From<(Position, &[char; 58])> for FieldResult<KeywordsField> {
    fn from(value: (Position, &[char; 58])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        match chars[0] {
            '*' => {
                let value = chars.iter().collect::<String>();
                Self::Ok(KeywordsField { value, meta })
            }
            _ => {
                let fld = IdkField::from((value.0, chars.as_slice()));
                Self::Idk(fld)
            }
        }
    }
}

impl PMixin for KeywordsField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "Error".to_string())]
    }
}
