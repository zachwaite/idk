mod cst;

use cst::{Comment, DDSEntry, Idk, ParserException, PhysicalFile};
use pfdds_lexer::{IllegalLexerState, Lexer, Span, Token, TokenKind, TokenMeta};
use std::{cell::RefCell, collections::VecDeque};
use thiserror::Error;

// use this for unrecoverable errors
#[derive(Error, Debug)]
pub enum IllegalParserState {
    #[error("lexer error")]
    IllegalLexerState(#[from] IllegalLexerState),
    #[error("empty token buffer")]
    EmptyTokenBuffer,
    #[error("attempted to access nonexistant token")]
    TokenBufferIndexError,
    #[error("not implemented")]
    NotImplemented,
}

pub struct Parser<'a> {
    pub lexer: &'a Lexer,
    pub active_buffer: RefCell<VecDeque<Token>>,
    pub idk_buffer: RefCell<VecDeque<Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a Lexer) -> Result<Self, IllegalParserState> {
        let mut active_buffer = VecDeque::new();
        active_buffer.push_back(lexer.next_token()?);
        active_buffer.push_back(lexer.next_token()?);
        let idk_buffer = VecDeque::new();
        let parser = Self {
            lexer,
            active_buffer: RefCell::new(active_buffer),
            idk_buffer: RefCell::new(idk_buffer),
        };
        Ok(parser)
    }

    pub fn fill_active_buffer(&self, n: usize) {
        let mut tokens = self.active_buffer.borrow_mut();
        let len = tokens.len();
        for _ in 0..(n + 1 - len) {
            let tok = match self.lexer.next_token() {
                Ok(tok) => tok,
                Err(_) => {
                    let tok = Token::new(TokenKind::Eof, "", Span::empty());
                    tok
                }
            };
            tokens.push_back(tok);
        }
    }

    pub fn pop_active_buffer(&self) -> Result<Token, IllegalParserState> {
        if self.active_buffer.borrow().len() <= 2 {
            self.fill_active_buffer(2);
        }
        match self.active_buffer.borrow_mut().pop_front() {
            Some(tok) => Ok(tok),
            None => Err(IllegalParserState::EmptyTokenBuffer),
        }
    }

    pub fn front_kind(&self) -> Result<TokenKind, IllegalParserState> {
        let len = self.active_buffer.borrow().len();
        if len == 0 {
            return Err(IllegalParserState::TokenBufferIndexError);
        }
        let kind = self.active_buffer.borrow()[0].kind;
        Ok(kind)
    }

    pub fn peek_n(&self, n: usize) -> Result<TokenMeta, IllegalParserState> {
        let len = self.active_buffer.borrow().len();
        match len {
            0 | 1 => Err(IllegalParserState::TokenBufferIndexError),
            _ => {
                self.fill_active_buffer(n);
                let meta = TokenMeta::from(&self.active_buffer.borrow()[n]);
                Ok(meta)
            }
        }
    }

    pub fn flush_idk_buffer(&self) -> Result<Idk, IllegalParserState> {
        let mut combined_text = String::new();
        let mut combined_span = Span::empty();
        for x in self.idk_buffer.borrow_mut().drain(0..) {
            combined_text.push_str(&x.text);
            combined_span = Span::to_cover_both(combined_span, x.span);
        }
        let idk = Idk {
            exception: ParserException::NotImplemented,
            text: combined_text,
            span: combined_span,
        };
        Ok(idk)
    }

    pub fn shrug_and_advance(&self) -> Result<(), IllegalParserState> {
        let token = self.pop_active_buffer()?;
        self.idk_buffer.borrow_mut().push_back(token);
        Ok(())
    }

    pub fn shrug_and_advance_until(&self, kind: TokenKind) -> Result<(), IllegalParserState> {
        while self.front_kind()? != kind && self.front_kind()? != TokenKind::Eof {
            self.shrug_and_advance()?;
        }
        Ok(())
    }
    pub fn parse_sequence(&self) -> Result<Sequence, Idk> {}

    pub fn parse_comment_entry(&self) -> Result<DDSEntry, IllegalParserState> {
        let mut combined_text = String::new();
        let mut combined_span = Span::empty();

        let tok = self.pop_active_buffer()?;

        Err(IllegalParserState::NotImplemented)
    }

    // parsers
    pub fn parse_entry(&self) -> Result<Vec<DDSEntry>, IllegalParserState> {
        let mut out: Vec<DDSEntry> = vec![];

        // guard
        self.shrug_and_advance_until(TokenKind::Sequence)?;
        if self.idk_buffer.borrow().len() > 0 {
            let idk = self.flush_idk_buffer()?;
            out.push(DDSEntry::Idk(idk));
        }

        // stopped on sequence
        // check for comment
        if matches!(
            self.peek_n(2),
            Ok(TokenMeta {
                kind: TokenKind::Comment(_),
                ..
            })
        ) {
            return Ok(vec![self.parse_comment_entry()?]);
        }

        // check for record format or key
        // check for name
        // fall back to idk

        self.shrug_and_advance_until(TokenKind::Eol)?;
        Ok(out)
    }

    pub fn parse_physical_file(&self) -> Result<PhysicalFile, IllegalParserState> {
        let mut file = PhysicalFile { entries: vec![] };

        while self.front_kind()? != TokenKind::Eof {
            let mut new_entries = self.parse_entry()?;
            file.entries.extend(new_entries.drain(0..))
        }

        Ok(file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop() {
        let input = &r#"
00001A*   FILE         - Cow Born Event                                         
00002A*   APPLICATION  - Dairy Farm Management                                  
     A*   DESCRIPTION  - Record of cow birth                                    
     A*                                                                         
     A**************************************************************************
     A          R BORNFMT                   TEXT('Cow Born Fmt')                
     A            ID             8  0       TEXT('Database ID')                 
     A            BNAME          8          TEXT('Barn Name')                   
     A          K ID                                                            
"#[1..];
        let lexer = Lexer::new(input);
        let parser = Parser::new(&lexer).unwrap();
        let file = parser.parse_physical_file().unwrap();
        println!("{}", file);
    }
}
