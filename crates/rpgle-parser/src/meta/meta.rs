use super::diagnostic::Diagnostic;
use super::position::Position;
use super::span::Span;

pub struct Meta {
    pub span: Span,
    pub digs: Vec<Diagnostic>,
}

impl From<((usize, usize), (usize, usize))> for Meta {
    fn from(value: ((usize, usize), (usize, usize))) -> Self {
        let start = Position::from(value.0);
        let end = Position::from(value.1);
        let span = Span { start, end };
        Self { span, digs: vec![] }
    }
}
