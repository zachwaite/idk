use std::{cell::RefCell, fmt};
use thiserror::Error;

// use this for unrecoverable errors
#[derive(Error, Debug)]
pub enum IllegalLexerState {
    #[error("attempted to read beyond EOF")]
    ReadBeyondEOF,
    #[error("Impossible Destination!")]
    ImpossibleDestination,
}

// use this for diagnostics
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LexerException {
    IncompletePositionalEntry,
    UnknownCommentPrefix,
    NotImplemented,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CommentType {
    InlineComment,
    LineComment,
    NoComment,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FormType {
    Empty,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    // error
    Idk(LexerException),

    // end
    Eof,
    Eol,
    // mode switchers
    FullFree,
    Free,
    EndFree,

    // positional tokens
    Sequence,
    Comment(CommentType),
    FormType(FormType),
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = match self {
            Self::Idk(_) => format!("Idk"),
            Self::Eof => format!("Eof"),
            Self::Eol => format!("Eol"),
            Self::FullFree => format!("FullFree"),
            Self::Free => format!("Free"),
            Self::EndFree => format!("EndFree"),
            Self::Sequence => format!("Sequence"),
            Self::Comment(_) => format!("Comment"),
            Self::FormType(_) => format!("FormType"),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub idx: usize,
    pub row: usize,
    pub col: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!(
            "Position {{index: {}, row: {}, col: {}}}",
            self.idx, self.row, self.col,
        );
        write!(f, "{}", s)
    }
}

impl Position {
    pub fn empty() -> Self {
        Self {
            idx: 0,
            row: 0,
            col: 0,
        }
    }

    pub fn new(row: usize, col: usize, idx: usize) -> Self {
        Self { idx, row, col }
    }

    fn advance(&mut self) {
        self.idx += 1;
        self.col += 1;
    }

    fn advance_and_return(&mut self) {
        self.idx += 1;
        self.col = 0;
        self.row += 1;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!(
            "({}, {}) -> ({}, {})",
            self.start.row, self.start.col, self.end.row, self.end.col,
        );
        write!(f, "{}", s)
    }
}

impl Span {
    pub fn empty() -> Self {
        Self {
            start: Position::empty(),
            end: Position::empty(),
        }
    }

    pub fn to_cover_both(span1: Self, span2: Self) -> Self {
        let start_position = if span1.start.idx <= span2.start.idx {
            span1.start
        } else {
            span2.start
        };
        let end_position = if span1.end.idx > span2.end.idx {
            span1.end
        } else {
            span2.end
        };
        Self {
            start: start_position,
            end: end_position,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TokenMeta {
    pub kind: TokenKind,
    pub span: Span,
}

impl From<&Token> for TokenMeta {
    fn from(t: &Token) -> Self {
        Self {
            kind: t.kind,
            span: t.span,
        }
    }
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
pub enum LexerMode {
    Init,
    FullFree,
    Free,
    Fixed,
}

pub struct LexerState {
    pub position: Position,
    pub read_position: Position,
    pub mode: LexerMode,
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
    pub state: RefCell<LexerState>,
    pub input: Vec<char>,
}

pub fn text_at(lexer: &Lexer, span: Span) -> String {
    let start_pos = span.start.idx;
    let end_pos = span.end.idx;
    lexer.input[start_pos..end_pos].iter().collect::<String>()
}

pub fn ch(lexer: &Lexer) -> Option<&char> {
    let idx = lexer.state.borrow().position.idx;
    lexer.input.get(idx)
}

pub fn peek_n(lexer: &Lexer, n: usize) -> Option<&char> {
    let idx = lexer.state.borrow().position.idx;
    lexer.input.get(idx + n)
}

pub fn peek(lexer: &Lexer) -> Option<&char> {
    peek_n(lexer, 1)
}

pub fn read_char(lexer: &Lexer) -> Result<(), IllegalLexerState> {
    match ch(lexer) {
        Some('\n') => {
            lexer.state.borrow_mut().position.advance_and_return();
            Ok(())
        }
        Some(_) => {
            lexer.state.borrow_mut().position.advance();
            Ok(())
        }
        None => Err(IllegalLexerState::ReadBeyondEOF),
    }
}

pub fn read_until_column(lexer: &Lexer, col: usize) -> Result<(), IllegalLexerState> {
    // read until the cursor is on col
    while ch(lexer).is_some() && lexer.state.borrow().position.col < col {
        read_char(lexer)?;
    }
    Ok(())
}

pub fn read_until_end_of_line(lexer: &Lexer) -> Result<(), IllegalLexerState> {
    while !matches!(ch(lexer), Some('\n')) {
        read_char(lexer)?;
    }
    Ok(())
}

pub fn read_until_end_of_file(lexer: &Lexer) -> Result<(), IllegalLexerState> {
    while ch(lexer).is_some() {
        read_char(lexer)?;
    }
    Ok(())
}

pub fn new_lexer(input: &str) -> Lexer {
    let position = Position::empty();
    let mut read_position = Position::empty();
    read_position.advance();
    let state = LexerState {
        position,
        read_position,
        mode: LexerMode::Init,
    };
    let lex = Lexer {
        input: input.chars().into_iter().collect::<Vec<char>>(),
        state: RefCell::new(state),
    };
    if let Some('*') = peek(&lex) {
        lex.state.borrow_mut().mode = LexerMode::FullFree;
    } else {
        lex.state.borrow_mut().mode = LexerMode::Fixed;
    }
    return lex;
}
