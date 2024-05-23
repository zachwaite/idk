mod cst;

use cst::{
    Comment, DDSEntry, EntryAttributeError, EntryMeta, Field, Idk, Key, PhysicalFile, RecordFormat,
};
use pfdds_lexer::{
    CommentType, IllegalLexerState, Lexer, NameType, Span, Token, TokenKind, TokenMeta,
};
use std::{cell::RefCell, collections::VecDeque};
use thiserror::Error;

// use this for unrecoverable errors
#[derive(Error, Debug)]
pub enum IllegalParserState {
    #[error("lexer error")]
    IllegalLexerStateError(#[from] IllegalLexerState),
    #[error("empty token buffer")]
    EmptyTokenBufferError,
    #[error("attempted to access nonexistant token")]
    TokenBufferIndexError,
    #[error("Token for {0} is required and not found")]
    MissingRequiredTokenError(TokenKind),
    #[error("DDSEntry parsing must end on EOL or EOF token. Found {0}")]
    ExpectedEolEofError(TokenKind),
    #[error("Reached Impossible Destination!")]
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
            None => Err(IllegalParserState::EmptyTokenBufferError),
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
        let mut meta = EntryMeta::empty();
        for t in self.idk_buffer.borrow_mut().drain(0..) {
            meta.push_token(t);
        }
        let idk = Idk { meta };
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

    // parsers - level 2 (DDSEntry Members)
    pub fn parse_line_comment(&self) -> Result<Comment, IllegalParserState> {
        // you are on a sequence token and have a line comment token in front of you
        let mut meta = EntryMeta::empty();

        // guard
        if !matches!(
            self.peek_n(2),
            Ok(TokenMeta {
                kind: TokenKind::Comment(CommentType::LineComment),
                ..
            })
        ) {
            return Err(IllegalParserState::MissingRequiredTokenError(
                TokenKind::Comment(CommentType::LineComment),
            ));
        }

        meta.push_token(self.pop_active_buffer()?); // sequence
        meta.push_token(self.pop_active_buffer()?); // formtype
        let tok = self.pop_active_buffer()?; // comment
        meta.push_token(tok.clone());
        let comment = Comment {
            text: tok.text.clone(),
            meta,
        };
        Ok(comment)
    }

    pub fn parse_record_format(&self) -> Result<RecordFormat, IllegalParserState> {
        // you are on a sequence token and have a line comment token in front of you
        let mut meta = EntryMeta::empty();

        // guard
        if !matches!(
            self.peek_n(4),
            Ok(TokenMeta {
                kind: TokenKind::NameType(NameType::RecordFormat),
                ..
            })
        ) {
            return Err(IllegalParserState::MissingRequiredTokenError(
                TokenKind::NameType(NameType::RecordFormat),
            ));
        }

        meta.push_token(self.pop_active_buffer()?); // sequence
        meta.push_token(self.pop_active_buffer()?); // formtype
        meta.push_token(self.pop_active_buffer()?); // comment
        meta.push_token(self.pop_active_buffer()?); // condition
        meta.push_token(self.pop_active_buffer()?); // nametype
        meta.push_token(self.pop_active_buffer()?); // reserved
        let tok = self.pop_active_buffer()?; // name
        meta.push_token(tok.clone());
        meta.push_token(self.pop_active_buffer()?); // referencetype
        meta.push_token(self.pop_active_buffer()?); // lengthtype
        meta.push_token(self.pop_active_buffer()?); // datatype
        meta.push_token(self.pop_active_buffer()?); // decimal
        meta.push_token(self.pop_active_buffer()?); // usage
        meta.push_token(self.pop_active_buffer()?); // location
        meta.push_token(self.pop_active_buffer()?); // idk/keyword args
        let name = match tok.text.trim() {
            "" => Err(EntryAttributeError::MissingRequiredAttribute(
                "name".to_string(),
            )),
            n => Ok(n.to_string()),
        };
        let record_format = RecordFormat { name, meta };

        Ok(record_format)
    }

    pub fn parse_key(&self) -> Result<Key, IllegalParserState> {
        // you are on a sequence token and have a line comment token in front of you
        let mut meta = EntryMeta::empty();

        // guard
        if !matches!(
            self.peek_n(4),
            Ok(TokenMeta {
                kind: TokenKind::NameType(NameType::Key),
                ..
            })
        ) {
            return Err(IllegalParserState::MissingRequiredTokenError(
                TokenKind::NameType(NameType::Key),
            ));
        }

        meta.push_token(self.pop_active_buffer()?); // sequence
        meta.push_token(self.pop_active_buffer()?); // formtype
        meta.push_token(self.pop_active_buffer()?); // comment
        meta.push_token(self.pop_active_buffer()?); // condition
        meta.push_token(self.pop_active_buffer()?); // nametype
        meta.push_token(self.pop_active_buffer()?); // reserved
        let tok = self.pop_active_buffer()?; // name
        meta.push_token(tok.clone());
        meta.push_token(self.pop_active_buffer()?); // referencetype
        meta.push_token(self.pop_active_buffer()?); // lengthtype
        meta.push_token(self.pop_active_buffer()?); // datatype
        meta.push_token(self.pop_active_buffer()?); // decimal
        meta.push_token(self.pop_active_buffer()?); // usage
        meta.push_token(self.pop_active_buffer()?); // location
        meta.push_token(self.pop_active_buffer()?); // idk/keyword args

        let name = match tok.text.trim() {
            "" => Err(EntryAttributeError::MissingRequiredAttribute(
                "name".to_string(),
            )),
            n => Ok(n.to_string()),
        };
        let key = Key { name, meta };
        Ok(key)
    }

    pub fn parse_field(&self) -> Result<Field, IllegalParserState> {
        // you are on a sequence token and have a line comment token in front of you
        let mut meta = EntryMeta::empty();

        // guard
        if !matches!(
            self.peek_n(6),
            Ok(TokenMeta {
                kind: TokenKind::Name,
                ..
            })
        ) {
            return Err(IllegalParserState::MissingRequiredTokenError(
                TokenKind::Name,
            ));
        }

        meta.push_token(self.pop_active_buffer()?); // sequence
        meta.push_token(self.pop_active_buffer()?); // formtype
        meta.push_token(self.pop_active_buffer()?); // comment
        meta.push_token(self.pop_active_buffer()?); // condition
        meta.push_token(self.pop_active_buffer()?); // nametype
        meta.push_token(self.pop_active_buffer()?); // reserved
        let tok = self.pop_active_buffer()?; // name
        meta.push_token(tok.clone());
        meta.push_token(self.pop_active_buffer()?); // referencetype
        meta.push_token(self.pop_active_buffer()?); // lengthtype
        meta.push_token(self.pop_active_buffer()?); // datatype
        meta.push_token(self.pop_active_buffer()?); // decimal
        meta.push_token(self.pop_active_buffer()?); // usage
        meta.push_token(self.pop_active_buffer()?); // location
        meta.push_token(self.pop_active_buffer()?); // idk/keyword args
                                                    //
        let name = match tok.text.trim() {
            "" => Err(EntryAttributeError::MissingRequiredAttribute(
                "name".to_string(),
            )),
            n => Ok(n.to_string()),
        };
        let field = Field { name, meta };
        Ok(field)
    }

    // parsers - level 1 (DDSEntry)
    pub fn parse_entry(&self) -> Result<DDSEntry, IllegalParserState> {
        // an entry should start with a sequence token and match some peeks

        // ensure you're starting on a sequence and the idk buf is empty
        self.shrug_and_advance_until(TokenKind::Sequence)?;
        if self.idk_buffer.borrow().len() > 0 {
            let idk = self.flush_idk_buffer()?;
            return Ok(DDSEntry::Idk(idk));
        }

        // comment
        if matches!(
            self.peek_n(2),
            Ok(TokenMeta {
                kind: TokenKind::Comment(CommentType::LineComment),
                ..
            })
        ) {
            let comment = self.parse_line_comment()?;
            return Ok(DDSEntry::Comment(comment));
        }

        // record format
        if matches!(
            self.peek_n(4),
            Ok(TokenMeta {
                kind: TokenKind::NameType(NameType::RecordFormat),
                ..
            })
        ) {
            let record_format = self.parse_record_format()?;
            return Ok(DDSEntry::RecordFormat(record_format));
        }

        // key
        if matches!(
            self.peek_n(4),
            Ok(TokenMeta {
                kind: TokenKind::NameType(NameType::Key),
                ..
            })
        ) {
            let key = self.parse_key()?;
            return Ok(DDSEntry::Key(key));
        }

        // field
        if matches!(
            self.peek_n(6),
            Ok(TokenMeta {
                kind: TokenKind::Name,
                ..
            })
        ) {
            let field = self.parse_field()?;
            return Ok(DDSEntry::Field(field));
        }

        // IDK anything else
        self.shrug_and_advance_until(TokenKind::Eol)?;
        if self.idk_buffer.borrow().len() > 0 {
            let idk = self.flush_idk_buffer()?;
            return Ok(DDSEntry::Idk(idk));
        }

        Err(IllegalParserState::ImpossibleDestinationError)
    }

    // parsers - level 0
    pub fn parse_physical_file(&self) -> Result<PhysicalFile, IllegalParserState> {
        let mut file = PhysicalFile { entries: vec![] };

        while self.front_kind()? != TokenKind::Eof {
            let new_entry = self.parse_entry()?;
            file.entries.push(new_entry);

            // guard to ensure you're ending on a newline
            if self.front_kind()? == TokenKind::Eol {
                // toss the newline, the display of the PhysicalFile assumes it and joins,
                self.pop_active_buffer()?;
            } else if self.front_kind()? != TokenKind::Eof {
                return Err(IllegalParserState::ExpectedEolEofError(self.front_kind()?));
            }
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
        let rs = parser.parse_physical_file();
        // match &rs {
        //     Ok(file) => println!("\n\n```dds\n{}\n```\n", file.to_raw_text()),
        //     Err(e) => panic!("\nERROR: {}\n", e),
        // }
        assert_eq!(input, rs.unwrap().to_raw_text())
    }
}
