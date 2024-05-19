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
    UnknownFormType,
    UnknownCommentPrefix,
    UnknownNameType,
    UnknownReservedValue,
    UnknownReferenceType,
    UnknownDataType,
    UnknownUsageType,
    NotImplemented,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FormType {
    Blank,
    A,
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
pub enum NameType {
    RecordFormat,
    Field,
    Key,
    SelectOmit,
}

impl fmt::Display for NameType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::RecordFormat => format!("RecordFormat"),
            Self::Field => format!("Field"),
            Self::Key => format!("Key"),
            Self::SelectOmit => format!("SelectOmit"),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReferenceType {
    Reference,
    NotReference,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LengthType {
    Length,
    NoLength,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DataType {
    NotSpecified,
    PackedDecimal,
    ZonedDecimal,
    Binary,
    FloatingPoint,
    Character,
    Hexadecimal,
    Date,
    Time,
    Timestamp,
    BinaryCharacter,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DecimalType {
    NoDecimal,
    Decimal,
    DecimalOverride,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UsageType {
    Blank,       // Blank = B or nothing if not a field row
    InputOutput, // B
    Input,       // I
    Neither,     // N
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    // error
    Idk(LexerException),

    // end
    Eof,

    // positional tokens
    Sequence,
    FormType(FormType),
    Comment(CommentType),
    Condition, // NA for physical files
    SpecType(SpecType),
    NameType(NameType),
    Reserved,
    Name,
    ReferenceType(ReferenceType),
    Length(LengthType),
    DataType(DataType),
    Decimal(DecimalType),
    Usage(UsageType),
    Location,
    PlusContinuation,
    MinusContinuation,
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
            TokenKind::FormType(_) => format!("FormType"),
            TokenKind::Comment(_) => format!("Comment"),
            TokenKind::SpecType(_) => format!("SpecType"),
            TokenKind::Condition => format!("Condition"),
            TokenKind::NameType(_) => format!("NameType"),
            TokenKind::Reserved => format!("Reserved"),
            TokenKind::Name => format!("Name"),
            TokenKind::ReferenceType(_) => format!("ReferenceType"),
            TokenKind::Length(_) => format!("LengthType"),
            TokenKind::DataType(_) => format!("DataType"),
            TokenKind::Decimal(_) => format!("Decimal"),
            TokenKind::Usage(_) => format!("Usage"),
            TokenKind::Location => format!("Location"),
            TokenKind::PlusContinuation => format!("PlusContinuation"),
            TokenKind::MinusContinuation => format!("MinusContinuation"),
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
                let typ = FormType::Blank;
                Ok(Token::new(TokenKind::FormType(typ), &txt, span))
            }
            ['A'] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = FormType::A;
                Ok(Token::new(TokenKind::FormType(typ), &txt, span))
            }
            _ => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let ex = LexerException::UnknownFormType;
                let tok = Token::new(TokenKind::Idk(ex), &txt, span);
                return Ok(tok);
            }
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

    fn read_condition(&self) -> Result<Token, IllegalLexerState> {
        let c = 16;
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
            Ok(Token::new(TokenKind::Condition, &txt, span))
        } else {
            let ex = LexerException::IncompletePositionalEntry;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }

    fn read_name_type(&self) -> Result<Token, IllegalLexerState> {
        let c = 17;
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
                let typ = NameType::Field;
                Ok(Token::new(TokenKind::NameType(typ), &txt, span))
            }
            ['R'] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = NameType::RecordFormat;
                Ok(Token::new(TokenKind::NameType(typ), &txt, span))
            }
            ['K'] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = NameType::Key;
                Ok(Token::new(TokenKind::NameType(typ), &txt, span))
            }
            _ => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let ex = LexerException::UnknownNameType;
                let tok = Token::new(TokenKind::Idk(ex), &txt, span);
                return Ok(tok);
            }
        }
    }

    fn read_reserved(&self) -> Result<Token, IllegalLexerState> {
        let c = 18;
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
                Ok(Token::new(TokenKind::Reserved, &txt, span))
            }
            _ => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let ex = LexerException::UnknownReservedValue;
                let tok = Token::new(TokenKind::Idk(ex), &txt, span);
                return Ok(tok);
            }
        }
    }

    fn read_name(&self) -> Result<Token, IllegalLexerState> {
        let c = 28;
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
            Ok(Token::new(TokenKind::Name, &txt, span))
        } else {
            let ex = LexerException::IncompletePositionalEntry;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }

    fn read_reference_type(&self) -> Result<Token, IllegalLexerState> {
        let c = 29;
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
                let typ = ReferenceType::NotReference;
                Ok(Token::new(TokenKind::ReferenceType(typ), &txt, span))
            }
            ['R'] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = ReferenceType::NotReference;
                Ok(Token::new(TokenKind::ReferenceType(typ), &txt, span))
            }
            _ => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let ex = LexerException::UnknownReferenceType;
                let tok = Token::new(TokenKind::Idk(ex), &txt, span);
                return Ok(tok);
            }
        }
    }

    fn read_length(&self) -> Result<Token, IllegalLexerState> {
        let c = 34;
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
            [' ', ' ', ' ', ' ', ' '] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = LengthType::NoLength;
                Ok(Token::new(TokenKind::Length(typ), &txt, span))
            }
            _ => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = LengthType::Length;
                Ok(Token::new(TokenKind::Length(typ), &txt, span))
            }
        }
    }

    fn read_data_type(&self) -> Result<Token, IllegalLexerState> {
        let c = 35;
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
                let typ = DataType::NotSpecified;
                Ok(Token::new(TokenKind::DataType(typ), &txt, span))
            }
            ['P'] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = DataType::PackedDecimal;
                Ok(Token::new(TokenKind::DataType(typ), &txt, span))
            }
            ['S'] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = DataType::ZonedDecimal;
                Ok(Token::new(TokenKind::DataType(typ), &txt, span))
            }
            ['B'] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = DataType::Binary;
                Ok(Token::new(TokenKind::DataType(typ), &txt, span))
            }
            ['F'] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = DataType::FloatingPoint;
                Ok(Token::new(TokenKind::DataType(typ), &txt, span))
            }
            ['A'] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = DataType::Character;
                Ok(Token::new(TokenKind::DataType(typ), &txt, span))
            }
            ['H'] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = DataType::Hexadecimal;
                Ok(Token::new(TokenKind::DataType(typ), &txt, span))
            }
            ['L'] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = DataType::Date;
                Ok(Token::new(TokenKind::DataType(typ), &txt, span))
            }
            ['T'] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = DataType::Time;
                Ok(Token::new(TokenKind::DataType(typ), &txt, span))
            }
            ['Z'] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = DataType::Timestamp;
                Ok(Token::new(TokenKind::DataType(typ), &txt, span))
            }
            ['5'] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = DataType::BinaryCharacter;
                Ok(Token::new(TokenKind::DataType(typ), &txt, span))
            }
            _ => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let ex = LexerException::UnknownDataType;
                let tok = Token::new(TokenKind::Idk(ex), &txt, span);
                return Ok(tok);
            }
        }
    }

    fn read_decimal(&self) -> Result<Token, IllegalLexerState> {
        let c = 37;
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
            ['+', _] | ['-', _] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = DecimalType::DecimalOverride;
                Ok(Token::new(TokenKind::Decimal(typ), &txt, span))
            }
            [' ', ' '] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = DecimalType::NoDecimal;
                Ok(Token::new(TokenKind::Decimal(typ), &txt, span))
            }
            _ => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = DecimalType::Decimal;
                Ok(Token::new(TokenKind::Decimal(typ), &txt, span))
            }
        }
    }

    fn read_usage(&self) -> Result<Token, IllegalLexerState> {
        let c = 38;
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
                let typ = UsageType::Blank;
                Ok(Token::new(TokenKind::Usage(typ), &txt, span))
            }
            ['I'] => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let typ = UsageType::Input;
                Ok(Token::new(TokenKind::Usage(typ), &txt, span))
            }
            _ => {
                let span = Span {
                    start_row: row,
                    start_col,
                    end_row: row,
                    end_col,
                };
                let txt = self.text_at(span);
                let ex = LexerException::UnknownUsageType;
                let tok = Token::new(TokenKind::Idk(ex), &txt, span);
                return Ok(tok);
            }
        }
    }

    fn read_location(&self) -> Result<Token, IllegalLexerState> {
        let c = 44;
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
            Ok(Token::new(TokenKind::Location, &txt, span))
        } else {
            let ex = LexerException::IncompletePositionalEntry;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }

    fn read_keyword_entries(&self) -> Result<Token, IllegalLexerState> {
        let c = 80;
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
            // TODO: implement
            let ex = LexerException::NotImplemented;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            self.read_until_next_line()?;
            Ok(tok)
        } else {
            let ex = LexerException::IncompletePositionalEntry;
            let tok = Token::new(TokenKind::Idk(ex), &txt, span);
            Ok(tok)
        }
    }

    pub fn next_token(&self) -> Result<Token, IllegalLexerState> {
        let col = self.state.borrow().position.col;
        match col {
            0 => self.read_sequence(),
            5 => self.read_form_type(),
            6 => self.read_comment(),
            7 => self.read_condition(),
            16 => self.read_name_type(),
            17 => self.read_reserved(),
            18 => self.read_name(),
            28 => self.read_reference_type(),
            29 => self.read_length(),
            34 => self.read_data_type(),
            35 => self.read_decimal(),
            37 => self.read_usage(),
            38 => self.read_location(),
            44 => self.read_keyword_entries(),
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
     A            BNAME          8          TEXT('Barn Name')                   
     A          K ID                                                            
"#[1..];
        let expected: Vec<Token> = vec![
            // row 0
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
                TokenKind::FormType(FormType::A),
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
            // row 1
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
                TokenKind::FormType(FormType::A),
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
            // row 2
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
                TokenKind::FormType(FormType::A),
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
            // row 4
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
                TokenKind::FormType(FormType::A),
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
            // row 4
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
                TokenKind::FormType(FormType::A),
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
            // row 5
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start_row: 5,
                    start_col: 0,
                    end_row: 5,
                    end_col: 5,
                },
            ),
            Token::new(
                TokenKind::FormType(FormType::A),
                "A",
                Span {
                    start_row: 5,
                    start_col: 5,
                    end_row: 5,
                    end_col: 6,
                },
            ),
            Token::new(
                TokenKind::Comment(CommentType::NoComment),
                " ",
                Span {
                    start_row: 5,
                    start_col: 6,
                    end_row: 5,
                    end_col: 7,
                },
            ),
            Token::new(
                TokenKind::Condition,
                "         ",
                Span {
                    start_row: 5,
                    start_col: 7,
                    end_row: 5,
                    end_col: 16,
                },
            ),
            Token::new(
                TokenKind::NameType(NameType::RecordFormat),
                "R",
                Span {
                    start_row: 5,
                    start_col: 16,
                    end_row: 5,
                    end_col: 17,
                },
            ),
            Token::new(
                TokenKind::Reserved,
                " ",
                Span {
                    start_row: 5,
                    start_col: 17,
                    end_row: 5,
                    end_col: 18,
                },
            ),
            Token::new(
                TokenKind::Name,
                "BORNFMT   ",
                Span {
                    start_row: 5,
                    start_col: 18,
                    end_row: 5,
                    end_col: 28,
                },
            ),
            Token::new(
                TokenKind::ReferenceType(ReferenceType::NotReference),
                " ",
                Span {
                    start_row: 5,
                    start_col: 28,
                    end_row: 5,
                    end_col: 29,
                },
            ),
            Token::new(
                TokenKind::Length(LengthType::NoLength),
                "     ",
                Span {
                    start_row: 5,
                    start_col: 29,
                    end_row: 5,
                    end_col: 34,
                },
            ),
            Token::new(
                TokenKind::DataType(DataType::NotSpecified),
                " ",
                Span {
                    start_row: 5,
                    start_col: 34,
                    end_row: 5,
                    end_col: 35,
                },
            ),
            Token::new(
                TokenKind::Decimal(DecimalType::NoDecimal),
                "  ",
                Span {
                    start_row: 5,
                    start_col: 35,
                    end_row: 5,
                    end_col: 37,
                },
            ),
            Token::new(
                TokenKind::Usage(UsageType::Blank),
                " ",
                Span {
                    start_row: 5,
                    start_col: 37,
                    end_row: 5,
                    end_col: 38,
                },
            ),
            Token::new(
                TokenKind::Location,
                "      ",
                Span {
                    start_row: 5,
                    start_col: 38,
                    end_row: 5,
                    end_col: 44,
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "TEXT('Cow Born Fmt')                ",
                Span {
                    start_row: 5,
                    start_col: 44,
                    end_row: 5,
                    end_col: 80,
                },
            ),
            // row 6
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start_row: 6,
                    start_col: 0,
                    end_row: 6,
                    end_col: 5,
                },
            ),
            Token::new(
                TokenKind::FormType(FormType::A),
                "A",
                Span {
                    start_row: 6,
                    start_col: 5,
                    end_row: 6,
                    end_col: 6,
                },
            ),
            Token::new(
                TokenKind::Comment(CommentType::NoComment),
                " ",
                Span {
                    start_row: 6,
                    start_col: 6,
                    end_row: 6,
                    end_col: 7,
                },
            ),
            Token::new(
                TokenKind::Condition,
                "         ",
                Span {
                    start_row: 6,
                    start_col: 7,
                    end_row: 6,
                    end_col: 16,
                },
            ),
            Token::new(
                TokenKind::NameType(NameType::Field),
                " ",
                Span {
                    start_row: 6,
                    start_col: 16,
                    end_row: 6,
                    end_col: 17,
                },
            ),
            Token::new(
                TokenKind::Reserved,
                " ",
                Span {
                    start_row: 6,
                    start_col: 17,
                    end_row: 6,
                    end_col: 18,
                },
            ),
            Token::new(
                TokenKind::Name,
                "ID        ",
                Span {
                    start_row: 6,
                    start_col: 18,
                    end_row: 6,
                    end_col: 28,
                },
            ),
            Token::new(
                TokenKind::ReferenceType(ReferenceType::NotReference),
                " ",
                Span {
                    start_row: 6,
                    start_col: 28,
                    end_row: 6,
                    end_col: 29,
                },
            ),
            Token::new(
                TokenKind::Length(LengthType::Length),
                "    8",
                Span {
                    start_row: 6,
                    start_col: 29,
                    end_row: 6,
                    end_col: 34,
                },
            ),
            Token::new(
                TokenKind::DataType(DataType::NotSpecified),
                " ",
                Span {
                    start_row: 6,
                    start_col: 34,
                    end_row: 6,
                    end_col: 35,
                },
            ),
            Token::new(
                TokenKind::Decimal(DecimalType::Decimal),
                " 0",
                Span {
                    start_row: 6,
                    start_col: 35,
                    end_row: 6,
                    end_col: 37,
                },
            ),
            Token::new(
                TokenKind::Usage(UsageType::Blank),
                " ",
                Span {
                    start_row: 6,
                    start_col: 37,
                    end_row: 6,
                    end_col: 38,
                },
            ),
            Token::new(
                TokenKind::Location,
                "      ",
                Span {
                    start_row: 6,
                    start_col: 38,
                    end_row: 6,
                    end_col: 44,
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "TEXT('Database ID')                 ",
                Span {
                    start_row: 6,
                    start_col: 44,
                    end_row: 6,
                    end_col: 80,
                },
            ),
            // row 7
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start_row: 7,
                    start_col: 0,
                    end_row: 7,
                    end_col: 5,
                },
            ),
            Token::new(
                TokenKind::FormType(FormType::A),
                "A",
                Span {
                    start_row: 7,
                    start_col: 5,
                    end_row: 7,
                    end_col: 6,
                },
            ),
            Token::new(
                TokenKind::Comment(CommentType::NoComment),
                " ",
                Span {
                    start_row: 7,
                    start_col: 6,
                    end_row: 7,
                    end_col: 7,
                },
            ),
            Token::new(
                TokenKind::Condition,
                "         ",
                Span {
                    start_row: 7,
                    start_col: 7,
                    end_row: 7,
                    end_col: 16,
                },
            ),
            Token::new(
                TokenKind::NameType(NameType::Field),
                " ",
                Span {
                    start_row: 7,
                    start_col: 16,
                    end_row: 7,
                    end_col: 17,
                },
            ),
            Token::new(
                TokenKind::Reserved,
                " ",
                Span {
                    start_row: 7,
                    start_col: 17,
                    end_row: 7,
                    end_col: 18,
                },
            ),
            Token::new(
                TokenKind::Name,
                "BNAME     ",
                Span {
                    start_row: 7,
                    start_col: 18,
                    end_row: 7,
                    end_col: 28,
                },
            ),
            Token::new(
                TokenKind::ReferenceType(ReferenceType::NotReference),
                " ",
                Span {
                    start_row: 7,
                    start_col: 28,
                    end_row: 7,
                    end_col: 29,
                },
            ),
            Token::new(
                TokenKind::Length(LengthType::Length),
                "    8",
                Span {
                    start_row: 7,
                    start_col: 29,
                    end_row: 7,
                    end_col: 34,
                },
            ),
            Token::new(
                TokenKind::DataType(DataType::NotSpecified),
                " ",
                Span {
                    start_row: 7,
                    start_col: 34,
                    end_row: 7,
                    end_col: 35,
                },
            ),
            Token::new(
                TokenKind::Decimal(DecimalType::NoDecimal),
                "  ",
                Span {
                    start_row: 7,
                    start_col: 35,
                    end_row: 7,
                    end_col: 37,
                },
            ),
            Token::new(
                TokenKind::Usage(UsageType::Blank),
                " ",
                Span {
                    start_row: 7,
                    start_col: 37,
                    end_row: 7,
                    end_col: 38,
                },
            ),
            Token::new(
                TokenKind::Location,
                "      ",
                Span {
                    start_row: 7,
                    start_col: 38,
                    end_row: 7,
                    end_col: 44,
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "TEXT('Barn Name')                   ",
                Span {
                    start_row: 7,
                    start_col: 44,
                    end_row: 7,
                    end_col: 80,
                },
            ),
            // row 8
            Token::new(
                TokenKind::Sequence,
                "     ",
                Span {
                    start_row: 8,
                    start_col: 0,
                    end_row: 8,
                    end_col: 5,
                },
            ),
            Token::new(
                TokenKind::FormType(FormType::A),
                "A",
                Span {
                    start_row: 8,
                    start_col: 5,
                    end_row: 8,
                    end_col: 6,
                },
            ),
            Token::new(
                TokenKind::Comment(CommentType::NoComment),
                " ",
                Span {
                    start_row: 8,
                    start_col: 6,
                    end_row: 8,
                    end_col: 7,
                },
            ),
            Token::new(
                TokenKind::Condition,
                "         ",
                Span {
                    start_row: 8,
                    start_col: 7,
                    end_row: 8,
                    end_col: 16,
                },
            ),
            Token::new(
                TokenKind::NameType(NameType::Key),
                "K",
                Span {
                    start_row: 8,
                    start_col: 16,
                    end_row: 8,
                    end_col: 17,
                },
            ),
            Token::new(
                TokenKind::Reserved,
                " ",
                Span {
                    start_row: 8,
                    start_col: 17,
                    end_row: 8,
                    end_col: 18,
                },
            ),
            Token::new(
                TokenKind::Name,
                "ID        ",
                Span {
                    start_row: 8,
                    start_col: 18,
                    end_row: 8,
                    end_col: 28,
                },
            ),
            Token::new(
                TokenKind::ReferenceType(ReferenceType::NotReference),
                " ",
                Span {
                    start_row: 8,
                    start_col: 28,
                    end_row: 8,
                    end_col: 29,
                },
            ),
            Token::new(
                TokenKind::Length(LengthType::NoLength),
                "     ",
                Span {
                    start_row: 8,
                    start_col: 29,
                    end_row: 8,
                    end_col: 34,
                },
            ),
            Token::new(
                TokenKind::DataType(DataType::NotSpecified),
                " ",
                Span {
                    start_row: 8,
                    start_col: 34,
                    end_row: 8,
                    end_col: 35,
                },
            ),
            Token::new(
                TokenKind::Decimal(DecimalType::NoDecimal),
                "  ",
                Span {
                    start_row: 8,
                    start_col: 35,
                    end_row: 8,
                    end_col: 37,
                },
            ),
            Token::new(
                TokenKind::Usage(UsageType::Blank),
                " ",
                Span {
                    start_row: 8,
                    start_col: 37,
                    end_row: 8,
                    end_col: 38,
                },
            ),
            Token::new(
                TokenKind::Location,
                "      ",
                Span {
                    start_row: 8,
                    start_col: 38,
                    end_row: 8,
                    end_col: 44,
                },
            ),
            Token::new(
                TokenKind::Idk(LexerException::NotImplemented),
                "                                    ",
                Span {
                    start_row: 8,
                    start_col: 44,
                    end_row: 8,
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
