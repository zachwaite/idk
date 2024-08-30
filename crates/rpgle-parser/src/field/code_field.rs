use std::fmt::Display;

use super::result::FieldResult;
use crate::meta::{Meta, Position};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeField {
    pub value: String,
    pub meta: Meta,
}

impl Display for CodeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 93])> for FieldResult<CodeField> {
    fn from(value: (Position, &[char; 93])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let value = chars.iter().collect::<String>().trim().to_string();
        Self::Ok(CodeField { value, meta })
    }
}
