use std::fmt::Display;

use super::result::FieldResult;
use crate::meta::{Meta, Position};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdkField {
    pub value: String,
    pub meta: Meta,
}

impl Display for IdkField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char])> for IdkField {
    fn from(value: (Position, &[char])) -> Self {
        let chars = value.1;
        let meta = Meta::from(value);
        let fld = IdkField {
            value: chars.iter().collect::<String>(),
            meta,
        };
        fld
    }
}

impl From<(Position, &[char])> for FieldResult<IdkField> {
    fn from(value: (Position, &[char])) -> Self {
        let fld = IdkField::from(value);
        Self::Idk(fld)
    }
}
