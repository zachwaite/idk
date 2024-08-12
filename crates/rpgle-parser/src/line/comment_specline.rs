use crate::field::{CommentField, FieldResult, FormtypeField, SequenceField};
use crate::meta::pluck_array3 as pluck;
use crate::meta::Position;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentSpecLine {
    pub sequence: FieldResult<SequenceField>,
    pub form_type: FieldResult<FormtypeField>,
    pub comment: FieldResult<CommentField>,
}

impl Display for CommentSpecLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        msg.push_str(&self.sequence.to_string());
        msg.push_str(&self.form_type.to_string());
        msg.push_str(&self.comment.to_string());
        write!(f, "{}", msg)
    }
}

impl From<(usize, &[char; 100])> for CommentSpecLine {
    fn from(value: (usize, &[char; 100])) -> Self {
        let row = value.0;
        let start = Position::from((row, 0));
        let chars = value.1;
        Self {
            sequence: FieldResult::from((start, pluck::<100, 0, 5, 95>(chars))),
            form_type: FieldResult::from((start, pluck::<100, 5, 1, 94>(chars))),
            comment: FieldResult::from((start, pluck::<100, 6, 94, 0>(chars))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::Formtype;

    #[test]
    fn test_ok() {
        let row = 0usize;
        let txt = "     F**********************************************************************************************";
        let chars: [char; 100] = txt.chars().collect::<Vec<char>>().try_into().unwrap();
        let rs = CommentSpecLine::from((row, &chars));
        assert!(matches!(rs.sequence, FieldResult::Ok(SequenceField { .. })));
        if let FieldResult::Ok(fld) = rs.sequence {
            assert_eq!(fld.value, "     ".to_string())
        }
        assert!(matches!(
            rs.form_type,
            FieldResult::Ok(FormtypeField { .. })
        ));
        if let FieldResult::Ok(fld) = rs.form_type {
            assert_eq!(fld.value, Formtype::F)
        }
        assert!(matches!(rs.comment, FieldResult::Ok(CommentField { .. })));
        if let FieldResult::Ok(fld) = rs.comment {
            assert_eq!(fld.value, txt[6..].to_string());
        }
    }
}
