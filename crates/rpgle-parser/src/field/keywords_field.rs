use nonempty::NonEmpty;
use std::fmt::Display;

use super::result::{FieldBehavior, FieldResult};
use crate::free::{DToken, FToken, HToken};
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};

// raw
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RawKeywordsField {
    pub value: NonEmpty<char>,
    pub meta: Meta,
}
impl Display for RawKeywordsField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta.text)
    }
}
impl FieldBehavior for RawKeywordsField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "Error".to_string())]
    }
}
impl From<(Position, &[char; 57])> for FieldResult<RawKeywordsField> {
    fn from(value: (Position, &[char; 57])) -> Self {
        let pos = value.0;
        let chars = value.1;
        let meta = Meta::from((pos, chars.as_slice()));
        let value = NonEmpty::from_vec(chars.iter().map(|c| *c).collect::<Vec<char>>())
            .expect("&[char; 57] is guaranteed to be nonempty");
        Self::Ok(RawKeywordsField { value, meta })
    }
}
impl From<(Position, &[char; 94])> for FieldResult<RawKeywordsField> {
    fn from(value: (Position, &[char; 94])) -> Self {
        let pos = value.0;
        let chars = value.1;
        let meta = Meta::from((pos, chars.as_slice()));
        let value = NonEmpty::from_vec(chars.iter().map(|c| *c).collect::<Vec<char>>())
            .expect("&[char; 94] is guaranteed to be nonempty");
        Self::Ok(RawKeywordsField { value, meta })
    }
}

// hspec
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HKeywordsField {
    pub tokens: Vec<HToken>,
}
impl Display for HKeywordsField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self
            .tokens
            .iter()
            .map(|t| t.to_string())
            .collect::<String>();
        write!(f, "{}", out)
    }
}
impl FieldBehavior for HKeywordsField {
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

// fspec
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FKeywordsField {
    pub tokens: Vec<FToken>,
}
impl FieldBehavior for FKeywordsField {
    fn span(&self) -> Span {
        todo!()
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        self.tokens
            .iter()
            .flat_map(|t| t.highlight())
            .collect::<Vec<(Span, String)>>()
    }
}

// dspec
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DKeywordsField {
    pub tokens: Vec<DToken>,
}
impl FieldBehavior for DKeywordsField {
    fn span(&self) -> Span {
        todo!()
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        self.tokens
            .iter()
            .flat_map(|t| t.highlight())
            .collect::<Vec<(Span, String)>>()
    }
}
