use std::fmt::Display;

use super::idk_field::IdkField;
use super::result::{Field, FieldResult};
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Device {
    PRINTER,
    DISK,
    WORKSTN,
    SPECIAL,
    SEQ,
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::PRINTER => "PRINTER".to_string(),
            Self::DISK => "DISK".to_string(),
            Self::WORKSTN => "WORKSTN".to_string(),
            Self::SPECIAL => "SPECIAL".to_string(),
            Self::SEQ => "SEQ".to_string(),
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceField {
    pub value: Device,
    pub meta: Meta,
}

impl Display for DeviceField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 7])> for FieldResult<DeviceField> {
    fn from(value: (Position, &[char; 7])) -> Self {
        let chars = value.1;
        let txt = chars.iter().filter(|c| **c != ' ').collect::<String>();
        let maybe = match txt.as_str() {
            "PRINTER" => Some(Device::PRINTER),
            "DISK" => Some(Device::DISK),
            "WORKSTN" => Some(Device::WORKSTN),
            "SPECIAL" => Some(Device::SPECIAL),
            "SEQ" => Some(Device::SEQ),
            _ => None,
        };
        if let Some(x) = maybe {
            let fld = DeviceField {
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

impl Field for DeviceField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "@keyword.storage".to_string())]
    }
}
