use crate::line::{IdkSpecLine, SpecLine};
use crate::spec::{CSpec, CommentSpec, DSpec, FSpec, HSpec, IdkSpec, Spec};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug)]
pub enum ParserException {
    LongLineException(String),
}

impl ParserException {
    pub fn long_line(line: &str) -> Self {
        let msg = format!("This line is too long to coerce to 100 chars: {}", line);
        Self::LongLineException(msg)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CST {
    pub specs: Vec<Spec>,
}

impl Display for CST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self
            .specs
            .iter()
            .map(|spec| spec.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", out)
    }
}

impl TryFrom<&str> for CST {
    type Error = ParserException;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // check all lines are 100 chars long so we can safely convert to [char;100]
        // return early if not all meet this condition
        let mut padded_lines: Vec<[char; 100]> = vec![];
        for line in value.split("\n") {
            if line.len() == 100 {
                let rs: [char; 100] = line.chars().collect::<Vec<char>>().try_into().unwrap();
                padded_lines.push(rs);
            } else if line.len() < 100 {
                let mut rs: [char; 100] = std::iter::repeat(' ')
                    .take(100)
                    .collect::<Vec<char>>()
                    .try_into()
                    .unwrap();
                for (i, char) in line.chars().enumerate() {
                    rs[i] = char;
                }
                padded_lines.push(rs);
            } else {
                return Err(ParserException::long_line(line));
            }
        }

        // parse each line into a specline
        // speclines have a context granularity of "line" and could be parallelized
        let speclines = padded_lines
            .iter()
            .enumerate()
            .map(|(i, chars)| SpecLine::from((i, chars)))
            .collect::<Vec<SpecLine>>();

        // parse speclines into specs
        // this is a reduction and adjacent lines need to know about each other
        let cst = CST::from(speclines);
        Ok(cst)
    }
}

impl From<Vec<SpecLine>> for CST {
    fn from(value: Vec<SpecLine>) -> Self {
        let lines = value;
        let mut specs: Vec<Spec> = vec![];
        let mut i = 0;
        while i < lines.len() {
            let cur = &lines[i];
            match cur {
                SpecLine::Idk(line) => {
                    let spec = IdkSpec { line: line.clone() };
                    specs.push(Spec::Idk(spec));
                    i += 1;
                }
                SpecLine::Comment(line) => {
                    let spec = CommentSpec { line: line.clone() };
                    specs.push(Spec::Comment(spec));
                    i += 1;
                }
                SpecLine::HSpec(line) => {
                    let spec = HSpec {
                        line: line.clone(),
                        continuations: vec![],
                    };
                    specs.push(Spec::H(spec));
                    i += 1;
                }
                SpecLine::FSpec(line) => {
                    let spec = FSpec {
                        line: line.clone(),
                        continuations: vec![],
                    };
                    i += 1;
                    specs.push(Spec::F(spec));
                }
                SpecLine::FSpecContinuation(line) => {
                    if let Some(Spec::F(spec)) = specs.last_mut() {
                        // if the last spec is an fspec, this continues it
                        spec.continuations.push(line.clone());
                        i += 1;
                    } else {
                        // if there is no prior last spec or it is not an fspec, then cast to idk
                        let raw: (usize, [char; 100]) = line.to_raw();
                        let spec = IdkSpec {
                            line: IdkSpecLine::from((raw.0, &raw.1)),
                        };
                        specs.push(Spec::Idk(spec));
                        i += 1;
                    }
                }
                SpecLine::DSpec(line) => {
                    let spec = DSpec {
                        line: line.clone(),
                        continuations: vec![],
                    };
                    i += 1;
                    specs.push(Spec::D(spec));
                }
                SpecLine::DSpecContinuation(line) => {
                    if let Some(Spec::D(spec)) = specs.last_mut() {
                        spec.continuations.push(line.clone());
                        i += 1;
                    } else {
                        let raw: (usize, [char; 100]) = line.to_raw();
                        let spec = IdkSpec {
                            line: IdkSpecLine::from((raw.0, &raw.1)),
                        };
                        specs.push(Spec::Idk(spec));
                        i += 1;
                    }
                }
                SpecLine::CSpec(line) => {
                    let spec = CSpec {
                        line: line.clone(),
                        continuations: vec![],
                    };
                    i += 1;
                    specs.push(Spec::C(spec));
                }
            };
        }

        Self { specs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta;
    use std::env;

    #[test]
    fn test_round_trip_snapshot() {
        let input = &r#"
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
            [1..];
        let cst = CST::try_from(input).unwrap();
        insta::assert_yaml_snapshot!(cst);
        let observed = cst.to_string();
        let expected = input;
        if env::var("DEBUG").is_ok() {
            let _ = std::fs::write("/tmp/observed.rpgle", &observed);
            let _ = std::fs::write("/tmp/expected.rpgle", &expected);
            let _ = std::fs::write(
                "/tmp/specs.txt",
                cst.specs
                    .iter()
                    .map(|spec| format!("{}\n", spec.kind()))
                    .collect::<String>(),
            );
        }
        assert_eq!(observed, expected);
    }
}
