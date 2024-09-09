use super::lexer::{
    ch, is_identifier_char, peek_n, peek_until, read_all, read_char, read_identifier,
    read_spaces_or_tabs, read_string_literal, Lexer, LexerState,
};
use crate::meta::{Meta, PMixin, Position, Span};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DTokenKind {
    Idk,
    Whitespace,
    Identifier,
    LParen,
    RParen,
    Indicator,
    Colon,
    StringLiteral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DToken {
    pub kind: DTokenKind,
    pub meta: Meta,
}

impl Display for DToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl PMixin for DToken {
    fn highlight(&self) -> Vec<(Span, String)> {
        let hlgroup = match self.kind {
            DTokenKind::Idk => "Normal",
            DTokenKind::Whitespace => "Normal",
            DTokenKind::Colon => "Normal",
            DTokenKind::Identifier => "Identifier",
            DTokenKind::Indicator => "@variable.builtin",
            DTokenKind::LParen => "Normal",
            DTokenKind::RParen => "Normal",
            DTokenKind::StringLiteral => "String",
        };
        vec![(self.span(), hlgroup.to_string())]
    }

    fn span(&self) -> crate::Span {
        self.meta.span
    }
}

fn next_token(lexer: &Lexer) -> Option<DToken> {
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
            let kind = DTokenKind::Whitespace;
            (kind, chars)
        }
        // lparen
        Some('(') => {
            let chars = vec![read_char(lexer)];
            let kind = DTokenKind::LParen;
            (kind, chars)
        }
        // rparen
        Some(')') => {
            let chars = vec![read_char(lexer)];
            let kind = DTokenKind::RParen;
            (kind, chars)
        }
        // colon
        Some(':') => {
            let chars = vec![read_char(lexer)];
            let kind = DTokenKind::Colon;
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
                        let kind = DTokenKind::Indicator;
                        (kind, chars)
                    } else {
                        let chars = read_all(lexer);
                        let kind = DTokenKind::Idk;
                        (kind, chars)
                    }
                }
                None => {
                    let chars = read_all(lexer);
                    let kind = DTokenKind::Idk;
                    (kind, chars)
                }
            }
        }
        // quote
        Some('\'') => match peek_until(lexer, '\'') {
            Some(_) => {
                let chars = read_string_literal(lexer);
                let kind = DTokenKind::StringLiteral;
                (kind, chars)
            }
            None => {
                let mut chars = vec!['\''];
                chars.append(&mut read_all(lexer));
                let kind = DTokenKind::Idk;
                (kind, chars)
            }
        },
        // identifier
        Some(x) => match is_identifier_char(&x) {
            true => {
                let chars = read_identifier(lexer);
                let kind = DTokenKind::Identifier;
                (kind, chars)
            }
            false => {
                let chars = read_all(lexer);
                let kind = DTokenKind::Idk;
                (kind, chars)
            }
        },
        _ => {
            let chars = read_all(lexer);
            let kind = DTokenKind::Idk;
            (kind, chars)
        }
    };
    let meta = Meta::from((start, chars.as_slice()));
    let tok = DToken { kind, meta };
    Some(tok)
}

pub fn tokenize_dspec_kw(pos: Position, chars: &[char; 57]) -> Vec<DToken> {
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
