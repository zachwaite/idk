use std::{collections::HashSet, fmt::Display};

use super::{
    CSpecLine, CommentSpecLine, DSpecLine, DSpecLineContinuation, ExtF2CSpecLine, FSpecLine,
    FSpecLineContinuation, FreeCSpecLine, HSpecLine, IdkSpecLine, TraditionalCSpecLine,
};
use crate::field::{has_extf2_optoken, Field};

pub enum SpecLine {
    Idk(IdkSpecLine),
    Comment(CommentSpecLine),
    HSpec(HSpecLine),
    FSpec(FSpecLine),
    FSpecContinuation(FSpecLineContinuation),
    DSpec(DSpecLine),
    DSpecContinuation(DSpecLineContinuation),
    CSpec(CSpecLine),
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
            ('D', _) => {
                let unique_chars = chars[6..42].iter().collect::<HashSet<&char>>();
                if unique_chars.len() == 1 && unique_chars.contains(&' ') {
                    let line = DSpecLineContinuation::from((idx, chars));
                    SpecLine::DSpecContinuation(line)
                } else {
                    let line = DSpecLine::from((idx, chars));
                    SpecLine::DSpec(line)
                }
            }
            ('C', _) => {
                let line = if has_extf2_optoken(chars) {
                    let extf2 = ExtF2CSpecLine::from((idx, chars));
                    CSpecLine::ExtF2(extf2)
                } else {
                    let traditional = TraditionalCSpecLine::from((idx, chars));
                    CSpecLine::Traditional(traditional)
                };
                SpecLine::CSpec(line)
            }
            (' ', ' ') => {
                let free = FreeCSpecLine::from((idx, chars));
                let line = CSpecLine::Free(free);
                SpecLine::CSpec(line)
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
            Self::DSpec(line) => write!(f, "{}", line.to_string()),
            Self::DSpecContinuation(line) => write!(f, "{}", line.to_string()),
            Self::CSpec(line) => write!(f, "{}", line.to_string()),
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
            Self::DSpec(_) => "DSpecLine".to_string(),
            Self::DSpecContinuation(_) => "DSpecLineContinuation".to_string(),
            Self::CSpec(_) => "CSpecLine".to_string(),
        }
    }
}
impl Field for SpecLine {
    fn highlight(&self) -> Vec<(crate::Span, String)> {
        match self {
            SpecLine::Idk(line) => line.highlight(),
            SpecLine::Comment(line) => line.highlight(),
            SpecLine::HSpec(line) => line.highlight(),
            SpecLine::FSpec(line) => line.highlight(),
            SpecLine::FSpecContinuation(line) => line.highlight(),
            SpecLine::DSpec(line) => line.highlight(),
            SpecLine::DSpecContinuation(line) => line.highlight(),
            SpecLine::CSpec(line) => line.highlight(),
        }
    }

    fn span(&self) -> crate::Span {
        match self {
            SpecLine::Idk(line) => line.span(),
            SpecLine::Comment(line) => line.span(),
            SpecLine::HSpec(line) => line.span(),
            SpecLine::FSpec(line) => line.span(),
            SpecLine::FSpecContinuation(line) => line.span(),
            SpecLine::DSpec(line) => line.span(),
            SpecLine::DSpecContinuation(line) => line.span(),
            SpecLine::CSpec(line) => line.span(),
        }
    }
}
