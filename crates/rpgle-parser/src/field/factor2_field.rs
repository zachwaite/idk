use std::fmt::Display;

use super::result::FieldResult;
use crate::free::Token;
use crate::meta::{Meta, PMixin, Position, Span};
use nonempty::NonEmpty;
use serde::{Deserialize, Serialize};

// raw
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawFactor2Field {
    pub value: NonEmpty<char>,
    pub meta: Meta,
}

impl Display for RawFactor2Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta.text)
    }
}

impl From<(Position, &[char; 13])> for FieldResult<RawFactor2Field> {
    fn from(value: (Position, &[char; 13])) -> Self {
        let pos = value.0;
        let chars = value.1;
        let meta = Meta::from((pos, chars.as_slice()));
        let value = NonEmpty::from_vec(chars.iter().map(|c| *c).collect::<Vec<char>>())
            .expect("&[char; 13] is guaranteed to be nonempty");
        Self::Ok(RawFactor2Field { value, meta })
    }
}

impl From<(Position, &[char; 65])> for FieldResult<RawFactor2Field> {
    fn from(value: (Position, &[char; 65])) -> Self {
        let pos = value.0;
        let chars = value.1;
        let meta = Meta::from((pos, chars.as_slice()));
        let value = NonEmpty::from_vec(chars.iter().map(|c| *c).collect::<Vec<char>>())
            .expect("&[char; 65] is guaranteed to be nonempty");
        Self::Ok(RawFactor2Field { value, meta })
    }
}

impl PMixin for RawFactor2Field {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "Normal".to_string())]
    }
}

// cooked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Factor2Field {
    pub tokens: Vec<Token>,
}
impl Display for Factor2Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self
            .tokens
            .iter()
            .map(|t| t.to_string())
            .collect::<String>();
        write!(f, "{}", out)
    }
}
impl PMixin for Factor2Field {
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
