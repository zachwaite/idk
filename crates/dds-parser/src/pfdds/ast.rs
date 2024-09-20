use std::cell::RefCell;
use crate::line::DDSLine;
use crate::meta::{IHighlight, Span};
use crate::pfdds::{RecordFormat, Field, Keyfield, Entry, CST, FileEntry};
use crate::field::FieldResult;
use serde::{Serialize, Deserialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct AST {
    pub entries: Vec<Entry>
}

enum Phase {
    FileEntry,
    Main,
}

struct ParserState {
    idx: usize,
    phase: Phase,
}

struct Parser {
    state: RefCell<ParserState>,
    input: Vec<DDSLine>,
}
impl Parser {
    fn enter_main_phase(&self) {
        self.state.borrow_mut().phase = Phase::Main;
    }
}


fn peek_n(parser: &Parser, n: usize) -> Option<&DDSLine> {
    let idx = parser.state.borrow().idx;
    parser.input.get(idx + n)
}

fn read_line(parser: &Parser) -> &DDSLine {
    let out = peek_n(parser, 0).expect("read_line() requires a length check first");
    parser.state.borrow_mut().idx += 1;
    out
}

fn read_fileentry(parser: &Parser) -> Option<Entry> {
    // known to exist and be a ContinuationLine
    if let DDSLine::Continuation(cur) = read_line(parser) {
        let mut continuations = vec![cur];
        loop {
            if let Some(DDSLine::Continuation(peeked)) = peek_n(parser, 0) {
                let _ = read_line(parser);
                continuations.push(peeked);
            } else {
                break;
            }
        }
        return Some(Entry::FE(FileEntry::from(continuations)))
    } 
    unreachable!()
}

fn read_recordformat(parser: &Parser) -> Option<Entry> {
    // now entered main parsing phase
    parser.enter_main_phase();
    // known to exist and be a RecordFormatLine
    if let DDSLine::RecordFormat(cur) = read_line(parser) {
        let mut continuations = vec![];
        loop {
            if let Some(DDSLine::Continuation(peeked)) = peek_n(parser, 0) {
                let _ = read_line(parser);
                continuations.push(peeked);
            } else {
                break;
            }
        }
        return Some(Entry::R(RecordFormat::from((cur, continuations))))
    } 
    unreachable!()
}

fn read_field(parser: &Parser) -> Option<Entry> {
    // known to exist and be a Field
    if let DDSLine::Field(cur) = read_line(parser) {
        let mut continuations = vec![];
        loop {
            if let Some(DDSLine::Continuation(peeked)) = peek_n(parser, 0) {
                let _ = read_line(parser);
                continuations.push(peeked);
            } else {
                break;
            }
        }
        return Some(Entry::F(Field::from((cur, continuations))))
    } 
    unreachable!()
}

fn read_keyfield(parser: &Parser) -> Option<Entry> {
    // known to exist and be a Keyfield
    if let DDSLine::Key(cur) = read_line(parser) {
        let mut continuations = vec![];
        loop {
            if let Some(DDSLine::Continuation(peeked)) = peek_n(parser, 0) {
                let _ = read_line(parser);
                continuations.push(peeked);
            } else {
                break;
            }
        }
        return Some(Entry::K(Keyfield::from((cur, continuations))))
    } 
    unreachable!()
}


fn next_entry(parser: &Parser) -> Option<Entry> {
    match peek_n(parser, 0) {
        Some(line) => {
            match line {
                DDSLine::RecordFormat(_) => read_recordformat(parser),
                DDSLine::Field(_) => read_field(parser),
                DDSLine::Key(_) => read_keyfield(parser),
                DDSLine::Comment(_) => None, // ignore
                DDSLine::Idk(_) => None, // ignore
                DDSLine::Continuation(_) => {
                    let p = if let Phase::FileEntry = &parser.state.borrow().phase {
                        true
                    } else {
                        false
                    };
                    if p {
                        read_fileentry(parser)
                    } else {
                        unreachable!()
                    }
                }
            }
        }
        None => None
    }
}

impl From<&CST> for AST {
    fn from(value: &CST) -> Self {
        let cst = value;
        let state = ParserState { idx: 0, phase: Phase::FileEntry };
        let parser = Parser {
            state: RefCell::new(state),
            input: cst.lines.iter().filter(|line| match line {
                DDSLine::RecordFormat(_) => true,
                DDSLine::Field(_) => true,
                DDSLine::Key(_) => true,
                DDSLine::Continuation(_) => true,
                DDSLine::Comment(_) => false,
                DDSLine::Idk(_) => false,
            }).map(|line| line.clone()).collect::<Vec<DDSLine>>()
        };
        let mut entries = vec![];
        loop {
            match next_entry(&parser) {
                Some(entry) => {
                    entries.push(entry);
                }
                None => break,
            }
        }
        AST { entries }
    }
}

pub fn highlight_ast(ast: AST) -> Vec<((usize, usize), (usize, usize), String)> {
    ast.entries.iter().flat_map(|e| e.highlight()).map(|tup| {
        ((tup.0.start.row, tup.0.start.col), (tup.0.end.row, tup.0.end.col), tup.1)
    }).collect::<Vec<_>>()
}

pub fn query_definition(ast: &AST, pattern: &str) -> Option<Span> {
    for entry in ast.entries.iter() {
        if let Entry::F(fld) = entry {
            if let FieldResult::Ok(namefield) = &fld.name {
                if namefield.value.to_uppercase() == pattern.to_uppercase() {
                    return Some(namefield.meta.span);
                }
            }
        }
    }
    None
}
