use std::fmt::Display;

use super::result::FieldResult;
use crate::meta::{Meta, Position};

#[derive(Debug, Clone)]
pub struct FormtypeField {
    pub value: String,
    pub meta: Meta,
}

impl Display for FormtypeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 1])> for FieldResult<FormtypeField> {
    fn from(value: (Position, &[char; 1])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let fld = FormtypeField {
            value: chars.iter().collect::<String>(),
            meta,
        };
        Self::Ok(fld)
    }
}
