use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::line::{DSpecLine, DSpecLineContinuation};

#[derive(Serialize, Deserialize)]
pub struct DSpec {
    pub line: DSpecLine,
    pub continuations: Vec<DSpecLineContinuation>,
}

impl Display for DSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut outs = vec![self.line.to_string()];
        for cont in self.continuations.iter() {
            outs.push(cont.to_string());
        }
        let out = outs.join("\n");
        write!(f, "{}", out)
    }
}
