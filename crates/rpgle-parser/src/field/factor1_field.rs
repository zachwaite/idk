use std::fmt::Display;

use super::result::{FieldResult, PMixin};
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Factor1Field {
    pub value: String,
    pub meta: Meta,
}

impl Display for Factor1Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 13])> for FieldResult<Factor1Field> {
    fn from(value: (Position, &[char; 13])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let value = chars.iter().filter(|c| **c != ' ').collect::<String>();
        Self::Ok(Factor1Field { value, meta })
    }
}

impl PMixin for Factor1Field {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "Identifier".to_string())]
    }
}
