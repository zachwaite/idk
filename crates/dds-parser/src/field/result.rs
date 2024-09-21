use std::fmt::Display;

use super::idk_field::IdkField;
use crate::meta::{IHighlight, ISpan, Span};
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

impl<T> IHighlight for FieldResult<T>
where
    T: IHighlight,
{
    fn highlight(&self) -> Vec<(Span, String)> {
        match self {
            FieldResult::Ok(fld) => fld.highlight(),
            FieldResult::Idk(fld) => fld.highlight(),
        }
    }
}

impl<T> ISpan for FieldResult<T>
where
    T: ISpan,
{
    fn span(&self) -> Span {
        match self {
            FieldResult::Ok(fld) => fld.span(),
            FieldResult::Idk(fld) => fld.span(),
        }
    }
}