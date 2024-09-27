use crate::meta::Position;
use nonempty::NonEmpty;
use std::cell::RefCell;

pub struct LexerState {
    pub origin: Position,
    pub col: usize,
}

pub struct Lexer {
    pub state: RefCell<LexerState>,
    pub input: NonEmpty<char>,
}

pub fn ch(lexer: &Lexer) -> Option<&char> {
    let idx = lexer.state.borrow().col;
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

pub fn peek_n(lexer: &Lexer, n: usize) -> Option<&char> {
    let idx = lexer.state.borrow().col;
    lexer.input.get(idx + n)
}

pub fn peek_all(lexer: &Lexer) -> Vec<&char> {
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

pub fn peek_until(lexer: &Lexer, x: char) -> Option<&char> {
    let mut c = 1;
    loop {
        if let Some(xx) = peek_n(lexer, c) {
            if *xx == x {
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

pub fn peek_until_any(lexer: &Lexer, xs: Vec<char>) -> Option<&char> {
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
pub fn read_char(lexer: &Lexer) -> char {
    let out = *ch(lexer).expect("read_char() requires a length check prior to call");
    lexer.state.borrow_mut().col += 1;
    out
}

pub fn read_all(lexer: &Lexer) -> Vec<char> {
    let mut out = vec![];
    while ch(lexer).is_some() {
        out.push(read_char(lexer));
    }
    out
}

pub fn read_until(lexer: &Lexer, x: char) -> Vec<char> {
    let mut c = 0;
    let mut out = vec![];
    loop {
        match ch(lexer) {
            Some(xx) => {
                if *xx == x {
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

pub fn read_spaces_or_tabs(lexer: &Lexer) -> Vec<char> {
    let mut out = vec![];
    while ch(lexer).is_some() && is_space_or_tab(&ch(lexer).unwrap()) {
        out.push(read_char(lexer));
    }
    out
}

pub fn read_identifier(lexer: &Lexer) -> Vec<char> {
    let mut out = vec![];
    while ch(lexer).is_some() && is_identifier_char(&ch(lexer).unwrap()) {
        out.push(read_char(lexer));
    }
    out
}

pub fn read_number(lexer: &Lexer) -> Vec<char> {
    let mut out = vec![];
    while ch(lexer).is_some() && is_numeric(&ch(lexer).unwrap()) {
        out.push(read_char(lexer));
    }
    out
}

pub fn read_string_literal(lexer: &Lexer) -> Vec<char> {
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
