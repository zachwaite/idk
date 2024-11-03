use crate::field::{Factor1Field, FieldResult, OperationField, RawCodeField, RawFactor2Field};
use crate::line::{
    CSpecLine, CSpecLineContinuation, ExtF2CSpecLine, ExtF2CSpecLineContinuation,
    TraditionalCSpecLine,
};
use crate::line::{FreeCSpecLine, FreeCSpecLineContinuation};
use crate::meta::{Meta, PMixin, Position, Span};
use nonempty::{nonempty, NonEmpty};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::env;
use std::fmt;
use std::fmt::Display;

use super::{
    legacy_tokenize, legacy_tokenize_extf2, tokenize, tokenize_extf2, tokenize_traditional_f2,
    Token, TokenKind,
};

struct ParserState {
    idx: usize,
}

struct Parser<'a> {
    state: RefCell<ParserState>,
    input: &'a NonEmpty<Token>,
}
impl<'a> Parser<'a> {
    fn new(input: &'a NonEmpty<Token>) -> Self {
        Self {
            state: RefCell::new(ParserState { idx: 0 }),
            input,
        }
    }
    fn highlights(&self) -> Vec<(Span, String)> {
        self.input
            .iter()
            .flat_map(|t| t.highlight())
            .collect::<Vec<(Span, String)>>()
    }
    fn peek_n(&self, n: usize) -> Result<&Token, String> {
        match self.input.get(n) {
            Some(t) => Ok(t),
            None => Err("EOL".to_string()),
        }
    }
    fn peek_until_any(
        &self,
        kinds: &Vec<TokenKind>,
        start: usize,
    ) -> Result<(usize, Vec<&Token>), String> {
        let _ = self.peek_n(start)?;
        let mut n = start;
        let mut out = vec![];
        loop {
            if let Ok(t) = self.peek_n(n) {
                if kinds.contains(&t.kind) {
                    break;
                }
                out.push(t);
                n += 1;
                continue;
            }
            break;
        }
        Ok((n, out))
    }
    fn peek_while_any(
        &self,
        kinds: &Vec<TokenKind>,
        start: usize,
    ) -> Result<(usize, Vec<&Token>), String> {
        let _ = self.peek_n(start)?;
        let mut n = start;
        let mut out = vec![];
        loop {
            if let Ok(t) = self.peek_n(n) {
                if !kinds.contains(&t.kind) {
                    break;
                }
                out.push(t);
                n += 1;
                continue;
            }
            break;
        }
        Ok((n, out))
    }
    fn peek_while_whitespace(&self, start: usize) -> Result<(usize, Vec<&Token>), String> {
        let _ = self.peek_n(start)?;
        let spaces = vec![TokenKind::Whitespace];
        self.peek_while_any(&spaces, start)
    }
    fn peek_while_insignificant(&self, start: usize) -> Result<(usize, Vec<&Token>), String> {
        let _ = self.peek_n(start)?;
        let tks = vec![TokenKind::Whitespace, TokenKind::Comment];
        self.peek_while_any(&tks, start)
    }
    fn peek_n_guard(&self, kind: &TokenKind, n: usize) -> Result<(usize, &Token), String> {
        let t = self.peek_n(n)?;
        if &t.kind != kind {
            let msg = format!("Expected {:#?}, found {:#?}", &kind, &t.kind);
            return Err(msg);
        }
        Ok((n + 1, t))
    }
    fn advance(&self) {
        self.state.borrow_mut().idx += 1;
    }
    fn advance_until_position(&self, pos: usize) {
        while self.state.borrow().idx < pos {
            self.advance()
        }
    }
    fn read(&self) -> Result<&Token, String> {
        let i = self.state.borrow().idx;
        self.state.borrow_mut().idx += 1;
        self.peek_n(i)
    }
    fn read_until_any(&self, kinds: &Vec<TokenKind>) -> Result<Vec<&Token>, String> {
        let _ = self.peek_n(0)?;
        let mut out = vec![];
        loop {
            if let Ok(t) = self.peek_n(0) {
                if kinds.contains(&t.kind) {
                    break;
                }
                out.push(self.read()?);
                continue;
            }
            break;
        }
        Ok(out)
    }
    fn read_while_any(&self, kinds: &Vec<TokenKind>) -> Result<Vec<&Token>, String> {
        let _ = self.peek_n(0)?;
        let mut out = vec![];
        loop {
            if let Ok(t) = self.peek_n(0) {
                if !kinds.contains(&t.kind) {
                    break;
                }
                out.push(self.read()?);
                continue;
            }
            break;
        }
        Ok(out)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Op {
    Begsr {
        name: String,
        meta: Meta,
        highlights: Vec<(Span, String)>,
    },
    Endsr {
        meta: Meta,
        highlights: Vec<(Span, String)>,
    },
    Exsr {
        name: String,
        meta: Meta,
        highlights: Vec<(Span, String)>,
    },
    Callp {
        name: String,
        meta: Meta,
        highlights: Vec<(Span, String)>,
    },
    Idk {
        meta: Meta,
        error: String,
        tokens: Vec<Token>,
        highlights: Vec<(Span, String)>,
    },
}
impl Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let txt = match self {
            Op::Idk { meta, .. } => &meta.text,
            Op::Exsr { meta, .. } => &meta.text,
            Op::Begsr { meta, .. } => &meta.text,
            Op::Endsr { meta, .. } => &meta.text,
            Op::Callp { meta, .. } => &meta.text,
        };
        write!(f, "{}", txt)
    }
}
impl Op {
    pub fn kind(&self) -> String {
        let s = match self {
            Op::Begsr { .. } => "BEGSR",
            Op::Endsr { .. } => "ENDSR",
            Op::Exsr { .. } => "EXSR",
            Op::Callp { .. } => "CALLP",
            Op::Idk { .. } => "IDK",
        };
        s.to_string()
    }
    pub fn span(&self) -> Span {
        match self {
            Op::Idk { meta, .. } => meta.span,
            Op::Exsr { meta, .. } => meta.span,
            Op::Begsr { meta, .. } => meta.span,
            Op::Endsr { meta, .. } => meta.span,
            Op::Callp { meta, .. } => meta.span,
        }
    }
    pub fn highlight(&self) -> Vec<(Span, String)> {
        match self {
            Op::Idk { highlights, .. } => highlights.clone(),
            Op::Exsr { highlights, .. } => highlights.clone(),
            Op::Begsr { highlights, .. } => highlights.clone(),
            Op::Endsr { highlights, .. } => highlights.clone(),
            Op::Callp { highlights, .. } => highlights.clone(),
        }
    }
}

