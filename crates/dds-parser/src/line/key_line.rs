use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use crate::meta::{IHighlight, ISpan, Span, pos};
use crate::field::{DatatypeField, FieldResult, FormtypeField, IgnoredField, LengthField, NameField, NametypeField, ReferenceField, SequenceField,
 DecimalPositionsField, UsageField, RawKeywordsField,
};
use crate::meta::pluck_array3 as pluck;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyLine {
    pub sequence: FieldResult<SequenceField>,
    pub form_type: FieldResult<FormtypeField>,
    pub comment: FieldResult<IgnoredField>,
    pub condition: FieldResult<IgnoredField>,
    pub name_type: FieldResult<NametypeField>,
    pub reserved: FieldResult<IgnoredField>,
    pub name: FieldResult<NameField>,
    pub reference: FieldResult<ReferenceField>,
    pub length: FieldResult<LengthField>,
    pub data_type: FieldResult<DatatypeField>,
    pub decimal_positions: FieldResult<DecimalPositionsField>,
    pub usage: FieldResult<UsageField>,
    pub location: FieldResult<IgnoredField>,
    pub keywords: FieldResult<RawKeywordsField>

}
impl From<(usize, &[char; 80])> for KeyLine {
    fn from(value: (usize, &[char; 80])) -> Self {
        let row = value.0;
        let chars = value.1;
        Self {
            sequence: FieldResult::from((pos(row, 0), pluck::<80, 0, 5, 75>(chars))),
            form_type: FieldResult::from((pos(row, 5), pluck::<80, 5, 1, 74>(chars))),
            comment: FieldResult::from((pos(row, 6), pluck::<80, 6, 1, 73>(chars))),
            condition: FieldResult::from((pos(row, 7), pluck::<80, 7, 9, 64>(chars))),
            name_type: FieldResult::from((pos(row, 16), pluck::<80, 16, 1, 63>(chars))),
            reserved: FieldResult::from((pos(row, 17), pluck::<80, 17, 1, 62>(chars))),
            name: FieldResult::from((pos(row, 18), pluck::<80, 18, 10, 52>(chars))),
            reference: FieldResult::from((pos(row, 28), pluck::<80, 28, 1, 51>(chars))),
            length: FieldResult::from((pos(row, 29), pluck::<80, 29, 5, 46>(chars))),
            data_type: FieldResult::from((pos(row, 34), pluck::<80, 34, 1, 45>(chars))),
            decimal_positions: FieldResult::from((pos(row, 35), pluck::<80, 35, 2, 43>(chars))),
            usage: FieldResult::from((pos(row, 37), pluck::<80, 37, 1, 42>(chars))),
            location: FieldResult::from((pos(row, 38), pluck::<80, 38, 6, 36>(chars))),
            keywords: FieldResult::from((pos(row, 44), pluck::<80, 44, 36, 0>(chars))),
        }
    }
}
impl Display for KeyLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", [
          self.sequence.to_string(),
          self.form_type.to_string(),
          self.comment.to_string(),
          self.condition.to_string(),
          self.name_type.to_string(),
          self.reserved.to_string(),
          self.name.to_string(),
          self.reference.to_string(),
          self.length.to_string(),
          self.data_type.to_string(),
          self.decimal_positions.to_string(),
          self.usage.to_string(),
          self.location.to_string(),
          self.keywords.to_string(),
        ].concat())
    }
}
impl IHighlight for KeyLine {
    fn highlight(&self) -> Vec<(Span, String)> {
        let mut out = vec![];
        out.append(&mut self.sequence.highlight());
        out.append(&mut self.form_type.highlight());
        out.append(&mut self.comment.highlight());
        out.append(&mut self.condition.highlight());
        out.append(&mut self.name_type.highlight());
        out.append(&mut self.reserved.highlight());
        out.append(&mut self.name.highlight());
        out.append(&mut self.reference.highlight());
        out.append(&mut self.length.highlight());
        out.append(&mut self.data_type.highlight());
        out.append(&mut self.decimal_positions.highlight());
        out.append(&mut self.usage.highlight());
        out.append(&mut self.location.highlight());
        out.append(&mut self.keywords.highlight());
        out
    }
}
impl ISpan for KeyLine {
    fn span(&self) -> Span {
        Span::from((
            self.sequence.span(),
            self.keywords.span(),
        ))
    }
}

