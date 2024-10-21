use crate::cst::CST;
use crate::field::FieldResult;
use crate::free::Op;
use crate::line::{CSpecLine, SpecLine};
use crate::meta::{PMixin, Span};
use crate::spec::{CSpec, DSpec, FSpec, HSpec, Spec};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Debug, Serialize, Deserialize)]
pub struct AST {
    pub specs: Vec<Spec>,
}

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

impl From<&CST> for AST {
    fn from(value: &CST) -> AST {
        let cst = value;
        let state = ParserState { idx: 0 };
        let parser = Parser {
            state: RefCell::new(state),
            input: cst
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
                .collect::<Vec<SpecLine>>(),
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

pub fn highlight_ast(ast: AST) -> Vec<((usize, usize), (usize, usize), String)> {
    let mut out = vec![];
    for spec in ast.specs.iter() {
        match spec {
            Spec::H(spec) => {
                out.append(&mut spec.highlight());
            }
            Spec::D(spec) => {
                out.append(&mut spec.highlight());
            }
            Spec::F(spec) => {
                out.append(&mut spec.highlight());
            }
            Spec::C(spec) => {
                out.append(&mut spec.highlight());
            }
            _ => continue,
        }
    }
    out.into_iter()
        .map(|tup| {
            (
                (tup.0.start.row, tup.0.start.col),
                (tup.0.end.row, tup.0.end.col),
                tup.1,
            )
        })
        .collect::<Vec<_>>()
}

pub fn query_definition(ast: &AST, pattern: &str) -> Option<Span> {
    for spec in ast.specs.iter() {
        if let Spec::D(dspec) = spec {
            if let FieldResult::Ok(namefield) = &dspec.name {
                if namefield.value.to_uppercase() == pattern.to_uppercase() {
                    return Some(namefield.meta.span);
                }
            }

            if let FieldResult::Ok(kwfield) = &dspec.keywords {
                for t in kwfield.tokens.iter() {
                    for m in t.metas.iter() {
                        if m.text.to_uppercase().contains(&pattern.to_uppercase()) {
                            return Some(dspec.sequence.span());
                        }
                    }
                }
            }
        }
        if let Spec::C(cspec) = spec {
            if let FieldResult::Ok(codefield) = &cspec.code {
                if let Op::Begsr { name, .. } = &codefield.op {
                    if name.trim().to_uppercase() == pattern.trim().to_uppercase() {
                        return Some(codefield.op.span());
                    }
                }
            }
        }
    }
    None
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
