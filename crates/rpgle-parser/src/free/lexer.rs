use crate::meta::Position;
use std::cell::RefCell;

pub struct LexerState {
    pub origin: Position,
    pub col: usize,
}

pub struct Lexer {
    pub state: RefCell<LexerState>,
    pub input: Vec<char>,
}

pub fn ch(lexer: &Lexer) -> Option<&char> {
    let idx = lexer.state.borrow().col;
    lexer.input.get(idx)
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
    let idx = lexer.state.borrow().col;
    lexer.input.get(idx + n)
}

pub fn read_char(lexer: &Lexer) -> char {
    let out = *ch(lexer).expect("read_char() requires a length check prior to call");
    lexer.state.borrow_mut().col += 1;
    out
}

pub fn read_all(lexer: &Lexer) -> Vec<char> {
    let mut out = vec![];
    while ch(lexer).is_some() {
        out.push(read_char(lexer));
    }
    out
}

pub fn read_spaces_or_tabs(lexer: &Lexer) -> Vec<char> {
    let mut out = vec![];
    while ch(lexer).is_some() && is_space_or_tab(&ch(lexer).unwrap()) {
        out.push(read_char(lexer));
    }
    out
}

pub fn read_identifier(lexer: &Lexer) -> Vec<char> {
    let mut out = vec![];
    while ch(lexer).is_some() && is_identifier_char(&ch(lexer).unwrap()) {
        out.push(read_char(lexer));
    }
    out
}
