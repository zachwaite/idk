use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use crate::meta::{IHighlight, ISpan, Span, pos};
use crate::field::{FieldResult, IgnoredField, RawKeywordsField};
use crate::meta::pluck_array3 as pluck;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuationLine {
    pub nothing: FieldResult<IgnoredField>,
    pub keywords: FieldResult<RawKeywordsField>

}
impl From<(usize, &[char; 80])> for ContinuationLine {
    fn from(value: (usize, &[char; 80])) -> Self {
        let row = value.0;
        let chars = value.1;
        Self {
            nothing: FieldResult::from((pos(row, 0), pluck::<80, 0, 44, 36>(chars))),
            keywords: FieldResult::from((pos(row, 44), pluck::<80, 44, 36, 0>(chars))),
        }
    }
}
impl Display for ContinuationLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", [
          self.nothing.to_string(),
          self.keywords.to_string(),
        ].concat())
    }
}
impl IHighlight for ContinuationLine {
    fn highlight(&self) -> Vec<(Span, String)> {
        let mut out = vec![];
        out.append(&mut self.nothing.highlight());
        out.append(&mut self.keywords.highlight());
        out
    }
}
impl ISpan for ContinuationLine {
    fn span(&self) -> Span {
        Span::from((
            self.nothing.span(),
            self.keywords.span(),
        ))
    }
}

