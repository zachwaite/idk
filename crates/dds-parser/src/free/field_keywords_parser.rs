use super::core::{
    ch, cut_into_metas, is_identifier_char, peek_n, peek_until, read_all, read_char,
    read_identifier, read_spaces_or_tabs, read_string_literal, Lexer, LexerState, MetaChar,
};
use crate::field::FieldResult;
use crate::line::{FieldLine, ContinuationLine};
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FTokenKind {
    Idk,
    Whitespace,
    Identifier,
    LParen,
    RParen,
    Indicator,
    Colon,
    StringLiteral,
    Plus,
    Minus,
    Slash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FToken {
    pub kind: FTokenKind,
    pub metas: Vec<Meta>,
}

impl FToken {
    pub fn highlight(&self) -> Vec<(Span, String)> {
        let hlgroup = match self.kind {
            FTokenKind::Idk => "Normal",
            FTokenKind::Whitespace => "Normal",
            FTokenKind::Colon => "Normal",
            FTokenKind::Identifier => "Identifier",
            FTokenKind::Indicator => "@variable.builtin",
            FTokenKind::LParen => "Normal",
            FTokenKind::RParen => "Normal",
            FTokenKind::StringLiteral => "String",
            FTokenKind::Plus => "Normal",
            FTokenKind::Minus => "Normal",
            FTokenKind::Slash => "Normal",
        };
        let mut out = vec![];
        for meta in self.metas.iter() {
            out.push((meta.span, hlgroup.to_string()));
        }
        out
    }
}

fn next_token(lexer: &Lexer) -> Option<FToken> {
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
                let kind = FTokenKind::Whitespace;
                (kind, chars)
            }
            // lparen
            '(' => {
                let chars = vec![read_char(lexer)];
                let kind = FTokenKind::LParen;
                (kind, chars)
            }
            // rparen
            ')' => {
                let chars = vec![read_char(lexer)];
                let kind = FTokenKind::RParen;
                (kind, chars)
            }
            // colon
            ':' => {
                let chars = vec![read_char(lexer)];
                let kind = FTokenKind::Colon;
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
            // plus
            '+' => {
                let chars = vec![read_char(lexer)];
                let kind = FTokenKind::Plus;
                (kind, chars)
            }
            // minus
            '-' => {
                let chars = vec![read_char(lexer)];
                let kind = FTokenKind::Minus;
                (kind, chars)
            }
            // slash
            '/' => {
                let chars = vec![read_char(lexer)];
                let kind = FTokenKind::Slash;
                (kind, chars)
            }
            // quote
            '\'' => match peek_until(lexer, '\'') {
                Some(MetaChar { .. }) => {
                    let chars = read_string_literal(lexer);
                    let kind = FTokenKind::StringLiteral;
                    (kind, chars)
                }
                None => {
                    let chars = read_all(lexer);
                    let kind = FTokenKind::Idk;
                    (kind, chars)
                }
            },
            // identifier
            x => match is_identifier_char(&x) {
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
        },
        None => {
            let chars = read_all(lexer);
            let kind = FTokenKind::Idk;
            (kind, chars)
        }
    };
    let metas = cut_into_metas(pchars);
    let tok = FToken { kind, metas };
    Some(tok)
}

pub fn tokenize_fld_kw(
    line: &FieldLine,
    continuations: Vec<&ContinuationLine>,
) -> Vec<FToken> {
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
