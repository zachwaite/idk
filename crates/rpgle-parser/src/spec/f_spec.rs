use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::line::{FSpecLine, FSpecLineContinuation};

#[derive(Serialize, Deserialize)]
pub struct FSpec {
    pub line: FSpecLine,
    pub continuations: Vec<FSpecLineContinuation>,
}

impl Display for FSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = self.line.to_string();
        for cont in self.continuations.iter() {
            out.push_str(&cont.to_string());
        }
        write!(f, "{}", out)
    }
}
