use crate::meta::Span;

pub trait PMixin {
    fn highlight(&self) -> Vec<(Span, String)>;
    fn span(&self) -> Span;
}

pub trait SpanBehavior {
    fn span(&self) -> Span;
}
