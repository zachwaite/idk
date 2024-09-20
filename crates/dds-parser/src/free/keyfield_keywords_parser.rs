use super::core::{
    ch, cut_into_metas, is_identifier_char, peek_n, peek_until, read_all, read_char,
    read_identifier, read_spaces_or_tabs, read_string_literal, Lexer, LexerState, MetaChar,
};
use crate::field::FieldResult;
use crate::line::{KeyLine, ContinuationLine};
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KTokenKind {
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
pub struct KToken {
    pub kind: KTokenKind,
    pub metas: Vec<Meta>,
}

impl KToken {
    pub fn highlight(&self) -> Vec<(Span, String)> {
        let hlgroup = match self.kind {
            KTokenKind::Idk => "Normal",
            KTokenKind::Whitespace => "Normal",
            KTokenKind::Colon => "Normal",
            KTokenKind::Identifier => "Identifier",
            KTokenKind::Indicator => "@variable.builtin",
            KTokenKind::LParen => "Normal",
            KTokenKind::RParen => "Normal",
            KTokenKind::StringLiteral => "String",
        };
        let mut out = vec![];
        for meta in self.metas.iter() {
            out.push((meta.span, hlgroup.to_string()));
        }
        out
    }
}

fn next_token(lexer: &Lexer) -> Option<KToken> {
    // guard
    if ch(lexer).is_none() {
        return None;
    }
    // happy path
    let (kind, pchars) = match ch(lexer) {
        Some(MetaChar { value: c, .. }) => match c {
            // whitespace
            ' ' | '\t' => {
                let chars = read_spaces_or_tabs(lexer);
                let kind = KTokenKind::Whitespace;
                (kind, chars)
            }
            // lparen
            '(' => {
                let chars = vec![read_char(lexer)];
                let kind = KTokenKind::LParen;
                (kind, chars)
            }
            // rparen
            ')' => {
                let chars = vec![read_char(lexer)];
                let kind = KTokenKind::RParen;
                (kind, chars)
            }
            // colon
            ':' => {
                let chars = vec![read_char(lexer)];
                let kind = KTokenKind::Colon;
                (kind, chars)
            }
            // asterisk
            '*' => {
                let peeked = peek_n(lexer, 1);
                match peeked {
                    Some(x) => {
                        if is_identifier_char(&x.value) {
                            let c = read_char(lexer);
                            let mut chars = vec![c];
                            chars.append(&mut read_identifier(lexer));
                            let kind = KTokenKind::Indicator;
                            (kind, chars)
                        } else {
                            let chars = read_all(lexer);
                            let kind = KTokenKind::Idk;
                            (kind, chars)
                        }
                    }
                    None => {
                        let chars = read_all(lexer);
                        let kind = KTokenKind::Idk;
                        (kind, chars)
                    }
                }
            }
            // quote
            '\'' => match peek_until(lexer, '\'') {
                Some(MetaChar { .. }) => {
                    let chars = read_string_literal(lexer);
                    let kind = KTokenKind::StringLiteral;
                    (kind, chars)
                }
                None => {
                    let chars = read_all(lexer);
                    let kind = KTokenKind::Idk;
                    (kind, chars)
                }
            },
            // identifier
            x => match is_identifier_char(&x) {
                true => {
                    let chars = read_identifier(lexer);
                    let kind = KTokenKind::Identifier;
                    (kind, chars)
                }
                false => {
                    let chars = read_all(lexer);
                    let kind = KTokenKind::Idk;
                    (kind, chars)
                }
            },
        },
        None => {
            let chars = read_all(lexer);
            let kind = KTokenKind::Idk;
            (kind, chars)
        }
    };
    let metas = cut_into_metas(pchars);
    let tok = KToken { kind, metas };
    Some(tok)
}

pub fn tokenize_kf_kw(
    line: &KeyLine,
    continuations: Vec<&ContinuationLine>,
) -> Vec<KToken> {
    let tokens = match &line.keywords {
        FieldResult::Ok(kw) => {
            let mut mchars = vec![];
            // line
            for (i, c) in kw.value.chars().enumerate() {
                let p = Position {
                    row: kw.meta.span.start.row,
                    col: kw.meta.span.start.col + i,
                };
                mchars.push(MetaChar {
                    value: c,
                    position: p,
                });
            }
            // continuations
            for cont in continuations {
                match &cont.keywords {
                    FieldResult::Ok(kw) => {
                        for (i, c) in kw.value.chars().enumerate() {
                            let p = Position {
                                row: kw.meta.span.start.row,
                                col: kw.meta.span.start.col + i,
                            };
                            mchars.push(MetaChar {
                                value: c,
                                position: p,
                            });
                        }
                    }
                    _ => continue,
                }
            }
            // process
            let state = LexerState {
                position: kw.meta.span.start,
                idx: 0,
            };
            let lexer = Lexer {
                state: RefCell::new(state),
                input: mchars,
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
    };
    tokens
}