fn parse_exsr(parser: &Parser) -> Result<Op, String> {
    // free only
    // return Some(exsr_op) if it can get Op::Exsr out and have no significant tokens left
    // phase 1: peek and capture the parts
    let (i, _spaces1) = parser.peek_while_whitespace(0)?;
    let (i, _opcode) = parser.peek_n_guard(&TokenKind::Exsr, i)?;
    let (i, _spaces2) = parser.peek_while_whitespace(i)?;
    let (i, name) = parser.peek_n_guard(&TokenKind::Identifier, i)?;
    let (i, _semi) = parser.peek_n_guard(&TokenKind::Semicolon, i)?;
    let (i, _insigs) = parser.peek_while_insignificant(i).unwrap_or((i, vec![]));
    // phase 2: capture meta and advance if all significant tokens consumed
    if i == parser.input.len() {
        let meta = Meta::from((
            &parser.input.head.meta,
            parser
                .input
                .tail
                .iter()
                .map(|t| &t.meta)
                .collect::<Vec<&Meta>>(),
        ));
        let highlights = parser.highlights();

        let _ = parser.advance_until_position(i);
        return Ok(Op::Exsr {
            name: name.meta.text.to_string(),
            meta,
            highlights,
        });
    } else {
        Err("parse_exsr(): did not consume entire input".to_string())
    }
}
fn parse_begsr(parser: &Parser) -> Result<Op, String> {
    // free only
    let (i, _spaces1) = parser.peek_while_whitespace(0)?;
    let (i, _opcode) = parser.peek_n_guard(&TokenKind::Begsr, i)?;
    let (i, _spaces2) = parser.peek_while_whitespace(i)?;
    let (i, name) = parser.peek_n_guard(&TokenKind::Identifier, i)?;
    let (i, _semi) = parser.peek_n_guard(&TokenKind::Semicolon, i)?;
    let (i, _insigs) = parser.peek_while_insignificant(i).unwrap_or((i, vec![]));
    if i == parser.input.len() {
        let meta = Meta::from((
            &parser.input.head.meta,
            parser
                .input
                .tail
                .iter()
                .map(|t| &t.meta)
                .collect::<Vec<&Meta>>(),
        ));
        let highlights = parser.highlights();

        let _ = parser.advance_until_position(i);
        return Ok(Op::Begsr {
            name: name.meta.text.to_string(),
            meta,
            highlights,
        });
    }
    Err("ERROR".to_string())
}
fn parse_endsr(parser: &Parser) -> Result<Op, String> {
    // free only
    let (i, _spaces1) = parser.peek_while_whitespace(0)?;
    let (i, _opcode) = parser.peek_n_guard(&TokenKind::Endsr, i)?;
    let (i, _semi) = parser.peek_n_guard(&TokenKind::Semicolon, i)?;
    let (i, _insigs) = parser.peek_while_insignificant(i).unwrap_or((i, vec![]));
    if i == parser.input.len() {
        let meta = Meta::from((
            &parser.input.head.meta,
            parser
                .input
                .tail
                .iter()
                .map(|t| &t.meta)
                .collect::<Vec<&Meta>>(),
        ));
        let highlights = parser.highlights();

        let _ = parser.advance_until_position(i);
        return Ok(Op::Endsr { meta, highlights });
    }
    Err("ERROR".to_string())
}
fn parse_callp(parser: &Parser) -> Result<Op, String> {
    // free only
    let (i, _spaces1) = parser.peek_while_whitespace(0)?;
    let (i, name) = parser.peek_n_guard(&TokenKind::Identifier, i)?;
    let (i, _spaces2) = parser.peek_while_whitespace(i)?;
    let (i, _lparen) = parser.peek_n_guard(&TokenKind::LParen, i)?;
    let (i, _parms) = parser.peek_until_any(&vec![TokenKind::RParen], i)?;
    let (i, _rparen) = parser.peek_n_guard(&TokenKind::RParen, i)?;
    let (i, _semi) = parser.peek_n_guard(&TokenKind::Semicolon, i)?;
    let (i, _insigs) = parser.peek_while_insignificant(i).unwrap_or((i, vec![]));
    if i == parser.input.len() {
        let meta = Meta::from((
            &parser.input.head.meta,
            parser
                .input
                .tail
                .iter()
                .map(|t| &t.meta)
                .collect::<Vec<&Meta>>(),
        ));
        let highlights = parser.highlights();

        let _ = parser.advance_until_position(i);
        return Ok(Op::Callp {
            name: name.meta.text.to_string(),
            meta,
            highlights,
        });
    }
    Err("ERROR".to_string())
}

