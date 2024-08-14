use std::{collections::HashSet, fmt::Display};

use super::{CommentSpecLine, FSpecLine, FSpecLineContinuation, HSpecLine, IdkSpecLine};

pub enum SpecLine {
    Idk(IdkSpecLine),
    Comment(CommentSpecLine),
    HSpec(HSpecLine),
    FSpec(FSpecLine),
    FSpecContinuation(FSpecLineContinuation),
}

impl From<(usize, &[char; 100])> for SpecLine {
    fn from(value: (usize, &[char; 100])) -> Self {
        let idx = value.0;
        let chars = value.1;
        let p6 = chars[5];
        let p7 = chars[6];
        match (p6, p7) {
            (_, '*') => {
                let line = CommentSpecLine::from((idx, chars));
                SpecLine::Comment(line)
            }
            ('H', _) => {
                let line = HSpecLine::from((idx, chars));
                SpecLine::HSpec(line)
            }
            ('F', _) => {
                let unique_chars = chars[6..42].iter().collect::<HashSet<&char>>();
                if unique_chars.len() == 1 && unique_chars.contains(&' ') {
                    let line = FSpecLineContinuation::from((idx, chars));
                    SpecLine::FSpecContinuation(line)
                } else {
                    let line = FSpecLine::from((idx, chars));
                    SpecLine::FSpec(line)
                }
            }
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
            Self::HSpec(line) => write!(f, "{}", line.to_string()),
            Self::FSpec(line) => write!(f, "{}", line.to_string()),
            Self::FSpecContinuation(line) => write!(f, "{}", line.to_string()),
        }
    }
}

impl SpecLine {
    pub fn kind(&self) -> String {
        match self {
            Self::Idk(_) => "IdkSpecLine".to_string(),
            Self::Comment(_) => "CommentSpecLine".to_string(),
            Self::HSpec(_) => "HSpecLine".to_string(),
            Self::FSpec(_) => "FSpecLine".to_string(),
            Self::FSpecContinuation(_) => "FSpecLineContinuation".to_string(),
        }
    }
}
