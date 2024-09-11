use std::fmt::Display;

use super::result::FieldResult;
use crate::free::{
    tokenize_dspec_kw, tokenize_fspec_kw, tokenize_hspec_kw, DToken, FToken, HToken,
};
use crate::meta::{Meta, PMixin, Position, Span};
use serde::{Deserialize, Serialize};

// raw
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawKeywordsField {
    pub value: String,
    pub meta: Meta,
}
impl Display for RawKeywordsField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta.text)
    }
}
impl PMixin for RawKeywordsField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "Normal".to_string())]
    }
}
impl From<(Position, &[char; 57])> for FieldResult<RawKeywordsField> {
    fn from(value: (Position, &[char; 57])) -> Self {
        let pos = value.0;
        let chars = value.1;
        let meta = Meta::from((pos, chars.as_slice()));
        Self::Ok(RawKeywordsField {
            value: chars.iter().collect::<String>(),
            meta,
        })
    }
}

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
        let tokens = tokenize_hspec_kw(pos, chars);
        Self::Ok(HKeywordsField { tokens })
    }
}

// fspec
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FKeywordsField {
    pub tokens: Vec<FToken>,
}
impl Display for FKeywordsField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self
            .tokens
            .iter()
            .map(|t| t.to_string())
            .collect::<String>();
        write!(f, "{}", out)
    }
}
impl PMixin for FKeywordsField {
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
impl From<(Position, &[char; 57])> for FieldResult<FKeywordsField> {
    fn from(value: (Position, &[char; 57])) -> Self {
        let pos = value.0;
        let chars = value.1;
        let tokens = tokenize_fspec_kw(pos, chars);
        Self::Ok(FKeywordsField { tokens })
    }
}

// dspec
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DKeywordsField {
    pub tokens: Vec<DToken>,
}
impl Display for DKeywordsField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self
            .tokens
            .iter()
            .map(|t| t.to_string())
            .collect::<String>();
        write!(f, "{}", out)
    }
}
impl PMixin for DKeywordsField {
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
