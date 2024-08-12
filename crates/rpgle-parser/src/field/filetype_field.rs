use std::fmt::Display;

use super::idk_field::IdkField;
use super::result::FieldResult;
use crate::meta::{Meta, Position};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Filetype {
    Empty,
    I,
    O,
    U,
    C,
}

impl Display for Filetype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Empty => " ".to_string(),
            Self::I => "I".to_string(),
            Self::O => "O".to_string(),
            Self::U => "U".to_string(),
            Self::C => "C".to_string(),
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiletypeField {
    pub value: Filetype,
    pub meta: Meta,
}

impl Display for FiletypeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 1])> for FieldResult<FiletypeField> {
    fn from(value: (Position, &[char; 1])) -> Self {
        let chars = value.1;
        let formtype = match chars[0] {
            ' ' => Some(Filetype::Empty),
            'I' => Some(Filetype::I),
            'O' => Some(Filetype::O),
            'U' => Some(Filetype::U),
            'C' => Some(Filetype::C),
            _ => None,
        };
        if let Some(ft) = formtype {
            let fld = FiletypeField {
                value: ft,
                meta: Meta::from((value.0, chars.as_slice())),
            };
            Self::Ok(fld)
        } else {
            let fld = IdkField::from((value.0, chars.as_slice()));
            Self::Idk(fld)
        }
    }
}
