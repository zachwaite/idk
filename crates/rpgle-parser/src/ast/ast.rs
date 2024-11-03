use super::spec::{ast, ParseError, Spec};
use super::srcline::{srcline_from_specline, Srcline};
use crate::cst::CST;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AST {
    pub specs: Vec<Spec>,
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
