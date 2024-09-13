use core::fmt;

use pfdds_lexer::{Span, Token};

#[derive(Debug)]
pub enum EntryAttributeError {
    MissingRequiredAttribute(String),
}

impl EntryAttributeError {
    pub fn missing_required_attribute(txt: &str) -> Self {
        let msg = format!("Missing Required Attribute: {}", txt);
        Self::MissingRequiredAttribute(msg)
    }
}

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

    pub fn first_token(&mut self, t: Token) {
        self.span = t.span;
        self.text.push_str(&t.text);
    }

    pub fn to_raw_text(&self) -> String {
        return self.text.clone();
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

pub struct Field {
    pub name: Result<String, EntryAttributeError>,
    pub meta: EntryMeta,
}
impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match &self.name {
            Ok(n) => n.clone(),
            Err(EntryAttributeError::MissingRequiredAttribute(_)) => "???".to_string(),
        };
        let out = format!(
            "Field(\n    name=`{}`,\n    span={},\n    text=```\n{}\n```,\n)",
            name, self.meta.span, self.meta.text
        );
        write!(f, "{}", out)
    }
}

pub struct RecordFormat {
    pub name: Result<String, EntryAttributeError>,
    pub meta: EntryMeta,
}
impl fmt::Display for RecordFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match &self.name {
            Ok(n) => n.clone(),
            Err(EntryAttributeError::MissingRequiredAttribute(_)) => "???".to_string(),
        };
        let out = format!(
            "RecordFormat(\n    name=`{}`,\n    span={},\n    text=```\n{}\n```,\n)",
            name, self.meta.span, self.meta.text
        );
        write!(f, "{}", out)
    }
}

pub struct Key {
    pub name: Result<String, EntryAttributeError>,
    pub meta: EntryMeta,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match &self.name {
            Ok(n) => n.clone(),
            Err(EntryAttributeError::MissingRequiredAttribute(_)) => "???".to_string(),
        };
        let out = format!(
            "Key(\n    name=`{}`,\n    span={},\n    text=```\n{}\n```,\n)",
            name, self.meta.span, self.meta.text
        );
        write!(f, "{}", out)
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

impl DDSEntry {
    pub fn to_raw_text(&self) -> String {
        match self {
            DDSEntry::Comment(x) => x.meta.to_raw_text(),
            DDSEntry::Field(x) => x.meta.to_raw_text(),
            DDSEntry::RecordFormat(x) => x.meta.to_raw_text(),
            DDSEntry::Key(x) => x.meta.to_raw_text(),
            DDSEntry::Idk(x) => x.meta.to_raw_text(),
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

impl PhysicalFile {
    pub fn to_raw_text(&self) -> String {
        let mut out = self
            .entries
            .iter()
            .map(|e| e.to_raw_text())
            .collect::<Vec<String>>()
            .join("\n");
        out.push_str("\n");
        out
    }

    pub fn query_definition(&self, pattern: &str) -> Option<Span> {
        for entry in self.entries.iter() {
            if let DDSEntry::Field(fld) = entry {
                if let Ok(name) = &fld.name {
                    if name.as_str().to_uppercase() == pattern.to_uppercase() {
                        let out = fld.meta.span;
                        return Some(out);
                    }
                }
            }
        }
        None
    }
}
