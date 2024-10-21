use crate::spec::Spec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AST {
    pub specs: Vec<Spec>,
}
