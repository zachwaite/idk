use std::fmt::Display;

use super::idk_field::IdkField;
use super::result::{FieldBehavior, FieldResult};
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FileDesignation {
    Empty,
    P,
    S,
    R,
    T,
    F,
}

impl Display for FileDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Empty => " ".to_string(),
            Self::P => "P".to_string(),
            Self::S => "S".to_string(),
            Self::R => "R".to_string(),
            Self::T => "T".to_string(),
            Self::F => "F".to_string(),
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileDesignationField {
    pub value: FileDesignation,
    pub meta: Meta,
}

impl Display for FileDesignationField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 1])> for FieldResult<FileDesignationField> {
    fn from(value: (Position, &[char; 1])) -> Self {
        let chars = value.1;
        let maybe = match chars[0] {
            ' ' => Some(FileDesignation::Empty),
            'P' => Some(FileDesignation::P),
            'S' => Some(FileDesignation::S),
            'R' => Some(FileDesignation::R),
            'T' => Some(FileDesignation::T),
            'F' => Some(FileDesignation::F),
            _ => None,
        };
        if let Some(x) = maybe {
            let fld = FileDesignationField {
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

impl FieldBehavior for FileDesignationField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "@keyword.directive".to_string())]
    }
}
