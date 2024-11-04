// Public API for CST
use super::nvim::highlight_cst;
use super::srcline::{srcline, Srcline};

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

#[derive(Debug)]
pub enum ParseError {
    LongLineException(String),
    Unhandled,
}

impl ParseError {
    pub fn long_line(line: &str) -> Self {
        let msg = format!("This line is too long to coerce to 100 chars: {}", line);
        Self::LongLineException(msg)
    }
}

pub fn parse_cst(input: &str) -> Result<CST, ParseError> {
    // check all lines are 100 chars long so we can safely convert to [char;100]
    // return early if not all meet this condition
    let mut padded_lines: Vec<[char; 100]> = vec![];
    for line in input.split("\n") {
        if line.len() == 100 {
            let rs: [char; 100] = line.chars().collect::<Vec<char>>().try_into().unwrap();
            padded_lines.push(rs);
        } else if line.len() == 0 {
            continue;
        } else if line.len() < 100 {
            let mut rs: [char; 100] = std::iter::repeat(' ')
                .take(100)
                .collect::<Vec<char>>()
                .try_into()
                .expect("Line shorter than 100 chars");
            for (i, char) in line.chars().enumerate() {
                rs[i] = char;
            }
            padded_lines.push(rs);
        } else {
            return Err(ParseError::long_line(line));
        }
    }

    // parse each line into a srcline
    // srclines have a context granularity of "line" and could be parallelized
    let mut lines: Vec<Srcline> = vec![];
    for input in padded_lines.iter().enumerate() {
        if let Ok(line) = srcline(input.0, input.1) {
            lines.push(line);
        } else {
            // TODO: map error better
            return Err(ParseError::Unhandled);
        }
    }
    Ok(CST { lines })
}
