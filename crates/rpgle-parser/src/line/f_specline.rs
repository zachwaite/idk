use crate::field::{
    FieldResult, FiletypeField, FormtypeField, KeywordsField, NameField, SequenceField,
};
use crate::meta::pluck_array3 as pluck;
use crate::meta::Position;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FSpecLine {
    pub sequence: FieldResult<SequenceField>,
    pub form_type: FieldResult<FormtypeField>,
    pub name: FieldResult<NameField>,
    pub filetype: FieldResult<FiletypeField>,
    pub keywords: FieldResult<KeywordsField>,
}

impl Display for FSpecLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        msg.push_str(&self.sequence.to_string());
        msg.push_str(&self.form_type.to_string());
        msg.push_str(&self.name.to_string());
        msg.push_str(&self.keywords.to_string());
        write!(f, "{}", msg)
    }
}

impl From<(usize, &[char; 100])> for FSpecLine {
    fn from(value: (usize, &[char; 100])) -> Self {
        let row = value.0;
        let start = Position::from((row, 0));
        let chars = value.1;
        Self {
            sequence: FieldResult::from((start, pluck::<100, 0, 5, 95>(chars))),
            form_type: FieldResult::from((start, pluck::<100, 5, 1, 94>(chars))),
            name: FieldResult::from((start, pluck::<100, 6, 10, 84>(chars))),
            filetype: FieldResult::from((start, pluck::<100, 16, 1, 83>(chars))),
            keywords: FieldResult::from((start, pluck::<100, 17, 83, 0>(chars))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FSpecLineContinuation {
    pub sequence: FieldResult<SequenceField>,
    pub form_type: FieldResult<FormtypeField>,
    pub keywords: FieldResult<KeywordsField>,
}

impl Display for FSpecLineContinuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        msg.push_str(&self.sequence.to_string());
        msg.push_str(&self.form_type.to_string());
        write!(f, "{}", msg)
    }
}

impl From<(usize, &[char; 100])> for FSpecLineContinuation {
    fn from(value: (usize, &[char; 100])) -> Self {
        let row = value.0;
        let start = Position::from((row, 0));
        let chars = value.1;
        Self {
            sequence: FieldResult::from((start, pluck::<100, 0, 5, 95>(chars))),
            form_type: FieldResult::from((start, pluck::<100, 5, 1, 94>(chars))),
            keywords: FieldResult::from((start, pluck::<100, 6, 94, 0>(chars))),
        }
    }
}
