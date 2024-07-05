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
pub struct SubroutineCall {
    pub name: String,
    pub meta: StatementMeta,
}

impl fmt::Display for SubroutineCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!("SubroutineCall(name=`{}`)", self.name);
        write!(f, "{}", out)
    }
}

// domain types
#[derive(Clone)]
pub struct ExternalPgmCall {
    pub name: String,
    pub meta: StatementMeta,
}

impl fmt::Display for ExternalPgmCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!("ExtPgmCall(name=`{}`)", self.name);
        write!(f, "{}", out)
    }
}

#[derive(Clone)]
pub enum Call {
    Subroutine(SubroutineCall),
    ExternalPgm(ExternalPgmCall),
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = self.to_string();
        write!(f, "{}", out)
    }
}

impl Call {
    pub fn to_raw_text(&self) -> String {
        match self {
            Self::Subroutine(x) => x.meta.to_raw_text(),
            Self::ExternalPgm(x) => x.meta.to_raw_text(),
        }
    }
}

// domain types
#[derive(Clone)]
pub struct Mutation {
    pub keyword: String,
    pub name: String,
    pub meta: StatementMeta,
}

impl fmt::Display for Mutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!("{}(name=`{}`)", self.keyword, self.name);
        write!(f, "{}", out)
    }
}

#[derive(Clone)]
pub struct SubroutineDefinition {
    pub name: String,
    pub calls: Vec<SubroutineCall>,
    pub mutations: Vec<Mutation>,
    pub meta: StatementMeta,
}

impl fmt::Display for SubroutineDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!("SubroutineDefinition(name=`{}`)", self.name);
        write!(f, "{}", out)
    }
}

#[derive(Clone)]
pub struct ExternalPgmDefinition {
    pub name: String,
    pub meta: StatementMeta,
}

impl fmt::Display for ExternalPgmDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!("ExternalPgmDefinition(name=`{}`)", self.name);
        write!(f, "{}", out)
    }
}

#[derive(Clone)]
pub enum Definition {
    Subroutine(SubroutineDefinition),
    ExternalPgm(ExternalPgmDefinition),
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = self.to_string();
        write!(f, "{}", out)
    }
}

impl Definition {
    pub fn to_raw_text(&self) -> String {
        match self {
            Self::Subroutine(x) => x.meta.to_raw_text(),
            Self::ExternalPgm(x) => x.meta.to_raw_text(),
        }
    }
}

pub enum Statement {
    Call(SubroutineCall),
    Def(Definition),
    Mutation(Mutation),
    Idk(Idk),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Def(x) => write!(f, "{}", x.to_string()),
            Self::Call(x) => write!(f, "{}", x.to_string()),
            Self::Mutation(x) => write!(f, "{}", x.to_string()),
            Self::Idk(x) => write!(f, "{}", x.to_string()),
        }
    }
}

impl Statement {
    pub fn to_raw_text(&self) -> String {
        match self {
            Self::Def(x) => x.to_raw_text(),
            Self::Call(x) => x.meta.to_raw_text(),
            Self::Mutation(x) => x.meta.to_raw_text(),
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
