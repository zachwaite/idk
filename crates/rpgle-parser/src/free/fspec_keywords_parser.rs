use super::lexer::{
    ch, is_identifier_char, peek_n, read_all, read_char, read_identifier, read_spaces_or_tabs,
    Lexer, LexerState,
};
use crate::meta::{Meta, PMixin, Position, Span};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FTokenKind {
    Idk,
    Whitespace,
    Identifier,
    LParen,
    RParen,
    Indicator,
    Colon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FToken {
    pub kind: FTokenKind,
    pub meta: Meta,
}

impl Display for FToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl PMixin for FToken {
    fn highlight(&self) -> Vec<(Span, String)> {
        let hlgroup = match self.kind {
            FTokenKind::Idk => "Normal",
            FTokenKind::Whitespace => "Normal",
            FTokenKind::Colon => "Normal",
            FTokenKind::Identifier => "Identifier",
            FTokenKind::Indicator => "@variable.builtin",
            FTokenKind::LParen => "Normal",
            FTokenKind::RParen => "Normal",
        };
        vec![(self.span(), hlgroup.to_string())]
    }

    fn span(&self) -> crate::Span {
        self.meta.span
    }
}

fn next_token(lexer: &Lexer) -> Option<FToken> {
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
            let kind = FTokenKind::Whitespace;
            (kind, chars)
        }
        // lparen
        Some('(') => {
            let chars = vec![read_char(lexer)];
            let kind = FTokenKind::LParen;
            (kind, chars)
        }
        // rparen
        Some(')') => {
            let chars = vec![read_char(lexer)];
            let kind = FTokenKind::RParen;
            (kind, chars)
        }
        // colon
        Some(':') => {
            let chars = vec![read_char(lexer)];
            let kind = FTokenKind::Colon;
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
                        let kind = FTokenKind::Indicator;
                        (kind, chars)
                    } else {
                        let chars = read_all(lexer);
                        let kind = FTokenKind::Idk;
                        (kind, chars)
                    }
                }
                None => {
                    let chars = read_all(lexer);
                    let kind = FTokenKind::Idk;
                    (kind, chars)
                }
            }
        }
        // identifier
        Some(x) => match is_identifier_char(&x) {
            true => {
                let chars = read_identifier(lexer);
                let kind = FTokenKind::Identifier;
                (kind, chars)
            }
            false => {
                let chars = read_all(lexer);
                let kind = FTokenKind::Idk;
                (kind, chars)
            }
        },
        _ => {
            let chars = read_all(lexer);
            let kind = FTokenKind::Idk;
            (kind, chars)
        }
    };
    let meta = Meta::from((start, chars.as_slice()));
    let tok = FToken { kind, meta };
    Some(tok)
}

pub fn tokenize_fspec_kw(pos: Position, chars: &[char; 57]) -> Vec<FToken> {
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
