use super::position::Position;
use super::span::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub span: Span,
    pub text: String,
}

impl From<(Position, &[char])> for Meta {
    fn from(value: (Position, &[char])) -> Self {
        let start = value.0;
        let chars = value.1;
        let text = chars.iter().collect::<String>();
        let end = Position::from((start.row, start.col + chars.len()));
        let span = Span { start, end };
        Self { span, text }
    }
}
