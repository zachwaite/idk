use crate::line::SpecLine;
use crate::meta::{PMixin, Span};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::usize;

#[derive(Debug)]
pub enum ParseError {
    LongLineException(String),
}

impl ParseError {
    pub fn long_line(line: &str) -> Self {
        let msg = format!("This line is too long to coerce to 100 chars: {}", line);
        Self::LongLineException(msg)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CST {
    pub lines: Vec<SpecLine>,
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

impl PMixin for CST {
    fn highlight(&self) -> Vec<(Span, String)> {
        self.lines
            .iter()
            .flat_map(|line| line.highlight())
            .collect::<Vec<(Span, String)>>()
    }

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
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // check all lines are 100 chars long so we can safely convert to [char;100]
        // return early if not all meet this condition
        let mut padded_lines: Vec<[char; 100]> = vec![];
        for line in value.split("\n") {
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

        // parse each line into a specline
        // speclines have a context granularity of "line" and could be parallelized
        let speclines = padded_lines
            .iter()
            .enumerate()
            .map(|(i, chars)| SpecLine::from((i, chars)))
            .collect::<Vec<SpecLine>>();

        Ok(CST { lines: speclines })
    }
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
        let cst = CST::try_from(input.as_str()).unwrap();
        insta::assert_yaml_snapshot!(cst);
    }

    #[test]
    fn test_cst_round_trip() {
        let input = dfmslike_fixture();
        let cst = CST::try_from(input.as_str()).unwrap();
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
