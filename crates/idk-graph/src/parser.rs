use crate::cst::{Call, Definition, Idk, Program, Statement, StatementMeta};
use rpgle_lexer::{next_token, IllegalLexerState, Lexer, Span, Token, TokenKind, TokenMeta};
use std::{cell::RefCell, collections::VecDeque};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IllegalParserState {
    #[error("lexer error")]
    IllegalLexerStateError(#[from] IllegalLexerState),
    #[error("empty token buffer")]
    EmptyTokenBufferError,
    #[error("attempted to access nonexistant token")]
    TokenBufferIndexError,
    #[error("Impossible Destination!")]
    ImpossibleDestinationError,
}

pub struct Parser<'a> {
    pub lexer: &'a Lexer,
    pub active_buffer: RefCell<VecDeque<Token>>,
    pub idk_buffer: RefCell<VecDeque<Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a Lexer) -> Result<Self, IllegalParserState> {
        let mut active_buffer = VecDeque::new();
        active_buffer.push_back(next_token(lexer)?);
        active_buffer.push_back(next_token(lexer)?);
        let idk_buffer = VecDeque::new();
        let parser = Self {
            lexer,
            active_buffer: RefCell::new(active_buffer),
            idk_buffer: RefCell::new(idk_buffer),
        };
        Ok(parser)
    }
}

fn fill_active_buffer(parser: &Parser, n: usize) {
    let mut tokens = parser.active_buffer.borrow_mut();
    let len = tokens.len();
    for _ in 0..(n + 1 - len) {
        let tok = match next_token(parser.lexer) {
            Ok(tok) => tok,
            Err(_) => Token::new(TokenKind::Eof, "", Span::empty()),
        };
        tokens.push_back(tok);
    }
}

fn pop_active_buffer(parser: &Parser) -> Result<Token, IllegalParserState> {
    if parser.active_buffer.borrow().len() <= 2 {
        fill_active_buffer(parser, 2);
    }
    match parser.active_buffer.borrow_mut().pop_front() {
        Some(tok) => Ok(tok),
        None => Err(IllegalParserState::EmptyTokenBufferError),
    }
}

fn front_kind(parser: &Parser) -> Result<TokenKind, IllegalParserState> {
    match parser.active_buffer.borrow().get(0) {
        Some(tok) => Ok(tok.kind),
        None => Err(IllegalParserState::TokenBufferIndexError),
    }
}

fn peek_n(parser: &Parser, n: usize) -> Result<TokenMeta, IllegalParserState> {
    let len = parser.active_buffer.borrow().len();
    match len {
        0 | 1 => Err(IllegalParserState::TokenBufferIndexError),
        _ => {
            fill_active_buffer(parser, n);
            let meta = TokenMeta::from(&parser.active_buffer.borrow()[n]);
            Ok(meta)
        }
    }
}

fn flush_idk_buffer(parser: &Parser) -> Result<Idk, IllegalParserState> {
    let mut meta = StatementMeta::empty();
    for t in parser.idk_buffer.borrow_mut().drain(0..) {
        meta.push_token(t);
    }
    let idk = Idk { meta };
    Ok(idk)
}

fn shrug_and_advance(parser: &Parser) -> Result<(), IllegalParserState> {
    let token = pop_active_buffer(parser)?;
    parser.idk_buffer.borrow_mut().push_back(token);
    Ok(())
}

fn shrug_and_advance_until(parser: &Parser, kind: TokenKind) -> Result<(), IllegalParserState> {
    while front_kind(parser)? != kind && front_kind(parser)? != TokenKind::Eof {
        shrug_and_advance(parser)?;
    }
    Ok(())
}

// level 1

fn parse_subroutine_call(parser: &Parser) -> Result<Call, IllegalParserState> {
    let mut meta = StatementMeta::empty();
    meta.push_token(pop_active_buffer(parser)?); // Exsr
    while front_kind(parser)? != TokenKind::Identifier && front_kind(parser)? != TokenKind::Eof {
        meta.push_token(pop_active_buffer(parser)?);
    }
    let tok = pop_active_buffer(parser)?; // name
    meta.push_token(tok.clone());
    while front_kind(parser)? != TokenKind::Semicolon && front_kind(parser)? != TokenKind::Eof {
        meta.push_token(pop_active_buffer(parser)?);
    }
    let name = tok.text.trim().to_string();
    let call = Call { name, meta };
    Ok(call)
}

