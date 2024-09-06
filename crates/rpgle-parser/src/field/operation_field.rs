use std::fmt::Display;

use super::result::FieldResult;
use crate::meta::{Meta, PMixin, Position, Span};
use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum TraditionalOpcode {}
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum Extf2Opcode {}
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum Opcode {
//     Traditional(TraditionalOpcode),
//     Extf2(Extf2Opcode)
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationField {
    pub value: String,
    pub meta: Meta,
}

impl Display for OperationField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 10])> for FieldResult<OperationField> {
    fn from(value: (Position, &[char; 10])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let value = chars.iter().filter(|c| **c != ' ').collect::<String>();
        Self::Ok(OperationField { value, meta })
    }
}

impl From<(Position, &[char; 15])> for FieldResult<OperationField> {
    fn from(value: (Position, &[char; 15])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let value = chars.iter().filter(|c| **c != ' ').collect::<String>();
        Self::Ok(OperationField { value, meta })
    }
}

pub fn has_extf2_optoken(chars: &[char; 100]) -> bool {
    // TODO: extract to free parser
    match chars[25..=34] {
        ['C', 'A', 'L', 'L', 'P', ..] => true,
        ['D', 'O', 'U', ..] => true,
        // ...
        _ => false,
    }
}

impl PMixin for OperationField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "Identifier".to_string())]
    }
}
