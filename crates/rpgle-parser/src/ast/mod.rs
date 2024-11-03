mod ast;
mod nvim;
mod spec;
mod srcline;

pub use ast::{parse_ast, AST};
pub use nvim::{highlight_ast, query_definition};
pub use spec::Spec;