// DEPRECATED
impl From<(&FreeCSpecLine, Vec<&FreeCSpecLineContinuation>)> for Op {
    fn from(value: (&FreeCSpecLine, Vec<&FreeCSpecLineContinuation>)) -> Self {
        let tokens = tokenize(value.0, value.1);
        let parser = Parser {
            state: RefCell::new(ParserState { idx: 0 }),
            input: &tokens,
        };
        let maybe_op = parse_exsr(&parser)
            .or(parse_begsr(&parser))
            .or(parse_endsr(&parser))
            .or(parse_callp(&parser));
        match maybe_op {
            Ok(op) => op,
            Err(x) => {
                let meta = Meta::from((
                    &tokens.head.meta,
                    tokens.tail.iter().map(|t| &t.meta).collect::<Vec<&Meta>>(),
                ));
                let tokens: Vec<Token> = if env::var("DEBUG").is_ok() {
                    parser.input.iter().map(|t| t.clone()).collect::<_>()
                } else {
                    vec![]
                };
                Op::Idk {
                    meta,
                    highlights: parser.highlights(),
                    tokens,
                    error: x,
                }
            }
        }
    }
}
impl From<(&FieldResult<RawCodeField>, &[&FieldResult<RawCodeField>])> for Op {
    fn from(value: (&FieldResult<RawCodeField>, &[&FieldResult<RawCodeField>])) -> Self {
        let tokens = legacy_tokenize(value.0, value.1);
        let parser = Parser {
            state: RefCell::new(ParserState { idx: 0 }),
            input: &tokens,
        };
        let maybe_op = parse_exsr(&parser)
            .or(parse_begsr(&parser))
            .or(parse_endsr(&parser))
            .or(parse_callp(&parser));
        match maybe_op {
            Ok(op) => op,
            Err(x) => {
                let meta = Meta::from((
                    &tokens.head.meta,
                    tokens.tail.iter().map(|t| &t.meta).collect::<Vec<&Meta>>(),
                ));
                let tokens: Vec<Token> = if env::var("DEBUG").is_ok() {
                    parser.input.iter().map(|t| t.clone()).collect::<_>()
                } else {
                    vec![]
                };
                Op::Idk {
                    meta,
                    highlights: parser.highlights(),
                    tokens,
                    error: x,
                }
            }
        }
    }
}

