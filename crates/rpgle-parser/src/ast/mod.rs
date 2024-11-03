mod ast;
// mod legacy;
mod nvim;
mod spec;
mod srcline;

pub use ast::{specs_from_cst, AST};
pub use nvim::{highlight_ast, query_definition};
pub use spec::Spec;
