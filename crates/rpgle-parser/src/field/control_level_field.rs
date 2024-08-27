use std::fmt::Display;

use super::idk_field::IdkField;
use super::result::FieldResult;
use crate::meta::{Meta, Position};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ControlLevel {
    Empty,
    L0,
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
    L8,
    L9,
    LR,
    SR,
    AN,
    OR,
}

impl Display for ControlLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Empty => " ".to_string(),
            Self::L0 => "L0".to_string(),
            Self::L1 => "L1".to_string(),
            Self::L2 => "L2".to_string(),
            Self::L3 => "L3".to_string(),
            Self::L4 => "L4".to_string(),
            Self::L5 => "L5".to_string(),
            Self::L6 => "L6".to_string(),
            Self::L7 => "L7".to_string(),
            Self::L8 => "L8".to_string(),
            Self::L9 => "L9".to_string(),
            Self::LR => "LR".to_string(),
            Self::SR => "SR".to_string(),
            Self::AN => "AN".to_string(),
            Self::OR => "OR".to_string(),
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlLevelField {
    pub value: ControlLevel,
    pub meta: Meta,
}

impl Display for ControlLevelField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl From<(Position, &[char; 2])> for FieldResult<ControlLevelField> {
    fn from(value: (Position, &[char; 2])) -> Self {
        let chars = value.1;
        let maybe = match chars {
            [' ', ' '] => Some(ControlLevel::Empty),
            ['L', '0'] => Some(ControlLevel::L0),
            ['L', '1'] => Some(ControlLevel::L1),
            ['L', '2'] => Some(ControlLevel::L2),
            ['L', '3'] => Some(ControlLevel::L3),
            ['L', '4'] => Some(ControlLevel::L4),
            ['L', '5'] => Some(ControlLevel::L5),
            ['L', '6'] => Some(ControlLevel::L6),
            ['L', '7'] => Some(ControlLevel::L7),
            ['L', '8'] => Some(ControlLevel::L8),
            ['L', '9'] => Some(ControlLevel::L9),
            ['L', 'R'] => Some(ControlLevel::LR),
            ['S', 'R'] => Some(ControlLevel::SR),
            ['A', 'N'] => Some(ControlLevel::AN),
            ['O', 'R'] => Some(ControlLevel::OR),
            _ => None,
        };
        if let Some(x) = maybe {
            let fld = ControlLevelField {
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
