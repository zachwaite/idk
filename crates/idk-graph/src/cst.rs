use core::fmt;
use rpgle_lexer::{Span, Token};

#[derive(PartialEq, Clone)]
pub struct StatementMeta {
    pub span: Span,
    pub text: String,
}

impl fmt::Display for StatementMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!(
            "EntryMeta(\n    span={},\n    text=```\n{}\n```,\n)\n",
            self.span, self.text
        );
        write!(f, "{}", s)
    }
}

impl From<Token> for StatementMeta {
    fn from(t: Token) -> Self {
        Self {
            text: t.text,
            span: t.span,
        }
    }
}

impl StatementMeta {
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

    pub fn push_other(&mut self, other: &StatementMeta) {
        let old_span = self.span;
        self.span = Span::to_cover_both(old_span, other.span);
        self.text.push_str(&other.text);
    }

    pub fn to_raw_text(&self) -> String {
        return self.text.clone();
    }
}

pub struct Idk {
    pub meta: StatementMeta,
}

impl fmt::Display for Idk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!("Idk(...)");
        write!(f, "{}", out)
    }
}

// domain types
#[derive(Clone)]
pub struct Call {
    pub name: String,
    pub meta: StatementMeta,
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!("SubroutineCall(name=`{}`)", self.name);
        write!(f, "{}", out)
    }
}

#[derive(Clone)]
pub struct Definition {
    pub name: String,
    pub calls: Vec<Call>,
    pub meta: StatementMeta,
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!("SubroutineDefinition(name=`{}`)", self.name);
        write!(f, "{}", out)
    }
}

pub enum Statement {
    Call(Call),
    Def(Definition),
    Idk(Idk),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Def(x) => write!(f, "{}", x.to_string()),
            Self::Call(x) => write!(f, "{}", x.to_string()),
            Self::Idk(x) => write!(f, "{}", x.to_string()),
        }
    }
}

impl Statement {
    pub fn to_raw_text(&self) -> String {
        match self {
            Self::Def(x) => x.meta.to_raw_text(),
            Self::Call(x) => x.meta.to_raw_text(),
            Self::Idk(x) => x.meta.to_raw_text(),
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = self
            .statements
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", out)
    }
}

impl Program {
    pub fn new() -> Self {
        Self { statements: vec![] }
    }

    pub fn to_raw_text(&self) -> String {
        let mut out = self
            .statements
            .iter()
            .map(|x| x.to_raw_text())
            .collect::<Vec<String>>()
            .join("");
        out.push_str("\n");
        out
    }
}