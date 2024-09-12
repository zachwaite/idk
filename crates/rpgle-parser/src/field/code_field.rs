use std::fmt::Display;

use super::result::FieldResult;
use crate::free::{tokenize, Token};
use crate::meta::{Meta, PMixin, Position, Span};
use serde::{Deserialize, Serialize};

// raw
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawCodeField {
    pub value: String,
    pub meta: Meta,
}

impl Display for RawCodeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta.text)
    }
}
impl PMixin for RawCodeField {
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
        Self::Ok(RawCodeField {
            value: chars.iter().collect::<String>(),
            meta,
        })
    }
}

// cooked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeField {
    pub tokens: Vec<Token>,
}

impl Display for CodeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self
            .tokens
            .iter()
            .map(|t| t.to_string())
            .collect::<String>();
        write!(f, "{}", out)
    }
}
impl PMixin for CodeField {
    fn span(&self) -> Span {
        if self.tokens.len() == 0 {
            todo!()
        } else if self.tokens.len() == 1 {
            self.tokens[0].span()
        } else {
            let start = self.tokens[0].span();
            let end = self.tokens.last().expect("Token expected").span();
            Span::from((start, end))
        }
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        self.tokens
            .iter()
            .flat_map(|t| t.highlight())
            .collect::<Vec<(Span, String)>>()
    }
}
