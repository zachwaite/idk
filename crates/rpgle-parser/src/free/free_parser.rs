use super::lexer::{
    ch, is_identifier_char, is_numeric, peek_n, peek_until, read_all, read_char, read_identifier,
    read_number, read_spaces_or_tabs, read_string_literal, Lexer, LexerState,
};
use crate::field::FieldResult;
use crate::line::{
    ExtF2CSpecLine, ExtF2CSpecLineContinuation, FreeCSpecLine, FreeCSpecLineContinuation,
    TraditionalCSpecLine,
};
use crate::meta::{Meta, PMixin, Position, Span};
use nonempty::{nonempty, NonEmpty};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    Idk,
    Whitespace,
    Number,
    LParen,
    RParen,
    Colon,
    Semicolon,
    Equals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
    Plus,
    PlusEquals,
    Minus,
    MinusEquals,
    Asterisk,
    AsteriskEquals,
    Slash,
    SlashEquals,
    Comment,
    Identifier,
    StringLiteral,
    Indicator,
    IndicatorValue,
    FigurativeConstant,
    Builtin,
    // opcodes
    SetLL,
    SetGT,
    Chain,
    Read,
    ReadE,
    ReadPE,
    Write,
    Update,
    Delete,
    If,
    Or,
    And,
    Else,
    Elseif,
    Endif,
    Dou,
    Dow,
    Enddo,
    Iter,
    Leave,
    Reset,
    Eval,
    Clear,
    Begsr,
    Endsr,
    Exsr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub meta: Meta,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl PMixin for Token {
    fn highlight(&self) -> Vec<(Span, String)> {
        let hlgroup = match self.kind {
            TokenKind::Idk => "Normal",
            TokenKind::Whitespace => "Normal",
            TokenKind::Number => "@number",
            TokenKind::Colon => "Normal",
            TokenKind::Semicolon => "Normal",
            TokenKind::LParen => "Normal",
            TokenKind::RParen => "Normal",
            TokenKind::Equals => "Normal",
            TokenKind::LessThan => "Normal",
            TokenKind::LessThanOrEquals => "Normal",
            TokenKind::GreaterThan => "Normal",
            TokenKind::GreaterThanOrEquals => "Normal",
            TokenKind::Plus => "Normal",
            TokenKind::PlusEquals => "Normal",
            TokenKind::Minus => "Normal",
            TokenKind::MinusEquals => "Normal",
            TokenKind::Asterisk => "Normal",
            TokenKind::AsteriskEquals => "Normal",
            TokenKind::Slash => "Normal",
            TokenKind::SlashEquals => "Normal",
            TokenKind::Comment => "@comment",
            TokenKind::Identifier => "Identifier",
            TokenKind::StringLiteral => "String",
            TokenKind::Indicator => "@variable.builtin",
            TokenKind::IndicatorValue => "@boolean",
            TokenKind::FigurativeConstant => "@constant.builtin",
            TokenKind::Builtin => "@function.builtin",
            // opcodes
            TokenKind::SetLL => "@function.builtin",
            TokenKind::SetGT => "@function.builtin",
            TokenKind::Chain => "@function.builtin",
            TokenKind::Read => "@function.builtin",
            TokenKind::ReadE => "@function.builtin",
            TokenKind::ReadPE => "@function.builtin",
            TokenKind::Write => "@function.builtin",
            TokenKind::Update => "@function.builtin",
            TokenKind::Delete => "@function.builtin",
            TokenKind::If => "@function.builtin",
            TokenKind::Or => "@function.builtin",
            TokenKind::And => "@function.builtin",
            TokenKind::Else => "@function.builtin",
            TokenKind::Elseif => "@function.builtin",
            TokenKind::Endif => "@function.builtin",
            TokenKind::Dou => "@function.builtin",
            TokenKind::Dow => "@function.builtin",
            TokenKind::Enddo => "@function.builtin",
            TokenKind::Iter => "@function.builtin",
            TokenKind::Leave => "@function.builtin",
            TokenKind::Reset => "@function.builtin",
            TokenKind::Eval => "@function.builtin",
            TokenKind::Clear => "@function.builtin",
            TokenKind::Begsr => "@function.builtin",
            TokenKind::Endsr => "@function.builtin",
            TokenKind::Exsr => "@function.builtin",
        };
        vec![(self.span(), hlgroup.to_string())]
    }

    fn span(&self) -> crate::Span {
        self.meta.span
    }
}