// DEPRECATED
impl From<(&ExtF2CSpecLine, Vec<&ExtF2CSpecLineContinuation>)> for Op {
    fn from(value: (&ExtF2CSpecLine, Vec<&ExtF2CSpecLineContinuation>)) -> Self {
        let tokens = tokenize_extf2(value.0, value.1);
        let parser = Parser {
            state: RefCell::new(ParserState { idx: 0 }),
            input: &tokens,
        };
        let maybe_op = parse_exsr(&parser);
        match maybe_op {
            Ok(op) => op,
            Err(x) => {
                let meta = Meta::from((
                    &tokens.head.meta,
                    tokens.tail.iter().map(|t| &t.meta).collect::<Vec<&Meta>>(),
                ));
                let tokens: Vec<Token> = if env::var("DEBUG").is_ok() {
                    parser.input.iter().map(|t| t.clone()).collect::<_>()
                } else {
                    vec![]
                };
                Op::Idk {
                    meta,
                    highlights: parser.highlights(),
                    tokens,
                    error: x,
                }
            }
        }
    }
}
// using type alias means we can't elide the lifetime like we can when inlining..
// not sure if this really helps readability yet
type RawF2ResultInput<'a> = (
    &'a FieldResult<RawFactor2Field>,
    &'a [&'a FieldResult<RawFactor2Field>],
);
impl<'a> From<RawF2ResultInput<'a>> for Op {
    fn from(value: RawF2ResultInput) -> Self {
        let tokens = legacy_tokenize_extf2(value.0, value.1);
        let parser = Parser {
            state: RefCell::new(ParserState { idx: 0 }),
            input: &tokens,
        };
        let maybe_op = parse_exsr(&parser);
        match maybe_op {
            Ok(op) => op,
            Err(x) => {
                let meta = Meta::from((
                    &tokens.head.meta,
                    tokens.tail.iter().map(|t| &t.meta).collect::<Vec<&Meta>>(),
                ));
                let tokens: Vec<Token> = if env::var("DEBUG").is_ok() {
                    parser.input.iter().map(|t| t.clone()).collect::<_>()
                } else {
                    vec![]
                };
                Op::Idk {
                    meta,
                    highlights: parser.highlights(),
                    tokens,
                    error: x,
                }
            }
        }
    }
}

