// Public API for CST
pub use super::legacy::{highlight_cst, CST};

#[derive(Debug)]
pub enum ParseError {
    EmptyInput,
    Unhandled,
}

type SpanShape = ((usize, usize), (usize, usize));
impl CST {
    pub fn get_highlights(&self) -> Vec<(SpanShape, String)> {
        highlight_cst(self)
    }
}

// pub fn parse_cst(input: &str) -> Result<CST, ParseError> {
//     let mut lines = cst
//         .lines
//         .iter()
//         .map(|line| srcline_from_specline(line))
//         .collect::<Vec<Srcline>>();
//     let (specs, _) = ast(&mut lines)?;
//     Ok(AST { specs })
// }
pub fn parse_cst(input: &str) -> Result<CST, ParseError> {
    todo!()
}
