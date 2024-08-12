use super::position::Position;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!(
            "({}, {}) -> ({}, {})",
            self.start.row, self.start.col, self.end.row, self.end.col,
        );
        write!(f, "{}", s)
    }
}

impl From<((usize, usize), (usize, usize))> for Span {
    fn from(value: ((usize, usize), (usize, usize))) -> Self {
        Self {
            start: Position::from(value.0),
            end: Position::from(value.1),
        }
    }
}

impl Span {
    pub fn empty() -> Self {
        Self {
            start: Position::empty(),
            end: Position::empty(),
        }
    }

    pub fn to_cover_both(span1: Self, span2: Self) -> Self {
        // preferable/necessary to unwrap these because the row and col numbers are usize
        // and offer perfect comparison, yet the PartialOrd trait doesn't know that bc
        // it is generic
        let start_position = [span1.start, span2.start].into_iter().min().unwrap();
        let end_position = [span1.end, span2.end].into_iter().max().unwrap();
        Self {
            start: start_position,
            end: end_position,
        }
    }
}
