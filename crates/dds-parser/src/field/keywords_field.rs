use std::fmt::Display;

use super::result::FieldResult;
use crate::meta::{Meta, IHighlight, ISpan, Position, Span};
use serde::{Deserialize, Serialize};
use crate::free::{RFToken, FToken, KToken, FEToken};

// raw
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawKeywordsField {
    pub value: String,
    pub meta: Meta,
}
impl From<(Position, &[char; 36])> for FieldResult<RawKeywordsField> {
    fn from(value: (Position, &[char; 36])) -> Self {
        let pos = value.0;
        let chars = value.1;
        let meta = Meta::from((pos, chars.as_slice()));
        Self::Ok(RawKeywordsField {
            value: chars.iter().collect::<String>(),
            meta,
        })
    }
}
impl Display for RawKeywordsField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta.text)
    }
}
impl IHighlight for RawKeywordsField {
    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.meta.span, "Error".to_string())]
    }
}
impl ISpan for RawKeywordsField {
    fn span(&self) -> Span {
        self.meta.span
    }
}

// recordformat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RFKeywordsField {
    pub tokens: Vec<RFToken>,
}
impl IHighlight for RFKeywordsField {
    fn highlight(&self) -> Vec<(Span, String)> {
        self.tokens.iter().flat_map(|t| t.highlight()).collect::<Vec<(Span, String)>>()
    }
}

// field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FKeywordsField {
    pub tokens: Vec<FToken>,
}
impl IHighlight for FKeywordsField {
    fn highlight(&self) -> Vec<(Span, String)> {
        self.tokens.iter().flat_map(|t| t.highlight()).collect::<Vec<(Span, String)>>()
    }
}

// keyfield
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KKeywordsField {
    pub tokens: Vec<KToken>,
}
impl IHighlight for KKeywordsField {
    fn highlight(&self) -> Vec<(Span, String)> {
        self.tokens.iter().flat_map(|t| t.highlight()).collect::<Vec<(Span, String)>>()
    }
}

// fileentry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FEKeywordsField {
    pub tokens: Vec<FEToken>,
}
impl IHighlight for FEKeywordsField {
    fn highlight(&self) -> Vec<(Span, String)> {
        self.tokens.iter().flat_map(|t| t.highlight()).collect::<Vec<(Span, String)>>()
    }
}
