use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use crate::line::DDSLine;
use crate::meta::{IHighlight, ISpan, Span};

#[derive(Debug)]
pub enum ParserException {
    LongLineException(String),
}

impl ParserException {
    pub fn long_line(line: &str) -> Self {
        let msg = format!("This line is too long to coerce to 80 chars: {}", line);
        Self::LongLineException(msg)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CST {
    pub lines: Vec<DDSLine>,
}
impl Display for CST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = self
            .lines
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", out)
    }
}
impl IHighlight for CST {
    fn highlight(&self) -> Vec<(crate::meta::Span, String)> {
        self.lines.iter().flat_map(|line| line.highlight()).collect::<Vec<(Span, String)>>()
    }
}
impl ISpan for CST {
    fn span(&self) -> Span {
        if self.lines.len() == 0 {
            Span::empty()
        } else if self.lines.len() == 1 {
            self.lines[0].span()
        } else {
            let start = self.lines[0].span();
            let end = self.lines.last().expect("Expected Span").span();
            Span::from((start, end))
        }
    }
}
impl TryFrom<&str> for CST {
    type Error = ParserException;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // check all lines are 80 chars long so we can safely convert to [char;80]
        // return early if not all meet this condition
        const N: usize = 80;
        let mut padded_lines: Vec<[char; N]> = vec![];
        for line in value.split("\n") {
            if line.len() == N {
                let rs: [char; N] = line.chars().collect::<Vec<char>>().try_into().unwrap();
                padded_lines.push(rs);
            } else if line.len() == 0 {
                continue;
            } else if line.len() < N {
                let mut rs: [char; N] = std::iter::repeat(' ')
                    .take(N)
                    .collect::<Vec<char>>()
                    .try_into()
                    .expect("Line shorter than 80 chars");
                for (i, char) in line.chars().enumerate() {
                    rs[i] = char;
                }
                padded_lines.push(rs);
            } else {
                return Err(ParserException::long_line(line));
            }
        }

        // parse each line into a ddsline
        let speclines = padded_lines
            .iter()
            .enumerate()
            .map(|(i, chars)| DDSLine::from((i, chars)))
            .collect::<Vec<DDSLine>>();

        Ok(CST { lines: speclines })
    }
}

pub fn highlight_cst(cst: &CST) -> Vec<((usize, usize), (usize, usize), String)> {
    cst.highlight().into_iter().map(|tup| {
        ((tup.0.start.row, tup.0.start.col),(tup.0.end.row, tup.0.end.col),tup.1)
    }).collect::<Vec<_>>()
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct AST {
//     pub record_format: RecordFormat,
//     pub fields: Vec<PFField>,
//     pub keys: Vec<PFKey>,
// }

#[cfg(test)]
mod tests {
    use super::*;
    use insta;
    use std::env;

    #[test]
    fn test_round_trip_snapshot() {
        let input = &r#"
     A*                                                                         
     A*   FILE         - Cow Event                                              
     A*   APPLICATION  - Dairy Farm Management                                  
     A*   DESCRIPTION  - Central Events File                                    
     A*                                                                         
     A**************************************************************************
                                                                                
     A          R EVTFMT                    TEXT('Event Fmt')                   
     A            ID             8  0       TEXT('Database ID')                 
     A            EDAT           6  0       TEXT('Event Date YYYYMMDD')         
     A            ETIM           6  0       TEXT('Event Time HHMMSS')           
     A            ETYP           8          TEXT('Event Type')                  
     A* PRIMARY KEY                                                             
     A          K ID                                                            "#
     [1..];
     let cst = CST::try_from(input).unwrap();
     insta::assert_yaml_snapshot!(cst);
     let observed = cst.to_string();
     let expected = input;
     if env::var("DEBUG").is_ok() {
         let _ = std::fs::write("/tmp/observed.rpgle", &observed);
         let _ = std::fs::write("/tmp/expected.rpgle", &expected);
         let _ = std::fs::write("/tmp/cst.txt", format!("{:#?}", &cst));
     }
     assert_eq!(observed, expected);
    }
}
