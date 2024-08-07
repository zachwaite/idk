use std::fmt::Display;

use super::result::FieldResult;
use crate::meta::{Meta, Position};

#[derive(Debug, Clone)]
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

impl From<(Position, &[char])> for FieldResult<IdkField> {
    fn from(value: (Position, &[char])) -> Self {
        let chars = value.1;
        let meta = Meta::from(value);
        let fld = IdkField {
            value: chars.iter().collect::<String>(),
            meta,
        };
        Self::Idk(fld)
    }
}
