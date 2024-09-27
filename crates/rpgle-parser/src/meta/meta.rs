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
impl From<(&Meta, Vec<&Meta>)> for Meta {
    fn from(value: (&Meta, Vec<&Meta>)) -> Self {
        let start = value.0.span.start;
        let mut text = value.0.text.clone();
        let span = if let Some(last) = value.1.last() {
            Span {
                start,
                end: last.span.end,
            }
        } else {
            Span {
                start,
                end: value.0.span.end,
            }
        };
        for meta in value.1.iter() {
            text.push_str(&meta.text);
        }
        Self { span, text }
    }
}
