use std::fmt::Display;

use super::{CommentSpecLine, IdkSpecLine};

pub enum SpecLine {
    Idk(IdkSpecLine),
    Comment(CommentSpecLine),
}

impl From<(usize, &[char; 100])> for SpecLine {
    fn from(value: (usize, &[char; 100])) -> Self {
        let idx = value.0;
        let chars = value.1;
        let p6 = chars[6];
        match p6 {
            _ => {
                let line = IdkSpecLine::from((idx, chars));
                SpecLine::Idk(line)
            }
        }
    }
}

impl Display for SpecLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Idk(line) => write!(f, "{}", line.to_string()),
            Self::Comment(line) => write!(f, "{}", line.to_string()),
        }
    }
}

impl SpecLine {
    pub fn kind(&self) -> String {
        match self {
            Self::Idk(_) => "IdkSpecLine".to_string(),
            Self::Comment(_) => "CommentSpecLine".to_string(),
        }
    }
}
