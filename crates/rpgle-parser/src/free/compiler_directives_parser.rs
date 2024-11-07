use super::lexer::{
    ch, is_identifier_char, peek_n, read_all, read_char, read_identifier, Lexer, LexerState,
};
use crate::meta::{Meta, Position, Span};
use nonempty::NonEmpty;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DirectiveTokenKind {
    Idk,
    Title,
    Eject,
    Space,
    Copy,
    Include,
    Set,
    Restore,
    OverloadDetail,
    OverloadNoDetail,
    Define,
    Undefine,
    If,
    Elseif,
    Else,
    Endif,
    Eof,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DirectiveToken {
    pub kind: DirectiveTokenKind,
    pub meta: Meta,
}

impl Display for DirectiveToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl DirectiveToken {
    pub fn highlight(&self) -> Vec<(Span, String)> {
        let hlgroup = match self.kind {
            DirectiveTokenKind::Idk => "WarningMsg",
            DirectiveTokenKind::Title => "@keyword.directive.define",
            DirectiveTokenKind::Eject => "@keyword.directive.define",
            DirectiveTokenKind::Space => "@keyword.directive.define",
            DirectiveTokenKind::Copy => "@keyword.directive.define",
            DirectiveTokenKind::Include => "@keyword.directive.define",
            DirectiveTokenKind::Set => "@keyword.directive.define",
            DirectiveTokenKind::Restore => "@keyword.directive.define",
            DirectiveTokenKind::OverloadDetail => "@keyword.directive.define",
            DirectiveTokenKind::OverloadNoDetail => "@keyword.directive.define",
            DirectiveTokenKind::Define => "@keyword.directive.define",
            DirectiveTokenKind::Undefine => "@keyword.directive.define",
            DirectiveTokenKind::If => "@keyword.directive.define",
            DirectiveTokenKind::Elseif => "@keyword.directive.define",
            DirectiveTokenKind::Else => "@keyword.directive.define",
            DirectiveTokenKind::Endif => "@keyword.directive.define",
            DirectiveTokenKind::Eof => "@keyword.directive.define",
        };
        vec![(self.span(), hlgroup.to_string())]
    }

    pub fn span(&self) -> crate::Span {
        self.meta.span
    }
}

fn next_token(lexer: &Lexer) -> Option<DirectiveToken> {
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
        Some('/') => match peek_n(lexer, 1) {
            Some(x) => match is_identifier_char(&x) {
                true => {
                    let mut chars = vec![read_char(lexer)];
                    let mut litchars = read_identifier(lexer);
                    let literal = litchars.iter().collect::<String>();
                    let kind = match literal.to_uppercase().as_str() {
                        "TITLE" => DirectiveTokenKind::Title,
                        "EJECT" => DirectiveTokenKind::Eject,
                        "SPACE" => DirectiveTokenKind::Space,
                        "COPY" => DirectiveTokenKind::Copy,
                        "INCLUDE" => DirectiveTokenKind::Include,
                        "SET" => DirectiveTokenKind::Set,
                        "RESTORE" => DirectiveTokenKind::Restore,
                        "OVERLOADDETAIL" => DirectiveTokenKind::OverloadDetail,
                        "OVERLOADNODETAIL" => DirectiveTokenKind::OverloadNoDetail,
                        "DEFINE" => DirectiveTokenKind::Define,
                        "UNDEFINE" => DirectiveTokenKind::Undefine,
                        "IF" => DirectiveTokenKind::If,
                        "ELSEIF" => DirectiveTokenKind::Elseif,
                        "ELSE" => DirectiveTokenKind::Else,
                        "ENDIF" => DirectiveTokenKind::Endif,
                        "EOF" => DirectiveTokenKind::Eof,
                        _ => DirectiveTokenKind::Idk,
                    };
                    chars.append(&mut litchars);
                    if matches!(kind, DirectiveTokenKind::Idk) {
                        chars.append(&mut read_all(lexer));
                    }
                    (kind, chars)
                }
                false => {
                    let chars = read_all(lexer);
                    let kind = DirectiveTokenKind::Idk;
                    (kind, chars)
                }
            },
            _ => {
                let chars = read_all(lexer);
                let kind = DirectiveTokenKind::Idk;
                (kind, chars)
            }
        },
        _ => {
            let chars = read_all(lexer);
            let kind = DirectiveTokenKind::Idk;
            (kind, chars)
        }
    };
    let meta = Meta::from((start, chars.as_slice()));
    let tok = DirectiveToken { kind, meta };
    Some(tok)
}

pub fn tokenize_directive(pos: Position, chars: &[char; 94]) -> Vec<DirectiveToken> {
    let state = LexerState {
        origin: pos,
        col: 0,
    };
    let value = NonEmpty::from_vec(chars.iter().map(|c| *c).collect::<Vec<char>>())
        .expect("&[char; 94] is guaranteed to be nonempty");
    let lexer = Lexer {
        state: RefCell::new(state),
        input: value,
    };
    let mut tokens = vec![];
    let mut tmp = 0;
    loop {
        match next_token(&lexer) {
            Some(token) => {
                tokens.push(token);
            }
            None => {
                break;
            }
        }
        tmp += 1;
        if tmp > 1000 {
            break;
        }
    }
    tokens
}
