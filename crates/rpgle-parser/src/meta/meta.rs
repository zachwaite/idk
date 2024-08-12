use super::diagnostic::Diagnostic;
use super::position::Position;
use super::span::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub span: Span,
    pub text: String,
    pub digs: Vec<Diagnostic>,
}

impl From<(Position, &[char])> for Meta {
    fn from(value: (Position, &[char])) -> Self {
        let start = value.0;
        let chars = value.1;
        let text = chars.iter().collect::<String>();
        let end = Position::from((start.row, chars.len()));
        let span = Span { start, end };
        let digs = vec![];
        Self { span, text, digs }
    }
}
