use std::fmt::Display;

use super::result::{FieldBehavior, FieldResult};
use crate::free::{tokenize_directive, DirectiveToken};
use crate::meta::{Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompilerDirectiveField {
    pub tokens: Vec<DirectiveToken>,
}
impl Display for CompilerDirectiveField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self
            .tokens
            .iter()
            .map(|t| t.to_string())
            .collect::<String>();
        write!(f, "{}", out)
    }
}
impl FieldBehavior for CompilerDirectiveField {
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
impl From<(Position, &[char; 94])> for FieldResult<CompilerDirectiveField> {
    fn from(value: (Position, &[char; 94])) -> Self {
        let pos = value.0;
        let chars = value.1;
        let tokens = tokenize_directive(pos, chars);
        Self::Ok(CompilerDirectiveField { tokens })
    }
}
