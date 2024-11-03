use std::fmt::Display;

use super::idk_field::IdkField;
use super::result::FieldResult;
use crate::meta::{Meta, PMixin, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DefinitionType {
    Empty,
    C,
    DS,
    PR,
    PI,
    S,
}
impl DefinitionType {
    pub fn is_pr(&self) -> bool {
        if let Self::PR = self {
            return true;
        } else {
            return false;
        }
    }
}

impl Display for DefinitionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Empty => " ".to_string(),
            Self::C => "C".to_string(),
            Self::DS => "DS".to_string(),
            Self::PR => "PR".to_string(),
            Self::PI => "PI".to_string(),
            Self::S => "S".to_string(),
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DefinitionTypeField {
    pub value: DefinitionType,
    pub meta: Meta,
}

impl Display for DefinitionTypeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 2])> for FieldResult<DefinitionTypeField> {
    fn from(value: (Position, &[char; 2])) -> Self {
        let chars = value.1;
        let maybe = match chars.iter().collect::<String>().to_uppercase().as_str() {
            "  " => Some(DefinitionType::Empty),
            "C " => Some(DefinitionType::C),
            "DS" => Some(DefinitionType::DS),
            "PR" => Some(DefinitionType::PR),
            "PI" => Some(DefinitionType::PI),
            "SI" => Some(DefinitionType::S),
            _ => None,
        };
        if let Some(x) = maybe {
            let fld = DefinitionTypeField {
                value: x,
                meta: Meta::from((value.0, chars.as_slice())),
            };
            Self::Ok(fld)
        } else {
            let fld = IdkField::from((value.0, chars.as_slice()));
            Self::Idk(fld)
        }
    }
}

impl PMixin for DefinitionTypeField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "@type.definition".to_string())]
    }
}
