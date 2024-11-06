use std::fmt::Display;

use super::idk_field::IdkField;
use crate::meta::{PMixin, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FieldResult<T> {
    Ok(T),
    Idk(IdkField),
}

impl<T> Display for FieldResult<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok(fld) => write!(f, "{}", fld.to_string()),
            Self::Idk(fld) => write!(f, "{}", fld.to_string()),
        }
    }
}

impl<T> PMixin for FieldResult<T>
where
    T: PMixin,
{
    fn highlight(&self) -> Vec<(Span, String)> {
        match self {
            FieldResult::Ok(fld) => fld.highlight(),
            FieldResult::Idk(fld) => fld.highlight(),
        }
    }

    fn span(&self) -> Span {
        match self {
            FieldResult::Ok(fld) => fld.span(),
            FieldResult::Idk(fld) => fld.span(),
        }
    }
}

impl<T> FieldResult<T> {
    pub fn is_ok(&self) -> bool {
        match self {
            Self::Ok(_) => true,
            Self::Idk(_) => false,
        }
    }

    pub fn is_err(&self) -> bool {
        match self {
            Self::Ok(_) => false,
            Self::Idk(_) => true,
        }
    }

    pub fn try_as(&self) -> Option<&T> {
        match self {
            Self::Ok(x) => Some(x),
            Self::Idk(_) => None,
        }
    }
}