// DEPRECATED
impl From<&TraditionalCSpecLine> for Op {
    fn from(value: &TraditionalCSpecLine) -> Self {
        match &value.operation {
            FieldResult::Ok(opfield) => {
                if opfield.value.to_uppercase() == "BEGSR" {
                    match &value.factor1 {
                        FieldResult::Ok(f1) => Op::Begsr {
                            name: f1.value.clone(),
                            meta: f1.meta.clone(),
                            highlights: vec![],
                        },
                        FieldResult::Idk(idk) => Op::Idk {
                            meta: idk.meta.clone(),
                            error: "BAD F1".to_string(),
                            tokens: vec![],
                            highlights: vec![],
                        },
                    }
                } else if opfield.value.to_uppercase() == "ENDSR" {
                    Op::Endsr {
                        meta: opfield.meta.clone(),
                        highlights: vec![],
                    }
                } else if opfield.value.to_uppercase() == "EXSR" {
                    match &value.factor1 {
                        FieldResult::Ok(f1) => Op::Exsr {
                            name: f1.value.clone(),
                            meta: f1.meta.clone(),
                            highlights: vec![],
                        },
                        FieldResult::Idk(idk) => Op::Idk {
                            meta: idk.meta.clone(),
                            error: "BAD F1".to_string(),
                            tokens: vec![],
                            highlights: vec![],
                        },
                    }
                } else {
                    Op::Idk {
                        meta: opfield.meta.clone(),
                        error: "IGNORED OP".to_string(),
                        tokens: vec![],
                        highlights: vec![],
                    }
                }
            }
            FieldResult::Idk(opfield) => Op::Idk {
                meta: opfield.meta.clone(),
                error: "IDK OP".to_string(),
                tokens: vec![],
                highlights: vec![],
            },
        }
    }
}

type TraditionalResultFields<'a> = (
    &'a FieldResult<OperationField>,
    &'a FieldResult<Factor1Field>,
);
type TraditionalResultInput<'a> = (
    TraditionalResultFields<'a>,
    &'a [TraditionalResultFields<'a>],
);
impl<'a> From<TraditionalResultInput<'a>> for Op {
    fn from(value: TraditionalResultInput) -> Self {
        let operation = value.0 .0;
        let factor1 = value.0 .1;
        match operation {
            FieldResult::Ok(opfield) => {
                if opfield.value.to_uppercase() == "BEGSR" {
                    match factor1 {
                        FieldResult::Ok(f1) => Op::Begsr {
                            name: f1.value.clone(),
                            meta: f1.meta.clone(),
                            highlights: vec![],
                        },
                        FieldResult::Idk(idk) => Op::Idk {
                            meta: idk.meta.clone(),
                            error: "BAD F1".to_string(),
                            tokens: vec![],
                            highlights: vec![],
                        },
                    }
                } else if opfield.value.to_uppercase() == "ENDSR" {
                    Op::Endsr {
                        meta: opfield.meta.clone(),
                        highlights: vec![],
                    }
                } else if opfield.value.to_uppercase() == "EXSR" {
                    match factor1 {
                        FieldResult::Ok(f1) => Op::Exsr {
                            name: f1.value.clone(),
                            meta: f1.meta.clone(),
                            highlights: vec![],
                        },
                        FieldResult::Idk(idk) => Op::Idk {
                            meta: idk.meta.clone(),
                            error: "BAD F1".to_string(),
                            tokens: vec![],
                            highlights: vec![],
                        },
                    }
                } else {
                    Op::Idk {
                        meta: opfield.meta.clone(),
                        error: "IGNORED OP".to_string(),
                        tokens: vec![],
                        highlights: vec![],
                    }
                }
            }
            FieldResult::Idk(opfield) => Op::Idk {
                meta: opfield.meta.clone(),
                error: "IDK OP".to_string(),
                tokens: vec![],
                highlights: vec![],
            },
        }
    }
}
