use crate::field::{CommentField, FieldResult, FormtypeField, PMixin, SequenceField};
use crate::meta::pluck_array3 as pluck;
use crate::meta::{Position, Span};
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
        let chars = value.1;
        Self {
            sequence: FieldResult::from((Position::from((row, 0)), pluck::<100, 0, 5, 95>(chars))),
            form_type: FieldResult::from((Position::from((row, 5)), pluck::<100, 5, 1, 94>(chars))),
            comment: FieldResult::from((Position::from((row, 6)), pluck::<100, 6, 94, 0>(chars))),
        }
    }
}

impl PMixin for CommentSpecLine {
    fn span(&self) -> Span {
        let start = self.sequence.span();
        let end = self.comment.span();
        Span::from((start, end))
    }

    fn highlight(&self) -> Vec<(Span, String)> {
        let mut out = vec![];
        out.append(&mut self.sequence.highlight());
        out.append(&mut self.form_type.highlight());
        out.append(&mut self.comment.highlight());
        out
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
