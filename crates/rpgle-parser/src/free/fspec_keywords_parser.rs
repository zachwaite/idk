use super::core::{
    ch, cut_into_metas, is_identifier_char, peek_n, read_all, read_char, read_identifier,
    read_spaces_or_tabs, Lexer, LexerState, MetaChar,
};
use crate::field::FieldResult;
use crate::line::{FSpecLine, FSpecLineContinuation};
use crate::meta::{Meta, Position, Span};
use nonempty::NonEmpty;
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
    let (kind, mchars) = match ch(lexer) {
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
    let metas = cut_into_metas(mchars);
    let tok = FToken { kind, metas };
    Some(tok)
}

pub fn tokenize_fspec_kw(
    line: &FSpecLine,
    continuations: Vec<&FSpecLineContinuation>,
) -> Vec<FToken> {
    let tokens = match &line.keywords {
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
            for cont in continuations {
                match &cont.keywords {
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
    };
    tokens
}
