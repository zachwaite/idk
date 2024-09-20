use crate::meta::Span;

pub trait IHighlight {
    fn highlight(&self) -> Vec<(Span, String)>;
}

pub trait ISpan {
    fn span(&self) -> Span;
}
