use core::fmt;

use pfdds_lexer::{Span, Token};

#[derive(PartialEq, Clone)]
pub struct EntryMeta {
    pub span: Span,
    pub text: String,
}

impl fmt::Display for EntryMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!(
            "EntryMeta(\n    span={},\n    text=```\n{}\n```,\n)\n",
            self.span, self.text
        );
        write!(f, "{}", s)
    }
}

impl From<Token> for EntryMeta {
    fn from(t: Token) -> Self {
        Self {
            text: t.text,
            span: t.span,
        }
    }
}

impl EntryMeta {
    pub fn empty() -> Self {
        Self {
            span: Span::empty(),
            text: String::new(),
        }
    }

    pub fn push_token(&mut self, t: Token) {
        let old_span = self.span;
        self.span = Span::to_cover_both(old_span, t.span);
        self.text.push_str(&t.text);
    }
}

pub struct Idk {
    pub meta: EntryMeta,
}

impl fmt::Display for Idk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!("Idk(\n    meta={}\n)", self.meta);
        write!(f, "{}", out)
    }
}

pub struct Comment {
    pub text: String,
    pub meta: EntryMeta,
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!(
            "Comment(\n    text=`{}`,\n    meta={},\n)",
            self.text, self.meta
        );
        write!(f, "{}", out)
    }
}

pub struct Field {}
impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "TODO")
    }
}

pub struct RecordFormat {
    name: String,
    meta: EntryMeta,
}
impl fmt::Display for RecordFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!(
            "RecordFormat(\n    name={},\n    span={},\n    text=```\n{}\n```,\n)",
            self.name, self.meta.span, self.meta.text
        );
        write!(f, "{}", out)
    }
}

pub struct Key {}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "TODO")
    }
}

pub enum DDSEntry {
    Comment(Comment),
    Field(Field),
    RecordFormat(RecordFormat),
    Key(Key),
    Idk(Idk),
}

impl fmt::Display for DDSEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Comment(x) => write!(f, "{}", x.to_string()),
            Self::Field(x) => write!(f, "{}", x.to_string()),
            Self::RecordFormat(x) => write!(f, "{}", x.to_string()),
            Self::Key(x) => write!(f, "{}", x.to_string()),
            Self::Idk(x) => write!(f, "{}", x.to_string()),
        }
    }
}

pub struct PhysicalFile {
    pub entries: Vec<DDSEntry>,
}

impl fmt::Display for PhysicalFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = self
            .entries
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", out)
    }
}
