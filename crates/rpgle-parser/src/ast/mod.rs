mod core;
mod legacy;
mod nvim;

pub use core::AST;
pub use nvim::{highlight_ast, query_definition};
