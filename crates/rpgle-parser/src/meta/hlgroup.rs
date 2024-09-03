use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Hlgroup {
    Normal,
}

impl Display for Hlgroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "{}", "Normal".to_string()),
        }
    }
}
