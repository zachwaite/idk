use serde::{Deserialize, Serialize};
use crate::meta::{IHighlight, Span};
use crate::field::{DatatypeField, FieldResult, FormtypeField, IgnoredField, LengthField, NameField, NametypeField, ReferenceField, SequenceField,
 DecimalPositionsField, UsageField, RFKeywordsField,
};
use crate::free::tokenize_rf_kw;
use crate::line::{RecordFormatLine, ContinuationLine};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordFormat {
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
    pub keywords: FieldResult<RFKeywordsField>
}
impl From<(&RecordFormatLine, Vec<&ContinuationLine>)> for RecordFormat {
    fn from(value: (&RecordFormatLine, Vec<&ContinuationLine>)) -> Self {
        let line = value.0;
        let continuations = value.1;
        let tokens = tokenize_rf_kw(line, continuations);
        let kwfield = RFKeywordsField { tokens };

        Self {
            sequence: line.sequence.clone(),
            form_type: line.form_type.clone(),
            comment: line.comment.clone(),
            condition: line.condition.clone(),
            name_type: line.name_type.clone(),
            reserved: line.reserved.clone(),
            name: line.name.clone(),
            reference: line.reference.clone(),
            length: line.length.clone(),
            data_type: line.data_type.clone(),
            decimal_positions: line.decimal_positions.clone(),
            usage: line.usage.clone(),
            location: line.location.clone(),
            keywords: FieldResult::Ok(kwfield),
        }
    }
}
impl IHighlight for RecordFormat {
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
