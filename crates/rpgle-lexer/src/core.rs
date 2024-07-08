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
    Control,
    File,
    Definition,
    Input,
    Calculation,
    Output,
    Procedure,
    Idk,
}

impl fmt::Display for FormType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            Self::Empty => format!("Empty"),
            Self::Control => format!("Control"),
            Self::File => format!("File"),
            Self::Definition => format!("Definition"),
            Self::Input => format!("Input"),
            Self::Calculation => format!("Calculation"),
            Self::Output => format!("Output"),
            Self::Procedure => format!("Procedure"),
            Self::Idk => format!("IDK"),
        };
        write!(f, "{}", out)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FileType {
    Input,
    Output,
    Update,
    Combined,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FileDesignation {
    Output,
    Primary,
    Secondary,
    RecordAddress,
    Table,
    FullProcedural,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FileAdditionType {
    NoAdd,
    Add,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FileSequenceType {
    Ascending,
    Descending,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FileFormatType {
    ProgramDescribed,
    ExternallyDescribed,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DefinitionType {
    Blank,
    Constant,
    DataStructure,
    Prototype,
    ProcedureInterface,
    Standalone,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DefinitionDataType {
    Blank,
    Character,
    Binary,
    UCS2,
    Date,
    Float,
    Graphic,
    Integer,
    Indicator,
    Object,
    Packed,
    Zoned,
    Time,
    Unsigned,
    Timestamp,
    Pointer,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CompilerDirectiveType {
    Free,
    EndFree,
    Title,
    Eject,
    Space,
    Copy,
    Include,
    If,
    Elseif,
    Else,
    Endif,
    Eof,
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
    // Intentional whitespace
    Reserved,

    // positional tokens
    // fspec
    Sequence,
    Comment(CommentType),
    FormType(FormType),
    Name,
    FileType(FileType),
    FileDesignation(FileDesignation),
    FileAddition(FileAdditionType),
    FileSequence(FileSequenceType),
    FileFormat(FileFormatType),
    // dspec
    DefinitionType(DefinitionType),
    DefinitionDataType(DefinitionDataType),
    DefinitionDecimals,
    // compiler directive
    CompilerDirectiveType(CompilerDirectiveType),
    // free tokens
    Indicator,
    Whitespace,
    Equals,
    NotEquals,
    Plus,
    Minus,
    Asterisk,
    Slash,
    PlusEqual,
    MinusEqual,
    AsteriskEqual,
    SlashEqual,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
    Semicolon,
    Colon,
    IndicatorValue,
    StringLiteral,
    Number,
    Identifier,
    BuiltinIdentifier,
    LParen,
    RParen,
    // fixed keywords
    // H
    Option,
    Datedit,
    Datfmt,
    Timfmt,
    Dftactgrp,
    Debug,
    // F
    Rename,
    Ignore,
    Prefix,
    // D
    Extpgm,
    Dim,
    // keywords
    SetLL,
    SetGT,
    Chain,
    Read,
    ReadE,
    ReadPE,
    Write,
    Update,
    Delete,
    If,
    Or,
    And,
    Else,
    Elseif,
    Endif,
    End,
    Dou,
    Dow,
    Enddo,
    Iter,
    Leave,
    Reset,
    Eval,
    Clear,
    Begsr,
    Endsr,
    Exsr,
    Move,
    Plist,
    Parm,
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
            Self::FormType(t) => format!("FormType({})", t),
            Self::Name => format!("Name"),
            Self::FileType(_) => format!("FileType"),
            Self::FileDesignation(_) => format!("FileDesignation"),
            Self::FileAddition(_) => format!("FileAddition"),
            Self::FileSequence(_) => format!("FileSequence"),
            Self::FileFormat(_) => format!("FileFormatType"),
            Self::DefinitionType(_) => format!("DefinitionType"),
            Self::DefinitionDataType(_) => format!("DefinitionDataType"),
            Self::DefinitionDecimals => format!("DefinitionDecimals"),
            Self::CompilerDirectiveType(_) => format!("CompilerDirectiveType"),
            Self::Reserved => format!("Reserved"),
            Self::Indicator => format!("Indicator"),
            Self::IndicatorValue => format!("IndicatorValue"),
            Self::Whitespace => format!("Whitespace"),
            Self::Equals => format!("Equals"),
            Self::NotEquals => format!("NotEquals"),
            Self::LessThan => format!("LessThan"),
            Self::LessThanOrEquals => format!("LessThanOrEquals"),
            Self::GreaterThan => format!("GreaterThan"),
            Self::GreaterThanOrEquals => format!("GreaterThanOrEquals"),
            Self::Plus => format!("Plus"),
            Self::Minus => format!("Minus"),
            Self::Asterisk => format!("Asterisk"),
            Self::Slash => format!("Slash"),
            Self::PlusEqual => format!("PlusEqual"),
            Self::MinusEqual => format!("MinusEqual"),
            Self::AsteriskEqual => format!("AsteriskEqual"),
            Self::SlashEqual => format!("SlashEqual"),
            Self::Semicolon => format!("Semicolon"),
            Self::Colon => format!("Colon"),
            Self::SetLL => format!("SetLL"),
            Self::SetGT => format!("SetGT"),
            Self::Chain => format!("Chain"),
            Self::Read => format!("Chain"),
            Self::ReadE => format!("ReadE"),
            Self::ReadPE => format!("ReadPE"),
            Self::Write => format!("Write"),
            Self::Update => format!("Update"),
            Self::Delete => format!("Delete"),
            Self::Identifier => format!("Identifier"),
            Self::BuiltinIdentifier => format!("BuiltinIdentifier"),
            Self::LParen => format!("LParen"),
            Self::RParen => format!("RParen"),
            Self::Option => format!("Option"),
            Self::Datedit => format!("Datedit"),
            Self::Datfmt => format!("Datfmt"),
            Self::Timfmt => format!("Timfmt"),
            Self::Dftactgrp => format!("Dftactgrp"),
            Self::Debug => format!("Debug"),
            Self::Rename => format!("Rename"),
            Self::Ignore => format!("Ignore"),
            Self::Prefix => format!("Prefix"),
            Self::Extpgm => format!("Extpgm"),
            Self::Dim => format!("Dim"),
            Self::StringLiteral => format!("StringLiteral"),
            Self::Number => format!("Number"),
            Self::If => format!("If"),
            Self::Else => format!("Else"),
            Self::Elseif => format!("Elseif"),
            Self::Endif => format!("EndIf"),
            Self::End => format!("End"),
            Self::Begsr => format!("Begsr"),
            Self::Endsr => format!("Endsr"),
            Self::Exsr => format!("Exsr"),
            Self::Dou => format!("Dou"),
            Self::Dow => format!("Dow"),
            Self::Enddo => format!("Enddo"),
            Self::Or => format!("Or"),
            Self::And => format!("And"),
            Self::Iter => format!("Iter"),
            Self::Leave => format!("Leave"),
            Self::Reset => format!("Reset"),
            Self::Eval => format!("Eval"),
            Self::Clear => format!("Clear"),
            Self::Move => format!("Move"),
            Self::Plist => format!("Plist"),
            Self::Parm => format!("Parm"),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
    pub idx: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!(
            "Position {{row: {}, col: {}, index: {}}}",
            self.row, self.col, self.idx
        );
        write!(f, "{}", s)
    }
}

impl Position {
    pub fn empty() -> Self {
        Self {
            row: 0,
            col: 0,
            idx: 0,
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
    Idk,
    FullFree,
    Free,
    HSpec,
    FSpec,
    ISpec,
    OSpec,
    PSpec,
    CSpec,
    DSpec,
    LineComment,
    CompilerDirective,
}

impl fmt::Display for LexerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Init => format!("Init"),
            Self::Idk => format!("Idk"),
            Self::FullFree => format!("FullFree"),
            Self::Free => format!("Free"),
            Self::HSpec => format!("HSpec"),
            Self::FSpec => format!("FSpec"),
            Self::ISpec => format!("ISpec"),
            Self::OSpec => format!("OSpec"),
            Self::PSpec => format!("PSpec"),
            Self::CSpec => format!("CSpec"),
            Self::DSpec => format!("DSpec"),
            Self::LineComment => format!("LineComment"),
            Self::CompilerDirective => format!("CompilerDirective"),
        };
        write!(f, "{}", s)
    }
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

pub fn is_letter(ch: &char) -> bool {
    ch.is_ascii_alphabetic()
}

pub fn is_numeric(ch: &char) -> bool {
    ch.is_numeric()
}

pub fn is_identifier_char(ch: &char) -> bool {
    ch.is_alphanumeric() || *ch == '@' || *ch == '$' || *ch == '-' || *ch == '#'
}

pub fn is_space_or_tab(ch: &char) -> bool {
    match ch {
        ' ' => true,
        '\t' => true,
        _ => false,
    }
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

pub fn read_identifier(lexer: &Lexer) -> Result<(), IllegalLexerState> {
    // read until the cursor is on something not alphanumeric
    while ch(lexer).is_some() && is_identifier_char(&ch(lexer).unwrap()) {
        read_char(lexer)?;
    }
    Ok(())
}

pub fn read_number(lexer: &Lexer) -> Result<(), IllegalLexerState> {
    // read until the cursor is on something not alphanumeric
    while ch(lexer).is_some() && is_numeric(&ch(lexer).unwrap()) {
        read_char(lexer)?;
    }
    Ok(())
}

pub fn read_string_literal(lexer: &Lexer) -> Result<(), IllegalLexerState> {
    read_char(lexer)?;
    while ch(lexer).is_some() && !matches!(ch(lexer), Some('\n')) {
        if matches!(ch(lexer), Some('\'')) {
            break;
        } else {
            read_char(lexer)?;
        }
    }
    Ok(())
}

pub fn read_spaces_or_tabs(lexer: &Lexer) -> Result<(), IllegalLexerState> {
    while ch(lexer).is_some() && is_space_or_tab(&ch(lexer).unwrap()) {
        read_char(lexer)?;
    }
    Ok(())
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
    return lex;
}
