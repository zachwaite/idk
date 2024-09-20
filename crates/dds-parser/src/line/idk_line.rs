use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use crate::meta::{IHighlight, ISpan, Span, Position};
use crate::field::{IdkField, FieldResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdkLine {
    pub idk: FieldResult<IdkField>
}
impl From<(usize, &[char; 80])> for IdkLine {
    fn from(value: (usize, &[char; 80])) -> Self {
        Self {
            idk: FieldResult::from((
                Position::from((value.0, 0)),
                value.1.as_slice()
            ))
        }
    }
}
impl Display for IdkLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = self.idk.to_string();
        write!(f, "{}", msg)
    }
}
impl IHighlight for IdkLine {
    fn highlight(&self) -> Vec<(Span, String)> {
        self.idk.highlight()
    }
}
impl ISpan for IdkLine {
    fn span(&self) -> Span {
        self.idk.span()
    }
}


