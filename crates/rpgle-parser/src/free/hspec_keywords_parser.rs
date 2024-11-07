use super::lexer::{
    ch, is_identifier_char, peek_n, peek_until, read_all, read_char, read_identifier,
    read_spaces_or_tabs, read_string_literal, Lexer, LexerState,
};
use crate::field::{FieldResult, RawKeywordsField};
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HTokenKind {
    Idk,
    Whitespace,
    Identifier,
    LParen,
    RParen,
    Indicator,
    Colon,
    StringLiteral,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

impl HToken {
    pub fn highlight(&self) -> Vec<(Span, String)> {
        let hlgroup = match self.kind {
            HTokenKind::Idk => "Normal",
            HTokenKind::Whitespace => "Normal",
            HTokenKind::Colon => "Normal",
            HTokenKind::Identifier => "Identifier",
            HTokenKind::Indicator => "@variable.builtin",
            HTokenKind::LParen => "Normal",
            HTokenKind::RParen => "Normal",
            HTokenKind::StringLiteral => "String",
        };
        vec![(self.span(), hlgroup.to_string())]
    }

    pub fn span(&self) -> crate::Span {
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
        Some('\'') => match peek_until(lexer, '\'') {
            Some(_) => {
                let chars = read_string_literal(lexer);
                let kind = HTokenKind::StringLiteral;
                (kind, chars)
            }
            None => {
                let mut chars = vec!['\''];
                chars.append(&mut read_all(lexer));
                let kind = HTokenKind::Idk;
                (kind, chars)
            }
        },
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

pub fn legacy_tokenize_hspec_kw(kwfield: &FieldResult<RawKeywordsField>) -> Vec<HToken> {
    match kwfield {
        FieldResult::Ok(kw) => {
            let pos = kw.meta.span.start;
            let state = LexerState {
                origin: pos,
                col: 0,
            };
            let lexer = Lexer {
                state: RefCell::new(state),
                input: kw.value.clone(), // TDE: use lifetime
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
        _ => vec![],
    }
}
