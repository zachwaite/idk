use crate::field::{Field, FieldResult, IdkField};
use crate::meta::{Position, Span};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdkSpecLine {
    pub idk: FieldResult<IdkField>,
}

impl Display for IdkSpecLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = self.idk.to_string();
        write!(f, "{}", msg)
    }
}

impl From<(usize, &[char; 100])> for IdkSpecLine {
    fn from(value: (usize, &[char; 100])) -> Self {
        let row = value.0;
        let start = Position::from((row, 0));
        let chars = value.1;
        Self {
            idk: FieldResult::from((start, chars.as_slice())),
        }
    }
}

impl Field for IdkSpecLine {
    fn span(&self) -> Span {
        self.idk.span()
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        self.idk.highlight()
    }
}
