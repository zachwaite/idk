use std::fmt::Display;

use super::idk_field::IdkField;
use crate::meta::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub trait PMixin {
    fn highlight(&self) -> Vec<(Span, String)>;
    fn span(&self) -> Span;
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
