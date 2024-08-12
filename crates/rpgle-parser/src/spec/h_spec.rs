use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::line::{HSpecLine, HSpecLineContinuation};

#[derive(Serialize, Deserialize)]
pub struct HSpec {
    pub line: HSpecLine,
    pub continuations: Vec<HSpecLineContinuation>,
}

impl Display for HSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = self.line.to_string();
        for cont in self.continuations.iter() {
            out.push_str(&cont.to_string());
        }
        write!(f, "{}", out)
    }
}
