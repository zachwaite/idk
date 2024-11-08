// Public API for CST
use super::nvim::highlight_cst;
use super::srcline::{srcline, Srcline};

use serde::{Deserialize, Serialize};
use std::fmt::Display;

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

impl Display for CST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self
            .lines
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", out)
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
            // TDE: map error better
            return Err(ParseError::Unhandled);
        }
    }
    Ok(CST { lines })
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta;
    use std::env;

    fn dfmslike_fixture() -> String {
        r#"
     H OPTION(*nodebugio:*srcstmt)                                                                  
     FCowEvt    UF A E           K DISK                                                             
     FBornEvt   UF A E           K DISK                                                             
     FCowEvtL2  IF   E           K DISK     Rename(EVTFMT:VEVTFMT)                                  
     F                                     Prefix(V)                                                
     F**********************************************************************************************
     D**********************************************************************************************
     D LastId          S              8  0                                                          
     D QCmdExc         PR                  EXTPGM('QCMDEXC')                                        
     D  Command                    2000                                                             
     D  Length                       15  5                                                          
     C**********************************************************************************************
      /free                                                                                         
       Exsr $SetLstId;                                                                              
       Exsr $CrtEvts;                                                                               
       QCmdExc(Foo:Bar);                                                                            
       *inlr = *on;                                                                                 
                                                                                                    
       Begsr $SetLstId;                                                                             
         SetLL *Loval CowEvtL2;                                                                     
         If Not %Eof;                                                                               
           Read CowEvtL2;                                                                           
             QCmdExc(FOO:BaR);                                                                      
           LastId = Vid;                                                                            
         Else;                                                                                      
          LastId = 1;                                                                               
         Endif;                                                                                     
       Endsr;                                                                                       
                                                                                                    
     C     $CrtBRNEVT    BegSr                                                                      
         EID = Id;                                                                                  
         BNAME = 'BESSE';                                                                           
         BDAT = 20240101;                                                                           
         Write BORNFMT;                                                                             
     C                   ENDSR                                                                      
                                                                                                    
       Begsr $CrtCowEvt;                                                                            
         Id = LastId + 1;                                                                           
         Edat = 20240101;                                                                           
         Etim = 125959;                                                                             
         Etyp = 'BORN';                                                                             
         Write EVTFMT;                                                                              
       Endsr;                                                                                       
                                                                                                    
       Begsr $CrtEvts;                                                                              
         Exsr $CrtCowEvt;                                                                           
         Exsr $CrtBrnEvt;                                                                           
       Endsr;                                                                                       "#
            [1..].to_string()
    }

    #[test]
    fn test_cst_snapshot() {
        let input = dfmslike_fixture();
        let cst = parse_cst(input.as_str()).unwrap();
        insta::assert_yaml_snapshot!(cst);
    }

    #[test]
    fn test_cst_round_trip() {
        let input = dfmslike_fixture();
        let cst = parse_cst(input.as_str()).unwrap();
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
