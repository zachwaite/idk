// Public API for CST
use super::legacy;
pub use super::legacy::ParseError;
use super::nvim::highlight_cst;
use super::srcline::{srcline_from_specline, Srcline};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CST {
    pub lines: Vec<Srcline>,
}

type SpanShape = ((usize, usize), (usize, usize));
impl CST {
    pub fn get_highlights(&self) -> Vec<(SpanShape, String)> {
        highlight_cst(self)
    }
}

pub fn parse_cst(input: &str) -> Result<CST, ParseError> {
    let oldcst = legacy::CST::try_from(input)?;
    let lines = oldcst
        .lines
        .iter()
        .map(|line| srcline_from_specline(line))
        .collect::<Vec<Srcline>>();
    Ok(CST { lines })
}
