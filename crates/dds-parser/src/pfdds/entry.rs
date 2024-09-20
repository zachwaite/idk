use super::{RecordFormat, Field, Keyfield, FileEntry};
use crate::meta::{IHighlight, Span};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Entry {
    FE(FileEntry),
    R(RecordFormat),
    F(Field),
    K(Keyfield)
}
impl IHighlight for Entry {
    fn highlight(&self) -> Vec<(Span, String)> {
        match self {
            Entry::FE(e) => e.highlight(), 
            Entry::R(e) => e.highlight(), 
            Entry::F(e) => e.highlight(), 
            Entry::K(e) => e.highlight(), 
        }

    }
}

