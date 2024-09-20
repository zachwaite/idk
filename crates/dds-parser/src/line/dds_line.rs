use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use crate::meta::{IHighlight, ISpan, Span};
use super::{RecordFormatLine, FieldLine, KeyLine, ContinuationLine, CommentLine, IdkLine};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DDSLine {
    RecordFormat(RecordFormatLine),
    Field(FieldLine),
    Key(KeyLine),
    Continuation(ContinuationLine),
    Comment(CommentLine),
    Idk(IdkLine),
}
impl From<(usize, &[char; 80])> for DDSLine {
    fn from(value: (usize, &[char; 80])) -> Self {
        let idx = value.0;
        let chars = value.1;
        let p7 = chars[6];
        let p17 = chars[16];
        match (p7, p17) {
            ('*', _) => DDSLine::Comment(CommentLine::from((idx, chars))),
            (_, 'R') => DDSLine::RecordFormat(RecordFormatLine::from((idx, chars))),
            _ => {
                if chars[18].is_alphabetic() {
                    DDSLine::Field(FieldLine::from((idx, chars)))
                } else {
                    let unique_chars = chars[44..].iter().collect::<HashSet<&char>>();
                    if unique_chars.len() == 1 && unique_chars.contains(&' ') {
                        DDSLine::Idk(IdkLine::from((idx, chars)))
                    } else {
                        DDSLine::Continuation(ContinuationLine::from((idx, chars)))
                    }
                }
            }
        }
    }
}
impl Display for DDSLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DDSLine::RecordFormat(line) => write!(f, "{}", line.to_string()),
            DDSLine::Field(line) => write!(f, "{}", line.to_string()),
            DDSLine::Key(line) => write!(f, "{}", line.to_string()),
            DDSLine::Continuation(line) => write!(f, "{}", line.to_string()),
            DDSLine::Comment(line) => write!(f, "{}", line.to_string()),
            DDSLine::Idk(line) => write!(f, "{}", line.to_string()),
        }
    }
}
impl IHighlight for DDSLine {
    fn highlight(&self) -> Vec<(Span, String)> {
        match self {
            DDSLine::RecordFormat(line) => line.highlight(),
            DDSLine::Field(line) => line.highlight(),
            DDSLine::Key(line) => line.highlight(),
            DDSLine::Continuation(line) => line.highlight(),
            DDSLine::Comment(line) => line.highlight(),
            DDSLine::Idk(line) => line.highlight(),
        }
    }
}
impl ISpan for DDSLine {
    fn span(&self) -> Span {
        match self {
            DDSLine::RecordFormat(line) => line.span(),
            DDSLine::Field(line) => line.span(),
            DDSLine::Key(line) => line.span(),
            DDSLine::Continuation(line) => line.span(),
            DDSLine::Comment(line) => line.span(),
            DDSLine::Idk(line) => line.span(),
        }
    }
}
