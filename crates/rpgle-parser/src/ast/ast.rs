// Public API for AST
use super::nvim::{highlight_ast, query_definition};
use super::spec::{ast, ParseError, Spec};
use super::srcline::{srcline_from_specline, Srcline};
use crate::cst::CST;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AST {
    pub specs: Vec<Spec>,
}

type SpanShape = ((usize, usize), (usize, usize));
impl AST {
    pub fn get_highlights(&self) -> Vec<(SpanShape, String)> {
        highlight_ast(self)
    }

    pub fn try_get_definition(&self, pattern: &str) -> Option<SpanShape> {
        let span = query_definition(self, pattern)?;
        let start = (span.start.row, span.start.col);
        let end = (span.end.row, span.end.col);
        Some((start, end))
    }
}

pub fn parse_ast(cst: &CST) -> Result<AST, ParseError> {
    let mut lines = cst
        .lines
        .iter()
        .map(|line| srcline_from_specline(line))
        .collect::<Vec<Srcline>>();
    let (specs, _) = ast(&mut lines)?;
    Ok(AST { specs })
}
