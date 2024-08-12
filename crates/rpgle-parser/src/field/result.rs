use std::fmt::Display;

use super::idk_field::IdkField;
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
