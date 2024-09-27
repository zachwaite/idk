use crate::meta::{Meta, Position, Span};
use nonempty::NonEmpty;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MetaChar {
    pub value: char,
    pub position: Position,
}

pub fn cut_into_metas(mchars: Vec<MetaChar>) -> Vec<Meta> {
    // assume mchars is sorted ascending
    if let Some(mchar0) = mchars.get(0) {
        let mut out = vec![];
        let mut buf = vec![];
        let mut current_row = mchar0.position.row;
        for mchar in mchars.iter() {
            if mchar.position.row == current_row {
                buf.push(mchar);
            } else if mchar.position.row > current_row {
                // flush
                let start = buf.first().expect("expect nonempty vec").position;
                let end = buf.last().expect("expect nonempty vec").position;
                let meta = Meta {
                    span: Span { start, end },
                    text: buf.iter().map(|mc| mc.value).collect::<String>(),
                };
                out.push(meta);
                current_row = mchar.position.row;
                buf.drain(0..);
                // store this one
                buf.push(mchar);
            } else {
                panic!("expected mchars to be sorted ascending")
            }
        }
        // flush again
        let start = buf.first().expect("expect nonempty vec").position;
        let end = Position {
            row: buf.last().expect("expect nonempty vec").position.row,
            col: buf.last().expect("expect nonempty vec").position.col + 1,
        };
        let meta = Meta {
            span: Span { start, end },
            text: buf.iter().map(|mc| mc.value).collect::<String>(),
        };
        out.push(meta);
        return out;
    }
    vec![]
}

pub struct LexerState {
    pub position: Position,
    pub idx: usize,
}

pub struct Lexer {
    pub state: RefCell<LexerState>,
    pub input: NonEmpty<MetaChar>,
}

pub fn ch(lexer: &Lexer) -> Option<&MetaChar> {
    let idx = lexer.state.borrow().idx;
    lexer.input.get(idx)
}

pub fn is_identifier_char(ch: &char) -> bool {
    ch.is_alphanumeric() || *ch == '@' || *ch == '$' || *ch == '-' || *ch == '#'
}

pub fn is_numeric(ch: &char) -> bool {
    ch.is_numeric()
}

pub fn is_space_or_tab(ch: &char) -> bool {
    match ch {
        ' ' => true,
        '\t' => true,
        _ => false,
    }
}

pub fn peek_n(lexer: &Lexer, n: usize) -> Option<&MetaChar> {
    let idx = lexer.state.borrow().idx;
    lexer.input.get(idx + n)
}

pub fn peek_all(lexer: &Lexer) -> Vec<&MetaChar> {
    let mut out = vec![];
    let mut c = 0;
    loop {
        match peek_n(lexer, c) {
            Some(x) => {
                out.push(x);
                c += 1;
            }
            None => {
                break;
            }
        }
    }
    out
}

pub fn peek_until(lexer: &Lexer, x: char) -> Option<&MetaChar> {
    let mut c = 1;
    loop {
        if let Some(xx) = peek_n(lexer, c) {
            if xx.value == x {
                return Some(xx);
            } else {
                c += 1;
                if c < 1000 {
                    continue;
                } else {
                    todo!()
                }
            };
        } else {
            return None;
        }
    }
}

pub fn peek_until_any(lexer: &Lexer, xs: Vec<char>) -> Option<&MetaChar> {
    for x in xs.iter() {
        match peek_until(lexer, *x) {
            Some(xx) => return Some(xx),
            None => {
                continue;
            }
        }
    }
    None
}

// `read_X()` functions expect you to verify they are valid before calling
pub fn read_char(lexer: &Lexer) -> MetaChar {
    let out = *ch(lexer).expect("read_char() requires a length check prior to call");
    let nextpos = match peek_n(lexer, 1) {
        Some(mc) => mc.position,
        None => Position {
            row: out.position.row,
            col: out.position.col + 1,
        },
    };
    let mut state = lexer.state.borrow_mut();
    state.idx += 1;
    state.position = nextpos;
    out
}

pub fn read_all(lexer: &Lexer) -> Vec<MetaChar> {
    let mut out = vec![];
    while ch(lexer).is_some() {
        out.push(read_char(lexer));
    }
    out
}

pub fn read_until(lexer: &Lexer, x: char) -> Vec<MetaChar> {
    let mut c = 0;
    let mut out = vec![];
    loop {
        match ch(lexer) {
            Some(xx) => {
                if xx.value == x {
                    break;
                } else {
                    out.push(read_char(lexer));
                    c += 1;
                    if c < 1000 {
                        continue;
                    } else {
                        break;
                    }
                }
            }
            None => {
                break;
            }
        }
    }
    out
}

pub fn read_spaces_or_tabs(lexer: &Lexer) -> Vec<MetaChar> {
    let mut out = vec![];
    while ch(lexer).is_some() && is_space_or_tab(&ch(lexer).unwrap().value) {
        out.push(read_char(lexer));
    }
    out
}

pub fn read_identifier(lexer: &Lexer) -> Vec<MetaChar> {
    let mut out = vec![];
    while ch(lexer).is_some() && is_identifier_char(&ch(lexer).unwrap().value) {
        out.push(read_char(lexer));
    }
    out
}

pub fn read_number(lexer: &Lexer) -> Vec<MetaChar> {
    let mut out = vec![];
    while ch(lexer).is_some() && is_numeric(&ch(lexer).unwrap().value) {
        out.push(read_char(lexer));
    }
    out
}

pub fn read_string_literal(lexer: &Lexer) -> Vec<MetaChar> {
    let mut out = vec![];
    // The caller must check the first char is a quote
    // AND the last char is a quote - see `peek_until()`
    let first_quote = read_char(lexer);
    out.push(first_quote);

    let mut literals = read_until(lexer, '\'');
    out.append(&mut literals);

    let last_quote = read_char(lexer);
    out.push(last_quote);
    out
}
