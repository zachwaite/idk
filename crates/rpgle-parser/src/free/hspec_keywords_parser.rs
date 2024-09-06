use crate::field::PMixin;
use crate::meta::{Meta, Position};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HTokenKind {
    Idk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HToken {
    pub kind: HTokenKind,
    pub meta: Meta,
}

impl Display for HToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl PMixin for HToken {
    fn highlight(&self) -> Vec<(crate::Span, String)> {
        todo!()
    }

    fn span(&self) -> crate::Span {
        todo!()
    }
}

pub fn tokenize(pos: Position, chars: &[char; 94]) -> Vec<HToken> {
    let tok = HToken {
        kind: HTokenKind::Idk,
        meta: Meta::from((pos, chars.as_slice())),
    };
    vec![tok]
}
