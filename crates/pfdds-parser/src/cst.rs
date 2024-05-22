use core::fmt;

use pfdds_lexer::Span;

pub enum IllegalState {
    NotImplemented,
}

pub enum ParserException {
    UnexpectedToken,
    NotImplemented,
}

impl fmt::Display for ParserException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            Self::UnexpectedToken => format!("UnexpectedToken"),
            Self::NotImplemented => format!("NotImplemented"),
        };
        write!(f, "{}", out)
    }
}

pub struct Idk {
    pub exception: ParserException,
    pub text: String,
    pub span: Span,
}

impl fmt::Display for Idk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.exception {
            ParserException::UnexpectedToken | ParserException::NotImplemented => {
                let out = format!(
                    "Idk(\n    exception={}\n    span={}\n    text=```\n{}\n```\n)",
                    self.exception, self.span, self.text
                );
                write!(f, "{}", out)
            }
        }
    }
}

pub struct Comment {
    text: String,
    span: Span,
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!("Comment(\ntext={},\nspan={},\n)", self.text, self.span);
        write!(f, "{}", out)
    }
}

pub struct Field {}
impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "TODO")
    }
}

pub struct RecordFormat {}
impl fmt::Display for RecordFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "TODO")
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
