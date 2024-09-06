use std::{collections::HashSet, fmt::Display};

use super::result::FieldResult;
use super::IdkField;
use crate::meta::{Meta, PMixin, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NothingField {
    pub value: String,
    pub meta: Meta,
}

impl Display for NothingField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

// for f-spec continuation
impl From<(Position, &[char; 37])> for FieldResult<NothingField> {
    fn from(value: (Position, &[char; 37])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let unique_chars = chars.iter().collect::<HashSet<&char>>();
        if unique_chars.len() == 1 && unique_chars.contains(&' ') {
            let value = chars.iter().collect::<String>();
            Self::Ok(NothingField { value, meta })
        } else {
            let fld = IdkField::from((value.0, chars.as_slice()));
            Self::Idk(fld)
        }
    }
}

// for cspecline
impl From<(Position, &[char; 5])> for FieldResult<NothingField> {
    fn from(value: (Position, &[char; 5])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let unique_chars = chars.iter().collect::<HashSet<&char>>();
        if unique_chars.len() == 1 && unique_chars.contains(&' ') {
            let value = chars.iter().collect::<String>();
            Self::Ok(NothingField { value, meta })
        } else {
            let fld = IdkField::from((value.0, chars.as_slice()));
            Self::Idk(fld)
        }
    }
}

impl From<(Position, &[char; 7])> for FieldResult<NothingField> {
    fn from(value: (Position, &[char; 7])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let unique_chars = chars.iter().collect::<HashSet<&char>>();
        if unique_chars.len() == 1 && unique_chars.contains(&' ') {
            let value = chars.iter().collect::<String>();
            Self::Ok(NothingField { value, meta })
        } else {
            let fld = IdkField::from((value.0, chars.as_slice()));
            Self::Idk(fld)
        }
    }
}

impl PMixin for NothingField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "Normal".to_string())]
    }
}
