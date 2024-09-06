use crate::meta::Span;

pub trait PMixin {
    fn highlight(&self) -> Vec<(Span, String)>;
    fn span(&self) -> Span;
}