fn next_token(lexer: &Lexer) -> Option<Token> {
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
            let kind = TokenKind::Whitespace;
            (kind, chars)
        }
        // quote
        Some('\'') => match peek_until(lexer, '\'') {
            Some(_) => {
                let chars = read_string_literal(lexer);
                let kind = TokenKind::StringLiteral;
                (kind, chars)
            }
            None => {
                let mut chars = vec!['\''];
                chars.append(&mut read_all(lexer));
                let kind = TokenKind::Idk;
                (kind, chars)
            }
        },
        // lparen
        Some('(') => {
            let chars = vec![read_char(lexer)];
            let kind = TokenKind::LParen;
            (kind, chars)
        }
        // rparen
        Some(')') => {
            let chars = vec![read_char(lexer)];
            let kind = TokenKind::RParen;
            (kind, chars)
        }
        // colon
        Some(':') => {
            let chars = vec![read_char(lexer)];
            let kind = TokenKind::Colon;
            (kind, chars)
        }
        // semicolon
        Some(';') => {
            let chars = vec![read_char(lexer)];
            let kind = TokenKind::Semicolon;
            (kind, chars)
        }
        // equals
        Some('=') => {
            let chars = vec![read_char(lexer)];
            let kind = TokenKind::Equals;
            (kind, chars)
        }
        // less than
        Some('<') => match peek_n(lexer, 1) {
            Some('=') => {
                let _ = read_char(lexer);
                let _ = read_char(lexer);
                let chars = vec!['<', '='];
                let kind = TokenKind::LessThanOrEquals;
                (kind, chars)
            }
            _ => {
                let _ = read_char(lexer);
                let chars = vec!['<'];
                let kind = TokenKind::LessThan;
                (kind, chars)
            }
        },
        // greater than
        Some('>') => match peek_n(lexer, 1) {
            Some('=') => {
                let _ = read_char(lexer);
                let _ = read_char(lexer);
                let chars = vec!['>', '='];
                let kind = TokenKind::GreaterThanOrEquals;
                (kind, chars)
            }
            _ => {
                let _ = read_char(lexer);
                let chars = vec!['<'];
                let kind = TokenKind::GreaterThan;
                (kind, chars)
            }
        },
        // plus
        Some('+') => match peek_n(lexer, 1) {
            Some('=') => {
                let _ = read_char(lexer);
                let _ = read_char(lexer);
                let chars = vec!['+', '='];
                let kind = TokenKind::PlusEquals;
                (kind, chars)
            }
            _ => {
                let _ = read_char(lexer);
                let chars = vec!['+'];
                let kind = TokenKind::Plus;
                (kind, chars)
            }
        },
        // minus
        Some('-') => match peek_n(lexer, 1) {
            Some('=') => {
                let _ = read_char(lexer);
                let _ = read_char(lexer);
                let chars = vec!['-', '='];
                let kind = TokenKind::MinusEquals;
                (kind, chars)
            }
            _ => {
                let _ = read_char(lexer);
                let chars = vec!['-'];
                let kind = TokenKind::Minus;
                (kind, chars)
            }
        },
        // slash
        Some('/') => match peek_n(lexer, 1) {
            Some('=') => {
                let _ = read_char(lexer);
                let _ = read_char(lexer);
                let chars = vec!['/', '='];
                let kind = TokenKind::SlashEquals;
                (kind, chars)
            }
            Some('/') => {
                let _ = read_char(lexer);
                let _ = read_char(lexer);
                let mut chars = vec!['/', '/'];
                chars.append(&mut read_all(lexer));
                let kind = TokenKind::Comment;
                (kind, chars)
            }
            _ => {
                let _ = read_char(lexer);
                let chars = vec!['/'];
                let kind = TokenKind::Slash;
                (kind, chars)
            }
        },
        // asterisk
        Some('*') => {
            match peek_n(lexer, 1) {
                Some('=') => {
                    let _ = read_char(lexer);
                    let _ = read_char(lexer);
                    let chars = vec!['*', '='];
                    let kind = TokenKind::AsteriskEquals;
                    (kind, chars)
                }
                Some(x) => {
                    match (is_identifier_char(x), is_numeric(x)) {
                        (true, false) => {
                            //indicator
                            let _ = read_char(lexer);
                            let mut chars = vec!['*'];
                            let mut litchars = read_identifier(lexer);
                            let literal = litchars.iter().collect::<String>();
                            let kind = match literal.to_uppercase().as_str() {
                                "ON" => TokenKind::IndicatorValue,
                                "OFF" => TokenKind::IndicatorValue,
                                "BLANK" => TokenKind::FigurativeConstant,
                                "BLANKS" => TokenKind::FigurativeConstant,
                                "ZERO" => TokenKind::FigurativeConstant,
                                "ZEROS" => TokenKind::FigurativeConstant,
                                "HIVAL" => TokenKind::FigurativeConstant,
                                "LOVAL" => TokenKind::FigurativeConstant,
                                "NULL" => TokenKind::FigurativeConstant,
                                x => {
                                    if x.starts_with("ALL") {
                                        TokenKind::FigurativeConstant
                                    } else {
                                        TokenKind::Indicator
                                    }
                                }
                            };
                            chars.append(&mut litchars);
                            (kind, chars)
                        }
                        (_, _) => {
                            let _ = read_char(lexer);
                            let chars = vec!['*'];
                            let kind = TokenKind::Asterisk;
                            (kind, chars)
                        }
                    }
                }
                _ => {
                    let _ = read_char(lexer);
                    let chars = vec!['*'];
                    let kind = TokenKind::Asterisk;
                    (kind, chars)
                }
            }
        }
        // pct
        Some('%') => match peek_n(lexer, 1) {
            Some(x) => match is_identifier_char(x) {
                true => {
                    let _ = read_char(lexer);
                    let mut chars = vec!['%'];
                    chars.append(&mut read_identifier(lexer));
                    let kind = TokenKind::Builtin;
                    (kind, chars)
                }
                false => {
                    let chars = read_all(lexer);
                    let kind = TokenKind::Idk;
                    (kind, chars)
                }
            },
            None => {
                let chars = read_all(lexer);
                let kind = TokenKind::Idk;
                (kind, chars)
            }
        },
        // identifier
        Some(x) => match is_identifier_char(&x) {
            true => match is_numeric(&x) {
                true => {
                    let chars = read_number(lexer);
                    let kind = TokenKind::Number;
                    (kind, chars)
                }
                false => {
                    let chars = read_identifier(lexer);
                    let literal: String = chars.iter().collect::<String>();
                    let kind = match literal.to_uppercase().as_str() {
                        "SETLL" => TokenKind::SetLL,
                        "SETGT" => TokenKind::SetGT,
                        "CHAIN" => TokenKind::Chain,
                        "READ" => TokenKind::Read,
                        "READE" => TokenKind::ReadE,
                        "READPE" => TokenKind::ReadPE,
                        "WRITE" => TokenKind::Write,
                        "UPDATE" => TokenKind::Update,
                        "DELETE" => TokenKind::Delete,
                        "IF" => TokenKind::If,
                        "OR" => TokenKind::Or,
                        "AND" => TokenKind::And,
                        "ELSE" => TokenKind::Else,
                        "ELSEIF" => TokenKind::Elseif,
                        "ENDIF" => TokenKind::Endif,
                        "DOU" => TokenKind::Dou,
                        "DOW" => TokenKind::Dow,
                        "ENDDO" => TokenKind::Enddo,
                        "ITER" => TokenKind::Iter,
                        "LEAVE" => TokenKind::Leave,
                        "RESET" => TokenKind::Reset,
                        "EVAL" => TokenKind::Eval,
                        "CLEAR" => TokenKind::Clear,
                        "BEGSR" => TokenKind::Begsr,
                        "ENDSR" => TokenKind::Endsr,
                        "EXSR" => TokenKind::Exsr,
                        _ => TokenKind::Identifier,
                    };
                    (kind, chars)
                }
            },
            false => {
                let chars = read_all(lexer);
                let kind = TokenKind::Idk;
                (kind, chars)
            }
        },
        _ => {
            let chars = read_all(lexer);
            let kind = TokenKind::Idk;
            (kind, chars)
        }
    };
    let meta = Meta::from((start, chars.as_slice()));
    let tok = Token { kind, meta };
    Some(tok)
}
// TDE: remove dupage
pub fn tokenize_traditional_f2(line: &TraditionalCSpecLine) -> NonEmpty<Token> {
    match &line.factor2 {
        FieldResult::Ok(code) => {
            let pos = code.meta.span.start;
            let state = LexerState {
                origin: pos,
                col: 0,
            };
            let lexer = Lexer {
                state: RefCell::new(state),
                input: code.value.clone(), // TDE: lifetime
            };
            let mut tokens = nonempty![next_token(&lexer).expect("guaranteed at least 1 token")];
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
        FieldResult::Idk(fld) => {
            let tok = Token {
                kind: TokenKind::Idk,
                meta: fld.meta.clone(),
            };
            nonempty![tok].into()
        }
    }
}

pub fn tokenize_extf2(
    line: &ExtF2CSpecLine,
    continuations: Vec<&ExtF2CSpecLineContinuation>,
) -> NonEmpty<Token> {
    match &line.factor2 {
        FieldResult::Ok(code) => {
            let pos = code.meta.span.start;
            let state = LexerState {
                origin: pos,
                col: 0,
            };
            let lexer = Lexer {
                state: RefCell::new(state),
                input: code.value.clone(), // TDE: lifetime
            };
            let mut tokens = nonempty![next_token(&lexer).expect("guaranteed at least 1 token")];
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
        FieldResult::Idk(fld) => {
            let tok = Token {
                kind: TokenKind::Idk,
                meta: fld.meta.clone(),
            };
            nonempty![tok].into()
        }
    }
}

pub fn tokenize(
    line: &FreeCSpecLine,
    continuations: Vec<&FreeCSpecLineContinuation>,
) -> NonEmpty<Token> {
    match &line.code {
        FieldResult::Ok(code) => {
            let pos = code.meta.span.start;
            let state = LexerState {
                origin: pos,
                col: 0,
            };
            let lexer = Lexer {
                state: RefCell::new(state),
                input: code.value.clone(),
            };
            let mut tokens = nonempty![next_token(&lexer).expect("guaranteed at least 1 token")];
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
            tokens.into()
        }
        FieldResult::Idk(fld) => {
            let tok = Token {
                kind: TokenKind::Idk,
                meta: fld.meta.clone(),
            };
            nonempty![tok].into()
        }
    }
}
