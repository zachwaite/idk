use std::fmt::Display;

use super::result::FieldResult;
use super::IdkField;
use crate::meta::{Meta, IHighlight, ISpan, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentField {
    pub value: String,
    pub meta: Meta,
}
impl From<(Position, &[char; 74])> for FieldResult<CommentField> {
    fn from(value: (Position, &[char; 74])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        match chars[0] {
            '*' => {
                let value = chars.iter().collect::<String>();
                Self::Ok(CommentField { value, meta })
            }
            _ => {
                let fld = IdkField::from((value.0, chars.as_slice()));
                Self::Idk(fld)
            }
        }
    }
}

impl Display for CommentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}
impl IHighlight for CommentField {
    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.meta.span, "@comment".to_string())]
    }
}
impl ISpan for CommentField {
    fn span(&self) -> Span {
        self.meta.span
    }
}
