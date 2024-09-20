use serde::{Deserialize, Serialize};
use crate::meta::{IHighlight, Span};
use crate::field::{FieldResult, FEKeywordsField};
use crate::free::tokenize_fe_kw;
use crate::line::ContinuationLine;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileEntry {
    pub keywords: FieldResult<FEKeywordsField>
}
impl From<Vec<&ContinuationLine>> for FileEntry {
    fn from(value: Vec<&ContinuationLine>) -> Self {
        let continuations = value;
        let tokens = tokenize_fe_kw(continuations);
        let kwfield = FEKeywordsField { tokens };

        Self {
            keywords: FieldResult::Ok(kwfield),
        }
    }
}
impl IHighlight for FileEntry {
    fn highlight(&self) -> Vec<(Span, String)> {
        let mut out = vec![];
        out.append(&mut self.keywords.highlight());
        out
    }
}
