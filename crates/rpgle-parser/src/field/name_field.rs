use std::fmt::Display;

use super::result::{FieldBehavior, FieldResult};
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NameField {
    pub value: String,
    pub meta: Meta,
}

impl Display for NameField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 10])> for FieldResult<NameField> {
    fn from(value: (Position, &[char; 10])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let value = chars.iter().filter(|c| **c != ' ').collect::<String>();
        Self::Ok(NameField { value, meta })
    }
}

impl From<(Position, &[char; 15])> for FieldResult<NameField> {
    fn from(value: (Position, &[char; 15])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let value = chars.iter().filter(|c| **c != ' ').collect::<String>();
        Self::Ok(NameField { value, meta })
    }
}

impl FieldBehavior for NameField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "Identifier".to_string())]
    }
}
