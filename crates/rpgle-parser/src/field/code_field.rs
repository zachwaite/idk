use std::fmt::Display;

use super::result::{FieldBehavior, FieldResult};
use crate::free::Op;
use crate::meta::{Meta, Position, Span};
use nonempty::NonEmpty;
use serde::{Deserialize, Serialize};

// raw
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RawCodeField {
    pub value: NonEmpty<char>,
    pub meta: Meta,
}

impl Display for RawCodeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta.text)
    }
}
impl FieldBehavior for RawCodeField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "Normal".to_string())]
    }
}
impl From<(Position, &[char; 93])> for FieldResult<RawCodeField> {
    fn from(value: (Position, &[char; 93])) -> Self {
        let pos = value.0;
        let chars = value.1;
        let meta = Meta::from((pos, chars.as_slice()));
        let value = NonEmpty::from_vec(chars.iter().map(|c| *c).collect::<Vec<char>>())
            .expect("&[char; 93] is guaranteed to be nonempty");
        Self::Ok(RawCodeField { value, meta })
    }
}

// cooked
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeField {
    pub op: Op,
}

impl Display for CodeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.op.to_string())
    }
}
impl FieldBehavior for CodeField {
    fn span(&self) -> Span {
        self.op.span()
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        self.op.highlight()
    }
}
