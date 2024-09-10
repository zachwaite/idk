use std::fmt::Display;

use super::result::FieldResult;
use crate::free::{tokenize, Token};
use crate::meta::{PMixin, Position, Span};
use serde::{Deserialize, Serialize};

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

impl From<(Position, &[char; 15])> for FieldResult<Factor2Field> {
    fn from(value: (Position, &[char; 15])) -> Self {
        let pos = value.0;
        let chars = value.1;
        let tokens = tokenize(pos, chars);
        Self::Ok(Factor2Field { tokens })
    }
}

impl From<(Position, &[char; 66])> for FieldResult<Factor2Field> {
    fn from(value: (Position, &[char; 66])) -> Self {
        let pos = value.0;
        let chars = value.1;
        let tokens = tokenize(pos, chars);
        Self::Ok(Factor2Field { tokens })
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
