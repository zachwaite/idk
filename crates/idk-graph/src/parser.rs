use crate::cst::{Call, Definition, Idk, Mutation, Program, Statement, StatementMeta};
use rpgle_lexer::{
    next_token, CommentType, FormType, IllegalLexerState, Lexer, Span, Token, TokenKind,
};
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

pub fn peek_n(parser: &Parser, n: usize) -> Result<Token, IllegalParserState> {
    let tokens = parser.active_buffer.borrow();
    match tokens.len() {
        0 | 1 => Err(IllegalParserState::EmptyTokenBufferError),
        len if len > n => Ok(tokens[n].clone()), // TODO: remove clone
        _ => {
            drop(tokens);
            fill_active_buffer(parser, n);

            let tokens = parser.active_buffer.borrow();
            Ok(tokens[n].clone())
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

fn parse_write(parser: &Parser) -> Result<Mutation, IllegalParserState> {
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
    let name = tok.text.trim().to_uppercase();
    let keyword = "Write".to_string();
    let out = Mutation {
        keyword,
        name,
        meta,
    };
    Ok(out)
}

fn parse_update(parser: &Parser) -> Result<Mutation, IllegalParserState> {
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
    let name = tok.text.trim().to_uppercase();
    let keyword = "Update".to_string();
    let out = Mutation {
        keyword,
        name,
        meta,
    };
    Ok(out)
}

fn parse_subroutine_definition(parser: &Parser) -> Result<Definition, IllegalParserState> {
    let mut meta = StatementMeta::empty();
    let mut is_free = false; // TDE: refactor
                             // signature
    let name = match front_kind(parser)? {
        TokenKind::Begsr => {
            meta.push_token(pop_active_buffer(parser)?); // Begsr
            while front_kind(parser)? != TokenKind::Identifier
                && front_kind(parser)? != TokenKind::Eof
            {
                meta.push_token(pop_active_buffer(parser)?);
            }
            let tok = pop_active_buffer(parser)?; // name
            meta.push_token(tok.clone());
            while front_kind(parser)? != TokenKind::Semicolon
                && front_kind(parser)? != TokenKind::Eof
            {
                meta.push_token(pop_active_buffer(parser)?);
            }
            is_free = true;
            tok.text.trim().to_string()
        }
        TokenKind::FormType(FormType::Calculation) => {
            meta.push_token(pop_active_buffer(parser)?); // C
            while front_kind(parser)? != TokenKind::Identifier
                && front_kind(parser)? != TokenKind::Eof
            {
                meta.push_token(pop_active_buffer(parser)?);
            }
            let tok = pop_active_buffer(parser)?; // name
            meta.push_token(tok.clone());
            while front_kind(parser)? != TokenKind::Eol && front_kind(parser)? != TokenKind::Eof {
                meta.push_token(pop_active_buffer(parser)?);
            }
            tok.text.trim().to_string()
        }
        _ => "".to_string(),
    };

    // body, calls, mutations
    let mut calls = vec![];
    let mut mutations = vec![];
    while front_kind(parser)? != TokenKind::Endsr && front_kind(parser)? != TokenKind::Eof {
        let kind = front_kind(parser)?;
        match kind {
            TokenKind::Write => {
                let mutation = parse_write(parser)?;
                meta.push_other(&mutation.meta);
                mutations.push(mutation);
            }
            TokenKind::Update => {
                let mutation = parse_update(parser)?;
                meta.push_other(&mutation.meta);
                mutations.push(mutation);
            }
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
    if is_free {
        while front_kind(parser)? != TokenKind::Semicolon && front_kind(parser)? != TokenKind::Eof {
            meta.push_token(pop_active_buffer(parser)?);
        }
    } else {
        // traditional
        while front_kind(parser)? != TokenKind::Eol && front_kind(parser)? != TokenKind::Eof {
            meta.push_token(pop_active_buffer(parser)?);
        }
    }

    // guard
    if matches!(front_kind(parser), Ok(TokenKind::Eof)) {
        return Err(IllegalParserState::ImpossibleDestinationError);
    }
    meta.push_token(pop_active_buffer(parser)?); // push semicolon

    let def = Definition {
        name,
        calls,
        mutations,
        meta,
    };
    Ok(def)
}

fn parse_statement(parser: &Parser) -> Result<Statement, IllegalParserState> {
    while !matches!(front_kind(parser), Ok(TokenKind::Begsr))
        && !matches!(front_kind(parser), Ok(TokenKind::Exsr))
        && !matches!(front_kind(parser), Ok(TokenKind::Write))
        && !matches!(front_kind(parser), Ok(TokenKind::Update))
        && !matches!(
            front_kind(parser),
            Ok(TokenKind::FormType(FormType::Calculation))
        )
        && !matches!(front_kind(parser), Ok(TokenKind::Eof))
    {
        shrug_and_advance(parser)?;
    }

    if parser.idk_buffer.borrow().len() > 0 {
        let idk = flush_idk_buffer(parser)?;
        return Ok(Statement::Idk(idk));
    }
    match front_kind(parser) {
        Ok(TokenKind::Begsr) | Ok(TokenKind::FormType(FormType::Calculation)) => {
            let def = parse_subroutine_definition(parser)?;
            Ok(Statement::Def(def))
        }
        Ok(TokenKind::Exsr) => {
            let call = parse_subroutine_call(parser)?;
            Ok(Statement::Call(call))
        }
        Ok(TokenKind::Write) => {
            let write = parse_write(parser)?;
            Ok(Statement::Mutation(write))
        }
        Ok(TokenKind::Update) => {
            let update = parse_update(parser)?;
            Ok(Statement::Mutation(update))
        }
        _ => Err(IllegalParserState::ImpossibleDestinationError),
    }
}

// level 0
pub fn parse_program(parser: &Parser) -> Result<Program, IllegalParserState> {
    let mut pgm = Program::new();
    while front_kind(parser)? != TokenKind::Eof {
        // guard against line comment for c-spec style
        let k2 = peek_n(parser, 1)?.kind;
        if matches!(k2, TokenKind::Comment(CommentType::LineComment)) {
            shrug_and_advance(parser)?;
        }
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
       Endsr;                                                                                       
"#[1..];
        let lexer = new_lexer(input);
        let parser = Parser::new(&lexer).unwrap();
        let rs = parse_program(&parser);
        assert_eq!(input, rs.unwrap().to_raw_text());
    }
}
