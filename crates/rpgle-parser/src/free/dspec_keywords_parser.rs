use super::core::{
    ch, cut_into_metas, is_identifier_char, peek_n, peek_until, read_all, read_char,
    read_identifier, read_spaces_or_tabs, read_string_literal, Lexer, LexerState, MetaChar,
};
use crate::field::{FieldResult, RawKeywordsField};
use crate::meta::{Meta, Position, Span};
use nonempty::NonEmpty;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
impl DTokenKind {
    pub fn is_identifier_or_literal(&self) -> bool {
        if matches!(self, Self::Identifier) || matches!(self, Self::StringLiteral) {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DToken {
    pub kind: DTokenKind,
    pub metas: Vec<Meta>,
}

impl DToken {
    pub fn highlight(&self) -> Vec<(Span, String)> {
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
        let mut out = vec![];
        for meta in self.metas.iter() {
            out.push((meta.span, hlgroup.to_string()));
        }
        out
    }
}

fn next_token(lexer: &Lexer) -> Option<DToken> {
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
                let kind = DTokenKind::Whitespace;
                (kind, chars)
            }
            // lparen
            '(' => {
                let chars = vec![read_char(lexer)];
                let kind = DTokenKind::LParen;
                (kind, chars)
            }
            // rparen
            ')' => {
                let chars = vec![read_char(lexer)];
                let kind = DTokenKind::RParen;
                (kind, chars)
            }
            // colon
            ':' => {
                let chars = vec![read_char(lexer)];
                let kind = DTokenKind::Colon;
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
            '\'' => match peek_until(lexer, '\'') {
                Some(MetaChar { .. }) => {
                    let chars = read_string_literal(lexer);
                    let kind = DTokenKind::StringLiteral;
                    (kind, chars)
                }
                None => {
                    let chars = read_all(lexer);
                    let kind = DTokenKind::Idk;
                    (kind, chars)
                }
            },
            // identifier
            x => match is_identifier_char(&x) {
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
        },
        None => {
            let chars = read_all(lexer);
            let kind = DTokenKind::Idk;
            (kind, chars)
        }
    };
    let metas = cut_into_metas(pchars);
    let tok = DToken { kind, metas };
    Some(tok)
}

pub fn legacy_tokenize_dspec_kw(
    kwfield: &FieldResult<RawKeywordsField>,
    contkw: &[&FieldResult<RawKeywordsField>],
) -> Vec<DToken> {
    match kwfield {
        FieldResult::Ok(kw) => {
            // line
            let mut mchars: NonEmpty<MetaChar> = NonEmpty::from_vec(
                kw.value
                    .iter()
                    .enumerate()
                    .map(|(i, c)| MetaChar {
                        value: *c,
                        position: Position {
                            row: kw.meta.span.start.row,
                            col: kw.meta.span.start.col + i,
                        },
                    })
                    .collect::<Vec<MetaChar>>(),
            )
            .expect("kw.value is NonEmpty, so mchars is guaranteed to be nonempty too");
            // continuations
            for kwfield in contkw {
                match kwfield {
                    FieldResult::Ok(kw) => {
                        for (i, c) in kw.value.iter().enumerate() {
                            let p = Position {
                                row: kw.meta.span.start.row,
                                col: kw.meta.span.start.col + i,
                            };
                            mchars.push(MetaChar {
                                value: *c,
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
    }
}
