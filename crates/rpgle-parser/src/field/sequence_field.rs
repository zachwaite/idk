use std::fmt::Display;

use super::result::{FieldBehavior, FieldResult};
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SequenceField {
    pub value: String,
    pub meta: Meta,
}

impl Display for SequenceField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 5])> for FieldResult<SequenceField> {
    fn from(value: (Position, &[char; 5])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let fld = SequenceField {
            value: chars.iter().collect::<String>(),
            meta,
        };
        Self::Ok(fld)
    }
}

impl FieldBehavior for SequenceField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "Normal".to_string())]
    }
}