fn parse_subroutine_definition(parser: &Parser) -> Result<Definition, IllegalParserState> {
    let mut meta = StatementMeta::empty();
    // signature
    meta.push_token(pop_active_buffer(parser)?); // Begsr
    while front_kind(parser)? != TokenKind::Identifier && front_kind(parser)? != TokenKind::Eof {
        meta.push_token(pop_active_buffer(parser)?);
    }
    let tok = pop_active_buffer(parser)?; // name
    meta.push_token(tok.clone());
    while front_kind(parser)? != TokenKind::Semicolon && front_kind(parser)? != TokenKind::Eof {
        meta.push_token(pop_active_buffer(parser)?);
    }
    let name = tok.text.trim().to_string();

    // body + calls
    let mut calls = vec![];
    while front_kind(parser)? != TokenKind::Endsr && front_kind(parser)? != TokenKind::Eof {
        let kind = front_kind(parser)?;
        match kind {
            TokenKind::Exsr => {
                let call = parse_subroutine_call(parser)?;
                meta.push_other(&call.meta);
                calls.push(call);
            }
            _ => {
                meta.push_token(pop_active_buffer(parser)?);
            }
        }
    }

    // end
    let tok = pop_active_buffer(parser)?; // Endsr
    meta.push_token(tok.clone());
    while front_kind(parser)? != TokenKind::Semicolon && front_kind(parser)? != TokenKind::Eof {
        meta.push_token(pop_active_buffer(parser)?);
    }

    // guard
    if matches!(front_kind(parser), Ok(TokenKind::Eof)) {
        return Err(IllegalParserState::ImpossibleDestinationError);
    }
    meta.push_token(pop_active_buffer(parser)?); // push semicolon

    let def = Definition { name, calls, meta };
    Ok(def)
}

fn parse_statement(parser: &Parser) -> Result<Statement, IllegalParserState> {
    while !matches!(front_kind(parser), Ok(TokenKind::Begsr))
        && !matches!(front_kind(parser), Ok(TokenKind::Exsr))
        && !matches!(front_kind(parser), Ok(TokenKind::Eof))
    {
        shrug_and_advance(parser)?;
    }
    if parser.idk_buffer.borrow().len() > 0 {
        let idk = flush_idk_buffer(parser)?;
        return Ok(Statement::Idk(idk));
    }
    match front_kind(parser) {
        Ok(TokenKind::Begsr) => {
            let def = parse_subroutine_definition(parser)?;
            Ok(Statement::Def(def))
        }
        Ok(TokenKind::Exsr) => {
            let call = parse_subroutine_call(parser)?;
            Ok(Statement::Call(call))
        }
        _ => Err(IllegalParserState::ImpossibleDestinationError),
    }
}

// level 0
pub fn parse_program(parser: &Parser) -> Result<Program, IllegalParserState> {
    let mut pgm = Program::new();
    while front_kind(parser)? != TokenKind::Eof {
        let new_stmt = parse_statement(parser)?;
        pgm.statements.push(new_stmt);
    }
    Ok(pgm)
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse_program, Parser};

    use rpgle_lexer::new_lexer;

    #[test]
    fn test_round_trip() {
        let input = &r#"
     H OPTION(*nodebugio:*srcstmt)                                                                  
     FCowEvt    UF A E           K DISK                                                             
     FBornEvt   UF A E           K DISK                                                             
     FCowEvtL2  IF   E           K DISK     Rename(EVTFMT:VEVTFMT) Prefix(V)                        
     F**********************************************************************************************
     D**********************************************************************************************
     D LastId          S              8  0                                                          
     C**********************************************************************************************
      /free                                                                                         
       Exsr $SetLstId;                                                                              
       Exsr $CrtEvts;                                                                               
       *inlr = *on;                                                                                 
                                                                                                    
       Begsr $SetLstId;                                                                             
         SetLL *Loval CowEvtL2;                                                                     
         If Not %Eof;                                                                               
           Read CowEvtL2;                                                                           
           LastId = Vid;                                                                            
         Else;                                                                                      
          LastId = 1;                                                                               
         Endif;                                                                                     
       Endsr;                                                                                       
                                                                                                    
       Begsr $CrtBrnEvt;                                                                            
         EID = Id;                                                                                  
         BNAME = 'BESSE';                                                                           
         BDAT = 20240101;                                                                           
         Write BORNFMT;                                                                             
       Endsr;                                                                                       
                                                                                                    
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
       Endsr;                                                                                       
"#[1..];
        let lexer = new_lexer(input);
        let parser = Parser::new(&lexer).unwrap();
        let rs = parse_program(&parser);
        assert_eq!(input, rs.unwrap().to_raw_text());
    }
}
