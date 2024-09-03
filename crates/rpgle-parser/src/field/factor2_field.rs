use std::fmt::Display;

use super::result::{Field, FieldResult};
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Factor2Field {
    pub value: String,
    pub meta: Meta,
}

impl Display for Factor2Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 15])> for FieldResult<Factor2Field> {
    fn from(value: (Position, &[char; 15])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let value = chars.iter().filter(|c| **c != ' ').collect::<String>();
        Self::Ok(Factor2Field { value, meta })
    }
}

impl From<(Position, &[char; 66])> for FieldResult<Factor2Field> {
    fn from(value: (Position, &[char; 66])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let value = chars.iter().filter(|c| **c != ' ').collect::<String>();
        Self::Ok(Factor2Field { value, meta })
    }
}

impl Field for Factor2Field {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "Normal".to_string())]
    }
}
