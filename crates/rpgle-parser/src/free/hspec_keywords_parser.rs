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

pub fn tokenize_hspec_kw(pos: Position, chars: &[char; 94]) -> Vec<HToken> {
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
