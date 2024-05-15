// use log;
use std::{cell::RefCell, fmt};
use thiserror::Error;

// use this for unrecoverable errors
#[derive(Error, Debug)]
pub enum IllegalLexerState {
    #[error("attempted to read beyond EOF")]
    ReadBeyondEOF,
    #[error("not implemented")]
    NotImplemented,
}

// use this for diagnostics
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LexerException {
    IncompletePositionalEntry,
    UnknownCommentPrefix,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SpecType {
    RecordFormat,
    Field,
    Key,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CommentType {
    NoComment,
    LineComment,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    // error
    Idk(LexerException),

    // end
    Eof,

    // positional tokens
    Sequence,
    FormType,
    Comment(CommentType),
    // Condition, NA for physical files
    SpecType(SpecType),
    // // identifiers and literals
    // Ident,
    // Int,

    // // operators
    // Assign,
    // Plus,
    // Minus,
    // Bang,
    // Asterisk,
    // Slash,

    // Lt,
    // Gt,
    // Eq,
    // NotEq,

    // // delimiters
    // Comma,
    // Semicolon,

    // LParen,
    // RParen,
    // LBrace,
    // RBrace,

    // // keywords
    // Function,
    // Let,
    // Return,
    // If,
    // Else,
    // True,
    // False,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = match self {
            Self::Idk(_) => format!("Idk"),
            Self::Eof => format!("Eof"),
            TokenKind::Sequence => format!("Sequence"),
            TokenKind::FormType => format!("FormType"),
            TokenKind::Comment(_) => format!("Comment"),
            TokenKind::SpecType(_) => format!("SpecType"),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span {
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!(
            "({}, {}) -> ({}, {})",
            self.start_row, self.start_col, self.end_row, self.end_col,
        );
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub span: Span,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!(
            "Token {{ kind: {}, literal: {} }}",
            self.kind.to_string(),
            self.text.clone()
        );
        write!(f, "{}", s)
    }
}

impl Token {
    pub fn new(kind: TokenKind, literal: &str, span: Span) -> Self {
        Self {
            kind,
            text: literal.to_string(),
            span,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    index: usize,
    row: usize,
    col: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!(
            "Position {{index: {}, row: {}, col: {}}}",
            self.index, self.row, self.col,
        );
        write!(f, "{}", s)
    }
}

impl Position {
    fn new() -> Self {
        Self {
            index: 0,
            row: 0,
            col: 0,
        }
    }

    fn advance(&mut self) {
        self.index += 1;
        self.col += 1;
    }

    fn advance_and_return(&mut self) {
        self.index += 1;
        self.col = 0;
        self.row += 1;
    }

    fn index_at(row: usize, col: usize) -> usize {
        let text_width = 80 + 1; // add 1 char for the newline
        row * text_width + col
    }
}

pub struct LexerState {
    position: Position,
    read_position: Position,
}

impl fmt::Display for LexerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!(
            "LexerState {{position: {}, read_position: {}}}",
            self.position.to_string(),
            self.read_position.to_string()
        );
        write!(f, "{}", s)
    }
}

pub struct Lexer {
    state: RefCell<LexerState>,
    input: Vec<char>,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let position = Position::new();
        let mut read_position = Position::new();
        read_position.advance();
        let state = LexerState {
            position,
            read_position,
        };
        let lex = Self {
            input: input.chars().into_iter().collect::<Vec<char>>(),
            state: RefCell::new(state),
        };
        return lex;
    }

    fn position(&self) -> usize {
        self.state.borrow().position.index
    }

    fn text_at(&self, span: Span) -> String {
        let start_pos = Position::index_at(span.start_row, span.start_col);
        let end_pos = Position::index_at(span.end_row, span.end_col);
        self.input[start_pos..end_pos].iter().collect::<String>()
    }

    fn ch(&self) -> Option<&char> {
        let idx = self.position();
        self.input.get(idx)
    }

    fn peek_n(&self, n: usize) -> Option<&char> {
        self.input.get(self.position() + n)
    }

    fn peek_char(&self) -> Option<&char> {
        self.peek_n(1)
    }

    fn read_char(&self) -> Result<(), IllegalLexerState> {
        match self.ch() {
            Some('\n') => {
                self.state.borrow_mut().position.advance_and_return();
                Ok(())
            }
            Some(_) => {
                self.state.borrow_mut().position.advance();
                Ok(())
            }
            None => Err(IllegalLexerState::ReadBeyondEOF),
        }
    }

    fn read_until_column(&self, col: usize) -> Result<(), IllegalLexerState> {
        // read until the cursor is on col
        while self.ch().is_some() && self.state.borrow().position.col < col {
            self.read_char()?;
        }
        Ok(())
    }

    fn read_until_next_line(&self) -> Result<(), IllegalLexerState> {
        // read until the cursor is at beginning of the next line
        let current_line = self.state.borrow().position.row;
        let next_line = current_line + 1;
        while self.ch().is_some() && self.state.borrow().position.row < next_line {
            self.read_char()?;
        }
        Ok(())
    }

    fn read_sequence(&self) -> Result<Token, IllegalLexerState> {
        let c = 5;
        let start_col = self.state.borrow().position.col;
        let row = self.state.borrow().position.row;
        self.read_until_column(c)?;
        let end_col = self.state.borrow().position.col;
        let span = Span {
            start_row: row,
            start_col,
            end_row: row,
            end_col,
        };
        let txt = self.text_at(span);
        if end_col == c {
            Ok(Token::new(TokenKind::Sequence, &txt, span))
        } else {
            let ex = LexerException::IncompletePositionalEntry;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }

    fn read_form_type(&self) -> Result<Token, IllegalLexerState> {
        let c = 6;
        let start_col = self.state.borrow().position.col;
        let row = self.state.borrow().position.row;
        self.read_until_column(c)?;
        let end_col = self.state.borrow().position.col;
        let span = Span {
            start_row: row,
            start_col,
            end_row: row,
            end_col,
        };
        let txt = self.text_at(span);
        if end_col == c {
            Ok(Token::new(TokenKind::FormType, &txt, span))
        } else {
            let ex = LexerException::IncompletePositionalEntry;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }

    fn read_comment(&self) -> Result<Token, IllegalLexerState> {
        let c = 7;
        let start_col = self.state.borrow().position.col;
        let row = self.state.borrow().position.row;
        self.read_until_column(c)?;
        let end_col = self.state.borrow().position.col;
        let span = Span {
            start_row: row,
            start_col,
            end_row: row,
            end_col,
        };
        let txt = self.text_at(span);
        // guard
        if end_col != c {
            let ex = LexerException::IncompletePositionalEntry;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            return Ok(tok);
        }
        let start_pos = Position::index_at(row, start_col);
        let end_pos = Position::index_at(row, end_col);
        match self.input[start_pos..end_pos] {
            [' '] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let nocomment = CommentType::NoComment;
                Ok(Token::new(TokenKind::Comment(nocomment), &txt, span))
            }
            ['*'] => {
                self.read_until_column(80)?;
                let end_col = self.state.borrow().position.col;
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let comment = CommentType::LineComment;
                self.read_until_next_line()?;
                Ok(Token::new(TokenKind::Comment(comment), &txt, span))
            }
            _ => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let ex = LexerException::UnknownCommentPrefix;
                Ok(Token::new(TokenKind::Idk(ex), &txt, span))
            }
        }
    }

    pub fn next_token(&self) -> Result<Token, IllegalLexerState> {
        let col = self.state.borrow().position.col;
        match col {
            0 => self.read_sequence(),
            5 => self.read_form_type(),
            6 => self.read_comment(),
            // 45 => self.read_funcs(),
            _ => Err(IllegalLexerState::NotImplemented),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = &r#"
00001A*   FILE         - Cow Born Event                                         
00002A*   APPLICATION  - Dairy Farm Management                                  
     A*   DESCRIPTION  - Record of cow birth                                    
     A*                                                                         
     A**************************************************************************
     A          R BORNFMT                   TEXT('Cow Born Fmt')                
     A            ID             8  0       TEXT('Database ID')                 
     A            EID            8  0       TEXT('Event ID')                    
     A            BNAME          8          TEXT('Barn Name')                   
     A            BDAT           8  0       TEXT('Birth Date')                  
     A* PRIMARY KEY                                                             
     A          K ID                                                            
"#[1..];
        let expected: Vec<Token> = vec![
            Token::new(
                TokenKind::Sequence,
                "00001",
                Span {
                    start_row: 0,
                    start_col: 0,
                    end_row: 0,
                    end_col: 5,
                },
            ),
            Token::new(
                TokenKind::FormType,
                "A",
                Span {
                    start_row: 0,
                    start_col: 5,
                    end_row: 0,
                    end_col: 6,
                },
            ),
            Token::new(
                TokenKind::Comment(CommentType::LineComment),
                "*   FILE         - Cow Born Event                                         ",
                Span {
                    start_row: 0,
                    start_col: 6,
                    end_row: 0,
                    end_col: 80,
                },
            ),
            Token::new(
                TokenKind::Sequence,
                "00002",
                Span {
                    start_row: 1,
                    start_col: 0,
                    end_row: 1,
                    end_col: 5,
                },
            ),
            Token::new(
                TokenKind::FormType,
                "A",
                Span {
                    start_row: 1,
                    start_col: 5,
                    end_row: 1,
                    end_col: 6,
                },
            ),
            Token::new(
                TokenKind::Comment(CommentType::LineComment),
                "*   APPLICATION  - Dairy Farm Management                                  ",
                Span {
                    start_row: 1,
                    start_col: 6,
                    end_row: 1,
                    end_col: 80,
                },
            ),
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start_row: 2,
                    start_col: 0,
                    end_row: 2,
                    end_col: 5,
                },
            ),
            Token::new(
                TokenKind::FormType,
                "A",
                Span {
                    start_row: 2,
                    start_col: 5,
                    end_row: 2,
                    end_col: 6,
                },
            ),
            Token::new(
                TokenKind::Comment(CommentType::LineComment),
                "*   DESCRIPTION  - Record of cow birth                                    ",
                Span {
                    start_row: 2,
                    start_col: 6,
                    end_row: 2,
                    end_col: 80,
                },
            ),
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start_row: 3,
                    start_col: 0,
                    end_row: 3,
                    end_col: 5,
                },
            ),
            Token::new(
                TokenKind::FormType,
                "A",
                Span {
                    start_row: 3,
                    start_col: 5,
                    end_row: 3,
                    end_col: 6,
                },
            ),
            Token::new(
                TokenKind::Comment(CommentType::LineComment),
                "*                                                                         ",
                Span {
                    start_row: 3,
                    start_col: 6,
                    end_row: 3,
                    end_col: 80,
                },
            ),
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start_row: 4,
                    start_col: 0,
                    end_row: 4,
                    end_col: 5,
                },
            ),
            Token::new(
                TokenKind::FormType,
                "A",
                Span {
                    start_row: 4,
                    start_col: 5,
                    end_row: 4,
                    end_col: 6,
                },
            ),
            Token::new(
                TokenKind::Comment(CommentType::LineComment),
                "**************************************************************************",
                Span {
                    start_row: 4,
                    start_col: 6,
                    end_row: 4,
                    end_col: 80,
                },
            ),
        ];
        let lexer = Lexer::new(input);
        for pair in expected.into_iter().enumerate() {
            let idx = pair.0.to_string();
            let expected_token = pair.1;
            let observed_token = lexer.next_token().unwrap();
            println!("{}", lexer.state.borrow().position);
            assert_eq!(observed_token, expected_token, "test #{}", idx);
        }
    }
}
