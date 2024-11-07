use std::fmt::Display;

use super::result::{FieldBehavior, FieldResult};
use super::IdkField;
use crate::meta::{Meta, Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommentField {
    pub value: String,
    pub meta: Meta,
}

impl Display for CommentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = &self.meta.text;
        write!(f, "{}", out)
    }
}

impl FieldBehavior for CommentField {
    fn span(&self) -> Span {
        self.meta.span
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        vec![(self.span(), "@comment".to_string())]
    }
}

// for line comment
impl From<(Position, &[char; 94])> for FieldResult<CommentField> {
    fn from(value: (Position, &[char; 94])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        match chars[0] {
            '*' => {
                let value = chars.iter().collect::<String>();
                Self::Ok(CommentField { value, meta })
            }
            _ => {
                let fld = IdkField::from((value.0, chars.as_slice()));
                Self::Idk(fld)
            }
        }
    }
}

// for cspec inline comment
impl From<(Position, &[char; 26])> for FieldResult<CommentField> {
    fn from(value: (Position, &[char; 26])) -> Self {
        let chars = value.1;
        let meta = Meta::from((value.0, chars.as_slice()));
        match chars[0] {
            '*' => {
                let value = chars.iter().collect::<String>();
                Self::Ok(CommentField { value, meta })
            }
            _ => {
                let fld = IdkField::from((value.0, chars.as_slice()));
                Self::Idk(fld)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let p = Position::empty();
        let chars: [char; 94] = std::iter::repeat('*')
            .take(94)
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        let rs: FieldResult<CommentField> = FieldResult::from((p, &chars));
        assert!(matches!(rs, FieldResult::Ok(CommentField { .. })));
        if let FieldResult::Ok(fld) = rs {
            assert_eq!(fld.value, chars.iter().collect::<String>());
        }
    }

    #[test]
    fn test_idk() {
        let p = Position::empty();
        let chars: [char; 94] = std::iter::repeat('?')
            .take(94)
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        let rs: FieldResult<CommentField> = FieldResult::from((p, &chars));
        assert!(matches!(rs, FieldResult::Idk(IdkField { .. })));
        if let FieldResult::Idk(fld) = rs {
            assert_eq!(fld.value, chars.iter().collect::<String>());
        }
    }
}
