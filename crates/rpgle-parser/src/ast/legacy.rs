use super::core::AST;
use crate::cst::CST;
use crate::line::{CSpecLine, SpecLine};
use crate::spec::{CSpec, DSpec, FSpec, HSpec, Spec};
use std::cell::RefCell;

struct ParserState {
    idx: usize,
}

// TDE: use lifetime
struct Parser {
    state: RefCell<ParserState>,
    input: Vec<SpecLine>,
}

fn peek_n(parser: &Parser, n: usize) -> Option<&SpecLine> {
    let idx = parser.state.borrow().idx;
    parser.input.get(idx + n)
}

fn read_line(parser: &Parser) -> &SpecLine {
    let out = peek_n(parser, 0).expect("read_line() requires a length check prior to call");
    parser.state.borrow_mut().idx += 1;
    out
}

impl From<&CST> for AST {
    fn from(value: &CST) -> AST {
        let cst = value;
        let inputs = cst
            .lines
            .iter()
            .filter(|line| match line {
                SpecLine::HSpec(_) => true,
                SpecLine::DSpec(_) => true,
                SpecLine::DSpecContinuation(_) => true,
                SpecLine::FSpec(_) => true,
                SpecLine::FSpecContinuation(_) => true,
                SpecLine::CSpec(_) => true,
                _ => false,
            })
            .map(|line| line.clone())
            .collect::<Vec<SpecLine>>();

        let state = ParserState { idx: 0 };
        let parser = Parser {
            state: RefCell::new(state),
            input: inputs,
        };
        let mut specs = vec![];
        loop {
            match next_spec(&parser) {
                Some(spec) => {
                    specs.push(spec);
                }
                None => break,
            }
        }
        AST { specs }
    }
}

fn next_spec(parser: &Parser) -> Option<Spec> {
    match peek_n(parser, 0) {
        Some(_) => {
            let specline = read_line(parser);
            match specline {
                SpecLine::DSpec(cur) => {
                    let mut continuations = vec![];
                    loop {
                        match peek_n(parser, 0) {
                            Some(specline) => match specline {
                                SpecLine::DSpecContinuation(peeked) => {
                                    let _ = read_line(parser);
                                    continuations.push(peeked);
                                    continue;
                                }
                                SpecLine::Idk(_) | SpecLine::Comment(_) => {
                                    let _ = read_line(parser);
                                    continue;
                                }
                                _ => {
                                    break;
                                }
                            },
                            None => {
                                break;
                            }
                        }
                    }
                    Some(Spec::D(DSpec::from((cur, continuations))))
                }
                SpecLine::HSpec(cur) => {
                    let spec = HSpec::from((cur, vec![]));
                    Some(Spec::H(spec))
                }
                SpecLine::FSpec(cur) => {
                    let mut continuations = vec![];
                    loop {
                        match peek_n(parser, 0) {
                            Some(specline) => match specline {
                                SpecLine::FSpecContinuation(peeked) => {
                                    let _ = read_line(parser);
                                    continuations.push(peeked);
                                    continue;
                                }
                                SpecLine::Idk(_) | SpecLine::Comment(_) => {
                                    let _ = read_line(parser);
                                    continue;
                                }
                                _ => {
                                    break;
                                }
                            },
                            None => {
                                break;
                            }
                        }
                    }
                    Some(Spec::F(FSpec::from((cur, continuations))))
                }
                SpecLine::CSpec(cur) => {
                    let spec = match cur {
                        CSpecLine::Traditional(line) => CSpec::from(line),
                        CSpecLine::ExtF2(line) => CSpec::from((line, vec![])),
                        CSpecLine::Free(line) => CSpec::from((line, vec![])),
                    };
                    Some(Spec::C(spec))
                }
                _ => None,
            }
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta;
    use std::env;

    #[test]
    fn test_ast_snapshot() {
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
        let ast = AST::from(&cst);
        insta::assert_yaml_snapshot!(ast);
    }
}
