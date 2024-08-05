use crate::meta::{Meta, Position, Span};

pub struct IdkField {
    pub value: String,
    pub meta: Meta,
}

impl From<(Position, &str)> for IdkField {
    fn from(value: (Position, &str)) -> Self {
        let start = value.0;
        let txt = value.1;
        let end = Position::from((start.row, txt.len()));
        let span = Span { start, end };
        let meta = Meta { span, digs: vec![] };
        Self {
            value: txt.to_string(),
            meta,
        }
    }
}
