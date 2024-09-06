use crate::field::PMixin;
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HTokenKind {
    Idk,
    Whitespace,
    Identifier,
    LParen,
    RParen,
    Indicator,
    Colon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HToken {
    pub kind: HTokenKind,
    pub meta: Meta,
}

impl Display for HToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl PMixin for HToken {
    fn highlight(&self) -> Vec<(Span, String)> {
        let hlgroup = match self.kind {
            HTokenKind::Idk => "Normal",
            HTokenKind::Whitespace => "Normal",
            HTokenKind::Colon => "Normal",
            HTokenKind::Identifier => "Identifier",
            HTokenKind::Indicator => "@variable.builtin",
            HTokenKind::LParen => "Normal",
            HTokenKind::RParen => "Normal",
        };
        vec![(self.span(), hlgroup.to_string())]
    }

    fn span(&self) -> crate::Span {
        self.meta.span
    }
}

struct LexerState {
    origin: Position,
    col: usize,
}

struct Lexer {
    state: RefCell<LexerState>,
    input: Vec<char>,
}

fn ch(lexer: &Lexer) -> Option<&char> {
    let idx = lexer.state.borrow().col;
    lexer.input.get(idx)
}

fn is_identifier_char(ch: &char) -> bool {
    ch.is_alphanumeric() || *ch == '@' || *ch == '$' || *ch == '-' || *ch == '#'
}

pub fn is_space_or_tab(ch: &char) -> bool {
    match ch {
        ' ' => true,
        '\t' => true,
        _ => false,
    }
}

fn peek_n(lexer: &Lexer, n: usize) -> Option<&char> {
    let idx = lexer.state.borrow().col;
    lexer.input.get(idx + n)
}

fn read_char(lexer: &Lexer) -> char {
    let out = *ch(lexer).expect("read_char() requires a length check prior to call");
    lexer.state.borrow_mut().col += 1;
    out
}

fn read_all(lexer: &Lexer) -> Vec<char> {
    let mut out = vec![];
    while ch(lexer).is_some() {
        out.push(read_char(lexer));
    }
    out
}

fn read_spaces_or_tabs(lexer: &Lexer) -> Vec<char> {
    let mut out = vec![];
    while ch(lexer).is_some() && is_space_or_tab(&ch(lexer).unwrap()) {
        out.push(read_char(lexer));
    }
    out
}

fn read_identifier(lexer: &Lexer) -> Vec<char> {
    let mut out = vec![];
    while ch(lexer).is_some() && is_identifier_char(&ch(lexer).unwrap()) {
        out.push(read_char(lexer));
    }
    out
}

fn next_token(lexer: &Lexer) -> Option<HToken> {
    // guard
    if ch(lexer).is_none() {
        return None;
    }
    // happy path
    let origin = lexer.state.borrow().origin;
    let idx = lexer.state.borrow().col;
    let start = Position {
        row: origin.row,
        col: origin.col + idx,
    };
    let (kind, chars) = match ch(lexer) {
        // whitespace
        Some(' ') | Some('\t') => {
            let chars = read_spaces_or_tabs(lexer);
            let kind = HTokenKind::Whitespace;
            (kind, chars)
        }
        // lparen
        Some('(') => {
            let chars = vec![read_char(lexer)];
            let kind = HTokenKind::LParen;
            (kind, chars)
        }
        // rparen
        Some(')') => {
            let chars = vec![read_char(lexer)];
            let kind = HTokenKind::RParen;
            (kind, chars)
        }
        // colon
        Some(':') => {
            let chars = vec![read_char(lexer)];
            let kind = HTokenKind::Colon;
            (kind, chars)
        }
        // asterisk
        Some('*') => {
            let peeked = peek_n(lexer, 1);
            match peeked {
                Some(x) => {
                    if is_identifier_char(x) {
                        let _ = read_char(lexer);
                        let mut chars = vec!['*'];
                        chars.append(&mut read_identifier(lexer));
                        let kind = HTokenKind::Indicator;
                        (kind, chars)
                    } else {
                        let chars = read_all(lexer);
                        let kind = HTokenKind::Idk;
                        (kind, chars)
                    }
                }
                None => {
                    let chars = read_all(lexer);
                    let kind = HTokenKind::Idk;
                    (kind, chars)
                }
            }
        }
        // identifier
        Some(x) => match is_identifier_char(&x) {
            true => {
                let chars = read_identifier(lexer);
                let kind = HTokenKind::Identifier;
                (kind, chars)
            }
            false => {
                let chars = read_all(lexer);
                let kind = HTokenKind::Idk;
                (kind, chars)
            }
        },
        _ => {
            let chars = read_all(lexer);
            let kind = HTokenKind::Idk;
            (kind, chars)
        }
    };
    let meta = Meta::from((start, chars.as_slice()));
    let tok = HToken { kind, meta };
    Some(tok)
}

pub fn tokenize(pos: Position, chars: &[char; 94]) -> Vec<HToken> {
    let state = LexerState {
        origin: pos,
        col: 0,
    };
    let lexer = Lexer {
        state: RefCell::new(state),
        input: chars.to_vec(),
    };
    let mut tokens = vec![];
    loop {
        match next_token(&lexer) {
            Some(token) => {
                tokens.push(token);
            }
            None => {
                break;
            }
        }
    }
    tokens
}
