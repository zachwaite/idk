use std::fmt::Display;

use super::result::FieldResult;
use super::IdkField;
use crate::free::{tokenize, HToken};
use crate::meta::{Meta, PMixin, Position, Span};
use serde::{Deserialize, Serialize};

// hspec
#[derive(Debug, Clone, Serialize, Deserialize)]
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
impl PMixin for HKeywordsField {
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
impl From<(Position, &[char; 94])> for FieldResult<HKeywordsField> {
    fn from(value: (Position, &[char; 94])) -> Self {
        let pos = value.0;
        let chars = value.1;
        let tokens = tokenize(pos, chars);
        Self::Ok(HKeywordsField { tokens })
    }
}

// generic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordsField {
    pub value: String,
    pub meta: Meta,
}
impl Display for KeywordsField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}
impl PMixin for KeywordsField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "Error".to_string())]
    }
}

// for f-spec keywords
impl From<(Position, &[char; 57])> for FieldResult<KeywordsField> {
    fn from(value: (Position, &[char; 57])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        match chars[0] {
            '*' => {
                let value = chars.iter().collect::<String>();
                Self::Ok(KeywordsField { value, meta })
            }
            _ => {
                let fld = IdkField::from((value.0, chars.as_slice()));
                Self::Idk(fld)
            }
        }
    }
}

// for d-spec keywords
impl From<(Position, &[char; 58])> for FieldResult<KeywordsField> {
    fn from(value: (Position, &[char; 58])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        match chars[0] {
            '*' => {
                let value = chars.iter().collect::<String>();
                Self::Ok(KeywordsField { value, meta })
            }
            _ => {
                let fld = IdkField::from((value.0, chars.as_slice()));
                Self::Idk(fld)
            }
        }
    }
}
