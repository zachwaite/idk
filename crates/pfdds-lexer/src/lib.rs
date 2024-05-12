use log;
use std::{cell::RefCell, fmt};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Illegal,
    Eof,

    // identifiers and literals
    Ident,
    Int,

    // operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,
    Eq,
    NotEq,

    // delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // keywords
    Function,
    Let,
    Return,
    If,
    Else,
    True,
    False,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = match self {
            Self::Illegal => format!("Illegal"),
            Self::Eof => format!("Eof"),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span {
    start_row: u32,
    start_col: u32,
    end_row: u32,
    end_col: u32,
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
    pub fn new(kind: TokenKind, literal: &str) -> Self {
        Self {
            kind,
            text: literal.to_string(),
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
            index: 1,
            row: 1,
            col: 1,
        }
    }

    fn advance(&mut self) {
        self.index += 1;
        self.col += 1;
    }

    fn advance_and_return(&mut self) {
        self.index += 1;
        self.col = 1;
        self.row += 1;
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

    fn ch(&self) -> Option<&char> {
        let idx = self.position();
        self.input.get(idx)
    }

    fn read_char(&self) -> () {
        match self.ch() {
            Some('\n') => {
                self.state.borrow_mut().position.advance_and_return();
            }
            Some(_) => {
                self.state.borrow_mut().position.advance();
            }
            None => {
                // noop
            }
        }
    }

    fn peek_n(&self, n: usize) -> Option<&char> {
        self.input.get(self.position() + n)
    }

    fn peek_char(&self) -> Option<&char> {
        self.peek_n(1)
    }

    // pub fn next_token(&self) -> Result<Token> {
    //     let col = self.state.borrow().position.col;
    //     match col {
    //         1 => self.read_sequence(),
    //         7 => self.read_type(),
    //         45 => self.read_funcs(),
    //     }
    //     Ok(tok)
    // }
}
