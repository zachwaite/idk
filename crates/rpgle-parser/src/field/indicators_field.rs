use std::fmt::Display;

use super::result::{FieldResult, PMixin};
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorsField {
    pub value: String,
    pub meta: Meta,
}

impl Display for IndicatorsField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 3])> for FieldResult<IndicatorsField> {
    fn from(value: (Position, &[char; 3])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let value = chars.iter().filter(|c| **c != ' ').collect::<String>();
        Self::Ok(IndicatorsField { value, meta })
    }
}

impl From<(Position, &[char; 5])> for FieldResult<IndicatorsField> {
    fn from(value: (Position, &[char; 5])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        let value = chars.iter().filter(|c| **c != ' ').collect::<String>();
        Self::Ok(IndicatorsField { value, meta })
    }
}

impl PMixin for IndicatorsField {
    fn span(&self) -> Span {
        self.meta.span
    }
    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "Normal".to_string())]
    }
}
