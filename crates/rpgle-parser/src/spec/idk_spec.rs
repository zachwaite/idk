use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::line::IdkSpecLine;

#[derive(Serialize, Deserialize)]
pub struct IdkSpec {
    pub line: IdkSpecLine,
}

impl Display for IdkSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.line.to_string();
        write!(f, "{}", out)
    }
}
